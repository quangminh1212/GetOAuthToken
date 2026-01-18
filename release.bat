@echo off
chcp 65001 >nul

REM Create log directory if not exists
if not exist "log" mkdir log

REM Log function
set "LOG_FILE=log\log.txt"
echo [%date% %time%] ========== BUILD RELEASE STARTED ========== >> "%LOG_FILE%"

echo ========================================
echo   Build Release - GetOAuthToken
echo ========================================
echo.

REM Kiểm tra Node.js
echo [%date% %time%] Checking Node.js installation... >> "%LOG_FILE%"
where node >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Node.js chưa được cài đặt!
    echo Vui lòng cài đặt Node.js từ: https://nodejs.org/
    echo [%date% %time%] ERROR: Node.js not found >> "%LOG_FILE%"
    pause
    exit /b 1
)
echo [%date% %time%] Node.js found >> "%LOG_FILE%"

REM Kiểm tra Rust
echo [%date% %time%] Checking Rust installation... >> "%LOG_FILE%"
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Rust chưa được cài đặt!
    echo Vui lòng cài đặt Rust từ: https://rustup.rs/
    echo [%date% %time%] ERROR: Rust not found >> "%LOG_FILE%"
    pause
    exit /b 1
)
echo [%date% %time%] Rust found >> "%LOG_FILE%"

echo [1/5] Cài đặt dependencies cho root project...
echo [%date% %time%] [1/5] Installing root dependencies... >> "%LOG_FILE%"
call npm install
if %errorlevel% neq 0 (
    echo [ERROR] Cài đặt dependencies thất bại!
    echo [%date% %time%] ERROR: Root npm install failed >> "%LOG_FILE%"
    pause
    exit /b 1
)
echo [%date% %time%] Root dependencies installed >> "%LOG_FILE%"

echo.
echo [2/5] Cài đặt dependencies cho client...
echo [%date% %time%] [2/5] Installing client dependencies... >> "%LOG_FILE%"
cd client
call npm install
if %errorlevel% neq 0 (
    echo [ERROR] Cài đặt client dependencies thất bại!
    echo [%date% %time%] ERROR: Client npm install failed >> "%LOG_FILE%"
    cd ..
    pause
    exit /b 1
)
echo [%date% %time%] Client dependencies installed >> "%LOG_FILE%"

echo.
echo [3/5] Build frontend (React/Vite)...
echo [%date% %time%] [3/5] Building frontend... >> "%LOG_FILE%"
call npm run build
if %errorlevel% neq 0 (
    echo [ERROR] Build frontend thất bại!
    echo [%date% %time%] ERROR: Frontend build failed >> "%LOG_FILE%"
    cd ..
    pause
    exit /b 1
)
echo [%date% %time%] Frontend build completed >> "%LOG_FILE%"
cd ..

echo.
echo [4/5] Build Tauri application...
echo [%date% %time%] [4/5] Building Tauri application... >> "%LOG_FILE%"
call npm run build
if %errorlevel% neq 0 (
    echo [ERROR] Build Tauri thất bại!
    echo [%date% %time%] ERROR: Tauri build failed >> "%LOG_FILE%"
    pause
    exit /b 1
)
echo [%date% %time%] Tauri build completed >> "%LOG_FILE%"

echo.
echo [5/5] Tạo thư mục release và copy file...
echo [%date% %time%] [5/5] Creating release directory and copying files... >> "%LOG_FILE%"
if not exist "release" mkdir release

REM Tìm file .exe trong thư mục target
set "EXE_PATH=src-tauri\target\release\app.exe"
set "MSI_PATH=src-tauri\target\release\bundle\msi\GetOAuthToken_1.0.0_x64_en-US.msi"
set "NSIS_PATH=src-tauri\target\release\bundle\nsis\GetOAuthToken_1.0.0_x64-setup.exe"

if exist "%EXE_PATH%" (
    echo Copying portable executable...
    copy "%EXE_PATH%" "release\GetOAuthToken-portable.exe"
    echo ✓ File portable: release\GetOAuthToken-portable.exe
    echo [%date% %time%] Portable executable copied >> "%LOG_FILE%"
)

if exist "%MSI_PATH%" (
    echo Copying MSI installer...
    copy "%MSI_PATH%" "release\"
    echo ✓ File MSI installer đã được copy
    echo [%date% %time%] MSI installer copied >> "%LOG_FILE%"
)

if exist "%NSIS_PATH%" (
    echo Copying NSIS installer...
    copy "%NSIS_PATH%" "release\"
    echo ✓ File NSIS installer đã được copy
    echo [%date% %time%] NSIS installer copied >> "%LOG_FILE%"
)

echo.
echo ========================================
echo   BUILD HOÀN TẤT!
echo ========================================
echo.
echo File portable executable: release\GetOAuthToken-portable.exe
echo.
echo Bạn có thể chạy trực tiếp file .exe mà không cần cài đặt!
echo [%date% %time%] ========== BUILD RELEASE COMPLETED SUCCESSFULLY ========== >> "%LOG_FILE%"
echo.
pause
