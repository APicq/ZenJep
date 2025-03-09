use crate::flightlistgenerator::flighttime::FlightDate;
use askama::Template;
use serde::Deserialize;
use time::{
    macros::{date, format_description},
    Date,
};

#[derive(Template, Clone)]
#[template(path = "filters.html")]
pub struct FlightFilter {
    flights_before: FlightDate,
    flights_before_enabled: bool,
    flights_after: FlightDate,
    flights_after_enabled: bool,
    aircraft_model_enabled: bool,
    aircraft_model: String,
    aircraft_family_enabled: bool,
    aircraft_family: String,
}

/// This struct reflects exactly the html input form
#[derive(Deserialize, Default, Debug)]
pub struct RawFilterParams {
    // date show only flights before this date
    // format : YYYY-?
    flights_before: String,
    flights_before_enabled: Option<bool>,
    flights_after: String,
    flights_after_enabled: Option<bool>,
    aircraft_model_enabled: Option<bool>,
    aircraft_model: String,
    aircraft_family_enabled: Option<bool>,
    aircraft_family: String,
}

impl Default for FlightFilter {
    fn default() -> Self {
        Self {
            flights_before: FlightDate(date!(1970 - 01 - 30)),
            flights_before_enabled: false,
            flights_after: FlightDate(date!(1970 - 01 - 30)),
            flights_after_enabled: false,
            aircraft_model_enabled: false,
            aircraft_model: "".into(),
            aircraft_family_enabled: false,
            aircraft_family: "".into(),
        }
    }
}

impl FlightFilter {
    /// Only way to create struct : from raw parts
    pub fn from_raw(params: RawFilterParams) -> Self {
        let date_format = format_description!("[year]-[month]-[day]");
        let flights_before = match Date::parse(&params.flights_before, date_format) {
            Ok(date) => FlightDate(date),
            Err(_) => FlightDate(date!(1970 - 01 - 31)),
        };
        let flights_before_enabled = matches!(params.flights_before_enabled, Some(true));
        let flights_after = match Date::parse(&params.flights_after, date_format) {
            Ok(date) => FlightDate(date),
            Err(_) => FlightDate(date!(1970 - 01 - 31)),
        };
        let flights_after_enabled = matches!(params.flights_after_enabled, Some(true));
        let aircraft_model = params.aircraft_model.to_string();
        let aircraft_model_enabled = matches!(params.aircraft_model_enabled, Some(true));
        let aircraft_family = params.aircraft_family.to_string();
        let aircraft_family_enabled = matches!(params.aircraft_family_enabled, Some(true));
        FlightFilter {
            flights_before,
            flights_before_enabled,
            flights_after,
            flights_after_enabled,
            aircraft_model_enabled,
            aircraft_model,
            aircraft_family_enabled,
            aircraft_family,
        }
    }
    // todo refactor or remove
    pub fn update_from(&mut self, input_filter: &FlightFilter) {
        self.flights_before = input_filter.flights_before;
        self.flights_before_enabled = input_filter.flights_before_enabled;
        self.flights_after = input_filter.flights_after;
        self.flights_after_enabled = input_filter.flights_after_enabled;
        self.aircraft_model_enabled = input_filter.aircraft_model_enabled;
        self.aircraft_model = input_filter.aircraft_model.clone();
        self.aircraft_family_enabled = input_filter.aircraft_family_enabled;
        self.aircraft_family = input_filter.aircraft_family.clone();
    }

    pub fn flights_before_enabled(&self) -> bool {
        self.flights_before_enabled
    }
    pub fn flights_before(&self) -> FlightDate {
        self.flights_before
    }
    pub fn flights_after_enabled(&self) -> bool {
        self.flights_after_enabled
    }
    pub fn flights_after(&self) -> FlightDate {
        self.flights_after
    }
    pub fn aircraft_model_enabled(&self) -> bool {
        self.aircraft_model_enabled
    }
    pub fn aircraft_model(&self) -> &str {
        &self.aircraft_model
    }
    pub fn aircraft_family_enabled(&self) -> bool {
        self.aircraft_family_enabled
    }
    pub fn aircraft_family(&self) -> &str {
        &self.aircraft_family
    }
}
