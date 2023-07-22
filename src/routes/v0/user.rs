use crate::routes::ApiError;
use actix_web::{post, web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("user").service(create_user));
}

#[post("/create")]
async fn create_user() -> Result<HttpResponse, ApiError> {
  todo!("Implement user creation")
}
