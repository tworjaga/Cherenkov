@echo off
echo ==========================================
echo   Cherenkov Radiation Monitoring System
echo ==========================================
echo.

REM Check if we're in the right directory
if not exist "web\package.json" (
    echo Error: Please run this script from the cherenkov root directory
    pause
    exit /b 1
)

echo Starting Cherenkov services...
echo.

REM Start Mock API Server in new window
echo [1/2] Starting Mock API Server on port 8080...
start "Cherenkov Mock API" cmd /k "cd mock-api && if not exist node_modules (echo Installing API dependencies... && npm install) && npm start"

REM Wait a moment for API to initialize
timeout /t 3 /nobreak > nul

REM Start Web Frontend in new window
echo [2/2] Starting Web Frontend on port 3000...
start "Cherenkov Web" cmd /k "cd web && if not exist node_modules (echo Installing Web dependencies... && npm install) && npm run dev"

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
