$TOKEN = Join-Path "Token"
$FILECHECKE = Join-Path "FileCheck"

function Token {
  if ($Token -eq $null) {
    Write-Output "Token is not set"
  } 
  else {
    
  }
}

function Display-Options {
  $OPTIONS = @(
    "Token",
    "File Check"
  )

  for ($i = 0; $i -lt $OPTIONS.Length; $i++) {
    Write-Host "$i : $($OPTIONS[$i])"
  }
}
Display-Options