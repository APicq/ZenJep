use crate::applicationstate::AppState;
use crate::flightlistgenerator::FlightList;
use crate::renderable::aircrafts::AircraftsHtml;
use crate::renderable::filters::FlightFilter;
use crate::renderable::htmlerror::HtmlError;
use tide::{Request, Response, Result};

pub async fn page_aircrafts(req: Request<AppState>) -> Result<Response> {
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
    // build the filter :
    let filter = if let Some(appstate_filter) = appstate.filter.try_lock() {
        appstate_filter.clone()
    } else {
        let error = HtmlError {
            message_1: "Error : state blocked cannot fetch filter".into(),
            message_2: "in show_jeppesen".into(),
        };
        return Ok(error.into());
    };
    // build the flight list
    let mut flight_list = match FlightList::load_from_yaml(yaml_file) {
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
    // filter the flightlist :
    flight_list.filter(&filter);
    // Build the aircrafts report
    let aircrafts_html = AircraftsHtml::from_flight_list(&flight_list);
    Ok(aircrafts_html.into())
}
