// use std::env;
// use dotenv::dotenv;

// use serenity::async_trait;
// use serenity::model::channel::Message;
// use serenity::model::gateway::Ready;
// use serenity::prelude::*;


// struct Handler;

// #[async_trait]
// impl EventHandler for Handler {
//   async fn message(&self, ctx: Context, msg: Message) {
//     if msg.content == "ping" {
//       if let Err(why) = msg.channel_id.say(&ctx.http, "pong").await {
//         println!("Error sending message: {:?}", why);
//       }
//     }
//   }

//   async fn ready(&self, _: Context, ready: Ready) {
//     println!("{} is connected!", ready.user.name);
//   }
// }

// #[tokio::main]
// async fn main() {
//   dotenv().ok();
//   let token = env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");
//   println!("Token: {}", token);
  
//   let intents = GatewayIntents::GUILD_MESSAGES
//   | GatewayIntents::DIRECT_MESSAGES
//   | GatewayIntents::GUILD_MESSAGE_REACTIONS;

//   let mut client = Client::builder(&token, intents)
//   .event_handler(Handler)
//   .await
//   .expect("Err creating client");

//   if let Err(why) = client.start().await {
//     println!("Client error: {:?}", why);
//   }
// }

use DiscordBot::bot::client::discord_run;

fn main() {
  discord_run();
}