@echo off
echo =======================================
echo     A Iniciar OpenRest (Fase 1)
echo =======================================
echo.

:: 1. Inicia o backend em Rust numa nova janela
echo [1/2] A iniciar o servidor Backend (cargo run -p backend)...
start "OpenRest Backend" cmd /k "cargo run -p backend"

:: 2. Inicia o frontend Tauri
echo [2/2] A iniciar o Frontend Tauri (npm run tauri dev)...
cd apps\posto
start "OpenRest Posto" cmd /k "npm run tauri dev"
cd ..\..

echo.
