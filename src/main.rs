extern crate crypto;
use crate::config::ServerConfig;
use actix_web::{
    get,
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use color_eyre::Result;
use dotenv::dotenv;
use handlers::*;
use tracing::info;

mod config;
mod db;
mod error;
mod handlers;
mod utils;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let config = ServerConfig::from_env()?;
    config.init_log()?;
    info!("{:?}", config);
    let pgpool = db::get_pgpool(&config.database_url).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pgpool.clone()))
            .wrap(Logger::default())
            .service(hello)
            .route("/user", web::post().to(add_user))
            .route("/user/{id}", web::get().to(get_user_by_name))
            .route("/login", web::post().to(login))
            .service(web::scope("/u").service(add_url))
    })
    .bind(&config.listen)?
    .run()
    .await?;

    Ok(())
}
