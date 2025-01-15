#############################################
# Project: Github-Repository-Manager
# 
#
#############################################

function Display_Menu() {
  # Menu list arr output
  $MENUS = @("Repository Search","Repository List","Repository Clone","Help","Options","Exit")

  for ($i = 0; $i -lt $MENUS.Length; $i++) {
    Write-Output "$i : $($MENUS[$i])"
  }
}

function Process_Select() {
  param (
    [int]$SELECT_INPUT_NUMBER
  )

  $BIN_PATH = "bin"
  $EDIT_FILE = Join-Path $BIN_PATH "Edit.ps1"
  $GITHUB_FILE = Join-Path $BIN_PATH "GithubRepo.ps1"
  $HELP_FILE = Join-Path $BIN_PATH "Help.ps1"
  $OPTIONS_FILE = Join-Path $BIN_PATH "Options.ps1"

  $CONFIG_PATH = "config"
  $TOKEN_FILE = Join-Path $CONFIG_PATH "token.ps1"
  $FILECHECK_FILE = Join-Path $CONFIG_PATH "FileCheck.ps1"

  switch ([int]::TryParse($SELECT_INPUT_NUMBER, [ref]$null)) {
    $true {
      switch ([int]$SELECT_INPUT_NUMBER) {
        0 { $GITHUB_FILE }
        1 { $GITHUB_FILE }
        2 { $GITHUB_FILE }
        3 { $HELP_FILE }
        4 { $OPTIONS_FILE }
        5 { Write-Output "Exit" }
        default { Write-Output "Please select the correct menu number" }
      }
    }
    $false {
      Write-Output "Please select the correct menu number"
    }
  }
}

function Select_Menus() {
  while ($true) {
    # Display Menu List Output
    Display_Menu

    # User key input line  
    $SELECT_INPUT_NUMBER = Read-Host "The Select menu to one : "
    
    if ([int]::TryParse($SELECT_INPUT_NUMBER, [ref]$null)) {
      Process_Select -SELECT_INPUT_NUMBER ([int]$SELECT_INPUT_NUMBER)

      if ([int]$SELECT_INPUT_NUMBER -eq 5) {
        break
      }
    }
    else {
      Write-Output "Please select the correct menu number"
    }
  }
}
Select_Menus