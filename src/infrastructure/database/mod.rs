use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:postgres@localhost:5432/test_db")
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}