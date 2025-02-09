use std::env;

use crate::commands;

use serenity::builder::{CreateInteractionResponseMessage, CreateInteractionResponse};
use serenity::async_trait;
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

pub struct ReadyHandler;

#[async_trait]
impl EventHandler for ReadyHandler {
  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
      let content = match command.data.name.as_str() {
        "ping" => Some(commands::ping::run(&command.data.options())),
        _ => Some("Not implemented : (".to_string()),
      };

      if let Some(content) = content {
        let data = CreateInteractionResponseMessage::new().content(content);
      }
    }
  }

  async fn ready(&self, _: Context, ready: Ready) {
    if let Some(shard) = ready.shard {
      println!("{} is connected on shard {}/{}", ready.user.name, shard.id, shard.total); // user.name is Discord bot name
    }

    // GUILD_ID is server in the server id int
    let guild_id = GuildId::new(
      env::var("GUILD_ID").expect("Expected guild id in environment").parse().expect("Guild id must be an integer") 
    );
  }
}