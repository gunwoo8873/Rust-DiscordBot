use std::env;
use dotenv::dotenv;

use serenity::prelude::*;

use crate::handlers::ping::Handler;

#[tokio::main]
pub async fn discord_run() {
  dotenv().ok();
  let token = env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");
  println!("Token: {}", token);
  
  let intents = GatewayIntents::GUILD_MESSAGES
  | GatewayIntents::DIRECT_MESSAGES
  | GatewayIntents::GUILD_MESSAGE_REACTIONS;

  let mut client = Client::builder(&token, intents)
  .event_handler(Handler)
  .await
  .expect("Err creating client");

  if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
  }
}
