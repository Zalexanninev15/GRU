# Github Release Updater

[![](https://img.shields.io/badge/platform-Windows-informational)](https://github.com/Zalexanninev15/GRU)
[![](https://img.shields.io/badge/written_on-Rust-000000.svg?logo=rust)](https://github.com/Zalexanninev15/GRU)
[![](https://img.shields.io/github/v/release/Zalexanninev15/GRU)](https://github.com/Zalexanninev15/GRU/releases/latest)
[![](https://img.shields.io/github/downloads/Zalexanninev15/GRU/total.svg)](https://github.com/Zalexanninev15/GRU/releases)
[![](https://img.shields.io/github/last-commit/Zalexanninev15/GRU/main.svg)](https://github.com/Zalexanninev15/GRU/commits/main)
[![](https://img.shields.io/github/stars/Zalexanninev15/GRU.svg)](https://github.com/Zalexanninev15/GRU/stargazers)
[![](https://img.shields.io/github/forks/Zalexanninev15/GRU.svg)](https://github.com/Zalexanninev15/GRU/network/members)
[![](https://img.shields.io/github/issues/Zalexanninev15/GRU.svg)](https://github.com/Zalexanninev15/GRU/issues?q=is%3Aopen+is%3Aissue)
[![](https://img.shields.io/github/issues-closed/Zalexanninev15/GRU.svg)](https://github.com/Zalexanninev15/GRU/issues?q=is%3Aissue+is%3Aclosed)
[![](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![](https://img.shields.io/badge/Donate-FFDD00.svg?logo=buymeacoffee&logoColor=black)](https://z15.neocities.org/donate)

## Description

Updater for applications from GitHub. It has a huge number of convenient settings and supports multiple rockers, and can download files not only from GitHub, but also based only on the release, supports pre-releases and a fairly deep asset search among releases.

## Features

- **Fine-Grained Update Customization**: Highly customizable asset download process with numerous arguments to adjust the update flow, making it flexible for a wide range of use cases.
- **Support for Releases Without Assets**: In cases where no assets are found, the application can directly download the EXE or files from an external URL ([example](https://github.com/Zalexanninev15/GRU#example-visual-studio-code)).
- **Multiple Downloader Options**: Choose between **curl**, **wget**, the built-in **gru** (**gru-classic**) or **tcpud** downloader for maximum flexibility in downloading assets.
- **Seamless Archive Extraction**: Leverage **7-Zip** via the command line to handle nearly all types of archive files during the extraction process.
- **Automatic Process Termination**: Automatically terminates the running application during updates to avoid issues with in-use applications, ensuring smooth updates.
- **Pre-Update Script Support**: Optionally execute a custom `prepare.bat` file before the update to perform additional tasks.
- **Post-Update Script Support**: Optionally execute a custom `script.bat` file after the update to perform additional tasks.
- **Comprehensive and Self-Sufficient Documentation**: View detailed documentation by running `gru.exe` without any arguments to understand every option and its usage.
- **Version Control for Precise Updates**: Fine-tune the update process by determining the current version and ensuring that the right release is applied based on your setup.
- **Automatic Cleanup**: Automatically remove temporary files post-update to keep your system clean. Most cases work without issues, but a few may require manual intervention.
- **Pre-release Support**: Optionally select pre-releases for updates if a stable release is not available or if the latest unstable release is preferred.
- **Asset Search Across Multiple Releases**: Search for assets across multiple recent releases (not just the latest), ensuring you always get the right version of the file.

## Available arguments

> To better understand this, the developer recommends reading the information about arguments in the utility itself, as it contains more details.

- `--app <application.exe>` — Set the EXE of the launcher/main application.
- `--main <path>` — Set the path to the main application located one level above the EXE.
- `--extract` / `--no-extract` — Decide whether to extract archive files or simply move the downloaded EXE.
- `--leave` / `--no-leave` — Control whether to keep or remove unnecessary folders.
- `--before` / `--no-before` — Run a `prepare.bat` script before downloading (optional).
- `--script` / `--no-script` — Optionally run `script.bat` after the download and extraction.
- `--silent` / `--no-silent` — Hide the console window during execution for a quieter experience.
- `--details` / `--no-details` — Show detailed download information using curl/wget.
- `--nupkg` / `--no-nupkg` —  Enabling the correct operation mode with nuget packages (.nupkg), which include the release of the downloaded application itself.
- `--tool <type>` — Choose between download tools like `curl`, `wget`, `gru`, `gru-classic`, or `tcpud` (curl-only).
- `--link <url>` — Use a direct download link when assets are unavailable in the release. Sometimes releases may not contain assets to download, but just be a place for a list of changes (What's new?). Supports version substitution in the link to the version from GitHub, if such a rule is implied by the developer of the downloaded application. To do this, enter the text `<version>` in the appropriate place, and it will be replaced with the version from GitHub.
- `--ua <user-agent>` — Customize the user-agent string for optimized download speeds.
- `--regex` / `--no-regex` — Use `--with` to search using a regular expression instead of a regular match.
- `--gh <personal access token>` — Use a GitHub personal access token for improved access if there are restrictions.
- `--wgetrc` / `--no-wgetrc` — Use the wget configuration file (.wgetrc).
- `--pre` / `--no-pre` — Use pre-releases if stable versions are unavailable.
- `--ghost` / `--no-ghost` — Search for assets across multiple recent releases, not just the latest. If a developer publishes different applications in the same repository in different releases.

This program offers total control over your application updates and downloads, ensuring a smooth, customizable, and clean update process every time.

> A simplified description is provided here, and a more detailed one is provided in the utility itself.

## System requirements

**OS:** Windows 10 build 1809+ (x64)/11. [Final version for x32](https://github.com/Zalexanninev15/GRU/releases/tag/1.4.0.1)

> Support for early OS versions is possible when using the built-in downloader ("gru" or "gru-classic"), as well as when specifying the path to the executable files for "curl" or "wget".
> Support for Windows 7/8/8.1 (x64) and old builds of Windows 10 (x64) is possible, but not guaranteed and not tested, it may be necessary to recompile the project for outdated systems.

## Usage

1. Copy the `UpdateTools` folder from the archive to the application folder.
2. Run the `gru.exe` file with the arguments (the full list of arguments can be obtained by running `gru.exe`, even without console, just launching it).
3. The file/archive will be downloaded. If it is an archive, then it will be unzipped to a folder a level higher than the current one (i.e. you need to remove `UpdateTools` from the path). If it is a single file, then it will simply be moved (also to a higher level). The archive will be deleted automatically after unpacking. The actions depend on the user-selected GRU launch arguments.

## Examples

### [GitHub Desktop Portable by gek64](https://github.com/gek64/GitHubDesktopPortable)

```batch
gru.exe --repo gek64/GitHubDesktopPortable --app GitHubDesktopPortable.exe --with "paf" --main App\GitHubDesktop\GitHubDesktop.exe --tool wget
```

### [Flameshot Portable](https://github.com/flameshot-org/flameshot)

```batch
gru.exe --repo flameshot-org/flameshot --app flameshot.exe --with "win64.zip" --script
```

📜 **Script:** [View](https://github.com/Zalexanninev15/GRU/blob/main/script.bat)

### [draw.io Desktop](https://github.com/jgraph/drawio-desktop)

```batch
gru.exe --repo jgraph/drawio-desktop --app app.exe --with "-windows-no-installer.exe" --no-extract
```

### [ScreenToGif](https://github.com/NickeManarin/ScreenToGif)

```batch
gru.exe --repo NickeManarin/ScreenToGif --app ScreenToGif.exe --with ".Portable.x64.zip" --tool curl
```

### [Visual Studio Code](https://github.com/microsoft/vscode)

```batch
gru.exe --repo microsoft/vscode --app Code.exe --with "null" --link "https://code.visualstudio.com/sha/download?build=stable&os=win32-x64-archive"
```

### [Insomnia](https://github.com/Kong/insomnia)

```batch
gru.exe --repo Kong/insomnia --app insomnia.exe --with "-full.nupkg" --nupkg
```

### [OBS Studio](https://github.com/obsproject/obs-studio)

```batch
gru.exe --repo obsproject/obs-studio --app obs64.exe --with "^OBS-Studio-(\d+\.\d+\.\d+)-Windows(-(?:x64|amd64|Portable|Portable-x64|x64-Portable))?\.zip$" --regex --main bin\64bit\obs64.exe
```

## Build

1. Install all dependencies with Administrator rights (it is recommended to use packages from the [Scoop package manager](https://scoop.sh/))

```powershell
Set-ExecutionPolicy -Scope CurrentUser  -ExecutionPolicy Bypass -Force
Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression
```

Then:

```batch
scoop install git rust-gnu gcc
scoop bucket add extras
scoop install extras/resource-hacker rustup-gnu
```

2. Download the repository

```batch
git clone https://github.com/Zalexanninev15/GRU
cd .\GRU\
```

3. Compile the GRU with my script in CMD!

```batch
build.bat
```

4. The resulting file `gru.exe` will be in the project folder, not the release folder.
