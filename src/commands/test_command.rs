use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;

// pub fn run(_options: &[ResolvedOption]) -> String {
  
// }

// Feture layout : Maincommand -> Subcommandgroup -> Subcommand
pub fn register() -> CreateCommand {
  CreateCommand::new("AWS").description("AWS Command")
  .add_option(
    CreateCommandOption::new(CommandOptionType::SubCommandGroup, "EC2", "EC2 Command")
    .add_sub_option(
      CreateCommandOption::new(CommandOptionType::SubCommandGroup, "status", "EC2 instance status check")
      .add_sub_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "start", "Start EC2 instance")
      )
      .add_sub_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "restart", "restart EC2 instance")
      )
      .add_sub_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "stop", "stop EC2 instance")
      )
      .add_sub_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "delete", "delete EC2 instance")
      )
    )
  )
}