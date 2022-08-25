use actix_web::{web, Result, Responder};
use serde::{ Deserialize};
use sqlx::PgPool;
use tracing::debug;

#[derive(Debug,Deserialize)]
struct LoginData{
    name : String,
    password : String
}

#[instrument(skip(db_pool))]
pub async fn login(
    info: web::Json<LoginData>,
    db_pool : web::Data<PgPool>
) -> Result<impl Responder>{
  debug!()


}