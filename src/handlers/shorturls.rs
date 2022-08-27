use actix_web::{get, post, web, Responder, Result};
use sqlx::PgPool;
use tracing::{debug, instrument};

use super::SResult;

#[post("/{url}")]
#[instrument]
pub async fn add_url(url: web::Path<String>, db_pool: web::Data<PgPool>) -> Result<impl Responder> {
    debug!("{}", url);
    Ok(SResult::default().to_string())
}
