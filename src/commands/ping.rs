use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn response(_options: &[ResolvedOption]) -> String {
  "".to_string()
}

pub fn request() -> CreateCommand {
  CreateCommand::new("ping").description("First discord bot command")
}