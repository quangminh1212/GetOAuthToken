@echo off
chcp 65001 > nul
title Cai dat Du an GetOAuthToken
color 0A

echo ==================================================
echo   BAT DAU CAI DAT DU AN GETOAUTHTOKEN
echo ==================================================
echo.

echo [1/2] Dang cai dat cac thu vien tai thuc muc goc...
call npm install
if %errorlevel% neq 0 (
    color 0C
    echo [LOI] Khong the cai dat thu vien goc. Vui long kiem tra lai Node.js.
    pause
    exit /b %errorlevel%
)

echo.
echo [2/2] Dang cai dat cac thu vien cho Client...
cd client
call npm install
if %errorlevel% neq 0 (
    color 0C
    echo [LOI] Khong the cai dat thu vien Client.
    cd ..
    pause
    exit /b %errorlevel%
)
cd ..

echo.
echo ==================================================
echo   CAI DAT HOAN TAT THANH CONG!
echo ==================================================
echo Ban co the chay file 'run.bat' de khoi dong du an.
echo.
pause
