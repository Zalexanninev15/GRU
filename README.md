# Github Releases Updater

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

- Convenient data transfer for third-party utilities
- Features of fine-tuning the update process
- Support for a custom script that runs after an application update (`script.bat` file)
- Clear and sufficiently self-sufficient documentation (run `gru.exe` without arguments)

## System requirements

- **OS:** Windows 7 or higher

## Usage

1. Copy the `UpdateTools` folder from the archive to the application folder
2. Run the `gru.exe` file with the arguments. [Read more about existing arguments](https://github.com/Zalexanninev15/GRU/blob/main/arguments.txt)

### Old example without the `--rv` argument, [GitHub Desktop Portable by gek64](https://github.com/gek64/GitHubDesktopPortable)

```batch
gru.exe --repo gek64/GitHubDesktopPortable --extract --app GitHubDesktopPortable.exe --with "paf" --no-leave --no-script --pause
```

💾 **TCPU Repa:** [View the addon using this example](https://tcpu.ru/info/REPA/Work/GitHub%20Desktop/info.html)

### Old example without the `--rv` argument, [Flameshot Portable](https://github.com/flameshot-org/flameshot)

```batch
gru.exe --repo flameshot-org/flameshot --extract --app flameshot.exe --with "win64.zip" --no-leave --script --pause
```

💾 **TCPU Repa:** [View the addon using this example](https://tcpu.ru/info/REPA/Multimedia/Flameshot/info.html)
📜 **Script:** [View](https://github.com/Zalexanninev15/GRU/blob/main/script.bat)

## Used tool

- [redl](https://github.com/gek64/redl) ([GPLv3](https://github.com/gek64/redl/blob/main/LICENSE))
