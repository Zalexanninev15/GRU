cargo build --release

ResourceHacker.exe -open .\target\release\gru.exe -save .\target\release\gru1.exe -action add -res .\ADMIN_MANIFEST.res -mask MANIFEST, -log CONSOLE
move .\target\release\gru1.exe .\gru.exe -Force

echo "Done!"
