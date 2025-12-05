@echo off
setlocal

chcp 65001 >nul

echo.
echo Running golangci-lint...

:: Check if golangci-lint is installed
set GOLANGCI_VERSION=v2.7.1
where golangci-lint >nul 2>&1
if errorlevel 1 (
    echo.
    echo ❌ golangci-lint is not installed
    echo.
    echo Please install it using:
    echo     go install github.com/golangci/golangci-lint/v2/cmd/golangci-lint@%GOLANGCI_VERSION%
    echo.
    echo Alternatively, you can run install-tools.bat to install all tools.
    echo.
    exit /b 1
)

:: Run golangci-lint
golangci-lint run --config .golangci.yml
if errorlevel 1 (
    echo.
    echo ❌ golangci-lint failed
    echo.
    exit /b 1
)

echo.
echo ✅ golangci-lint completed successfully
echo.
exit /b 0