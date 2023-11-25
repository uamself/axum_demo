use axum::Router;
use axum::routing::get;

pub fn get_test_routes() -> Router {
    Router::new().route("/test", get(|| async { "test" }))
}

