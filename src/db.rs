use crate::{AppError, AppResult};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(db_uri: String) -> AppResult<DbPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_uri.as_str())
        .await
        .map_err(|x| AppError::DBError(format!("{:?}", x)))?;

    Ok(pool)
}
