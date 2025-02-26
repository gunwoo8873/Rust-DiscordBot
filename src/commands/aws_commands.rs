use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

use aws_config::meta::region::Region;
use aws_sdk_ec2::Client::Instance;

struct Aws {
  pub region: String,
  pub command: String,
  pub action: String,
  pub number: i8,
}

impl Aws {
  async fn ec2_list() {
    println!("EC2 List");
  }
}

pub fn response(_options: &[ResolvedOption]) {
  if let Some(ResolvedOption { value, .. }) = _options.get(0) {
    match value {
      ResolvedValue::String(s) => println!("{}", s),
      ResolvedValue::Number(n) => println!("{}", n),
      _ => println!("Unknown"),
    }
  }
}

// Feture layout : Maincommand -> Subcommandgroup -> Subcommand
pub fn request() -> CreateCommand {
  CreateCommand::new("aws").description("AWS service access control")
  .add_option(CreateCommandOption::new(CommandOptionType::SubCommand, "region", "Current Region"))
  .add_option(CreateCommandOption::new(CommandOptionType::SubCommandGroup, "ec2", "EC2 Instance Command")
    .add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommand, "list", "Instance list")
      .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "filter", "Instance scale filter")
        .add_string_choice("all", "All Instance")
        .add_string_choice("running", "Running Instance")
        .add_string_choice("stopped", "Stopped Instance")
        .add_string_choice("terminated", "Terminated Instance")
        .required(true)
      )
    )
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