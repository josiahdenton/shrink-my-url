mod routes;
mod templates;

use axum::{routing, Router};
use flexi_logger::Logger;
use tower_http::services::ServeFile;

pub fn setup_routes() -> Router {
    // seperates routes at the feature level
    // each feature will be a new route
    Router::new().route("/", routing::get(routes::index))
        .route("/click", routing::post(routes::click))
        .nest_service("/styles.css", ServeFile::new("templates/styles/styles.css"))
        .nest_service("/htmx.min.js", ServeFile::new("templates/scripts/htmx.min.js"))
}

pub fn setup_logging() -> Result<flexi_logger::LoggerHandle, flexi_logger::FlexiLoggerError> {
    Logger::try_with_str("info")?.start()
}
