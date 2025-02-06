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
  # Path : Discord-Bot/.env
  DISCORD_BOT_TOKEN=YOUR_DISCORD_BOT_TOKEN
  ```

### AWS
* **Config**
  ```json
  // Path : Discord-Bot/src/service/config.json
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