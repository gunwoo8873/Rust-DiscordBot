# Github Repository Manager Script

### Project Information
* Member
    * `PITLANE`
* Dev start date
    * `2025-01.12`
* Description
    ```md
    This project the Powershell and Bash shell at Github Repository managerment script
    ```

## Project Manual
* Project Clone
    ```ps1
    git clone https://github.com/gunwoo8873/Repository-PSVersion.git
    ```
    
* Config User token and etc edit
    ```ps1
    # Path : config/token.ps1
    GITHUB_TOKEN = "your github access token"
    GITHUB_USERNAME = "your github username"
    GITHUB_URL = "your github overview url"
    ```

* PowerShell File Access
  ```ps
  Unblock-File -Path C:\Users\yourname\to\path\Run.ps1
  # or
  Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
  ```