$CURRENT_PATH = Get-Location .\Powershell

$BIN_PATH = Join-Path $CURRENT_PATH "bin"
$GITHUBREPO = Join-Path $BIN_PATH "GithubRepo"
$Help = Join-Path $BIN_PATH "Help.ps1"

$CONFIG_PATH = Join-Path $CURRENT_PATH "config"
$Options = Join-Path $CONFIG_PATH "Options"

function Display-Menus {
  $MENUS = @(
    "Github Repo",
    "Help"
    "Options"
    "Exit"
  )

  for ($i = 0; $i -lt $MENUS.Length; $i++) {
    Write-Host "$i : $($MENUS[$i])"
  }
}

function User-Input {
  param (
    [int] $USERINPUT
  )

  switch ([int]::TryParse($USERINPUT, [ref]$null)) {
    $true {
      switch ($USERINPUT) {
        0 { & $GITHUBREPO }
        1 { & $Help }
        2 { & $Options }
        3 { exit }
        default { Write-Host "Please select the correct menu number" }
      }
    }
    $false {
      Write-Host "Please select the correct menu number"
    }
  }
}

function Select-Menu {
  while ($true) {
    # The function Display-Menus is not defined in this script
    Display-Menus
    User-Input -USERINPUT (Read-Host "Select the menu number")
  }
}
Select-Menu