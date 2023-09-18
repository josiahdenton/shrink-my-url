mod routes;
mod templates;

use axum::{routing, Router};
use flexi_logger::Logger;
use tower_http::services::ServeFile;

pub fn setup_routes() -> Router {
    Router::new().route("/", routing::get(routes::index))
        .nest_service("/styles.css", ServeFile::new("templates/styles/styles.css"))
}

pub fn setup_logging() -> Result<flexi_logger::LoggerHandle, flexi_logger::FlexiLoggerError> {
    Logger::try_with_str("info")?.start()
}
