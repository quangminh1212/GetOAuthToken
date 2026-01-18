@echo off
chcp 65001 > nul
title Cai dat Du an GetOAuthToken
color 0A

REM Create log directory if not exists
if not exist "log" mkdir log

REM Log function
set "LOG_FILE=log\log.txt"
echo [%date% %time%] ========== SETUP STARTED ========== >> "%LOG_FILE%"

echo ==================================================
echo   BAT DAU CAI DAT DU AN GETOAUTHTOKEN
echo ==================================================
echo.

echo [1/2] Dang cai dat cac thu vien tai thuc muc goc...
echo [%date% %time%] [1/2] Installing root dependencies... >> "%LOG_FILE%"
call npm install
if %errorlevel% neq 0 (
    color 0C
    echo [LOI] Khong the cai dat thu vien goc. Vui long kiem tra lai Node.js.
    echo [%date% %time%] ERROR: Root npm install failed >> "%LOG_FILE%"
    pause
    exit /b %errorlevel%
)
echo [%date% %time%] Root dependencies installed successfully >> "%LOG_FILE%"

echo.
echo [2/2] Dang cai dat cac thu vien cho Client...
echo [%date% %time%] [2/2] Installing client dependencies... >> "%LOG_FILE%"
cd client
call npm install
if %errorlevel% neq 0 (
    color 0C
    echo [LOI] Khong the cai dat thu vien Client.
    echo [%date% %time%] ERROR: Client npm install failed >> "%LOG_FILE%"
    cd ..
    pause
    exit /b %errorlevel%
)
echo [%date% %time%] Client dependencies installed successfully >> "%LOG_FILE%"
cd ..

echo.
echo ==================================================
echo   CAI DAT HOAN TAT THANH CONG!
echo ==================================================
echo Ban co the chay file 'run.bat' de khoi dong du an.
echo [%date% %time%] ========== SETUP COMPLETED SUCCESSFULLY ========== >> "%LOG_FILE%"
echo.
pause
