@echo off
chcp 65001 > nul
title Test Du An GetOAuthToken
color 0E

echo ==================================================
echo   KIEM TRA DU AN GETOAUTHTOKEN
echo ==================================================
echo.

echo [1/5] Kiem tra Node.js...
node --version
if %errorlevel% neq 0 (
    color 0C
    echo [LOI] Node.js chua duoc cai dat!
    pause
    exit /b 1
)
echo [OK] Node.js da duoc cai dat

echo.
echo [2/5] Kiem tra npm...
npm --version
if %errorlevel% neq 0 (
    color 0C
    echo [LOI] npm chua duoc cai dat!
    pause
    exit /b 1
)
echo [OK] npm da duoc cai dat

echo.
echo [3/5] Kiem tra dependencies goc...
if not exist "node_modules" (
    color 0C
    echo [LOI] node_modules chua duoc cai dat! Chay setup.bat truoc.
    pause
    exit /b 1
)
echo [OK] Dependencies goc da duoc cai dat

echo.
echo [4/5] Kiem tra dependencies client...
if not exist "client\node_modules" (
    color 0C
    echo [LOI] client\node_modules chua duoc cai dat! Chay setup.bat truoc.
    pause
    exit /b 1
)
echo [OK] Dependencies client da duoc cai dat

echo.
echo [5/5] Kiem tra Rust (Cargo)...
cargo --version
if %errorlevel% neq 0 (
    color 0C
    echo [CANH BAO] Rust/Cargo chua duoc cai dat!
    echo Ban can cai dat Rust de build ung dung Tauri.
    echo Truy cap: https://rustup.rs/
    echo.
    echo Nhung ban van co the chay che do dev voi frontend.
    pause
) else (
    echo [OK] Rust/Cargo da duoc cai dat
)

echo.
echo ==================================================
echo   TAT CA CAC KIEM TRA DA HOAN TAT!
echo ==================================================
echo.
echo Du an san sang de chay. Su dung:
echo   - run.bat: Chay che do development
echo   - npm run build: Build ung dung production
echo.
pause
