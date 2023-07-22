use actix_web::{get, HttpResponse};
use serde_json::json;

#[get("/")]
pub async fn index_get() -> HttpResponse {
  let data = json!({
    "name": "hermes-mercurius",
    "version": env!("CARGO_PKG_VERSION"),
    "documentation": format!("https://docs.{DOMAIN}", DOMAIN = dotenvy::var("DOMAIN").unwrap()),
    "about": "Welcome traveler!"
  });

  HttpResponse::Ok().json(data)
}
