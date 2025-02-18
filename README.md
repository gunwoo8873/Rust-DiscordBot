# Discord Bot and AWS Resource Control Project

## Project Information
* **Member**  
  `PITLANE`
* **Development Date**  
  `2025. 01. ~ **`
* **Description**  
  `This project is a Discord bot that checks the status of AWS cloud systems, built using Rust.`

## Project Setup Guide

### Cargo Dependencies
* **Rustup** : Requires `nightly` toolchain version.
* **Serenity** : Discord bot framework for Rust.
* **Dotenv** : Reads `.env` file and integrates with `std::env`.
* **Tokio** : Provides async threading features.
* **MySQL** : Used for storing global slash command data.

  ```toml
  # Path : ./discord_bot/Cargo.toml

  # Discord bot dependencies
  [dependencies.serenity]
  default-features = false
  version = "0.12.4"
  features = [
      "builder",
      "chrono",
      "client",
      "collector",
      "framework",
      "gateway",
      "http",
      "model",
      "standard_framework",
      "utils",
      "rustls_backend",
  ]

  # Async HTTP library
  [dependencies.tokio]
  version = "1.43.0"
  features = ["full"]

  # Environment variable reader
  [dependencies.dotenv]
  version = "0.15.0"

  # Async database client
  [dependencies.sqlx]
  default-features = false
  version = "0.8.3"
  features = ["runtime-tokio", "mysql", "macros", "time"]
  ```

  > This project connects a Discord bot with a MySQL database.

### Configuration
* Environment Variables Create a .env file in the Discord-Bot/ directory with the following content:
  
  ```env
  # Path : /Discord-Bot/.env
  DISCORD_BOT_TOKEN=YOUR_DISCORD_BOT_TOKEN
  GUILD_ID=YOUR_DISCORD_GUILD_ID
  RUST_LOG=debug

  MYSQL_HOSTNAME=YOUR_MYSQL_DB_HOSTNAME
  MYSQL_PORT=YOUR_MYSQL_DB_PORT
  MYSQL_USERNAME=YOUR_MYSQL_DB_USERNAME
  MYSQL_PASSWORD=YOUR_MYSQL_DB_PASSWORD
  MYSQL_DATABASE_NAME=YOUR_MYSQL_DB_NAME
  ```

* AWS Configuration Configure AWS credentials by modifying the AWS CLI configuration file:
  ```config
  # Path : /Discord-Bot/config and ~/.aws/config
  [default]
  region = YOUR_MAIN_AWS_REGION

  [profile dev]
  region = YOUR_DEV_AWS_REGION

  [profile test]
  region = YOUR_TEST_AWS_REGION
  ```
  > This configuration sets the default AWS region and additional profiles for development and testing.

### Build and Run
* Build the project
  ```bash
  # Navigate to the project directory
  cd ./discord_bot

  # Build the project in release mode
  cargo build --release
  ```

* Run the bot
  ```bash
  cargo run
  ```
  > Ensure that the .env file is properly configured before running the bot.

## AWS SDK Reference
* For more information on AWS SDK for Rust, refer to the official documentation: [AWS SDK Rust Guide](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credentials.html)