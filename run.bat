@echo off
chcp 65001 > nul
title Chay Du an GetOAuthToken
color 0B

echo ==================================================
echo   DANG KHOI DONG DU AN...
echo ==================================================
echo.

call npm start

if %errorlevel% neq 0 (
    color 0C
    echo.
    echo [LOI] Chuong trinh da dung dot ngot hoac gap loi.
    pause
)
