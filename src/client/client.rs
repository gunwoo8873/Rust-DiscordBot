use std::env;
use std::time::Duration;
use dotenv::dotenv;

use serenity::prelude::*;

use tokio::time::sleep;

use crate::handlers::{
  // event::CommandHandler,
  ready::ReadyHandler,
};


#[tokio::main]
pub async fn discord_run() {
  dotenv().ok();

  // Discord bot token
  let token = env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");
  
  // let intents = GatewayIntents::GUILD_MESSAGES
  //   | GatewayIntents::DIRECT_MESSAGES
  //   | GatewayIntents::GUILD_MESSAGE_REACTIONS;

  // Client command handler [path : src/handlers]
  let mut client = Client::builder(token, GatewayIntents::empty())
  .event_handler(ReadyHandler)
  .await
  .expect("Error creating handler");

  let manager = client.shard_manager.clone();

  tokio::spawn(async move {
    loop {
      // Discord bot run time date log get time setting is 30s -> 60s
      sleep(Duration::from_secs(60)).await;

      // Shard manager is current discord bot status
      let shard_runners = manager.runners.lock().await;

      for (id, runner) in shard_runners.iter() {
        println!("Shard ID {} / {} / {:?}", id, runner.stage, runner.latency);
      }
    }
  });

  if let Err(why) = client.start_shards(2).await {
    println!("Client error: {:?}", why);
  }
}