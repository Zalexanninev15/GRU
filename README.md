# updater

[![](https://img.shields.io/badge/OS-Windows-informational?logo=windows)](https://github.com/Zalexanninev15/updater)
[![](https://img.shields.io/badge/written_on-Rust-000000.svg?logo=rust)](https://github.com/Zalexanninev15/updater)
[![](https://img.shields.io/github/v/release/Zalexanninev15/updater)](https://github.com/Zalexanninev15/updater/releases/latest)
[![](https://img.shields.io/github/downloads/Zalexanninev15/updater/total.svg)](https://github.com/Zalexanninev15/updater/releases)
[![](https://img.shields.io/github/last-commit/Zalexanninev15/updater/main.svg)](https://github.com/Zalexanninev15/updater/commits/main)
[![](https://img.shields.io/github/stars/Zalexanninev15/updater.svg)](https://github.com/Zalexanninev15/updater/stargazers)
[![](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![](https://img.shields.io/badge/donate-Buy_Me_a_Coffee-F94400.svg)](https://zalexanninev15.jimdofree.com/buy-me-a-coffee)

## Description

Updater for applications from GitHub

## Features

- Convenient data transfer for third-party utilities
- Sufficiently flexible update setup
- Support for a custom script that runs after an application update (`script.bat` file)
- Clear and sufficiently self-sufficient documentation (run `updater.exe` without arguments)
- The folder `$PLUGINSDIR`, which is contained in paf-portable, will be deleted after unpacking, because folder is unnecessary when running a portable application

## System requirements

- **OS:** Windows 7 or higher
- **Additionally:** May require wget, aria2, or curl (your choice). You can install the utility(s) on the system or put it next to `updater.exe`. Curl can be installed with the [CurlMini](https://github.com/Zalexanninev15/CurlMini) utility, and in Windows 10 version 1803 the curl utility is already installed on the system.

## Usage

1. Copy the `UpdateTools` folder from the archive to the application folder
2. Run the `updater.exe` file with the arguments. [Read more about existing arguments](https://github.com/Zalexanninev15/updater/blob/main/arguments.txt)

### Example, [GitHub Desktop Portable by gek64](https://github.com/gek64/GitHubDesktopPortable)

```batch
updater.exe --repo gek64/GitHubDesktopPortable --extract --app GitHubDesktopPortable.exe --with "paf" --no-script --pause
```

💾 **TCPU Repa:** [View the repack using this example](https://tcpu.ru/info/REPA/Work/GitHub%20Desktop/info.html)

### Example, [Flameshot Portable](https://github.com/flameshot-org/flameshot)

```batch
updater.exe --repo flameshot-org/flameshot --extract --app flameshot.exe --with "win64.zip" --script --pause
```

💾 **TCPU Repa:** [View the repack using this example](https://tcpu.ru/info/REPA/Multimedia/Flameshot/info.html)
📜 **Script:** [View](https://github.com/Zalexanninev15/updater/blob/main/script.bat)

## Used tool

- [redl](https://github.com/gek64/redl) ([GPLv3](https://github.com/gek64/redl/blob/main/LICENSE))
