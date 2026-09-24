use axum::{middleware, Router};
use tower_sessions::{MemoryStore, SessionManagerLayer};

pub mod duration_log;

pub fn apply(router: Router) -> Router {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false);
    router
        .layer(middleware::from_fn(duration_log::handle))
        .layer(session_layer)
}