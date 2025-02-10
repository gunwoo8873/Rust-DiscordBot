use std::env;

use crate::commands;

use serenity::async_trait;
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::builder::{CreateInteractionResponseMessage, CreateInteractionResponse};
use serenity::prelude::*;

pub struct ReadyHandler;

#[async_trait]
impl EventHandler for ReadyHandler {
  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    // Slash command options
    if let Interaction::Command(command) = interaction {
      let content = match command.data.name.as_str() {
        "ping" => Some(commands::ping::run(&command.data.options())),
        "uptime" => Some(commands::uptime::run(&command.data.options())),
        "help" => Some(commands::help::run(&command.data.options())),
        "atta" => Some(commands::attachmentinput::run(&command.data.options())),
        _ => Some("Not implemented : (".to_string()),
      };

      if let Some(content) = content {
        let data = CreateInteractionResponseMessage::new().content(content);
        let builder = CreateInteractionResponse::Message(data);

        if let Err(why) = command.create_response(&ctx.http, builder).await{
          println!("Cannot respond to slash command: {}", why);
        }
      }
    }
  }

  // Client on running to ready output handler
  async fn ready(&self, ctx: Context, ready: Ready) {
    if let Some(shard) = ready.shard {
      println!("{} is connected on shard {}/{}", ready.user.name, shard.id, shard.total);
    }

    let guild_id = GuildId::new(
      env::var("GUILD_ID")
      .expect("Expected GUILD_ID in environment")
      .parse()
      .expect("GUILD_ID must be an integer"),
    );

    let commands = guild_id
    .set_commands(&ctx.http, vec![
      commands::ping::register(),
      commands::uptime::register(),
      commands::help::register(),
      // commands::aws::register(),
      commands::attachmentinput::register(),
    ])
    .await;
    
    println!("Guild slash commands: {:#?}", commands);
  }
}