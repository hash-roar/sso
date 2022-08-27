use std::borrow::Borrow;

use actix_web::{web, Responder, Result};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{debug, instrument};

use crate::db::{self};
use crate::error::SError;
use crate::handlers::SResult;

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

    let user = db::User::get_by_name(info.name.to_owned(), db_pool.borrow())
        .await
        .map_err(|e| {
            let error: SError = e.into();
            error
        })?;
    if user.passwd != info.password {
        return Ok(SResult::new(1, "wrong passwd", "").to_string());
    }
    Ok(SResult::default().to_string())
}
