@echo off
setlocal enabledelayedexpansion

title GRU Builder

REM Set ResourceHacker path - check multiple possible locations
set ResourceHacker=
if exist "C:\Users\%USERNAME%\scoop\apps\resource-hacker\current\ResourceHacker.exe" (
    set "ResourceHacker=C:\Users\%USERNAME%\scoop\apps\resource-hacker\current\ResourceHacker.exe"
) else if exist "C:\Program Files\Resource Hacker\ResourceHacker.exe" (
    set "ResourceHacker=C:\Program Files\Resource Hacker\ResourceHacker.exe"
) else (
    echo Error: ResourceHacker.exe not found. Please install Resource Hacker or update the path.
    goto :error
)

echo Using ResourceHacker at: !ResourceHacker!

REM Create release directories if they don't exist
mkdir "%~dp0release\admin\UpdateTools" 2>nul
mkdir "%~dp0release\without_manifest\UpdateTools" 2>nul

REM Clean previous builds
echo Cleaning previous builds...
del /F /Q "%~dp0release\admin\UpdateTools\gru.exe" 2>nul
del /F /Q "%~dp0release\without_manifest\UpdateTools\gru.exe" 2>nul

REM Check required files exist
if not exist "ADMIN_MANIFEST.res" (
    echo Error: ADMIN_MANIFEST.res not found.
    goto :error
)

echo Building...
cd "%~dp0"

REM Clean and build
cargo clean
if !ERRORLEVEL! neq 0 (
    echo Error: cargo clean failed
    goto :error
)

cargo build --release
if !ERRORLEVEL! neq 0 (
    echo Error: Build failed
    goto :error
)

REM Verify the build output exists
if not exist "%~dp0target\release\gru.exe" (
    echo Error: Build output not found at target\release\gru.exe
    goto :error
)

REM Create build without admin manifest
echo Creating build without admin manifest...
copy /Y "%~dp0target\release\gru.exe" "%~dp0release\without_manifest\UpdateTools\gru.exe"
if !ERRORLEVEL! neq 0 (
    echo Error: Failed to copy build without manifest
    goto :error
)

REM Create build with admin manifest
echo Creating build with admin manifest...
"!ResourceHacker!" -open "target\release\gru.exe" -save "release\admin\UpdateTools\gru.exe" -action add -res "ADMIN_MANIFEST.res" -mask MANIFEST, -log CONSOLE
if !ERRORLEVEL! neq 0 (
    echo Error: Failed to create build with admin manifest
    goto :error
)

echo.
echo ========================================
echo           BUILD SUCCESSFUL!
echo ========================================
echo All files have been built and placed in their respective folders:
echo - release\admin\UpdateTools\gru.exe (with admin manifest)
echo - release\without_manifest\UpdateTools\gru.exe (without manifest)
echo ========================================
goto :end

:error
echo.
echo ========================================
echo           BUILD FAILED!
echo ========================================
echo Please check the error messages above.
pause
exit /b 1

:end
echo.
pause