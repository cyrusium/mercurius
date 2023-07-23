use crate::routes::ApiError;
use actix_web::{get, web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("search").service(route));
}

#[get("/route")]
async fn route() -> Result<HttpResponse, ApiError> {
  todo!("Implement route search")
}
