@echo off
echo ========================================
echo GetOAuthToken - View Log
echo ========================================
echo.

if not exist log\log.txt (
    echo Log file not found: log\log.txt
    echo Run the app first to generate logs.
    pause
    exit /b 1
)

echo Displaying log file: log\log.txt
echo ========================================
echo.

type log\log.txt

echo.
echo ========================================
echo End of log file
echo ========================================
pause
