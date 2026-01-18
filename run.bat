@echo off
chcp 65001 > nul
title Chay Du an GetOAuthToken
color 0B

REM Create log directory if not exists
if not exist "log" mkdir log

REM Log function
set "LOG_FILE=log\log.txt"
echo [%date% %time%] ========== APP STARTED ========== >> "%LOG_FILE%"

echo ==================================================
echo   DANG KHOI DONG DU AN...
echo ==================================================
echo.

call npm start

if %errorlevel% neq 0 (
    color 0C
    echo.
    echo [LOI] Chuong trinh da dung dot ngot hoac gap loi.
    echo [%date% %time%] ERROR: App crashed or encountered error (exit code: %errorlevel%) >> "%LOG_FILE%"
    pause
) else (
    echo [%date% %time%] App closed normally >> "%LOG_FILE%"
)
