`source-code/README.md`

# How to compile

### (1) Needed files:

```
📁src-tauri/                  
├ 📁.cargo/               
│ └ config.toml               # [Rust/Cargo] Enables CRT static linking
├ 📁capabilities/
│ └ window-state.json         # [Tauri] Permissions for window-state plugin
├ 📁src/                     
│ ├ 📁icons/                
│ │ └ icon-watg-{0..10}.png   # Icons for bundle and tray
│ ├ 📁scripts/              
│ │ ├ wa.js                   # JS badge logic + UI mods for WhatsApp
│ │ └ tg.js                   # JS badge logic for Telegram
│ └ main.rs                   # Main Rust source for Tauri app
├ Cargo.toml                  # [Rust/Cargo] Cates, dependencies, plugin, features]
└ tauri.conf.json             # [Tauri] Defines app/bundle
```

### (2) Build:

1. open a CLI in `src-tauri`
2. run `cargo tauri build --features unstable`

### (3) Output:

- You now can find `scr-tauri/target/releases/watg.exe`, this file is the portable app. Enjoy!
