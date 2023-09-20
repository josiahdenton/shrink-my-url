#[tokio::main]
async fn main() {
    // move these steps to lib.rs
    shrink::setup_logging().expect("failed to start logger");
    let app = shrink::setup_routes();

    axum::Server::bind(&([127, 0, 0, 1], 3000).into())
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
