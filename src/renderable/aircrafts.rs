use std::collections::HashMap;

use crate::flightlistgenerator::{flighttime::FlightTime, FlightList};
use askama::Template;

#[derive(Template)]
#[template(path = "aircrafts.html")]
pub struct AircraftsHtml {
    models: Vec<AircraftModelHtml>,
    total_time_all_models: FlightTime,
    families: Vec<AircraftFamilyHtml>,
    total_time_all_families: FlightTime,
}

struct AircraftModelHtml {
    name: String,
    time: FlightTime,
}

struct AircraftFamilyHtml {
    name: String,
    time: FlightTime,
}

impl AircraftsHtml {
    pub fn from_flight_list(flight_list: &FlightList) -> Self {
        // --------
        // models
        // --------
        // dictionary model name -> flight hours
        let mut dictionary_by_model = HashMap::new();

        for flight in flight_list.iter_flights_without_sims() {
            dictionary_by_model
                .entry(&flight.acmodel)
                .and_modify(|time| *time += flight.total_flight_time)
                .or_insert(flight.total_flight_time);
        }

        let mut list_by_ac_model = dictionary_by_model
            .into_iter()
            .map(|(ac_model, time)| AircraftModelHtml {
                name: ac_model.to_string(),
                time,
            })
            .collect::<Vec<AircraftModelHtml>>();
        list_by_ac_model.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

        let mut total_time_all_models = FlightTime::ZERO;
        for ac_model in list_by_ac_model.iter() {
            total_time_all_models += ac_model.time;
        }
        // --------
        // families
        // --------
        let mut dictionary_by_family = HashMap::new();

        for flight in flight_list.iter_flights_without_sims() {
            let family = flight_list
                .aircrafts
                .get_family_name(&flight.immatriculation)
                .unwrap();

            dictionary_by_family
                .entry(family)
                .and_modify(|time| *time += flight.total_flight_time)
                .or_insert(flight.total_flight_time);
        }

        let mut list_by_families = dictionary_by_family
            .into_iter()
            .map(|(aircraft_family, time)| AircraftFamilyHtml {
                name: aircraft_family.to_string(),
                time,
            })
            .collect::<Vec<AircraftFamilyHtml>>();
        list_by_families.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

        let mut total_time_all_families = FlightTime::ZERO;
        for ac_family in list_by_families.iter() {
            total_time_all_families += ac_family.time;
        }

        AircraftsHtml {
            models: list_by_ac_model,
            total_time_all_models,
            families: list_by_families,
            total_time_all_families,
        }
    }
}
