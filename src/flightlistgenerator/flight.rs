use super::airport::AirportList;
use super::flightlist::AircraftDataBase;
use super::flighttime::FlightDate;
use super::flighttime::FlightTime;
use super::flighttime::TimeOfDate;
use super::utils::mandatory_string;
use super::utils::optional_bool;
use crate::flightlistgenerator::utils::mandatory_datetime;
use crate::flightlistgenerator::utils::optional_duration;
use crate::flightlistgenerator::utils::optional_u8;
use crate::flightlistgenerator::validator::validate_date;
use crate::flightlistgenerator::validator::validate_duration;
use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use strict_yaml_rust::StrictYaml;
use time::Date;
//
//
// todo comment
// todo check unknown keys in yaml
/// Struct containing all informations about one flight.
/// the fields are fetched from the yaml file.
///
/// Mandatory fields :
/// date_start
/// date_end
/// todo...
#[derive(Debug, Clone)]
pub struct Flight {
    pub date: FlightDate,
    pub time_departure: TimeOfDate,
    pub airport_departure: String,
    pub time_arrival: TimeOfDate,
    pub airport_arrival: String,
    pub acmodel: String, // Aircraft model
    pub immatriculation: String,
    pub single_pilot_time_se: bool,
    pub single_pilot_time_me: bool,
    pub multi_pilot_time: FlightTime,
    pub total_flight_time: FlightTime,
    pub name_pic: String, // Pilot in Command

    pub takeoff_day: u8,
    pub takeoff_night: u8,
    pub landing_day: u8,
    pub landing_night: u8,

    pub operational_condition_time_ifr: FlightTime,
    pub operational_condition_time_night: FlightTime,
    pub pilot_in_command_time: FlightTime,
    pub copilot_time: FlightTime,
    pub dual_time: FlightTime,
    pub instructor_time: FlightTime,

    pub is_sim: bool,
    pub sim_date: Date,
    pub sim_type: String,
    pub sim_total_time_of_session: FlightTime,
    pub remark: String,

    pub end_of_page: bool,
    pub end_of_book: bool,
}

impl Flight {
    /// extract the fields of a yaml element into a flight
    pub fn from_yaml(
        yaml: &StrictYaml,
        aircraft_database: &mut AircraftDataBase,
        airport_list: &mut AirportList,
    ) -> Result<Flight> {
        let is_sim = optional_bool(yaml, "is_sim")?;
        if is_sim == Some(true) {
            Flight::simulator_from_yaml(yaml)
        } else {
            Flight::flight_from_yaml(yaml, aircraft_database, airport_list)
        }
    }

