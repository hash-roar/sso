use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum SError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    ServerError,
}

impl ResponseError for SError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            Self::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
