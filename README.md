# ![WATG logo](https://github.com/DavidBevi/WATG/blob/main/source-code/src-tauri/icon.png?raw=true) WATG: Whatsapp+Telegram <sub><sup>v0.3-Alpha</sup></sub>

> [!WARNING]
> - This app is made with Tauri for Windows (I'm unable to make it cross-platform 😢)
> - This is just a backup of my dev build, it's not ready for production (yet!)

<br/>

## How to use/dev it <sub><sup> → Meant for myself (I tend to forget), but if you dare to try it good luck, and gimme feedback! 😊</sup></sub>

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

<br/>

</details>
   
WATG window can show Whatsapp, show Telegram, be Hidden. To cycle through these states click on the tray icon.

<br/>

## Planned features <sub><sup> → Hopefully 🤞</sup></sub>

- [ ] make custom notifications
- [ ] make zoom adjustable and persistent
- [x] ~~make taskbar display unread count~~ done in v0.3
- [ ] make tray icon display unread count