    fn flight_from_yaml(
        yaml: &StrictYaml,
        aircraft_database: &mut AircraftDataBase,
        airport_list: &mut AirportList,
    ) -> Result<Flight> {
        // [date_start]
        let date_start = mandatory_datetime(yaml, "date_start")?;

        // [apt_departure_iata]
        let airport_departure = airport_list
            .add(
                &yaml["apt_departure_iata"],
                &yaml["apt_departure_icao"],
                &yaml["apt_departure_name"],
            )
            .context("Problem with airport departure")?;

        // [date_end]
        let date_end = mandatory_datetime(yaml, "date_end")?;

        let date = FlightDate(date_start.date());
        let time_departure = TimeOfDate(date_start.time());
        let time_arrival = TimeOfDate(date_end.time());
        // Quick date validation
        validate_date(&date_start, &date_end)
            .context("validation : fields [date_start] and [date_end]")?;

        // [apt_arrival_???]
        let airport_arrival = airport_list
            .add(
                &yaml["apt_arrival_iata"],
                &yaml["apt_arrival_icao"],
                &yaml["apt_arrival_name"],
            )
            .context("Problem with airport arrival")?;

        // acmodel immatriculation
        let (immatriculation, acmodel) = aircraft_database
            .add_flight(yaml)
            .context("Function AircraftDatabse.add_flight returned an error")?;
        let immatriculation = immatriculation.to_string();
        let acmodel = acmodel.to_string();

        // [single pilot time se]
        let single_pilot_time_se = aircraft_database.is_single_pilot_se(&immatriculation)?;
        // [single pilot time me]
        let single_pilot_time_me = aircraft_database.is_single_pilot_me(&immatriculation)?;

        // [duration_total]
        let total_flight_time = match optional_duration(yaml, "duration_total")? {
            Some(duration_total) => {
                validate_duration(duration_total, &date_start, &date_end)
                    .context("validation : field [duration_total]")?;
                FlightTime(duration_total)
            }
            None => FlightTime(date_end - date_start),
        };

        // automatically
        let multi_pilot_time = if aircraft_database.is_multi_pilot(&immatriculation)? {
            total_flight_time
        } else {
            FlightTime::ZERO
        };

        // [pic]
        let name_pic = mandatory_string(yaml, "pic")?.to_string();
        //.context("field [pic] missing or incorrect.")?
        //.to_string();

        // [takeoff_day]
        let takeoff_day = optional_u8(yaml, "takeoff_day")?.unwrap_or(0);
        // [takeoff_night]
        let takeoff_night = optional_u8(yaml, "takeoff_night")?.unwrap_or(0);
        // [landing_day]
        let landing_day = optional_u8(yaml, "landing_day")?.unwrap_or(0);
        // [landing_night]
        let landing_night = optional_u8(yaml, "landing_night")?.unwrap_or(0);

        // [oc_time_ifr]
        // todo perform some validation
        let operational_condition_time_ifr = match optional_duration(yaml, "oc_time_ifr")? {
            Some(oc_time_ifr) => FlightTime(oc_time_ifr),
            None => FlightTime::ZERO,
        };

        // [oc_time_night]
        let operational_condition_time_night = match optional_duration(yaml, "oc_time_night")? {
            Some(oc_time_night) => FlightTime(oc_time_night),
            None => FlightTime::ZERO,
        };

        // [duration_pic]
        let pilot_in_command_time = match optional_duration(yaml, "duration_pic")? {
            Some(duration_pic) => {
                validate_duration(duration_pic, &date_start, &date_end)
                    .context("validation : field [duration_pic]")?;
                FlightTime(duration_pic)
            }
            None => FlightTime::ZERO,
        };

        // No field, automatically added
        let copilot_time = if aircraft_database.is_multi_pilot(&immatriculation)?
            && pilot_in_command_time == FlightTime::ZERO
        {
            total_flight_time
        } else {
            FlightTime::ZERO
        };

        // [dual_time]
        let dual_time = match optional_duration(yaml, "dual_time")? {
            Some(dual_time) => FlightTime(dual_time),
            None => FlightTime::ZERO,
        };

        // [instructor_time]
        let instructor_time = match optional_duration(yaml, "instructor_time")? {
            Some(instructor_time) => FlightTime(instructor_time),
            None => FlightTime::ZERO,
        };

        // [remark]
        let remark = match &yaml["comment"] {
            StrictYaml::String(s) => s.to_string(),
            StrictYaml::BadValue => "".to_string(),
            _ => {
                bail!(format!("bad comment value"));
            }
        };
        let end_of_page = optional_bool(yaml, "end_of_page")?.unwrap_or(false);
        let end_of_book = optional_bool(yaml, "end_of_book")?.unwrap_or(false);

        // Validation :
        //
        let flight = Flight {
            date,
            time_departure,
            airport_departure,
            time_arrival,
            airport_arrival,
            acmodel,
            immatriculation,
            single_pilot_time_se,
            single_pilot_time_me,
            multi_pilot_time,
            total_flight_time,
            name_pic,

            takeoff_day,
            takeoff_night,
            landing_day,
            landing_night,

            operational_condition_time_ifr,
            operational_condition_time_night,
            pilot_in_command_time,
            copilot_time,
            dual_time,
            instructor_time,

            is_sim: false,
            sim_date: date_start.date(),
            sim_type: "".to_string(),
            sim_total_time_of_session: FlightTime::ZERO,

            remark,

            end_of_page,
            end_of_book,
        };
        Ok(flight)
    }

    // todo add field remark
    fn simulator_from_yaml(yaml: &StrictYaml) -> Result<Flight> {
        let sim_datetime = mandatory_datetime(yaml, "sim_date")?;
        let sim_date = sim_datetime.date();
        let sim_type = mandatory_string(yaml, "sim_type")?.to_string();
        let tmp_sim_total_time_of_session = optional_duration(yaml, "sim_total_time")?
            .context("[sim_total_time] missing or other problem.")?;
        let sim_total_time_of_session = FlightTime(tmp_sim_total_time_of_session);
        // [remark]
        let remark = match &yaml["comment"] {
            StrictYaml::String(s) => s.to_string(),
            StrictYaml::BadValue => "".to_string(),
            _ => {
                bail!(format!("bad comment value"));
            }
        };
        let end_of_page = optional_bool(yaml, "end_of_page")?.unwrap_or(false);
        let end_of_book = optional_bool(yaml, "end_of_book")?.unwrap_or(false);

        let flight = Flight {
            date: FlightDate(sim_date),
            time_departure: TimeOfDate(sim_datetime.time()),
            airport_departure: "".to_string(),
            time_arrival: TimeOfDate((sim_datetime + tmp_sim_total_time_of_session).time()),
            airport_arrival: "".to_string(),
            acmodel: "".to_string(),
            immatriculation: "".to_string(),
            single_pilot_time_se: false,
            single_pilot_time_me: false,
            multi_pilot_time: FlightTime::ZERO,
            total_flight_time: FlightTime::ZERO,
            name_pic: "".to_string(),
            takeoff_day: 0,
            takeoff_night: 0,
            landing_day: 0,
            landing_night: 0,
            operational_condition_time_ifr: FlightTime::ZERO,
            operational_condition_time_night: FlightTime::ZERO,
            pilot_in_command_time: FlightTime::ZERO,
            copilot_time: FlightTime::ZERO,
            dual_time: FlightTime::ZERO,
            instructor_time: FlightTime::ZERO,
            is_sim: true,
            sim_date,
            sim_type,
            sim_total_time_of_session,
            remark,

            end_of_page,
            end_of_book,
        };
        Ok(flight)
    }
}
