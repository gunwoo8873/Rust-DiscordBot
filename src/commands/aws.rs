use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;
use serenity::prelude::*;

// pub fn run(_options: &[ResolvedOption]) -> String {
  
// }

struct InstanceMap;

impl TypeMapKey for InstanceMap {
  type Value = Arc<Mutex<HashMap<u64, String>>>;
}

// Feture layout : Maincommand -> Subcommandgroup -> Subcommand
pub fn register() -> CreateCommand {
  CreateCommand::new("aws").description("AWS service access control")
  .add_option(CreateCommandOption::new(CommandOptionType::SubCommandGroup, "ec2", "EC2 Instance Command")
    .add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommand, "list", "Instance list"))
    .add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommand, "status", "Instance Status")
      .add_sub_option(CreateCommandOption::new(CommandOptionType::Number, "instance_number", "Instance Number")
        .required(true)
      )
      .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "actions", "Instance Actions")
        .add_string_choice("start", "Start Instance")
        .add_string_choice("stop", "Stop Instance")
        .add_string_choice("reboot", "Reboot Instance")
        .add_string_choice("terminate", "Terminate Instance")
        .required(true)
      )
    )
  )
  .add_option(CreateCommandOption::new(CommandOptionType::SubCommandGroup, "s3", "S3 Bucket Command")
    .add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommand, "list", "S3 Bucket list"))
    .add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommand, "create", "S3 Bucket Create")
      .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "bucketname", "S3 Bucket name create").required(true))
    )
    .add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommand, "update", "S3 Bucket update")
      .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "bucketname", "S3 Bucket name to update").required(true))
    )
    .add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommand, "delete", "S3 Bucket delete")
      .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "bucketname", "S3 Bucket name to delete").required(true))
    )
  )
  .add_option(CreateCommandOption::new(CommandOptionType::SubCommandGroup, "ecr", "ECR Registry Command")
  )
}