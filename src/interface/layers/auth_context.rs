use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_sessions::Session;
use crate::application::dto::auth_dto::LoginInfo;
use crate::error::AppError;
use crate::interface::layers::CURRENT_USER;

pub async fn handle(
    session: Session,
    req: Request,
    next: Next
) -> Result<Response, AppError> {
    let user_opt: Option<LoginInfo> = session
        .get("user_info")
        .await
        .map_err(|_| AppError::Authentication("session not found".into()))?;

    let user = match user_opt {
        Some(user) => user,
        None => return Err(AppError::Authentication("session not found".into()))
    };

    let response = CURRENT_USER
        .scope(user, async move {
            next.run(req).await
        })
        .await;

    Ok(response)
}