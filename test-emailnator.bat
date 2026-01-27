@echo off
echo ========================================
echo Testing Emailnator Integration
echo ========================================
echo.

echo [1/3] Checking Rust compilation...
cd src-tauri
cargo check
if %errorlevel% neq 0 (
    echo ERROR: Rust compilation failed!
    pause
    exit /b 1
)
cd ..
echo ✓ Rust compilation successful
echo.

echo [2/3] Checking Node dependencies...
if not exist "node_modules" (
    echo Installing root dependencies...
    call npm install
)
if not exist "client\node_modules" (
    echo Installing client dependencies...
    cd client
    call npm install
    cd ..
)
echo ✓ Dependencies installed
echo.

echo [3/3] Starting application...
echo.
echo ========================================
echo Application is starting...
echo ========================================
echo.
echo Test Instructions:
echo 1. Click Email icon (✉️) in the app
echo 2. Click "Generate Email"
echo 3. Copy the email address
echo 4. Send a test email to that address
echo 5. Click "Refresh Inbox" in the app
echo 6. Click on the email to extract code
echo.
echo Press Ctrl+C to stop the application
echo ========================================
echo.

call npm start
