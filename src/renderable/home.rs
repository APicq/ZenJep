use askama::Template;

#[derive(Template, Debug, Default)]
#[template(path = "index.html")]
pub struct HomeHtml {}
