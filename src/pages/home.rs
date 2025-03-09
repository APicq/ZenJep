use crate::applicationstate::AppState;
use crate::renderable::home::HomeHtml;
use tide::{Request, Response, Result};

pub async fn page_home(_req: Request<AppState>) -> Result<Response> {
    let home = HomeHtml {};
    let response = home.into();
    Ok(response)
}
