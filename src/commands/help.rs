use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

// pub fn run(options: &[ResolvedOption]) -> String {
//     
// }

pub fn register() -> CreateCommand {
  CreateCommand::new("help").description("Discord bot command support")
  .add_option(CreateCommandOption::new(CommandOptionType::String, "ping", "a")
    .add_sub_option(sub_option)
  )
  .add_option(CreateCommandOption::new(CommandOptionType::String, "aws", "b"))
}