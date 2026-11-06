pub mod interface;
pub mod infrastructure;
pub mod domain;
pub mod application;
pub mod error;

use std::error::Error;
use std::sync::Arc;
use crate::application::services::{auth_service, toll_road_service};
use crate::infrastructure::database::{create_pool, run_migrations};
use crate::infrastructure::repositories::{toll_road_repo, user_repo};
use crate::interface::routes::create_router;
use crate::interface::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error+Send + Sync>> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

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

    let toll_road_repository = toll_road_repo::TollRoadPostgresRepository::new(pool.clone());
    let toll_road_service = toll_road_service::TollRoadService::new(Arc::new(toll_road_repository));
    let user_repository = user_repo::UserRepositoryPostgres::new(pool.clone());
    let auth_service = auth_service::AuthService::new(Arc::new(user_repository));

    let state = AppState {
        toll_service: toll_road_service,
        auth_service,
    };

    let router = create_router(state);

    let server_addr = std::env::var("SERVER_ADDR")
        .unwrap_or_else(|_| "localhost:9090".to_string());

    let listener = tokio::net::TcpListener::bind(&server_addr)
        .await?;

    tracing::info!("Server listening on {}", server_addr);

    axum::serve(listener, router).await?;

    Ok(())
}
