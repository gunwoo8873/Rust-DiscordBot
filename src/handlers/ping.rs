use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

pub struct PingHandler;

#[async_trait]
impl EventHandler for PingHandler {
  async fn message(&self, ctx: Context, msg: Message) {
    if msg.content == "ping" {
      if let Err(why) = msg.channel_id.say(&ctx.http, "pong").await {
        println!("Error sending message: {:?}", why);
      }
    }
  }
}