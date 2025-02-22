use std::env;
use std::time::Duration;
use dotenv::dotenv;
use serenity::prelude::*;
use tokio::time::sleep;
use crate::http::ready::Handler;
use crate::http::aws_client::aws_run;

#[tokio::main]
pub async fn run() {
  dotenv().ok();

  // AWS Client run and get config
  aws_run().await;

  // Discord bot token
  let token = env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");
  
  // let intents = GatewayIntents::GUILD_MESSAGES
  //   | GatewayIntents::DIRECT_MESSAGES
  //   | GatewayIntents::GUILD_MESSAGE_REACTIONS;

  // Client command handler [path : src/handlers]
  let mut client = Client::builder(&token, GatewayIntents::empty())
  .event_handler(Handler)
  .await
  .expect("Error creating handler");

  let manager = client.shard_manager.clone();

  tokio::spawn(async move {
    loop {
      // Discord bot run time date log get time setting is 60s -> 120s
      sleep(Duration::from_secs(120)).await;

      // Shard manager is current discord bot status
      let shard_runners = manager.runners.lock().await;

      for (id, runner) in shard_runners.iter() {
        println!("Shard ID {} / {} / {:?}", id, runner.stage, runner.latency);
      }
    }
  });

  if let Err(error) = client.start_shards(2).await {
    println!("Client error: {:?}", error);
  }
}