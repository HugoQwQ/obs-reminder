@echo off
setlocal enabledelayedexpansion

REM --- Get version from Cargo.toml (ignore leading spaces) ---
for /f "tokens=2 delims==" %%i in ('findstr /R "^[ ]*version[ ]*=" Cargo.toml') do (
    set ver=%%i
    REM Remove quotes
    set ver=!ver:"=!
    REM Trim leading spaces
    for /f "tokens=* delims= " %%a in ("!ver!") do set ver=%%a
)

echo Building obs-reminder v%ver%

REM --- Build browser frontend ---
cd browser
call pnpm install
call pnpm run build
cd ..

REM --- Build Rust application ---
cargo build --release

REM --- Create output folder ---
set outputDir=obs-reminder-windows-x64-%ver%-stable
if exist "%outputDir%" rmdir /S /Q "%outputDir%"
mkdir "%outputDir%"

REM --- Copy executable ---
copy target\release\obs-reminder-client.exe "%outputDir%" >nul

REM --- Package with 7-Zip ---
if exist "%outputDir%.zip" del "%outputDir%.zip"
7z a -tzip "%outputDir%.zip" "%outputDir%\*" >nul
if exist "%outputDir%" rmdir /S /Q "%outputDir%"

echo.
echo Build completed: %outputDir%.zip
pause
