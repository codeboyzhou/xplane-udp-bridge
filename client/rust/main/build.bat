@echo off
setlocal enabledelayedexpansion

echo -------------------------------------------------
echo      Building Rust X-Plane UDP Bridge Client
echo -------------------------------------------------

echo Cleaning previous build...
cargo clean

echo Running: cargo fmt
cargo fmt
echo Finished: cargo fmt

echo Running: cargo clippy
cargo clippy --no-deps
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Cargo clippy failed.
    exit /b %ERRORLEVEL%
)
echo Finished: cargo clippy

echo Building release...
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Cargo build failed.
    exit /b %ERRORLEVEL%
)

echo Build completed successfully.
endlocal
