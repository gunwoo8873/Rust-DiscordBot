use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;

// pub fn run(_options: &[ResolvedOption]) -> String {
  
// }

// Feture layout : Maincommand -> Subcommandgroup -> Subcommand
pub fn register() -> CreateCommand {
  CreateCommand::new("aws").description("AWS Command")
  .add_option(CreateCommandOption::new(CommandOptionType::SubCommandGroup, "group_a", "Group A Command")
    .add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommand, "sub_a", "Sub A")
      .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "param_a", "Param A"))
    )
    .add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommand, "sub_b", "Sub B")
      .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "param_a", "Param A"))
    )
  )
}