use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub struct ReadyHandler;

#[async_trait]
impl EventHandler for ReadyHandler {
  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name); // user.name is Discord bot name
  }
}