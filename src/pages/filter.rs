use crate::applicationstate::AppState;
use crate::renderable::filters::FlightFilter;
use crate::renderable::htmlerror::HtmlError;
use tide::{Request, Response, Result};

pub async fn page_filter(request: Request<AppState>) -> Result<Response> {
    let appstate = request.state();
    if let Some(mut appstate_filter) = appstate.filter.try_lock() {
        match request.query() {
            // If we have a valid query,
            // we update the appstate with the new filter
            Ok(raw_filter_params) => {
                let new_filter = FlightFilter::from_raw(raw_filter_params);
                appstate_filter.update_from(&new_filter);
            }
            Err(e) => {
                // todo remove
                println!("Error with RawFilterParams {e:?}");
            }
        } // match
          //let filter_template = FlightFilter {};
        let response = appstate_filter.clone();
        Ok(response.into())
    } else {
        let error = HtmlError {
            message_1: "Cannot lock appstate".into(),
            message_2: "in show_filter".into(),
        };
        Ok(error.into())
    }
}
