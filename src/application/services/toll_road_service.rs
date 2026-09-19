use std::sync::Arc;
use crate::application::dto::toll_road_dto::TollRoadResponse;
use crate::domain::repositories::TollRoadRepository;
use crate::error::AppError;
use crate::infrastructure::repositories::toll_road_repo::TollRoadPostgresRepository;

#[derive(Clone)]
pub struct TollRoadService {
    repo: Arc<dyn TollRoadRepository>
}

impl TollRoadService {
    pub fn new(
        repo: Arc<dyn TollRoadRepository>
    ) -> Self {
        Self { repo }
    }

    pub async fn index(&self) -> Result<Vec<TollRoadResponse>, AppError> {
        let results = self.repo.find()
            .await?;

        Ok(results.into_iter().map(Into::into).collect())
    }
}