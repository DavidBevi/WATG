This is `watg/source-code/README.md`

# How to compile

### (1) Needed files:

```
src-tauri/                     # COMPILE HERE
├─ .cargo/                 
│  └─ config.toml             # Rust/Cargo config, enables CRT static linking
├─ capabilities/
│  └─ window-state.json       # Plugin permissions for window-state plugin
├─ src/                       
│  ├─ icons/                  
│  │  └─ icon-watg-*.png      # Tray+bundle icons
│  ├─ scripts/                
│  │  ├─ wa.js                # JS badge logic + UI mods for WhatsApp
│  │  └─ tg.js                # JS badge logic for Telegram
│  └─ main.rs                 # Main Rust source for Tauri app
├─ Cargo.toml                 # Defines crate, dependencies, plugin features
└─ tauri.conf.json            # Contains Tauri app/bundle config
```

### (2) CLI

1. open a CLI in `src-tauri`
2. run `cargo tauri build --features unstable`

### (3) Output 

- You now can find `scr-tauri/target/releases/watg.exe`, this file is the portable app. Enjoy!
