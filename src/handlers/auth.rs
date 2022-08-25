use std::borrow::Borrow;

use actix_web::{web, Responder, Result};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{debug, instrument};

use crate::db::{self};
use crate::handlers::SResult;



#[derive(Debug, Deserialize)]
struct LoginData {
    name: String,
    password: String,
}

#[instrument(skip(db_pool))]
pub async fn login(
    info: web::Json<LoginData>,
    db_pool: web::Data<PgPool>,
) -> Result<impl Responder> {
    debug!("login data:{:?}", info);

    let user = db::User::get_by_name(info.name, db_pool.borrow())
        .await
        .map_err(|e| e.into())?;
    if user.passwd != info.password {}{
        SResult::new
    }
    Ok(())
}
