use super::DbError;
use ::chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{
    types::chrono::{self, TimeZone, Utc},
    PgPool,
};
use tracing::{debug, instrument, warn};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub uid: Option<i32>,
    pub nick_name: String,
    pub student_id: String,
    pub email: String,
    pub contact: String,
    pub passwd: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Url {
    pub id: Option<i32>,
    pub short_url: String,
    pub dest_url: String,
    pub time: DateTime<Local>,
}

impl User {
    #[instrument(skip(pool))]
    pub async fn add(&self, pool: &PgPool) -> Result<(), DbError> {
        debug!("insert {:?}", self);
        let result = sqlx::query(
            "INSERT INTO users(nick_name,student_id,email,contact,passwd) VALUES($1,$2,$3,$4,$5)",
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
    // #[instrument]
    // pub async fn get_by_id(uid: i32, pool: &PgPool) -> Result<Option<User>> {
    //     let mut user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE uid=?")
    //         .bind(uid)
    //         .fetch_all(pool)
    //         .await?;

    //     debug!("get user:{:?}", user.first());
    //     Ok(user.pop())
    // }

    #[instrument(skip(pool))]
    pub async fn get_by_name(name: String, pool: &PgPool) -> Result<User, DbError> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE nick_name = $1")
            .bind(name)
            .fetch_one(pool)
            .await?;

        debug!("get user:{:?}", user);
        Ok(user)
    }
}

impl Url {
    pub fn new(short: String, dest: String) -> Self {
        Url {
            id: None,
            short_url: short,
            dest_url: dest,
            time: Local::now(),
        }
    }
    #[instrument(skip(pool))]
    pub async fn add(&self, pool: &PgPool) -> Result<(), DbError> {
        debug!("insert {:?}", self);
        // let result =
        sqlx::query("INSERT INTO urls(short_url,dest_url,time) values ($1,$2,$3)")
            .bind(&self.short_url)
            .bind(&self.dest_url)
            .bind(&self.time)
            .execute(pool)
            .await?;
        Ok(())
    }
}
