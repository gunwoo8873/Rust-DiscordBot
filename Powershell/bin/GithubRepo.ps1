$TOKENFILE = "config/token.ps1"
$APPFILE = "Run.ps1"

If (Test-Path $TOKENFILE)
{
    . $TOKENFILE
}
Else
{
    Write-Error ""
}