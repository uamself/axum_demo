use axum::Router;
use axum::routing::get;

pub fn get_user_routes() -> Router {
    Router::new().route("/user", get(|| async { "user" }))
}

