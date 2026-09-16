use axum::Router;
use axum::routing::get;
use crate::interface::handlers::{test_handlers, toll_road_handlers};
use crate::interface::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/test", get(test_handlers::test))
        .route("/toll-roads", get(toll_road_handlers::index))
        .with_state(state)
}