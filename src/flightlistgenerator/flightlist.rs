use std::{fmt::Display, fs::read_to_string, path::Path};

use anyhow::{Context, Result};
use strict_yaml_rust::{StrictYamlEmitter, StrictYamlLoader};

use crate::renderable::filters::FlightFilter;

use super::airport::AirportList;
pub(crate) use super::{aircraftnewversion::AircraftDataBase, flight::Flight};

/// Struct to parse the yaml file
///
/// The main struct containing all the flights
/// parsed from the yaml
/// content : a vector of flights
/// airports : a list of all the airports
/// aircrafts : a list of aircrafts
pub struct FlightList {
    pub flights: Vec<Flight>,
    pub airports: AirportList,
    pub aircrafts: AircraftDataBase,
}

impl FlightList {
    /// constructs a Flights struct from a yaml file
    pub fn load_from_yaml<P: AsRef<Path> + Display>(p: P) -> Result<Self> {
        log::info!("Opening file {}", p);
        //
        // Load the yaml file, take the first yaml document
        let file_content =
            read_to_string(&p).with_context(|| format!("Cannot open file : {}", &p))?;
        let yaml_documents = StrictYamlLoader::load_from_str(&file_content)?;
        let yaml_document = yaml_documents
            .get(0)
            .with_context(|| format!("Cannot find the first document in file : {}", &p))?;

        // Iterate over all the flights and add the flights to the flight list
        let mut flights = Vec::new();
        let mut airports = AirportList::new();
        let mut aircrafts = AircraftDataBase::empty();
        for (index, flight) in yaml_document
            .as_vec()
            .with_context(|| {
                format!(
                    "The first yaml document should be a vector of flights in file : {}",
                    &p
                )
            })?
            .iter()
            .enumerate()
        {
            let flight =
                Flight::from_yaml(flight, &mut aircrafts, &mut airports).with_context(|| {
                    let mut formatted_yaml = String::new();
                    let mut emitter = StrictYamlEmitter::new(&mut formatted_yaml);
                    emitter.compact(true);
                    emitter.dump(flight).unwrap();
                    format!("In flight number {index} {formatted_yaml}")
                })?;
            flights.push(flight);
        }
        log::info!("{} flights added to Flights", flights.len());
        Ok(FlightList {
            flights,
            airports,
            aircrafts,
        })
    }

    pub fn iter_flights_without_sims(&self) -> impl Iterator<Item = &Flight> {
        self.flights.iter().filter(|f| !f.is_sim)
    }

    pub fn filter(&mut self, filter: &FlightFilter) {
        if filter.flights_before_enabled() {
            self.flights
                .retain(|flight| flight.date <= filter.flights_before());
        }
        if filter.flights_after_enabled() {
            self.flights
                .retain(|flight| flight.date >= filter.flights_after());
        }
        if filter.aircraft_model_enabled() {
            self.flights.retain(|flight| {
                if !flight.is_sim {
                    let (_, model) = self
                        .aircrafts
                        .get_model_name(&flight.immatriculation)
                        .unwrap();
                    if model == filter.aircraft_model() {
                        true
                    } else {
                        false
                    }
                }
                // if
                else {
                    false
                }
            });
	}
	if filter.aircraft_family_enabled() {
            self.flights.retain(|flight| {
                if !flight.is_sim {
                    let family = self
                        .aircrafts
                        .get_family_name(&flight.immatriculation)
                        .unwrap();
                    if family == filter.aircraft_family() {
                        true
                    } else {
                        false
                    }
                }
                // if
                else {
                    false
                }
            });
	}
    }
}

