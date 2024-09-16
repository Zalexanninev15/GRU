Remove-Item .\gru.exe -Force

cargo build --release

ResourceHacker -open .\target\release\gru.exe -save .\target\release\gru1.exe -action add -res .\ADMIN_MANIFEST.res -mask MANIFEST, -log CONSOLE
Move-Item .\target\release\gru1.exe .\gru.exe -Force

Write-Host Done!
Write-Host File: gru.exe