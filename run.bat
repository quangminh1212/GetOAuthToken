@echo off
setlocal

cd /d "%~dp0"

echo [INFO] Starting XLab Refresh Token

where node >nul 2>nul
if errorlevel 1 (
  echo [ERROR] Node.js is not installed or not in PATH.
  echo [INFO] Install Node.js LTS, then run this file again.
  pause
  exit /b 1
)

where npm >nul 2>nul
if errorlevel 1 (
  echo [ERROR] npm is not installed or not in PATH.
  pause
  exit /b 1
)

where cargo >nul 2>nul
if errorlevel 1 (
  if exist "%USERPROFILE%\.cargo\bin\cargo.exe" (
    set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
  ) else (
    echo [ERROR] Rust Cargo is not installed or not in PATH.
    echo [INFO] Install Rust from https://rustup.rs/, then run this file again.
    pause
    exit /b 1
  )
)

if not exist "client\node_modules" (
  echo [INFO] Installing client dependencies...
  call npm install --prefix client
  if errorlevel 1 (
    echo [ERROR] npm install failed.
    pause
    exit /b 1
  )
)

echo [INFO] Launching Tauri dev app...
call npm run tauri:dev
if errorlevel 1 (
  echo [ERROR] Failed to launch project.
  pause
  exit /b 1
)

endlocal
