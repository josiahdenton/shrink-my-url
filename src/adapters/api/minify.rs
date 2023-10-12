use axum::{Extension, Form};
use serde::Deserialize;

use crate::{
    errors::{AppError, MinifyError},
    SharedState,
};

#[derive(Debug, Deserialize)]
pub struct MinifyUrlForm {
    pub url: String,
}

// extract from Content-Type: application/x-www-form-urlencoded
pub async fn create_minified_url(
    Extension(state): Extension<SharedState>,
    Form(url_form): Form<MinifyUrlForm>,
) -> Result<String, AppError> {
    if url_form.url.len() == 0 {
        return Err(MinifyError::BadRequest.into());
    }
    let mut shared_state = state.write().unwrap();

    log::info!("what does the post look like: {:?}", url_form);

    match shared_state.minifier.minify(&url_form.url) {
        Ok(alias) => Ok(String::from(alias)),
        Err(_) => Err(MinifyError::InternalError.into()),
    }
}
