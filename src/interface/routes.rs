use axum::{middleware, Router};
use axum::routing::{get, post};
use crate::interface::handlers::{auth_handler, test_handlers, toll_road_handlers};
use crate::interface::layers;
use crate::interface::state::AppState;

pub fn create_router(state: AppState) -> Router {
    let app = Router::new()
        .route("/test", get(test_handlers::test))
        .route("/toll-roads", get(toll_road_handlers::index))
        .route("/auth/login", post(auth_handler::login))
        .route("/auth/me", get(auth_handler::me))
        .route("/auth/logout", post(auth_handler::logout))
        .with_state(state);
        
    layers::apply(app)
}