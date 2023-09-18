use axum::Form;
use serde::Deserialize;

use crate::templates::IndexTemplate;

const NAME: &'static str = "Josiah";

pub async fn index() -> IndexTemplate<'static> {
    log::info!("get for index!");
    IndexTemplate::new(NAME)
}

pub async fn click() -> String {
    String::from("<p>You clicked me!</p>")
}

#[derive(Debug, Deserialize)]
pub struct MinifyUrlForm {
    pub url: String,
}

// extract from Content-Type: application/x-www-form-urlencoded
pub async fn minify(Form(url_form): Form<MinifyUrlForm>) -> String {
    log::info!("what does the post look like: {:?}", url_form);
    String::from("SUCCESS!")
}
