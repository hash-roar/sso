use std::borrow::Borrow;

use crate::{db::User, error::SError, utils::get_digest};
use actix_web::{web, Responder, Result};
use sqlx::PgPool;
use tracing::instrument;

use super::SResult;

#[instrument(skip(db_pool))]
pub async fn add_user(
    user_info: web::Json<User>,
    db_pool: web::Data<PgPool>,
) -> Result<impl Responder> {
    let mut user = user_info.into_inner();
    user.passwd = get_digest(&user.passwd);
    if let Err(_) = user.add(db_pool.borrow()).await {
        return Ok(SResult::new(1, "add user failed", "").to_string());
    }
    Ok(SResult::default().to_string())
}

#[instrument(skip(db_pool))]
pub async fn get_user_by_name(
    name: web::Path<String>,
    db_pool: web::Data<PgPool>,
) -> Result<impl Responder> {
    let pool = db_pool.into_inner();
    let user = User::get_by_name(name.to_string(), pool.as_ref())
        .await
        .map_err(|_| SError::ServerError)?;
    Ok(web::Json(user))
}
