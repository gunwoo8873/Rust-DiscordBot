$MYSQL_SERVICE_NAME = "mysqld"

function Get-ServiceStatus {
  param {
    [string]$service_action
  }

  switch ($service_action) {
    "start" {
      Start-Service -Name $MYSQL_SERVICE_NAME -ErrorAction SilentlyContinue
      if ($?) { Write-Host "Service '$MYSQL_SERVICE_NAME' started successfully." }
      else { Write-Host "Failed to start service '$MYSQL_SERVICE_NAME'." }
    }
    "stop" {
      Stop-Service -Name $MYSQL_SERVICE_NAME -Force -ErrorAction SilentlyContinue
      if ($?) { Write-Host "Service '$MYSQL_SERVICE_NAME' stopped successfully." }
      else { Write-Host "Failed to stop service '$MYSQL_SERVICE_NAME'." }
    }
    "status" {
      $service = Get-Service -Name $MYSQL_SERVICE_NAME -ErrorAction SilentlyContinue
      if ($service) {
          Write-Host "Service '$MYSQL_SERVICE_NAME' is currently: $($service.Status)"
      } else {
          Write-Host "Service '$MYSQL_SERVICE_NAME' not found."
      }
    }
    default {
      Write-Host "Invalid action. Please use 'start', 'stop', or 'status'."
    }
  }
}