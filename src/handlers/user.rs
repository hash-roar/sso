use std::borrow::Borrow;

use crypto::{sha2::Sha256, digest::Digest};
use crate::{db::{User}, error::SError};
use actix_web::{web, Responder, Result};
use sqlx::PgPool;
use tracing::{error, instrument};

#[instrument(skip(db_pool))]
pub async fn add_user(
    user_info: web::Json<User>,
    db_pool: web::Data<PgPool>,
) -> Result<impl Responder> {
    let mut user = user_info.into_inner();
    user.passwd = get_digest(&user.passwd);
    user.add(db_pool.borrow()).await.map_err(|e|{
        error!("error happen:{:#?}",e);
        SError::ServerError
    })?;
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


fn get_digest(data:&str) -> String{
    let mut sha = Sha256::new();
    sha.input_str(data);
    sha.result_str()
}

#[cfg(test)]
mod  tests{
    use crypto::{sha2::Sha256, digest::Digest};

    #[test]
    fn test_sha256(){
        let str = "this is str";
        let mut sha = Sha256::new();
        sha.input_str(str);
        println!("{}",sha.result_str());
    }

}