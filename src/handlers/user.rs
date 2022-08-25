use crate::{db::User, error::SError};
use actix_web::{web, Error, HttpResponse, Responder, Result};
use sqlx::PgPool;
use tracing::{error, instrument};

pub async fn add_user(
    user_info: web::Json<User>,
    db_pool: web::Data<PgPool>,
) -> Result<impl Responder> {
    let user = user_info.into_inner();
    Ok(web::Json(user))
}

#[instrument(skip(db_pool))]
pub async fn get_user_by_name(
    name: web::Path<String>,
    db_pool: web::Data<PgPool>,
) -> Result<impl Responder> {
    let pool = db_pool.into_inner();
    let user = User::get_by_name(name.to_string(), pool.as_ref())
        .await
        .map_err(|e| {
            error!("error happen:{:#?}", e);
            SError::ServerError
        })?;
    Ok(web::Json(user))
}
