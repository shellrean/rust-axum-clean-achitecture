use async_trait::async_trait;
use sqlx::{PgPool, Row};
use crate::domain::entities::toll_roads::TollRoads;
use crate::domain::repositories::TollRoadRepository;
use crate::error::AppError;

#[derive(Clone)]
pub struct TollRoadPostgresRepository {
    pool: PgPool
}

impl TollRoadPostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TollRoadRepository for TollRoadPostgresRepository {
    async fn find(&self) -> Result<Vec<TollRoads>, AppError> {
        let rows = sqlx::query(include_str!("sql/toll_road_repo_find.sql"))
            .fetch_all(&self.pool)
            .await?;

        let mut results = Vec::new();
        for r in rows {
            results.push(TollRoads {
                id: r.get("id"),
                code: r.get("code"),
                name: r.get("name"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at")
            })
        }

        Ok(results)
    }
}