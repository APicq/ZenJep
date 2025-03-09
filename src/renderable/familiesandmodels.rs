use crate::flightlistgenerator::{AircraftModel, FlightList};
use askama::Template;

#[derive(Template)]
#[template(path = "familiesandmodels.html")]
pub struct FamiliesAndModelsHtml {
    families_models: Vec<FamilyAndModel>,
    models: Vec<(String, AircraftModel)>,
}

struct FamilyAndModel {
    family_name: String,
    model_name: String,
}

impl FamiliesAndModelsHtml {
    pub fn from_flight_list(flight_list: &FlightList) -> Self {
        let mut families_models = Vec::new();
        for (model_name, family_name) in flight_list.aircrafts.iter_models_families() {
            families_models.push(FamilyAndModel {
                family_name: family_name.to_string(),
                model_name: model_name.to_string(),
            });
        }
        families_models.sort_by(|a, b| a.family_name.cmp(&b.family_name));
        let mut models = Vec::new();
        for (model_name, model) in flight_list.aircrafts.iter_models() {
            models.push((model_name.clone(), model.clone()));
        }
        models.sort_by(|a, b| a.0.cmp(&b.0));
        FamiliesAndModelsHtml {
            families_models,
            models,
        }
    }
}
