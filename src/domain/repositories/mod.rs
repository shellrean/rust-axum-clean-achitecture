use async_trait::async_trait;
use crate::domain::entities::toll_roads::TollRoads;
use crate::error::AppError;

#[async_trait]
pub trait TollRoadRepository: Sync + Send {
    async fn find(&self) -> Result<Vec<TollRoads>, AppError>;
}