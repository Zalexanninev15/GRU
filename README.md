# Github Release Updater | [Latest release](https://github.com/Zalexanninev15/GRU/releases/latest)

[![](https://img.shields.io/badge/OS-Windows-informational?logo=windows)](https://github.com/Zalexanninev15/GRU)
[![](https://img.shields.io/badge/written_on-Rust-000000.svg?logo=rust)](https://github.com/Zalexanninev15/GRU)
[![](https://img.shields.io/github/v/release/Zalexanninev15/GRU)](https://github.com/Zalexanninev15/GRU/releases/latest)
[![](https://img.shields.io/github/downloads/Zalexanninev15/GRU/total.svg)](https://github.com/Zalexanninev15/GRU/releases)
[![](https://img.shields.io/github/last-commit/Zalexanninev15/GRU/main.svg)](https://github.com/Zalexanninev15/GRU/commits/main)
[![](https://img.shields.io/github/stars/Zalexanninev15/GRU.svg)](https://github.com/Zalexanninev15/GRU/stargazers)
[![](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![](https://img.shields.io/badge/donate-Buy_Me_a_Coffee-F94400.svg)](https://teletype.in/@zalexanninev15/donate)

## Description

Updater for applications from GitHub

## Features

- Features of fine-tuning the update process, a huge number of arguments for incredibly fine-tuning asset downloads from GitHub and beyond...
- Support for releases without assets, the file itself will be downloaded from another site via a direct link ([example](https://github.com/Zalexanninev15/GRU#example-visual-studio-code))
- Using, at the user's choice, as many as three types of downloader: **curl**, **wget**, **native**.
- Using the console **7-Zip** allows you to unpack almost all kinds of release archives.
- Automatically kill the process of the updated application when updating, which eliminates possible difficulties when updating applications that are already running.
- Support for a custom script that runs after an application update (`script.bat` file).
- Clear and sufficiently self-sufficient documentation (run `gru.exe` without arguments).
- Support for determining the current version when using a specific argument for a more fine-grained application update process.
- Automatically cleans all temporary files after its work (there may be problems with some, but in most cases everything goes fine).

## System requirements

**OS:** Windows 10 build 1809+ (x64)/11. [Final version for x32](https://github.com/Zalexanninev15/GRU/releases/tag/1.4.0.1)

> Support for early OS versions is possible when using the "native" downloader, as well as when specifying the path to the executable files for "curl" or "wget".
> 
> Support for Windows 7/8/8.1 is possible, but not guaranteed and not tested, it may be necessary to recompile the project for outdated systems.

## Usage

1. Copy the `UpdateTools` folder from the archive to the application folder.
2. Run the `gru.exe` file with the arguments (the full list of arguments can be obtained by running `gru.exe`, even without console, just launching it.
3. The file/archive will be downloaded. If it is an archive, then it will be unzipped to a folder a level higher than the current one (i.e. you need to remove `UpdateTools` from the path). If it is a single file, then it will simply be moved (also to a higher level). The archive will be deleted automatically after unpacking.

### Example, [GitHub Desktop Portable by gek64](https://github.com/gek64/GitHubDesktopPortable)

```batch
gru.exe --repo gek64/GitHubDesktopPortable --app GitHubDesktopPortable.exe --with "paf" --main App\GitHubDesktop\GitHubDesktop.exe
```

### Example, [Flameshot Portable](https://github.com/flameshot-org/flameshot)

```batch
gru.exe --repo flameshot-org/flameshot --app flameshot.exe --with "win64.zip" --script
```

📜 **Script:** [View](https://github.com/Zalexanninev15/GRU/blob/main/script.bat)

### Example, [draw.io Desktop](https://github.com/jgraph/drawio-desktop)

```batch
gru.exe --repo jgraph/drawio-desktop --app app.exe --with "-windows-32bit-no-installer.exe" --no-extract
```

### Example, [ScreenToGif](https://github.com/NickeManarin/ScreenToGif)

```batch
gru.exe --repo NickeManarin/ScreenToGif --app ScreenToGif.exe --with ".Light.Portable.x86"
```

### Example, [Visual Studio Code](https://github.com/microsoft/vscode)

```batch
gru.exe --repo microsoft/vscode --app Code.exe --with "null" --link "https://code.visualstudio.com/sha/download?build=stable&os=win32-x64-archive"
```


## Build (with PowerShell)

1. Install all dependencies as Admin (it is recommended to use packages from the [Chocolatey package manager](https://chocolatey.org))

```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
choco install rust mingw git -y
```

2. Download the repository

```powershell
git clone https://github.com/Zalexanninev15/GRU
cd .\GRU\
```

3. Download Resource Hacker as ZIP ([download](https://www.angusj.com/resourcehacker/resource_hacker.zip)) and unzip it to the project folder (required for embedding the manifest to request Administrator rights).

4. Compile the GRU with my script in PowerShell!

```powershell
.\my_compiler.ps1
```

5. The resulting file (`gru.exe`) will be in the project folder, not the release folder.
