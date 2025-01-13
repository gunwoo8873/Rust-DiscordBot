$HELP = "bin/Help.ps1"
$TOKEFILE = "config/token.ps1"

function Select_Menus() {
    # Menu list arr output
    $MENUS = @("Repository List","Repository Clone","Options","Exit")
    $index = 0;

    foreach ($index in 0..($MENUS.Length - 1)) {
        Write-Output "$index : $($MENUS[$index])"
    }

    Read-Host "The Select menu to one : " $Select_Number
    Write-Host "$Select_Number"
}
Select_Menus