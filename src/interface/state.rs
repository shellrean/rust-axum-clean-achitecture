use crate::application::services::toll_road_service::TollRoadService;

#[derive(Clone)]
pub struct AppState {
    pub toll_service: TollRoadService
} 