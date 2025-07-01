`source-code/README.md`

# How to compile

### (1) Needed files:

```
📁src-tauri/                  
├ 📁.cargo/                
│ └ config.toml             C   Enables CRT static linking
├ 📁capabilities/
│ └ window-state.json       T   Permissions for window-state plugin
├ 📁src/                     
│ ├ 📁icons/                
│ │ └ icon-watg.ico         W   Icon for the Titlebar and Taskbar
│ │ └ tray-{0..10}.png      W   Icons for bundle and tray
│ ├ 📁scripts/              
│ │ ├ wa.js                 W   JS badge logic + UI mods for WhatsApp
│ │ └ tg.js                 W   JS badge logic for Telegram
│ └ main.rs                 W   Main Rust source for Tauri app
├ Cargo.toml                C   Crates, dependencies, plugins, features
└ tauri.conf.json           T   Defines app/bundle
```

Letters stand for which program needs it (**C**argo, **T**auri, **W**atg)

### (2) Build:

- Either open a CLI in `src-tauri` & run `cargo tauri build --features unstable`
- Or just open `𝐃𝐎𝐔𝐁𝐋𝐄-𝐂𝐋𝐈𝐂𝐊-𝐓𝐎-𝐁𝐔𝐈𝐋𝐃.bat` (it does the things above for you)

### (3) Output:

- You now can find `src-tauri\target\x86_64-pc-windows-msvc\release\watg.exe`, this file is the portable app. Enjoy!
