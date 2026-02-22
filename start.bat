@echo off
echo ==========================================
echo   Cherenkov Radiation Monitoring System
echo ==========================================
echo.

REM Get the directory where this script is located
set "SCRIPT_DIR=%~dp0"
cd /d "%SCRIPT_DIR%"

REM Check if required directories exist
if not exist "web\package.json" (
    echo Error: web\package.json not found
    echo Please ensure this script is in the cherenkov root directory
    pause
    exit /b 1
)

if not exist "mock-api\package.json" (
    echo Error: mock-api\package.json not found
    echo Please ensure this script is in the cherenkov root directory
    pause
    exit /b 1
)

echo Starting Cherenkov services...
echo.

REM Start Mock API Server in new window
echo [1/2] Starting Mock API Server on port 8080...
start "Cherenkov Mock API" cmd /k "cd /d "%SCRIPT_DIR%mock-api" && if not exist node_modules (echo Installing API dependencies... && npm install) && npm start"

REM Wait a moment for API to initialize
timeout /t 3 /nobreak > nul

REM Start Web Frontend in new window
echo [2/2] Starting Web Frontend on port 3000...
start "Cherenkov Web" cmd /k "cd /d "%SCRIPT_DIR%web" && if not exist node_modules (echo Installing Web dependencies... && npm install) && npm run dev"


echo.
echo ==========================================
echo   Services starting...
echo.
echo   Web UI:      http://localhost:3000
echo   Mock API:    http://localhost:8080
echo   GraphQL:     http://localhost:8080/graphql
echo   WebSocket:   ws://localhost:8080/ws
echo.
echo   Close the command windows to stop services
echo ==========================================
echo.

pause
