use std::sync::Arc;
use crate::application::dto::auth_dto::{LoginInfo, LoginRequest};
use crate::domain::repositories::UserRepository;
use crate::error::AppError;

#[derive(Clone)]
pub struct AuthService {
    repo: Arc<dyn UserRepository>
}

impl AuthService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }
    
    pub async fn login(&self, req: LoginRequest) -> Result<LoginInfo, AppError> {
        let user_opt = self.repo.find_by_email(req.email).await?;
        
        if user_opt.is_none() {
            return Err(AppError::Authentication("user not found".into()))
        }
        
        let user = user_opt.unwrap();
        
        let is_verified = bcrypt::verify(&req.password, &user.password)
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