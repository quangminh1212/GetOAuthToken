@echo off
echo ========================================
echo GetOAuthToken - Clear Log
echo ========================================
echo.

if exist log\log.txt (
    del log\log.txt
    echo Log file cleared: log\log.txt
) else (
    echo No log file to clear.
)

echo.
pause
