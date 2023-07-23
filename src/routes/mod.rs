use actix_web::http::StatusCode;
use actix_web::web;

pub use self::not_found::not_found;

mod index;
mod not_found;
pub mod v0;
pub mod v1;
pub mod v2; // maybe remove until its launch?
pub mod v3; // maybe remove until its launch?

pub fn root_config(cfg: &mut web::ServiceConfig) {
  cfg.service(index::index_get);
  cfg.configure(v0::config);
  cfg.configure(v1::config);
  cfg.configure(v2::config);
  cfg.configure(v3::config);
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
  #[error("Environment Error")]
  Env(#[from] dotenvy::Error),
  #[error("Database Error: {0}")]
  SqlxDatabase(#[from] sqlx::Error),
  #[error("Internal server error: {0}")]
  Xml(String),
  #[error("Deserialization error: {0}")]
  Json(#[from] serde_json::Error),
  #[error("Authentication Error: {0}")]
  CustomAuthentication(String),
  #[error("Invalid Input: {0}")]
  InvalidInput(String),
  #[error("Error while validating input: {0}")]
  Validation(String),
}

impl actix_web::ResponseError for ApiError {
  fn status_code(&self) -> StatusCode {
    match self {
      | ApiError::Env(..) => StatusCode::INTERNAL_SERVER_ERROR,
      | ApiError::SqlxDatabase(..) => StatusCode::INTERNAL_SERVER_ERROR,
      | ApiError::CustomAuthentication(..) => StatusCode::UNAUTHORIZED,
      | ApiError::Xml(..) => StatusCode::INTERNAL_SERVER_ERROR,
      | ApiError::Json(..) => StatusCode::BAD_REQUEST,
      | ApiError::InvalidInput(..) => StatusCode::BAD_REQUEST,
      | ApiError::Validation(..) => StatusCode::BAD_REQUEST,
    }
  }
}
