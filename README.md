# <img src="https://github.com/DavidBevi/WATG/blob/main/source-code/src-tauri/src/icons/icon-watg-2.png" height="27px"> WATG: Whatsapp+Telegram<sub><sup> - v0.4-Alpha - for Windows</sup></sub>

### This app aims to merge WhatsApp Web & Telegram Web, with:
- [x] Native window with reactive UI (using [my WA mod](https://github.com/DavidBevi/violentmonkey-scripts/blob/main/whatsapp-web-responsive.js))
- [x] Hide-in-tray ability
- [x] Unread count in taskbar / titlebar + in tray icon
- [ ] Custom notifications
- [ ] Customizable zoom settings


> [!WARNING]
> - This app is made with Tauri, but it's only for **Windows** because I'm unable to make it cross-platform 😢
> - This repo is meant as **a backup of my dev builds**, because WATG is still basic and rough.<br/>For example, it spawns where I want but only on my main computer, on my alt it spawns half off-screen 😕
> - If despite everything you download [WATG-0.4.1-Alpha.exe](https://github.com/DavidBevi/WATG/blob/main/executables/WATG-0.4.1-Alpha.exe?raw=true) good luck, and tell me how it goes! 😁


> [!TIP]
> - **WATG** can [**show Whatsapp**] - [**show Telegram**] - [**be Hidden**]. To cycle through these 3 states **click on the tray icon**.

<br/>


<br/>

# <img src="https://github.com/DavidBevi/WATG/blob/main/source-code/src-tauri/src/icons/icon-dev.png" height="27px"> DEV: <sub><sup> this is probably too basic for noobs and useless to experienced devs 🙃 anyway:</sup></sub>

1. Install prerequisites:
   1. [**Visual Studio Build Tools** 2022, v17.14.6 (June 2025)](https://download.visualstudio.microsoft.com/download/pr/4652b1eb-63f7-432d-84ab-06108c5d7cd7/579ca9f9b1824f8dfd2ca0dca0e7e3970ca2e4dba8ee91f2e938ed2c7f197054/vs_BuildTools.exe) <br/>› select `C++ build tools` with `MSVC v143`, `Windows10/11 SDK`
   2. [**Rust** 1.87.0](https://static.rust-lang.org/dist/rust-1.87.0-x86_64-pc-windows-msvc.msi)<br/>› this also installs `cargo` command
   3. **Tauri** 2.5.0 <br/>› install via CLI with command `cargo install tauri-cli --version 2.5.0`
2. Download [`source-code`](https://download-directory.github.io/?url=https%3A%2F%2Fgithub.com%2FDavidBevi%2FWATG%2Ftree%2Fmain%2Fsource-code) folder and unzip it where you want
   - (example: `C:\watg`)
3. Open a CLI in that folder and use these commands:
    - `cargo tauri dev --features unstable` to run it like a dev, with autoreload when source files are saved
    - `cargo tauri build --features unstable` to export a portable executable in `src-tauri\target\release`

<br/>
