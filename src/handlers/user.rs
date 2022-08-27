use std::borrow::Borrow;

use crate::{db::User, error::SError};
use actix_web::{web, Responder, Result};
use crypto::{digest::Digest, sha2::Sha256};
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
    user.add(db_pool.borrow())
        .await
        .map_err(|_| SError::ServerError)?;
    // Ok(SResult::new(0, "", user).into())
    let result: String = SResult::new(0, "", user).into();
    Ok(result)
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

fn get_digest(data: &str) -> String {
    let mut sha = Sha256::new();
    sha.input_str(data);
    sha.result_str()
}

#[cfg(test)]
mod tests {
    use crypto::{digest::Digest, sha2::Sha256};

    #[test]
    fn test_sha256() {
        let str = "this is str";
        let mut sha = Sha256::new();
        sha.input_str(str);
        println!("{}", sha.result_str());
    }
}
