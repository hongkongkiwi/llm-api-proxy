@echo off
REM Installation script for Windows (NSSM)
REM Run as Administrator

setlocal enabledelayedexpansion

REM Configuration
set INSTALL_DIR=C:\opt\anthropic-http-proxy
set CONFIG_DIR=C:\opt\anthropic-http-proxy
set LOG_DIR=C:\opt\anthropic-http-proxy\logs
set SERVICE_NAME=anthropic-http-proxy

echo Installing anthropic-http-proxy as Windows service...

REM Check if running as Administrator
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo Please run this script as Administrator
    pause
    exit /b 1
)

REM Check if NSSM is available
where nssm >nul 2>&1
if %errorLevel% neq 0 (
    echo NSSM is not installed or not in PATH
    echo Please download NSSM from https://nssm.cc and add it to PATH
    pause
    exit /b 1
)

REM Create directories
echo Creating directories...
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"
if not exist "%LOG_DIR%" mkdir "%LOG_DIR%"

REM Copy binary (assuming it's built)
if exist "target\release\anthropic-http-proxy.exe" (
    echo Copying binary...
    copy "target\release\anthropic-http-proxy.exe" "%INSTALL_DIR%\" >nul
) else (
    echo Warning: Binary not found at target\release\anthropic-http-proxy.exe
    echo Please build the project first: cargo build --release
)

REM Copy config file
if exist "config.toml" (
    echo Copying config file...
    copy "config.toml" "%CONFIG_DIR%\" >nul
) else (
    echo Warning: config.toml not found in current directory
    echo Please create a config file at %CONFIG_DIR%\config.toml
)

REM Install service using NSSM
echo Installing Windows service...
nssm install "%SERVICE_NAME%" "%INSTALL_DIR%\anthropic-http-proxy.exe"
nssm set "%SERVICE_NAME%" AppDirectory "%INSTALL_DIR%"
nssm set "%SERVICE_NAME%" AppEnvironmentExtra "CONFIG_PATH=%CONFIG_DIR%\config.toml" "PORT=8811"
nssm set "%SERVICE_NAME%" AppStdout "%LOG_DIR%\output.log"
nssm set "%SERVICE_NAME%" AppStderr "%LOG_DIR%\error.log"
nssm set "%SERVICE_NAME%" Start SERVICE_AUTO_START
nssm set "%SERVICE_NAME%" Description "Transparent HTTP proxy for LLM APIs (Anthropic, OpenAI)"

echo Installation complete!
echo.
echo To start the service:
echo   net start %SERVICE_NAME%
echo.
echo To check status:
echo   sc query %SERVICE_NAME%
echo.
echo To view logs:
echo   type "%LOG_DIR%\output.log"
echo   type "%LOG_DIR%\error.log"
echo.
echo To uninstall:
echo   nssm remove %SERVICE_NAME% confirm

pause