use axum::{middleware, Router};
use axum::middleware::from_fn_with_state;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use crate::application::dto::auth_dto::LoginInfo;
use crate::interface::state::AppState;

pub mod duration_log;
pub mod auth_context;

tokio::task_local! {
    pub static CURRENT_USER: LoginInfo
}

pub fn apply(
    state: AppState,
    public_router: Router<AppState>,
    private_router: Router<AppState>,
) -> Router {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false);
    
    let private_with_auth = private_router
        .layer(from_fn_with_state(state.clone(), auth_context::handle));
    
    public_router
        .merge(private_with_auth)
        .layer(middleware::from_fn(duration_log::handle))
        .layer(session_layer)
        .with_state(state)
}