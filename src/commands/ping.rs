use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
  "Test A Check".to_string()
}

pub fn register() -> CreateCommand {
  CreateCommand::new("ping").description("Test A command")
}