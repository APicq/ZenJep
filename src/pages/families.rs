use crate::applicationstate::AppState;
use crate::flightlistgenerator::FlightList;
use crate::renderable::familiesandmodels::FamiliesAndModelsHtml;
use crate::renderable::filters::FlightFilter;
use crate::renderable::htmlerror::HtmlError;
use tide::{Request, Response, Result};

pub async fn page_families(req: Request<AppState>) -> Result<Response> {
    let appstate = req.state();
    // fetch the name of the yaml file
    let yaml_file: String = if let Some(appstate_filename) = appstate.filename.try_lock() {
        appstate_filename.clone()
    } else {
        let error = HtmlError {
            message_1: "Error : state blocked".into(),
            message_2: "in show_jeppesen".into(),
        };
        return Ok(error.into());
    };
    // build the flight list
    let flight_list = match FlightList::load_from_yaml(yaml_file) {
        Ok(flight_list) => flight_list,
        Err(e) => {
            let error_message = format!("{e:#?}");
            let jeppesen_error = HtmlError {
                message_1: error_message,
                message_2: "in show_jeppesen2".into(),
            };
            return Ok(jeppesen_error.into());
        }
    };
    // build the report
    let ac_types_and_models = FamiliesAndModelsHtml::from_flight_list(&flight_list);
    Ok(ac_types_and_models.into())
}
