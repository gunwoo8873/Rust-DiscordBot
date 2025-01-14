#############################################
# Project: Github-Repository-Manager
# 
#
#############################################

# Need to run the script in the root directory
$GITHUB = "bin/GithubRepo.ps1"
$HELP = "bin/Help.ps1"
$EDIT = "bin/Edit.ps1"
$OPTIONS = "bin/Options.ps1"
$FILECHECK = "config/FileCheck.ps1"
$TOKEFILE = "config/token.ps1"

function Display_Menu() {
  # Menu list arr output
  $MENUS = @("Repository List","Repository Clone","Options","Exit")

  for ($i = 0; $i -lt $MENUS.Length; $i++) {
    Write-Output "$i : $($MENUS[$i])"
  }
}

function Process_Select() {
  param (
    [int]$SELECT_INPUT_NUMBER
  )

  switch ([int]::TryParse($SELECT_INPUT_NUMBER, [ref]$null)) {
    $true {
      switch ([int]$SELECT_INPUT_NUMBER) {
        0 { 
          Write-Output "Exit" 
        }
        1 {
          Write-Output "Exit"
        }
        2 {
          Write-Output "Exit" 
        }
        3 {
          Write-Output "Exit" 
        }
        default {
          Write-Output "Please select the correct menu number" 
        }
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

      if ([int]$SELECT_INPUT_NUMBER -eq 3) {
        break
      }
    } 
    else {
      Write-Output "Please select the correct menu number"
    }
  }
}
Select_Menus