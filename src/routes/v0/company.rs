use crate::routes::ApiError;
use actix_web::{patch, post, web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("company")
      .service(create_user)
      .service(update_name)
      .service(update_email)
      .service(update_password),
  );
}

#[post("/create")]
async fn create_user() -> Result<HttpResponse, ApiError> {
  todo!("Implement company creation")
}

#[patch("/update/name")]
async fn update_name() -> Result<HttpResponse, ApiError> {
  todo!("Implement company name update")
}

#[patch("/update/email")]
async fn update_email() -> Result<HttpResponse, ApiError> {
  todo!("Implement company email update")
}

#[patch("/update/password")]
async fn update_password() -> Result<HttpResponse, ApiError> {
  todo!("Implement company password update")
}

#[post("/create/post")]
async fn create_post() -> Result<HttpResponse, ApiError> {
  todo!("Implement post creation")
}

#[post("/create/bus")]
async fn create_bus() -> Result<HttpResponse, ApiError> {
  todo!("Implement bus creation")
}

#[post("/create/route")]
async fn create_route() -> Result<HttpResponse, ApiError> {
  todo!("Implement route creation")
}
