$BIN_PATH = Test-Path "bin"
$EDIT_FILE = Test-Path "..\$BIN_PATH\Edit.ps1"
$GITHUB_FILE = Test-Path "..\$BIN_PATH\GithubRepo.ps1"
$HELP_FILE = Test-Path "..\$BIN_PATH\Help.ps1"
$OPTIONS_FILE = Test-Path "..\$BIN_PATH/Options.ps1"

$CONFIG_PATH = Test-Path "config"
$TOKEN_FILE = Test-Path ".\$CONFIG_PATH\token.ps1"
$FILECHECK_FILE = Test-Path ".\$CONFIG_PATH\FileCheck.ps1"

$PATH_CHECK = @($BIN_PATH, $CONFIG_PATH)
$FILE_CHECK = @($EDIT_FILE, $GITHUB_FILE, $HELP_FILE, $OPTIONS_FILE, $TOKEN_FILE, $FILECHECK_FILE)

foreach ($currentItemName in $PATH_CHECK) {
  if ($currentItemName -eq $false) {
    Write-Output "The path does not exist"
  }
}