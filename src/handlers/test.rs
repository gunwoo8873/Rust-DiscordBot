use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::builder::CreateMessage;
use serenity::prelude::*;

pub struct TestHandler;

#[async_trait]
impl EventHandler for TestHandler {
  async fn message(&self, ctx: Context, msg: Message) {
    if msg.content == "test" {
      println!("Shard {}", ctx.shard_id);

      let builder = CreateMessage::new().content("Checking to test msg");
      let dm = msg.author.dm(&ctx, builder).await;

      if let Err(why) = dm {
        println!("Error sending message: {:?}", why);
      }
    }
  }
}