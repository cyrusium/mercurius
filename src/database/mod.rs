pub mod models;

use simplelog::info;
use sqlx::{
  migrate,
  migrate::MigrateDatabase,
  mysql::{MySqlPool, MySqlPoolOptions},
  Connection, MySql, MySqlConnection,
};
use std::time::Duration;

pub async fn connect() -> Result<MySqlPool, sqlx::Error> {
  info!("Initializing database connection");
  let database_url =
    dotenvy::var("DATABASE_URL").expect("`DATABASE_URL` not in .env");
  let pool = MySqlPoolOptions::new()
    .min_connections(
      dotenvy::var("DATABASE_MIN_CONNECTIONS")
        .ok()
        .and_then(|x| x.parse().ok())
        .unwrap_or(0),
    )
    .max_connections(
      dotenvy::var("DATABASE_MAX_CONNECTIONS")
        .ok()
        .and_then(|x| x.parse().ok())
        .unwrap_or(16),
    )
    .max_lifetime(Some(Duration::from_secs(60 * 60)))
    .connect(&database_url)
    .await?;

  Ok(pool)
}

pub async fn check_for_migrations() -> Result<(), sqlx::Error> {
  let uri = dotenvy::var("DATABASE_URL").expect("`DATABASE_URL` not in .env");
  let uri = uri.as_str();
  if !MySql::database_exists(uri).await? {
    info!("Creating database...");
    MySql::create_database(uri).await?;
  }

  info!("Applying migrations...");

  let mut conn: MySqlConnection = MySqlConnection::connect(uri).await?;
  migrate!()
    .run(&mut conn)
    .await
    .expect("Error while running database migrations!");

  Ok(())
}
