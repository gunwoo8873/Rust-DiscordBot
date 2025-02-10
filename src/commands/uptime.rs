use std::sync::OnceLock;
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

static START_TIME: OnceLock<Instant> = OnceLock::new();

pub fn run(_options: &[ResolvedOption]) -> String {
  let start_time = START_TIME.get_or_init(Instant::now);
  let now: DateTime<Utc> = Utc::now();
  let uptime = start_time.elapsed();

  format!(
    "Current time: {} / Run time: {}H {}M {}S",
    now.format("%Y-%m-%d %H:%M:%S"),
    uptime.as_secs() / 3600,
    (uptime.as_secs() % 3600) / 60,
    uptime.as_secs() % 60
  ).to_string()
}

pub fn register() -> CreateCommand {
  CreateCommand::new("time")
  .description("Get current time and run time at the time check")
}