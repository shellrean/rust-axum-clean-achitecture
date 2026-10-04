use std::sync::Arc;
use tokio::sync::Semaphore;
use crate::application::dto::auth_dto::{LoginInfo, LoginRequest};
use crate::domain::repositories::UserRepository;
use crate::error::AppError;

#[derive(Clone)]
pub struct AuthService {
    repo: Arc<dyn UserRepository>,
    semaphore: Arc<Semaphore>,
}

impl AuthService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self {
            repo,
            semaphore: Arc::new(Semaphore::new(10)),
        }
    }
    
    pub async fn login(&self, req: LoginRequest) -> Result<LoginInfo, AppError> {
        let user_opt = self.repo.find_by_email(req.email).await?;
        
        if user_opt.is_none() {
            return Err(AppError::Authentication("user not found".into()))
        }
        
        let user = user_opt.unwrap();

        let permit = self.semaphore.clone().acquire_owned().await
            .map_err(|_| AppError::Authentication("failed to acquire semaphore".into()))?;

        let is_verified = tokio::task::spawn_blocking(move || {
            let result = bcrypt::verify(&req.password, &user.password);
            drop(permit);
            result
        })
            .await
            .map_err(|e| AppError::Authentication(e.to_string()))?
            .map_err(|e| AppError::Authentication(e.to_string()))?;
        
        if !is_verified {
            return Err(AppError::Authentication("wrong password".into()));
        }
        
        Ok(LoginInfo {
            id: user.id,
            name: user.name,
            email: user.email,
        })
    }
}