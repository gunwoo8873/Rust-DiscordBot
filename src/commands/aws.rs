use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;

pub fn register() -> CreateCommand {
  CreateCommand::new("aws")
  .description("AWS using for service control and checking for manager bot")
  .add_option(CreateCommandOption::new(
    CommandOptionType::String, "EC2", ""
    
  ))
  .add_option(CreateCommandOption::new(
    CommandOptionType::String, "S3", ""
  ))
  .add_option(CreateCommandOption::new(
    CommandOptionType::String, "EKS", ""
  ))
  .add_option(CreateCommandOption::new(
    CommandOptionType::String, "IAM", ""
  ))
  .add_option(CreateCommandOption::new(
    CommandOptionType::String, "VPC", ""
  ))
  .add_option(CreateCommandOption::new(
    CommandOptionType::String, "Lambda", ""
  ))
}