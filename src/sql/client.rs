use std::env;
use dotenv::dotenv;

use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub struct Database {
  pool: MySqlPool,
}

impl Database {
  pub async fn conn() -> Result<Self, sqlx::Error> {
    dotenv().ok();
    let db_url = env::var("DOCKER_MYSQL_URL").expect("Expected a database url in the environment");
    let pool = MySqlPoolOptions::new()
    .max_connections(5)
    .connect(&db_url)
    .await?;

    Ok(Database { pool })
  }

  pub async fn conn_test(&self) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1")
    .execute(&self.pool)
    .await?;

    println!("\r
    ################################
    #  Connected to the database!  #
    ################################
    \r");
    Ok(())
  }
}