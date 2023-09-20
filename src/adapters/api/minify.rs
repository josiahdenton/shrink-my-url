use axum::Form;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MinifyUrlForm {
    pub url: String,
}

// extract from Content-Type: application/x-www-form-urlencoded
pub async fn create_minified_url(Form(url_form): Form<MinifyUrlForm>) -> String {
    log::info!("what does the post look like: {:?}", url_form);
    String::from("SUCCESS!")
}
