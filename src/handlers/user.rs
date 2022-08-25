use crate::db::User;
use actix_web::{web, Error, HttpResponse, Responder, Result};
use sqlx::PgPool;


pub async fn add_user(
    user_info: web::Json<User>,
    db_pool: web::Data<PgPool>,
) -> Result<impl Responder> {
    let user = user_info.into_inner();
    Ok(web::Json(user))
}
