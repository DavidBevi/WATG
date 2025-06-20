# WATG: Whatsapp+Telegram<sub><sup> — written in Tauri for Windows</sup></sub>

> [!WARNING]  
> This is just a backup of my dev build, it's not ready for production.

<br/><br/>

## How to use it <sub><sup> → mainly for myself in the future, I tend to forget</sup></sub>

<details>
  <summary>Installation instructions</summary>
  <br/>

1. install prerequisites:
   - `visual studio build tools 2022 (v17.14.6+36212.18.-june.2025-) ` > `MSVC compiler (v143)`
   - `rustc 1.87.0 (17067e9ac 2025-05-09)`
   - `tauri-cli 2.5.0`
2. download [main.zip](https://github.com/DavidBevi/WATG/archive/refs/heads/main.zip) and unzip it in a folder
   - (example: `C:\watg`)
3. open a `cmd` in that folder and use these commands:
    - `cargo tauri dev --features unstable` to run it like a dev, with autoreload when source files are saved
    - `cargo tauri build --features unstable` to export a portable executable in `src-tauri\target\release`

<br/>

</details>
   
To cycle betwheen Whatsapp and Telegram click on the tray icon

<br/><br/>

## Planned features <sub><sup> → hopefully</sup></sub>

- make zoom adjustable and persistent
- make WATG closable-in-tray
- make custom notifications
