@echo off
setlocal

chcp 65001 >nul

:: =============================
:: Go Tool Versions
:: =============================
set GOFUMPT_VERSION=v0.9.2
set GOIMPORTS_VERSION=v0.39.0
set GOLANGCI_VERSION=v2.7.1

:: =============================
:: Install Tools
:: =============================

:: --- gofumpt ---
echo.
echo Installing gofumpt (%GOFUMPT_VERSION%)...
go install mvdan.cc/gofumpt@%GOFUMPT_VERSION%
if errorlevel 1 goto :error

:: --- goimports ---
echo.
echo Installing goimports (%GOIMPORTS_VERSION%)...
go install golang.org/x/tools/cmd/goimports@%GOIMPORTS_VERSION%
if errorlevel 1 goto :error

:: --- golangci-lint ---
echo.
echo Installing golangci-lint (%GOLANGCI_VERSION%)...
go install github.com/golangci/golangci-lint/v2/cmd/golangci-lint@%GOLANGCI_VERSION%
if errorlevel 1 goto :error

echo.
echo ✅ All tools installed successfully
echo.
goto :eof

:error
echo.
echo ❌ Installation failed!
exit /b 1
