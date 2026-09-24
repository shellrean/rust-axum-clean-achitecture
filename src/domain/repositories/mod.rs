use async_trait::async_trait;
use crate::domain::entities::toll_roads::TollRoads;
use crate::domain::entities::user::User;
use crate::error::AppError;

#[async_trait]
pub trait TollRoadRepository: Sync + Send {
    async fn find(&self) -> Result<Vec<TollRoads>, AppError>;
}

#[async_trait]
pub trait UserRepository: Sync + Send {
    async fn find_by_email(&self, email: String) -> Result<Option<User>, AppError>;
}