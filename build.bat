@echo off
setlocal enabledelayedexpansion

REM Set ResourceHacker path
set ResourceHacker=C:\Users\max\scoop\apps\resource-hacker\current\ResourceHacker.exe

REM Create release directories if they don't exist
mkdir "%~dp0release\EN\admin" 2>nul
mkdir "%~dp0release\EN\without_manifest" 2>nul
mkdir "%~dp0release\RU\admin" 2>nul
mkdir "%~dp0release\RU\without_manifest" 2>nul

REM Clean previous builds
del /F /Q "%~dp0release\EN\admin\gru.exe" 2>nul
del /F /Q "%~dp0release\EN\without_manifest\gru.exe" 2>nul
del /F /Q "%~dp0release\RU\admin\gru.exe" 2>nul
del /F /Q "%~dp0release\RU\without_manifest\gru.exe" 2>nul

echo Building English version...
cd "%~dp0"
if exist "src_en" (
    echo Error: src_en directory already exists. Please remove it before running the script.
    goto :error
)
cargo clean
cargo build --release

REM Create English version without admin manifest
echo Creating English version without admin manifest...
copy /Y "%~dp0target\release\gru.exe" "%~dp0release\EN\without_manifest\gru.exe"

REM Create English version with admin manifest
echo Creating English version with admin manifest...
cd "%~dp0"
"%ResourceHacker%" -open "target\release\gru.exe" -save "release\EN\admin\gru.exe" -action add -res "ADMIN_MANIFEST.res" -mask MANIFEST, -log CONSOLE

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

REM Replace with Russian source code
echo Replacing with Russian source code...
ren "src" "src_old"
xcopy /E /I /Y "src_ru" "src" >nul

cargo clean
cargo build --release

REM Create Russian version without admin manifest
echo Creating Russian version without admin manifest...
copy /Y "%~dp0target\release\gru.exe" "%~dp0release\RU\without_manifest\gru.exe"

REM Create Russian version with admin manifest
echo Creating Russian version with admin manifest...
cd "%~dp0"
"%ResourceHacker%" -open "target\release\gru.exe" -save "release\RU\admin\gru.exe" -action add -res "ADMIN_MANIFEST.res" -mask MANIFEST, -log CONSOLE

REM Restore original src directory
echo Restoring original source code...
cd "%~dp0"
rmdir /S /Q "src" 2>nul
ren "src_old" "src"
rmdir /S /Q "src_en" 2>nul

echo Done!
echo All files have been built and placed in their respective folders:
echo - release\EN\admin\gru.exe (English with admin manifest)
echo - release\EN\without_manifest\gru.exe (English without manifest)
echo - release\RU\admin\gru.exe (Russian with admin manifest)
echo - release\RU\without_manifest\gru.exe (Russian without manifest)
goto :end

:error
echo Build process failed.

:end
pause