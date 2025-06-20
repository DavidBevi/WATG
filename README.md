# WATG<sub><sup> • Whatsapp+Telegram • written in Tauri for Windows</sup></sub>

This is just a backup of my dev build.
- It compiles with `rustc 1.87.0 (17067e9ac 2025-05-09) (host: x86_64-pc-windows-msvc)` + `tauri-cli 2.5.0`

To compile it:
1. install prerequisites above (Rust, Tauri)
2. download everything (green button "code" → "download ZIP")
3. unzip in a folder (example: "watg")
4. open a `cmd` in the root folder (example: "watg")
5. use command `cargo tauri build --features unstable` to export a portable executable
