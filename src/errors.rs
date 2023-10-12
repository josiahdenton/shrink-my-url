use askama_axum::IntoResponse;
use axum::{http::StatusCode, Json};
use serde_json::json;

pub enum AppError {
    Minifier(MinifyError),
}

impl From<MinifyError> for AppError {
    fn from(inner: MinifyError) -> Self {
        AppError::Minifier(inner)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> askama_axum::Response {
        let (status, error_message) = match self {
            AppError::Minifier(err) => match err {
                MinifyError::BadRequest => (StatusCode::BAD_REQUEST, "Failed to minify"),
                MinifyError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Oops"),
            },
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

pub enum MinifyError {
    BadRequest,
    InternalError,
}
