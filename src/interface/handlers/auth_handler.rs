use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use tower_sessions::Session;
use crate::application::dto::auth_dto::{LoginInfo, LoginRequest};
use crate::error::AppError;
use crate::interface::layers::CURRENT_USER;
use crate::interface::state::AppState;

pub async fn login(
    session: Session,
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginInfo>, AppError> {
    let result = state.auth_service.login(payload).await?;

    let session_result = session.insert("user_info", result.clone()).await;
    if session_result.is_err() {
        return Err(AppError::Authentication("issue in insert session".into()))
    }

    Ok(Json(result))
}

pub async fn me()-> Result<Json<LoginInfo>, AppError> {
    let user = CURRENT_USER.with(|user| user.clone());
    Ok(Json(user))
}

pub async fn logout(session: Session) -> Result<StatusCode, AppError> {
    session.delete().await.unwrap();

    Ok(StatusCode::OK)
}