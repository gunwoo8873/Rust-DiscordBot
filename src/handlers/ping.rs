use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    if msg.content == "ping" {
      if let Err(why) = msg.channel_id.say(&ctx.http, "pong").await {
        println!("Error sending message: {:?}", why);
      }
    }
  }

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}