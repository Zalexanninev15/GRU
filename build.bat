@echo off
setlocal enabledelayedexpansion

REM Set ResourceHacker path - check multiple possible locations
set ResourceHacker=
if exist "C:\Users\%USERNAME%\scoop\apps\resource-hacker\current\ResourceHacker.exe" (
    set ResourceHacker=C:\Users\%USERNAME%\scoop\apps\resource-hacker\current\ResourceHacker.exe
) else if exist "C:\Program Files\Resource Hacker\ResourceHacker.exe" (
    set ResourceHacker=C:\Program Files\Resource Hacker\ResourceHacker.exe
) else (
    echo Error: ResourceHacker.exe not found. Please install Resource Hacker or update the path.
    goto :error
)

echo Using ResourceHacker at: !ResourceHacker!

REM Create release directories if they don't exist
mkdir "%~dp0release\EN\admin\UpdateTools" 2>nul
mkdir "%~dp0release\EN\without_manifest\UpdateTools" 2>nul
mkdir "%~dp0release\RU\admin\UpdateTools" 2>nul
mkdir "%~dp0release\RU\without_manifest\UpdateTools" 2>nul

REM Clean previous builds
echo Cleaning previous builds...
del /F /Q "%~dp0release\EN\admin\UpdateTools\gru.exe" 2>nul
del /F /Q "%~dp0release\EN\without_manifest\UpdateTools\gru.exe" 2>nul
del /F /Q "%~dp0release\RU\admin\UpdateTools\gru.exe" 2>nul
del /F /Q "%~dp0release\RU\without_manifest\UpdateTools\gru.exe" 2>nul

REM Check required files exist
if not exist "ADMIN_MANIFEST.res" (
    echo Error: ADMIN_MANIFEST.res not found.
    goto :error
)

echo Building English version...
cd "%~dp0"

REM Check if backup already exists and handle it
if exist "src_en" (
    echo Warning: src_en directory already exists. Removing it...
    rmdir /S /Q "src_en" 2>nul
    if exist "src_en" (
        echo Error: Could not remove existing src_en directory.
        goto :error
    )
)

REM Clean and build English version
cargo clean
if !ERRORLEVEL! neq 0 (
    echo Error: cargo clean failed
    goto :error
)

cargo build --release
if !ERRORLEVEL! neq 0 (
    echo Error: English build failed
    goto :error
)

REM Verify the build output exists
if not exist "%~dp0target\release\gru.exe" (
    echo Error: Build output not found at target\release\gru.exe
    goto :error
)

REM Create English version without admin manifest
echo Creating English version without admin manifest...
copy /Y "%~dp0target\release\gru.exe" "%~dp0release\EN\without_manifest\UpdateTools\gru.exe"
if !ERRORLEVEL! neq 0 (
    echo Error: Failed to copy English version without manifest
    goto :error
)

REM Create English version with admin manifest
echo Creating English version with admin manifest...
"!ResourceHacker!" -open "target\release\gru.exe" -save "release\EN\admin\UpdateTools\gru.exe" -action add -res "ADMIN_MANIFEST.res" -mask MANIFEST, -log CONSOLE
if !ERRORLEVEL! neq 0 (
    echo Error: Failed to create English version with admin manifest
    goto :error
)

echo Building Russian version...
cd "%~dp0"

REM Check if the directories exist
if not exist "src" (
    echo Error: src directory not found.
    goto :error
)
if not exist "src_ru" (
    echo Error: src_ru directory not found.
    goto :error
)

REM Backup original src directory
echo Backing up English source code...
xcopy /E /I /Y "src" "src_en" >nul
if !ERRORLEVEL! neq 0 (
    echo Error: Failed to backup English source code
    goto :error
)

REM Replace with Russian source code (safer approach)
echo Replacing with Russian source code...
ren "src" "src_backup_temp"
if !ERRORLEVEL! neq 0 (
    echo Error: Failed to rename src directory
    goto :restore_from_backup
)

xcopy /E /I /Y "src_ru" "src" >nul
if !ERRORLEVEL! neq 0 (
    echo Error: Failed to copy Russian source code
    goto :restore_from_temp
)

REM Clean and build Russian version
cargo clean
if !ERRORLEVEL! neq 0 (
    echo Error: cargo clean failed for Russian build
    goto :restore_from_temp
)

cargo build --release
if !ERRORLEVEL! neq 0 (
    echo Error: Russian build failed
    goto :restore_from_temp
)

REM Verify the Russian build output exists
if not exist "%~dp0target\release\gru.exe" (
    echo Error: Russian build output not found
    goto :restore_from_temp
)

REM Create Russian version without admin manifest
echo Creating Russian version without admin manifest...
copy /Y "%~dp0target\release\gru.exe" "%~dp0release\RU\without_manifest\UpdateTools\gru.exe"
if !ERRORLEVEL! neq 0 (
    echo Error: Failed to copy Russian version without manifest
    goto :restore_from_temp
)

REM Create Russian version with admin manifest
echo Creating Russian version with admin manifest...
"!ResourceHacker!" -open "target\release\gru.exe" -save "release\RU\admin\UpdateTools\gru.exe" -action add -res "ADMIN_MANIFEST.res" -mask MANIFEST, -log CONSOLE
if !ERRORLEVEL! neq 0 (
    echo Error: Failed to create Russian version with admin manifest
    goto :restore_from_temp
)

REM Restore original src directory
echo Restoring original source code...
rmdir /S /Q "src" 2>nul
ren "src_backup_temp" "src"
if !ERRORLEVEL! neq 0 (
    echo Error: Failed to restore original src directory
    echo Manual intervention required - check src_backup_temp directory
    goto :error
)

REM Clean up backup
rmdir /S /Q "src_en" 2>nul

echo.
echo ========================================
echo           BUILD SUCCESSFUL!
echo ========================================
echo All files have been built and placed in their respective folders:
echo - release\EN\admin\gru.exe (English with admin manifest)
echo - release\EN\without_manifest\gru.exe (English without manifest)
echo - release\RU\admin\gru.exe (Russian with admin manifest)
echo - release\RU\without_manifest\gru.exe (Russian without manifest)
echo ========================================
goto :end

:restore_from_temp
echo Attempting to restore from temporary backup...
rmdir /S /Q "src" 2>nul
ren "src_backup_temp" "src"
goto :error

:restore_from_backup
echo Attempting to restore from English backup...
if exist "src_en" (
    xcopy /E /I /Y "src_en" "src" >nul
    rmdir /S /Q "src_en" 2>nul
)
goto :error

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