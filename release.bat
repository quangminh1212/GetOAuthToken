@echo off
chcp 65001 >nul
echo ========================================
echo   Build Release - GetOAuthToken
echo ========================================
echo.

REM Kiểm tra Node.js
where node >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Node.js chưa được cài đặt!
    echo Vui lòng cài đặt Node.js từ: https://nodejs.org/
    pause
    exit /b 1
)

REM Kiểm tra Rust
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Rust chưa được cài đặt!
    echo Vui lòng cài đặt Rust từ: https://rustup.rs/
    pause
    exit /b 1
)

echo [1/5] Cài đặt dependencies cho root project...
call npm install
if %errorlevel% neq 0 (
    echo [ERROR] Cài đặt dependencies thất bại!
    pause
    exit /b 1
)

echo.
echo [2/5] Cài đặt dependencies cho client...
cd client
call npm install
if %errorlevel% neq 0 (
    echo [ERROR] Cài đặt client dependencies thất bại!
    cd ..
    pause
    exit /b 1
)

echo.
echo [3/5] Build frontend (React/Vite)...
call npm run build
if %errorlevel% neq 0 (
    echo [ERROR] Build frontend thất bại!
    cd ..
    pause
    exit /b 1
)
cd ..

echo.
echo [4/5] Build Tauri application...
call npm run build
if %errorlevel% neq 0 (
    echo [ERROR] Build Tauri thất bại!
    pause
    exit /b 1
)

echo.
echo [5/5] Tạo thư mục release và copy file...
if not exist "release" mkdir release

REM Tìm file .exe trong thư mục target
set "EXE_PATH=src-tauri\target\release\app.exe"
set "MSI_PATH=src-tauri\target\release\bundle\msi\GetOAuthToken_1.0.0_x64_en-US.msi"
set "NSIS_PATH=src-tauri\target\release\bundle\nsis\GetOAuthToken_1.0.0_x64-setup.exe"

if exist "%EXE_PATH%" (
    echo Copying portable executable...
    copy "%EXE_PATH%" "release\GetOAuthToken-portable.exe"
    echo ✓ File portable: release\GetOAuthToken-portable.exe
)

if exist "%MSI_PATH%" (
    echo Copying MSI installer...
    copy "%MSI_PATH%" "release\"
    echo ✓ File MSI installer đã được copy
)

if exist "%NSIS_PATH%" (
    echo Copying NSIS installer...
    copy "%NSIS_PATH%" "release\"
    echo ✓ File NSIS installer đã được copy
)

echo.
echo ========================================
echo   BUILD HOÀN TẤT!
echo ========================================
echo.
echo File portable executable: release\GetOAuthToken-portable.exe
echo.
echo Bạn có thể chạy trực tiếp file .exe mà không cần cài đặt!
echo.
pause
