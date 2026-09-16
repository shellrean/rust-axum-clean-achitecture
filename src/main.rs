pub mod interface;
pub mod infrastructure;
pub mod domain;
pub mod application;
pub mod error;

use std::error::Error;
use tracing_subscriber::util::SubscriberInitExt;
use crate::application::services::toll_road_service;
use crate::infrastructure::database::{create_pool, run_migrations};
use crate::infrastructure::repositories::toll_road_repo;
use crate::interface::routes::create_router;
use crate::interface::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error+Send + Sync>> {

    tracing_subscriber::registry()
        .init();

    let pool = match create_pool().await {
        Ok(p) => {
            if let Err(e) = run_migrations(&p).await {
                tracing::warn!("failed to run migrations: {}", e);
            }
            p
        }
        Err(e) => {
            tracing::error!("failed to create database pool: {}", e);
            return Err(e.into())
        }
    };

    let toll_road_repository = toll_road_repo::TollRoadRepository::new(pool.clone());
    let toll_road_service = toll_road_service::TollRoadService::new(toll_road_repository);

    let state = AppState {
        toll_service: toll_road_service,
    };

    let router = create_router(state);

    let listener = tokio::net::TcpListener::bind("localhost:9090")
        .await?;

    axum::serve(listener, router).await?;

    Ok(())
}
