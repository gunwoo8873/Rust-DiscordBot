use std::env::var;
use std::time::Duration;
use dotenv::dotenv;

use serenity::prelude::*;

use tokio::time::sleep;

use crate::handlers::{
  event::Handler, 
  ready::ReadyHandler,
};


#[tokio::main]
pub async fn discord_run() {
  // Getting to 
  dotenv().ok();

  // Discord bot client token
  let token = var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");
  println!("Token: {}", token);
  
  // let intents = GatewayIntents::GUILD_MESSAGES
  //   | GatewayIntents::DIRECT_MESSAGES
  //   | GatewayIntents::GUILD_MESSAGE_REACTIONS;

  // Client event handler
  let mut client = Client::builder(token, GatewayIntents::empty())
  .event_handler(Handler)
  .event_handler(ReadyHandler)
  .await
  .expect("Error creating handler");

  let manager = client.shard_manager.clone();

  tokio::spawn(async move {
    loop {
      sleep(Duration::from_secs(30)).await;

      let shard_runners = manager.runners.lock().await;

      for (id, runner) in shard_runners.iter() {
        println!("Shard ID {} is {} with a latency of {:?}", id, runner.stage, runner.latency);
      }
    }
  });

  if let Err(why) = client.start_shards(2).await {
    println!("Client error: {:?}", why);
  }
}