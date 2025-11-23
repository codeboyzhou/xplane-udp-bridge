@echo off
setlocal enabledelayedexpansion

echo -------------------------------------------------
echo           Building Rust X-Plane Plugin
echo -------------------------------------------------

echo Cleaning previous build...
cargo clean

echo Loading .env file...
if not exist .env (
    echo ERROR: .env file not found.
    exit /b 1
)

echo Resolving XPLANE_PLUGIN_DIR variable from .env file...
for /f "usebackq tokens=1,2 delims==" %%A in ("./.env") do (
    if "%%A"=="XPLANE_PLUGIN_DIR" (
        set XPLANE_PLUGIN_DIR=%%~B
    )
)

if "%XPLANE_PLUGIN_DIR%"=="" (
    echo ERROR: XPLANE_PLUGIN_DIR not found in .env file.
    exit /b 1
)

echo Using X-Plane plugin directory:
echo    %XPLANE_PLUGIN_DIR%

echo Resolving LIBCLANG_PATH variable from .env file...
for /f "usebackq tokens=1,2 delims==" %%A in ("./.env") do (
    if "%%A"=="LIBCLANG_PATH" (
        set LIBCLANG_PATH=%%~B
    )
)

if "%LIBCLANG_PATH%"=="" (
    echo ERROR: LIBCLANG_PATH not found in .env file.
    exit /b 1
)

echo Using libclang path:
echo    %LIBCLANG_PATH%

echo Building release...
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Cargo build failed.
    exit /b %ERRORLEVEL%
)

set SRC=target\release\xplane_udp_bridge_plugin.dll
set DEST=%XPLANE_PLUGIN_DIR%\xplane-udp-bridge\64

if not exist "%SRC%" (
    echo ERROR: %SRC% does not exist.
    exit /b 1
)

if not exist "%DEST%" (
    echo WARNING: %DEST% does not exist.
    echo Creating directory: %DEST%
    mkdir "%DEST%"
    if %ERRORLEVEL% NEQ 0 (
        echo ERROR: Failed to create directory %DEST%.
        exit /b %ERRORLEVEL%
    )
    echo Directory %DEST% created successfully.
)

echo Copying %SRC% to %DEST%\win.xpl
copy /Y "%SRC%" "%DEST%\win.xpl"
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Copy failed with error code %ERRORLEVEL%.
    exit /b %ERRORLEVEL%
)

echo Build completed successfully.
endlocal
