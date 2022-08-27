use std::borrow::Borrow;

use actix_web::{web, Responder, Result};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{debug, instrument};

use crate::db::{self};
use crate::error::SError;
use crate::handlers::SResult;
use crate::utils::get_digest;

#[derive(Debug, Deserialize)]
pub struct LoginData {
    pub name: String,
    pub password: String,
}

#[instrument(skip(db_pool))]
pub async fn login(
    info: web::Json<LoginData>,
    db_pool: web::Data<PgPool>,
) -> Result<impl Responder> {
    debug!("login data:{:?}", info);

    let result = db::User::get_by_name(info.name.to_owned(), db_pool.borrow()).await;
    if let Err(e) = result {
        match e {
            db::DbError::NotFound => return Ok(SResult::new(1, "user not found", "").to_string()),
            _ => return Err(SError::ServerError.into()),
        }
    }
    let user = result.unwrap();
    let pass = get_digest(&info.password);
    if user.passwd != pass {
        return Ok(SResult::new(1, "wrong password", "").to_string());
    }
    Ok(SResult::default().to_string())
}
