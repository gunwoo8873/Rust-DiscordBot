# Discord bot and AWS resource control Project

## Project information
* Member  
  `PITLANE`
* Dev date  
  `2025. 01. ~ **`
* Discriotion  
  `This project is Discord bot and AWS cloud system status checking for on the rust language`

## Project useing for setup guide
### Discord
> That Discord and MySQL database client connected 
* **Config**
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

### AWS
> To be reference web site is [AWS SDK rust](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credentials.html)
* **Config**
  ```config
  # Path : /Discord-Bot/config and ~/.ssh/config
  [default]
  region = YOUR_MAIN_AWS_REGION

  [profile dev]
  region = YOUR_DEV_AWS_REGION

  [profile test]
  region = YOUR_TEST_AWS_REGION
  ```

### Cargo lib
* rustup : need to toolchain version is `nightly`
* serenity : Discord bot for rust programing language
* dotenv : read to .env file and need to add std::env
* tokio : async thread feture
* mysql : using for global slash command data save

```toml
# Path : ./discord_bot/Cargo.toml

# Discord bot project base lib
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

# Http protocol async feature lib
[dependencies.tokio]
version = "1.43.0"
features = ["full"]

# Read to .env file lib
[dependencies.dotenv]
version = "0.15.0"

# Database async feature lib
[dependencies.sqlx]
default-features = false
version = "0.8.3"
features = ["runtime-tokio", "mysql", "macros", "time"]
```

## Project run cmd
```bash
# Path : ./discord_bot
cargo build --release
```

```bash
cargo run
```