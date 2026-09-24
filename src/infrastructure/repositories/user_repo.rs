use async_trait::async_trait;
use sqlx::{PgPool, Row};
use crate::domain::entities::user::User;
use crate::domain::repositories::{UserRepository};
use crate::error::AppError;

#[derive(Clone)]
pub struct UserRepositoryPostgres {
    pool: PgPool
}

impl UserRepositoryPostgres {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryPostgres {
    async fn find_by_email(&self, email: String) -> Result<Option<User>, AppError> {
        let row = sqlx::query(include_str!("sql/user_repo_find_by_email.sql"))
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        
        let user = row.map(|row| {
            let user = User {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
                password: row.get("password"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Ok::<User, AppError>(user)
        }).transpose()?;
        Ok(user)
    }
}