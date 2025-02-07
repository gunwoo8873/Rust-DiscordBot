use std::env;
use dotenv::dotenv;

use serenity::prelude::*;

use crate::handlers::{
  ping::PingHandler, 
  ready::ReadyHandler,
  test::TestHandler
};

#[tokio::main]
pub async fn discord_run() {
  dotenv().ok();
  let token = env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");
  println!("Token: {}", token);
  
  let intents = GatewayIntents::GUILD_MESSAGES
  | GatewayIntents::DIRECT_MESSAGES
  | GatewayIntents::GUILD_MESSAGE_REACTIONS;

  let mut client = Client::builder(&token, intents)
  .event_handler(PingHandler)
  .event_handler(ReadyHandler)
  .event_handler(TestHandler)
  .await
  .expect("Error creating client");

  if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
  }
}
