Remove-Item .\gru.exe -Force

cargo clean
cargo build --release

ResourceHacker -open .\target\release\gru.exe -save .\gru.exe -action add -res .\ADMIN_MANIFEST.res -mask MANIFEST, -log CONSOLE

Write-Host Done!
Write-Host File: gru.exe