# ![WATG logo](https://github.com/DavidBevi/WATG/blob/main/source-code/src-tauri/icon.png?raw=true) WATG: Whatsapp+Telegram <sub><sup>v0.3-Alpha</sup></sub>

### This app aims to merge WhatsApp Web & Telegram Web, with:
- [x] Native window
- [x] Hide-in-tray ability
- [x] Unread count in taskbar
- [ ] Unread count in tray icon
- [ ] Custom notifications
- [ ] Zoom controls (independend between WA and TG)


> [!WARNING]
> - This app is made with Tauri, but it's only for **Windows** because I'm unable to make it cross-platform 😢
> - This repo is meant as **a backup of my dev builds**, because WATG is still basic and rough.<br/>For example, it spawns where I want but only on my main computer, on my alt it spawns half off-screen 😕
> - If despite everything you download [WATG-0.3-Alpha.exe](https://github.com/DavidBevi/WATG/blob/main/executables/WATG-0.3-Alpha.exe?raw=true) good luck, and tell me how it goes! 😁


> [!TIP]
> - **WATG** can [**show Whatsapp**] - [**show Telegram**] - [**be Hidden**]. To cycle through these 3 states **click on the tray icon**.

<br/>


<br/>

# DEV: <sub><sup> Meant for myself (I tend to forget), but if you try it good luck, and gimme feedback! 😊</sup></sub>

<details>
  <summary>Install dependencies and run commands</summary>
  <br/>

1. install prerequisites:
   - `visual studio build tools 2022 (v17.14.6+36212.18.-june.2025-) ` > `MSVC compiler (v143)`
   - `rustc 1.87.0 (17067e9ac 2025-05-09)`
   - `tauri-cli 2.5.0`
2. download source-code and unzip it in a folder
   - from [main.zip](https://github.com/DavidBevi/WATG/archive/refs/heads/main.zip), extract folder `source-code`
   - save it somewhere (example: `C:\watg`)
3. open a `cmd` in that folder and use these commands:
    - `cargo tauri dev --features unstable` to run it like a dev, with autoreload when source files are saved
    - `cargo tauri build --features unstable` to export a portable executable in `src-tauri\target\release`

</details>
