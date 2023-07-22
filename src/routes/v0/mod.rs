mod user;

pub use super::ApiError;
use actix_web::{web, HttpResponse, Responder};
use futures::FutureExt;
use serde_json::json;

const VERSION: u8 = 0;
const CODENAME: &str = "alpha";

lazy_static::lazy_static! {
  static ref MIN_API_VERSION_AVAILABLE: u8 = dotenvy::var("MIN_API_VERSION_AVAILABLE").unwrap().parse::<u8>().unwrap();
}

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
  if *MIN_API_VERSION_AVAILABLE > VERSION {
    cfg.service(
      web::scope(CODENAME).wrap_fn(|req, _srv| {
        async {
          Ok(req.into_response(
            HttpResponse::Gone()
            .content_type("application/json")
            .body(r#"{"error":"api_deprecated","description":"You are using an application that uses an outdated version of Hermes's API. Please either update it or switch to another application."}"#)
          ))
        }.boxed_local()
      })
    );
  } else {
    cfg.service(
      actix_web::web::scope(CODENAME)
        .configure(user::config)
        .default_service(web::get().to(index_get)),
    );
  }
}

async fn index_get() -> impl Responder {
  let data = json!({
    "name": format!("hermes-mercurius/{CODENAME}"),
    "version": env!("CARGO_PKG_VERSION"),
    "documentation": format!("https://docs.{DOMAIN}/{CODENAME}", DOMAIN = dotenvy::var("DOMAIN").unwrap()),
    "about": "Welcome traveler!"
  });

  HttpResponse::Ok().json(data)
}
