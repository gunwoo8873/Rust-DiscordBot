use std::env;
use dotenv::dotenv;

use std::time::Duration;
use tokio::time::sleep;

use serenity::prelude::*;
use crate::bot::handlers::Handler;
use crate::bot::aws_client::aws_run;
use crate::sql::client::Database;

#[tokio::main]
pub async fn run() {
  dotenv().ok();

  // AWS Client run and get config
  aws_run().await;

  // Database connection initialization
  let database = Database::conn().await.expect("Failed to connect to database");
  database.conn_test().await.expect("Failed to test database connection");

  // Discord bot token
  let token = env::var("DISCORD_BOT_TOKEN").expect("Expected a discord bot token in the environment");
  
  // let intents = GatewayIntents::GUILD_MESSAGES
  //   | GatewayIntents::DIRECT_MESSAGES
  //   | GatewayIntents::GUILD_MESSAGE_REACTIONS;

  // Discord bot client connect and slash commands handler call
  let mut client = Client::builder(&token, GatewayIntents::empty())
  .event_handler(Handler)
  .await
  .expect("Error creating handler");

  let manager = client.shard_manager.clone();

  tokio::spawn(async move {
    loop {
      // Discord bot run time date log get time setting is 60s -> 120s
      sleep(Duration::from_secs(120)).await;

      // manager is current discord bot shard status
      let shard_runners = manager.runners.lock().await;
      
      // Manager shard runners log output
      for (id, runner) in shard_runners.iter() {
        println!("Shard ID {} / {} / {:?}", id, runner.stage, runner.latency);
      }
    }
  });

  if let Err(error) = client.start_shards(2).await {
    println!("Client error: {:?}", error);
  }
}