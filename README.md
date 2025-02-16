# Solo ETC Project

## Project information
* Member  
  `PITLANE`
* Dev date  
  `2025. 01. ~ **`
* Discriotion  
  `This project is Discord bot and AWS cloud system status checking for on the rust language`

## Project useing for setup guide
### Discord
* **Config**
  ```env
  # Path : /Discord-Bot/.env
  DISCORD_BOT_TOKEN=YOUR_DISCORD_BOT_TOKEN
  GUILD_ID=YOUR_DISCORD_GUILD_ID
  RUST_LOG=debug

  MYSQL_HOSTNAME=YOUR_MYSQL_HOSTNAME
  MYSQL_PORT=YOUR_MYSQL_PROT
  MYSLQ_PASSWORD=YOUR_MYSQL_PASSWORD
  ```

### AWS
* **Config**
  ```config
  # Path : /Discord-Bot/
  [default]
  region = YOUR_MAIN_AWS_REGION

  [profile dev]
  region = YOUR_DEV_AWS_REGION

  [profile test]
  region = YOUR_TEST_AWS_REGION
  ```

### Cargo lib
* serenity : Discord bot for rust programing language
* dotenv : read to .env file and need to add std::env
* tokio : async thread feture

```toml
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

[dependencies.tokio]
version = "1.43.0"
features = ["full"]

[dependencies.dotenv]
version = "0.15.0"
```
