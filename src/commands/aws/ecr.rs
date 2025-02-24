pub fn request() -> CreateCommand {
  CreateCommand::new("ecr").description("AWS service ECR access control")
  .add_option(CreateCommandOption::new(CommandOptionType::SubCommandGroup, "ecr", "ECR Registry Command")
  )
}