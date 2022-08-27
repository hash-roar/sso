use color_eyre::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};

mod model;

pub use model::User;
use tracing::warn;

#[derive(Debug)]
pub enum DbError {
    NotFound,
    #[allow(dead_code)]
    DuplicateKey,
    InterError,
}

impl From<sqlx::Error> for DbError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => DbError::NotFound,
            _ => {
                warn!("error happen at database:{}", e);
                DbError::InterError
            }
        }
    }
}

pub async fn get_pgpool(dsn: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(dsn)
        .await?;
    Ok(pool)
}
