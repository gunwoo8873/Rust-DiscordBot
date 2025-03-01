$MYSQL_SERVICE = "MySQL92"

function Manage-Service {
  do {
    Write-Host "`n=== MySQL Service Control ==="
    Write-Host "1. Check Service Status"
    Write-Host "2. Start Service"
    Write-Host "3. Stop Service"
    Write-Host "4. Exit"

    $choice = Read-Host "Select an option (1-4)"

    switch ($choice) {
      "1" {
        $service = Get-Service -Name $MYSQL_SERVICE -ErrorAction SilentlyContinue
        if ($service) {
          Write-Host "Service '$MYSQL_SERVICE' is currently: $($service.Status)"
        } else {
          Write-Host "Service '$MYSQL_SERVICE' not found."
        }
      }
      "2" {
        Start-Service -Name $MYSQL_SERVICE -ErrorAction SilentlyContinue
        if ($?) { Write-Host "Service '$MYSQL_SERVICE' started successfully." }
        else { Write-Host "Failed to start service '$MYSQL_SERVICE'." }
      }
      "3" {
        Stop-Service -Name $MYSQL_SERVICE -Force -ErrorAction SilentlyContinue
        if ($?) { Write-Host "Service '$MYSQL_SERVICE' stopped successfully." }
        else { Write-Host "Failed to stop service '$MYSQL_SERVICE'." }
      }
    "4" {
        Write-Host "Exiting..."
        return
      }
      default {
        Write-Host "Invalid selection. Please choose a number between 1 and 4."
      }
    }
  } while ($true)
}

Manage-Service