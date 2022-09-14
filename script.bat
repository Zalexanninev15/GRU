@echo off
cd ..\f*\
xcopy /E /C /H /Y bin ..\
cd ..
del /f /q vc_redist.x64.exe
for /f "delims=" %%a in ('dir /s/b/ad "flameshot-*"') do cmd /k rd /s/q "%%~a"