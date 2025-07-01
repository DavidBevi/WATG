`src-tauri/README.md`

# How to compile
The following *should* be everything you need, but this is my first project and I'm not sure I'm doing it right 😅

<br/>



### Install prerequisites
  1. [**Visual Studio Build Tools** 2022, v17.14.6 (June 2025)](https://download.visualstudio.microsoft.com/download/pr/4652b1eb-63f7-432d-84ab-06108c5d7cd7/579ca9f9b1824f8dfd2ca0dca0e7e3970ca2e4dba8ee91f2e938ed2c7f197054/vs_BuildTools.exe) <br/>› select `C++ build tools` with: `MSVC v143`, `Windows10/11 SDK`
  2. [**Rust** 1.87.0](https://static.rust-lang.org/dist/rust-1.87.0-x86_64-pc-windows-msvc.msi)<br/>› this also installs `cargo` command
  3. **Tauri** 2.5.0 <br/>› install via CLI with command `cargo install tauri-cli --version 2.5.0`

<br/>



### Download [`src-tauri.zip`](https://download-directory.github.io/?url=https%3A%2F%2Fgithub.com%2FDavidBevi%2FWATG%2Ftree%2Fmain%2Fsrc-tauri) and unzip it
```
𝐅𝐈𝐋𝐄 𝐓𝐑𝐄𝐄 𝐎𝐅 𝐍𝐄𝐄𝐃𝐄𝐃 𝐅𝐈𝐋𝐄𝐒
C= needed by Cargo   |   T= needed by Tauri   |   W= code/res of WATG
-----------------------------------------------------------------------
📁src-tauri/                  
├ 📁.cargo/                
│ └ config.toml            C    Enables CRT static linking
├ 📁capabilities/
│ └ window-state.json      T    Permissions for window-state plugin
├ 📁src/                     
│ ├ 📁icons/                
│ │ └ icon-watg.ico        W    Icon for the Titlebar and Taskbar
│ │ └ tray-{0..10}.png     W    Icons for bundle and tray
│ ├ 📁scripts/              
│ │ ├ wa.js                W    JS badge logic + UI mods for WhatsApp
│ │ └ tg.js                W    JS badge logic for Telegram
│ └ main.rs                W    Main Rust source for WATG
├ Cargo.toml               C    Crates, dependencies, plugins, features
└ tauri.conf.json          T    Defines app/bundle
```

<br/>



### Build
- Open a CLI in `src-tauri` & run `cargo tauri build --features unstable`
- Or just open `𝐃𝐎𝐔𝐁𝐋𝐄-𝐂𝐋𝐈𝐂𝐊-𝐓𝐎-𝐁𝐔𝐈𝐋𝐃.bat`, which does the same things

<br/>



### Output
- You can now find `src-tauri\target\x86_64-pc-windows-msvc\release\watg.exe`, this file is the portable app. Enjoy!
