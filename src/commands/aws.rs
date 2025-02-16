use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;

// pub fn run(_options: &[ResolvedOption]) -> String {
// }

// Feture layout : Maincommand -> Subcommandgroup -> Subcommand
pub fn register() -> CreateCommand {
  CreateCommand::new("aws").description("AWS service access control")
  .add_option(CreateCommandOption::new(CommandOptionType::SubCommandGroup, "ec2", "EC2 관련 작업을 수행합니다.")
    .add_sub_option(
      CreateCommandOption::new(CommandOptionType::SubCommand, "status", "EC2 instance status check")
    )
    .add_sub_option(
      CreateCommandOption::new(CommandOptionType::SubCommand, "start", "EC2 instance start")
    )
    .add_sub_option(
      CreateCommandOption::new(CommandOptionType::SubCommand, "stop", "EC2 instance stop")
    )
    .add_sub_option(
      CreateCommandOption::new(CommandOptionType::SubCommand, "delete", "EC2 instance Delete")
    )
  )
  .add_option(CreateCommandOption::new(CommandOptionType::SubCommandGroup, "s3", "S3 관련 작업을 수행합니다.")
    .add_sub_option(
      CreateCommandOption::new(CommandOptionType::SubCommand, "list", "S3 bucket list load")
    )
    .add_sub_option(
      CreateCommandOption::new(CommandOptionType::SubCommand, "upload", "S3 for file upload")
    )
  )
  .add_option(CreateCommandOption::new(CommandOptionType::SubCommandGroup, "ecr", "ECR 관련 작업을 수행합니다.")
    .add_sub_option(
      CreateCommandOption::new(CommandOptionType::SubCommand, "list", "ECR repository list load")
    )
    .add_sub_option(
      CreateCommandOption::new(CommandOptionType::SubCommand, "delete", "ECR repository delete")
    )
  )
}