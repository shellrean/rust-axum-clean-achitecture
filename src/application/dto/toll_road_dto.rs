use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::entities::toll_roads::TollRoads;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TollRoadResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
}

impl From<TollRoads> for TollRoadResponse {
    fn from(road: TollRoads) -> Self {
        Self {
            id: road.id,
            code: road.code,
            name: road.name,
        }
    }
}