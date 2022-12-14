# Github Release Updater | [Latest release](https://github.com/Zalexanninev15/GRU/releases/latest)

[![](https://img.shields.io/badge/OS-Windows-informational?logo=windows)](https://github.com/Zalexanninev15/GRU)
[![](https://img.shields.io/badge/written_on-Rust-000000.svg?logo=rust)](https://github.com/Zalexanninev15/GRU)
[![](https://img.shields.io/github/v/release/Zalexanninev15/GRU)](https://github.com/Zalexanninev15/GRU/releases/latest)
[![](https://img.shields.io/github/downloads/Zalexanninev15/GRU/total.svg)](https://github.com/Zalexanninev15/GRU/releases)
[![](https://img.shields.io/github/last-commit/Zalexanninev15/GRU/main.svg)](https://github.com/Zalexanninev15/GRU/commits/main)
[![](https://img.shields.io/github/stars/Zalexanninev15/GRU.svg)](https://github.com/Zalexanninev15/GRU/stargazers)
[![](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![](https://img.shields.io/badge/donate-Buy_Me_a_Coffee-F94400.svg)](https://zalexanninev15.jimdofree.com/buy-me-a-coffee)

## Description

Updater for applications from GitHub

## Features

- Features of fine-tuning the update process
- Support for a custom script that runs after an application update (`script.bat` file)
- Clear and sufficiently self-sufficient documentation (run `gru.exe` without arguments)
- Support for determining the current version when using a specific argument for a more fine-grained application update process

## System requirements

- **OS:** Windows 7 or higher

## Usage

1. Copy the `UpdateTools` folder from the archive to the application folder
2. Run the `gru.exe` file with the arguments. [Read more about existing arguments](https://github.com/Zalexanninev15/GRU/blob/main/arguments.txt)
3. The file/archive will be downloaded. If it is an archive, then it will be unzipped to a folder a level higher than the current one (i.e. you need to remove `UpdateTools` from the path). If it is a single file, then it will simply be moved (also to a higher level). The archive will be deleted automatically after unpacking

### Example, [GitHub Desktop Portable by gek64](https://github.com/gek64/GitHubDesktopPortable)

```batch
gru.exe --repo gek64/GitHubDesktopPortable --app GitHubDesktopPortable.exe --with "paf" --main App\GitHubDesktop\GitHubDesktop.exe
```

### Example, [Flameshot Portable](https://github.com/flameshot-org/flameshot)

```batch
gru.exe --repo flameshot-org/flameshot --app flameshot.exe --with "win64.zip" --script
```

???? **Script:** [View](https://github.com/Zalexanninev15/GRU/blob/main/script.bat)

### Example, [draw.io Desktop](https://github.com/jgraph/drawio-desktop)

```batch
gru.exe --repo jgraph/drawio-desktop --app app.exe --with "-windows-32bit-no-installer.exe" --no-extract
```

## Build (with PowerShell)

1. Install all dependencies as Admin (it is recommended to use packages from the [Chocolatey package manager](https://chocolatey.org))
```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
choco install rust mingw git -y
```

2. Compile the GRU!

```powershell
git clone https://github.com/Zalexanninev15/GRU
cd .\GRU\
cargo build --release
```
