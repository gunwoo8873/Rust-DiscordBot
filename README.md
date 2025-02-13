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
  ```

### ~~AWS~~
* **Config**
  ```json
  // Path : /Discord-Bot/src/service/config.json
  {
    "AWS": {
      "REGION": "us-east-1",
      "IDENTITY_POOL_ID": "us-east-1:xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
      "USER_POOL_ID": "us-east-1_xxxxxxxxx",
      "APP_CLIENT_ID": "xxxxxxxxxxxxxxxxxxxxxxxxxx",
      "API_URL": "https://xxxxxxxxxx.execute-api.us-east-1.amazonaws.com/dev"
    }
  }
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
