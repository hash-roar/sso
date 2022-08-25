use color_eyre::Result;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, instrument, warn};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub uid: Option<i64>,
    pub nick_name: String,
    pub student_id: String,
    pub email: String,
    pub contact: String,
    pub passwd: String,
}

impl User {
    #[instrument]
    pub async fn add(&self, pool: &PgPool) -> Result<()> {
        debug!("insert {:?}", self);
        let result = sqlx::query(
            "INSERT INTO users(nick_name,student_id,email,contact,passwd) VALUES(?,?,?,?)",
        )
        .bind(&self.nick_name)
        .bind(&self.student_id)
        .bind(&self.email)
        .bind(&self.contact)
        .bind(&self.passwd)
        .execute(pool)
        .await?;
        if result.rows_affected() != 1 {
            warn!("add user affect rows:{}", result.rows_affected());
        }
        Ok(())
    }
    #[instrument]
    pub async fn get_by_id(uid: i64, pool: &PgPool) -> Result<Option<User>> {
        let mut user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE uid=?")
            .bind(uid)
            .fetch_all(pool)
            .await?;

        debug!("get user:{:?}", user.first());
        Ok(user.pop())
    }
}