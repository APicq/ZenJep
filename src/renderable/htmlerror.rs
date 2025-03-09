use askama::Template;

#[derive(Template, Debug, Default)]
#[template(path = "error_jeppesen.html")]
pub struct HtmlError {
    pub message_1: String,
    pub message_2: String,
}
