use color_eyre::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};

mod model;

pub use  model::User;


pub async fn get_pgpool(dsn: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(dsn)
        .await?;
    Ok(pool)
}
