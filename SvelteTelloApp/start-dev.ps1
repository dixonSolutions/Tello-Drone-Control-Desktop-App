# Tello Drone Control - Development Start Script

Write-Host "Starting Tello Drone Control Development Environment..." -ForegroundColor Green

# Ensure we're in the right directory
Set-Location "C:\Users\Rat Rad\Projects\SideProjects\TelloDrone\SvelteTelloApp"

# Refresh environment variables
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")

Write-Host "Starting Tauri dev server..." -ForegroundColor Cyan
Write-Host "This will take 5-10 minutes on first run while Rust compiles." -ForegroundColor Yellow
Write-Host "The desktop window will open automatically when ready." -ForegroundColor Yellow
Write-Host ""

# Start the dev server
npm run tauri:dev

