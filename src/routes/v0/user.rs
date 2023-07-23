use crate::routes::ApiError;
use actix_web::{patch, post, web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("user")
      .service(create_user)
      .service(update_name)
      .service(update_email)
      .service(update_password),
  );
}

#[post("/create")]
async fn create_user() -> Result<HttpResponse, ApiError> {
  todo!("Implement user creation")
}

#[patch("/update/name")]
async fn update_name() -> Result<HttpResponse, ApiError> {
  todo!("Implement user name update")
}

#[patch("/update/email")]
async fn update_email() -> Result<HttpResponse, ApiError> {
  todo!("Implement user email update")
}

#[patch("/update/password")]
async fn update_password() -> Result<HttpResponse, ApiError> {
  todo!("Implement user password update")
}
