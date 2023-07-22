use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use mercurius::routes;
use mercurius::utils::{args::get_args, env::parse_var, logging::init_logger};
use simplelog::{info, warn};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenvy::dotenv().unwrap();
  get_args();
  init_logger();
  check_env_vars().unwrap();

  // let search_config = search::SearchConfig {
  //   address: dotenvy::var("MEILISEARCH_ADDR").unwrap(),
  //   key: dotenvy::var("MEILISEARCH_KEY").unwrap(),
  // };

  // database::check_for_migrations()
  //   .await
  //   .expect("An error occurred while running migrations.");

  // // Database Connector
  // let pool = database::connect().await.expect("Database connection failed");
  // // Redis connector
  // let redis_cfg =
  //   Config::from_url(dotenvy::var("REDIS_URL").expect("Redis URL not set"));
  // let redis_pool = redis_cfg
  //   .create_pool(Some(Runtime::Tokio1))
  //   .expect("Redis connection failed");

  // let mut scheduler = scheduler::Scheduler::new();

  // // The interval in seconds at which the local database is indexed
  // // for searching.  Defaults to 1 hour if unset.
  // let local_index_interval = std::time::Duration::from_secs(
  //   parse_var("LOCAL_INDEX_INTERVAL").unwrap_or(3600),
  // );

  // let pool_ref = pool.clone();
  // let search_config_ref = search_config.clone();
  // scheduler.run(local_index_interval, move || {
  //   let pool_ref = pool_ref.clone();
  //   let search_config_ref = search_config_ref.clone();
  //   async move {
  //     info!("Indexing local database");
  //     let settings = IndexingSettings {
  //       index_local: true,
  //     };
  //     let result = index_projects(pool_ref, settings, &search_config_ref).await;
  //     if let Err(e) = result {
  //       warn!("Local project indexing failed: {:?}", e);
  //     }
  //     info!("Done indexing local database");
  //   }
  // });
  info!("Starting Actix HTTP server!");

  // Init App
  HttpServer::new(move || {
    App::new()
      .wrap(actix_web::middleware::Compress::default())
      .wrap(
        Cors::default()
          .allow_any_origin()
          .allow_any_header()
          .allow_any_method()
          .max_age(3600)
          .send_wildcard(),
      )
      // .wrap(
      //   RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
      //     .with_identifier(|req| {
      //       let connection_info = req.connection_info();
      //       let ip = String::from(
      //         if parse_var("CLOUDFLARE_INTEGRATION").unwrap_or(false) {
      //           if let Some(header) = req.headers().get("CF-Connecting-IP") {
      //             header.to_str().map_err(|_| ARError::Identification)?
      //           } else {
      //             connection_info.peer_addr().ok_or(ARError::Identification)?
      //           }
      //         } else {
      //           connection_info.peer_addr().ok_or(ARError::Identification)?
      //         },
      //       );

      //       Ok(ip)
      //     })
      //     .with_interval(std::time::Duration::from_secs(60))
      //     .with_max_requests(300)
      //     .with_ignore_key(dotenvy::var("RATE_LIMIT_IGNORE_KEY").ok()),
      // )
      .app_data(
        web::FormConfig::default().error_handler(|err, _req| {
          routes::ApiError::Validation(err.to_string()).into()
        }),
      )
      .app_data(
        web::PathConfig::default().error_handler(|err, _req| {
          routes::ApiError::Validation(err.to_string()).into()
        }),
      )
      .app_data(
        web::QueryConfig::default().error_handler(|err, _req| {
          routes::ApiError::Validation(err.to_string()).into()
        }),
      )
      .app_data(
        web::JsonConfig::default().error_handler(|err, _req| {
          routes::ApiError::Validation(err.to_string()).into()
        }),
      )
      .configure(routes::root_config)
      .default_service(web::get().to(routes::not_found))
  })
  .bind(("localhost", 8000))?
  .run()
  .await
}

// This is so that env vars not used immediately don't panic at runtime
fn check_env_vars() -> Result<(), ()> {
  let mut failed = false;

  fn check_var<T: std::str::FromStr>(var: &'static str) -> bool {
    let check = parse_var::<T>(var).is_none();
    if check {
      warn!(
        "Variable `{}` missing in dotenv or not of type `{}`",
        var,
        std::any::type_name::<T>()
      );
    }
    check
  }

  failed |= check_var::<String>("DOMAIN");

  failed |= check_var::<String>("LOG_PATH");
  failed |= check_var::<String>("LOG_FILE");
  check_var::<String>("LOGGING_LEVEL"); // Only warns the user if not set, as it has a default value
  check_var::<String>("FILE_LOGGING_LEVEL"); // Only warns the user if not set, as it has a default value

  failed |= check_var::<String>("BIND_ADDR");
  failed |= check_var::<u16>("BIND_PORT");

  failed |= check_var::<u8>("MIN_API_VERSION_AVAILABLE");

  if failed {
    Err(())
  } else {
    Ok(())
  }
}
