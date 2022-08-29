use std::borrow::Borrow;

use super::SResult;

use crate::{db::Url, error::SError, utils::get_digest};
use actix_web::{post, web, Responder, Result};

use sqlx::PgPool;
use tracing::{debug, instrument};

#[post("/{url}")]
#[instrument]
pub async fn add_url(url: web::Path<String>, db_pool: web::Data<PgPool>) -> Result<impl Responder> {
    debug!("{}", url);
    let dest_url = get_digest(&url);
    let url = Url::new(url.into_inner(), dest_url);
    url.add(db_pool.borrow())
        .await
        .map_err(|_| SError::ServerError)?;
    Ok(SResult::default().to_string())
}
