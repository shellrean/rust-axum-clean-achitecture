use axum::extract::State;
use axum::Json;
use crate::application::dto::toll_road_dto::TollRoadResponse;
use crate::error::AppError;
use crate::interface::state::AppState;

pub async fn index(State(state): State<AppState>) -> Result<Json<Vec<TollRoadResponse>>, AppError> {
    let results = state.toll_service.index().await?;
    Ok(Json(results))
}