use std::fmt::Display;

use askama::Template;

#[derive(Template, Debug)]
#[template(path = "testtemplate.html")]
pub struct TestTemplate {
    a_string: String,
    a_int: u8,
    a_toto: Toto,
}

impl TestTemplate {
    pub fn new() -> Self {
        TestTemplate {
            a_string: "test_template".into(),
            a_int: 12,
            a_toto: Toto { x: 44.34 },
        }
    }
}

#[derive(Debug)]
struct Toto {
    x: f64,
}

impl Display for Toto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "a_toto={}", self.x)
    }
}
