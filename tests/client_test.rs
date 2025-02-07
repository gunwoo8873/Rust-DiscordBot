use std::env;
use dotenv::dotenv;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_token_check() {
    dotenv().ok();
    let test_token = env::var("DISCORD_BOT_TOKEN").unwrap();
    let token = env::var("DISCORD_BOT_TOKEN").unwrap();
    assert_eq!(token, test_token);
  }
}