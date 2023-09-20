mod domain;
mod adapters;

use axum::{routing, Router};
use flexi_logger::Logger;
use tower_http::services::ServeFile;

use crate::adapters::api::index;
use crate::adapters::api::minify;

pub fn setup_routes() -> Router {
    // seperates routes at the feature level
    // each feature will be a new route
    log::info!("setting up routes!");
    Router::new().route("/", routing::get(index::serve_index))
        .route("/click", routing::post(index::click))
        .route("/minify", routing::post(minify::create_minified_url))
        .nest_service("/styles.css", ServeFile::new("assets/styles.css"))
        .nest_service("/htmx.min.js", ServeFile::new("assets/htmx.min.js"))
}

pub fn setup_logging() -> Result<flexi_logger::LoggerHandle, flexi_logger::FlexiLoggerError> {
    Logger::try_with_str("info")?.start()
}
