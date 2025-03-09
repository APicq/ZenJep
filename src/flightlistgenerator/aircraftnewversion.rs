use anyhow::{bail, Context, Result};
use indexmap::IndexMap;
use strict_yaml_rust::StrictYaml;

#[derive(Clone)]
pub struct AircraftModel {
    pub nb_engines: u8,
    pub mtow: u32, // MTOW in kg
    pub multipilot: bool,
    pub category_1: String,
    pub category_2: String,
    pub category_3: String,
}

/// Relation between immatriculation, aircraft model and family
pub struct AircraftDataBase {
    // immatriculation -> aircraftmodel name
    immatriculations: IndexMap<String, String>,
    /// dictionary model name -> model struct
    models: IndexMap<String, AircraftModel>,
    /// dictionary model name -> family name
    families: IndexMap<String, String>,
}

impl AircraftDataBase {
    pub fn empty() -> Self {
        AircraftDataBase {
            immatriculations: IndexMap::new(),
            models: IndexMap::new(),
            families: IndexMap::new(),
        }
    }

    /// parse a yaml flight and adds all elements in the database
    pub fn add_flight(&mut self, yaml: &StrictYaml) -> Result<(&str, &str)> {
        let immatriculation = yaml["immatriculation"]
            .as_str()
            .context("Immatriculation missing.")?;
        let acmodel = yaml["acmodel"].as_str();
        let actype = yaml["actype"].as_str();
        let nb_engines = yaml["nb_engines"].as_str();
        let mtow = yaml["mtow"].as_str();
        let multipilot = yaml["multipilot"].as_str();
        let category_1 = yaml["cat1"].as_str();
        let category_2 = yaml["cat2"].as_str();
        let category_3 = yaml["cat3"].as_str();
        match (
            acmodel, actype, nb_engines, mtow, multipilot, category_1, category_2, category_3,
        ) {
            // Case 1) if acmodel exists and acfamily exists and evything exists
            (
                Some(aircraft_model_name),
                Some(aircraft_family_name),
                Some(nb_engines),
                Some(mtow),
                Some(multipilot),
                Some(category_1),
                Some(category_2),
                Some(category_3),
            ) => {
                if self.immatriculations.contains_key(immatriculation) {
                    bail!(
                        "Error : immatriculation {immatriculation} already in \
			 the dictionary AircraftDataBase.immatriculations "
                    );
                }
                if self.models.contains_key(aircraft_model_name) {
                    bail!(
                        "Error : model {aircraft_model_name} already in \
			 the dictionary AircraftDataBase.models "
                    );
                }
                let aircraft_family_name = aircraft_family_name.to_string();
                let nb_engines = nb_engines
                    .parse::<u8>()
                    .with_context(|| format!("nb engines: {nb_engines}"))?;
                let mtow = mtow
                    .parse::<u32>()
                    .with_context(|| format!("mtow: {mtow}"))?;
                let multipilot = multipilot
                    .parse::<bool>()
                    .with_context(|| format!("multipilot: {multipilot:?}"))?;

                //Create new AircrafModel :
                let aircraft_model = AircraftModel {
                    nb_engines,
                    mtow,
                    multipilot,
                    category_1: category_1.to_string(),
                    category_2: category_2.to_string(),
                    category_3: category_3.to_string(),
                };
                // add the aircraft model,family and immatriculation :
                self.models
                    .insert(aircraft_model_name.to_string(), aircraft_model);
                self.immatriculations
                    .insert(immatriculation.to_string(), aircraft_model_name.to_string());
                self.families
                    .insert(aircraft_model_name.to_string(), aircraft_family_name);
            } // case 1
            // case 2) : no information provided but the immatriculation
            (None, None, None, None, None, None, None, None) => {
                if !self.immatriculations.contains_key(immatriculation) {
                    bail!("Error immatriculation {immatriculation} has no associated model.");
                }
            } // case 2
            // Case 3) Immatriculation and model
            (Some(aircraft_model_name), None, None, None, None, None, None, None) => {
                if self.immatriculations.contains_key(immatriculation) {
                    bail!(
                        "Error : immatriculation {immatriculation} has already a model associated"
                    );
                }
                if !self.models.contains_key(aircraft_model_name) {
                    bail!("Error : model {aircraft_model_name} has not been previously defined");
                }
                self.immatriculations
                    .insert(immatriculation.to_string(), aircraft_model_name.to_string());
            } // case 3
            _ => {
                bail!(
                    "Error aircraft definition. Only 3 possibilities : \
		     - field  [immatriculation] \
		     - fields [immatriculation] and [acmodel] \
		     - fields [immatriculation] [acmodel] [nb_engines] [mtow] [multipilot] [cat1] [cat2] [cat3] \
		     \
		     immatriculation={:?}, \
		     acmodel={:?} \
		     actype={:?}, \
		     nb_engines={:?} \
		     mtow={:?}, \
		     multipilot={:?}, \
		     cat1={:?}, \
		     cat2={:?}, \
		     cat3={:?}",
                    immatriculation,
                    acmodel,
                    actype,
                    nb_engines,
                    mtow,
                    multipilot,
                    category_1,
                    category_2,
                    category_3
                );
            }
        } // match
        let (immatriculation_ref, model_ref) = self
            .immatriculations
            .get_key_value(immatriculation)
            .unwrap();
        Ok((immatriculation_ref, model_ref))
    }
    /// Input: immatriculation
    /// Outptut: immatriculation,aircraft model
    pub fn get_model_name(&self, immatriculation: &str) -> Result<(&str, &str)> {
        let (immatriculation_ref, model_ref) = self
            .immatriculations
            .get_key_value(immatriculation)
            .with_context(|| format!("immatriculation {immatriculation} not found"))?;
        Ok((immatriculation_ref, model_ref))
    }
    pub fn get_family_name(&self, immatriculation: &str) -> Result<&str> {
        let (_, model_name) = self.get_model_name(immatriculation)?;
        Ok(self.families.get(model_name).unwrap())
    }
    pub fn is_single_pilot_se(&self, immatriculation: &str) -> Result<bool> {
        let (_, model_name) = self.get_model_name(immatriculation)?;
        let model = self.models.get(model_name).unwrap();
        if !model.multipilot && model.nb_engines == 1 && model.category_3 != "pax" {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub fn is_single_pilot_me(&self, immatriculation: &str) -> Result<bool> {
        let (_, model_name) = self.get_model_name(immatriculation)?;
        let model = self.models.get(model_name).unwrap();
        if !model.multipilot && model.nb_engines > 1 && model.category_3 != "pax" {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub fn is_multi_pilot(&self, immatriculation: &str) -> Result<bool> {
        let (_, model_name) = self.get_model_name(immatriculation)?;
        let model = self.models.get(model_name).unwrap();
        if model.multipilot && model.category_3 != "pax" {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    // todo try to name lifetime 'a
    pub fn iter_models_families(&self) -> indexmap::map::Iter<String, String> {
        self.families.iter()
    }

    pub fn iter_models(&self) -> indexmap::map::Iter<String, AircraftModel> {
        self.models.iter()
    }
}
