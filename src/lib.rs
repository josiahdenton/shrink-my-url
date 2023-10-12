mod adapters;
mod domain;
mod errors;

use std::sync::Arc;
use std::sync::RwLock;

use adapters::repository::mem_map::MemMap;
use axum::{routing, Router};
use domain::encoders::UUIDEncoder;
use domain::minify::Minifier;
use flexi_logger::Logger;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::services::ServeDir;

use crate::adapters::api::index;
use crate::adapters::api::minify;

#[derive(Debug, Default)]
struct State {
    minifier: Minifier<MemMap, UUIDEncoder>,
}

type SharedState = Arc<RwLock<State>>;

impl Default for Minifier<MemMap, UUIDEncoder> {
    fn default() -> Self {
        Minifier::new(MemMap::new(), UUIDEncoder::new())
    }
}

pub fn setup_router() -> Router {
    log::info!("setting up routes!");
    Router::new()
        .route("/", routing::get(index::serve_index))
        .route("/minify", routing::post(minify::create_minified_url))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(ServiceBuilder::new().layer(AddExtensionLayer::new(SharedState::default())))
}

pub fn setup_logging() -> Result<flexi_logger::LoggerHandle, flexi_logger::FlexiLoggerError> {
    Logger::try_with_str("info")?.start()
}
