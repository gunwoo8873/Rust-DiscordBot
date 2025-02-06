use std::env;
use dotenv::dotenv;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_token_check() {
    dotenv().ok();
    let token = env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");
    assert_eq!(token, env::var("DISCORD_BOT_TOKEN").unwrap());
  }
}