# Discord bot and AWS resource control Project

> [!WARNING]
> **Current in development to test version**  
> **Errors may occur while using it, and do not push the .env file to the github repository**  
> **Make your Discord bot token, channel ID and AWS all config private or use a database to verify security**


### 💡 **Why Rust?**
- **High Performance and Memory Safety**: Rust provides **high-performance asynchronous processing** 
  and **memory safety** without a garbage collector, making it ideal for **Discord Bot command handling**.
- **Concurrent Command Handling**: Using **Tokio** runtime, efficiently manages **non-blocking asynchronous requests** 
  for fast and stable bot performance.
- **Cloud Integration**: Integrated with **AWS SDK for Rust** to control **EC2 instances** securely using **IAM Role** 
  and **temporary security tokens**.

<figure align="center">
  <img src="./img/Discord-bot-slashcommand.png" alt="" width=600>
  <img src="./img/Discord-bot-slashcommand-input.png" alt="" width=600>
</figure>

## Project information
* Member :  
  [PITLANE](https://github.com/gunwoo8873), [zxcxz01](https://github.com/zxcxz01), [sssoeun](https://github.com/sssoeun), [Byun-Sung-Ho](https://github.com/Byun-Sung-Ho)
* Dev date :  
  2025. 01. ~ **
* Descriotion :  
  This project is Discord bot and AWS cloud system status checking for on the rust language
* Docs :  
  [Serenity Reference](https://github.com/serenity-rs)  
  [Discord Document](https://discord.com/developers/docs/intro)  
  [1. Slash Commands sample](./Docs/Slash_Command.md)  

> [!NOTE]
> **Want do looking for other projects click to url link**  
> [1. Team Project : AWS Service bedrock agent](https://github.com/Byun-Sung-Ho/appCodeForRocket)  
> [2. Discord-Bot NET.ver](https://github.com/gunwoo8873/NET-DiscordBot)

## Docker architecture
<figure align="center">
  <img src="./img/Docker-architecture.png" alt="" width=600>
</figure>

## Discord architecture
<figure align="center">
  <img src="./img/Discord-architecture.png" alt="" width=600>
</figure>

## Getting start guide
### Cargo lib
* **rustup** : need to toolchain version is `nightly`
* **serenity** : Discord bot for rust programing language
* **dotenv** : read to .env file and need to add std::env
* **tokio** : async thread feture
* **mysql** : using for global slash command data save

  ### Cargo.toml lib
  
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

  ### Project build
  ```bash
  # Path : ./discord_bot
  cargo build --release
  ```

  ### Local application run 
  ```bash
  cargo run
  ```

### Discord
> That Discord and MySQL database client connected

  * ### Config
    ```env
    # Path : /Discord-Bot/.env
    # Main Discord bot connection variables
    DISCORD_BOT_TOKEN=YOUR_DISCORD_BOT_TOKEN
    GUILD_ID=YOUR_DISCORD_GUILD_ID
    RUST_LOG=debug

    # Main MySQL connection variables
    MYSQL_HOSTNAME=YOUR_MYSQL_DB_HOSTNAME
    MYSQL_PORT=YOUR_MYSQL_DB_PORT
    MYSQL_USERNAME=YOUR_MYSQL_DB_USERNAME
    MYSQL_PASSWORD=YOUR_MYSQL_DB_PASSWORD
    MYSQL_DATABASE_NAME=YOUR_MYSQL_DB_NAME
  
    # Sub MySQL connection URL
    MYSQL_URL=YOUR_MYSQL_DB_URL
    ```


### AWS
> To be reference web site is [AWS SDK Rust](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credentials.html)

  * ### Config
    ```config
    # Path : ~/.ssh/config
    [default]
    region = YOUR_MAIN_BASE_REGION
    output = json

    [profile.dev]
    region = YOUR_DEV_BASE_REGION
    output = json

    [profile.release]
    region = YOUR_RELEASE_BASE_REGION
    output = json
    
    [profile.discord-bot]
    region = YOUR_DISCORD-BOT_BASE_REGION
    output = json
    ```

    ```config
    # Path : ~/.aws/credentials
    [default]
    aws_access_key_id = YOUR_AWS_IAM_ACCESS_KEY
    aws_secret_access_key = YOUR_AWS_IAM_SECRET_ACCESS_KEY
    
    [dev]
    aws_access_key_id = YOUR_AWS_IAM_ACCESS_KEY
    aws_secret_access_key = YOUR_AWS_IAM_SECRET_ACCESS_KEY

    [release]
    aws_access_key_id = YOUR_AWS_IAM_ACCESS_KEY
    aws_secret_access_key = YOUR_AWS_IAM_SECRET_ACCESS_KEY

    [discord-bot]
    aws_access_key_id = YOUR_AWS_IAM_ACCESS_KEY
    aws_secret_access_key = YOUR_AWS_IAM_SECRET_ACCESS_KEY
    ```

  * ### AWS CLI Install
    [AWS CLI](https://docs.aws.amazon.com/ko_kr/cli/latest/userguide/getting-started-install.html)  
    
  * ### AWS config prifle list command
    ```bash
    aws configure list-profiles
    ```

  * ### AWS Instance JSON format output command
    ```bash
    aws ec2 describe-instances
    ```
