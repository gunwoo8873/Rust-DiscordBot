use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;
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

    if msg.content == "test" {
      let channel = match msg.channel_id.to_channel(&ctx).await{
        Ok(channel) => channel,
        Err(why) => {
          println!("Error getting channel: {:?}", why);
          return;
        }
      };

      let response = MessageBuilder::new()
      .push("Hello, ")
      .push_bold_safe(&msg.author.name)
      .push("!")
      .mention(&channel)
      .push("!")
      .build();

      if let Err(why) = msg.channel_id.say(&ctx.http, &response).await{
        println!("Error sending message: {:?}", why);
      }
    };
  }
}