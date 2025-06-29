This file is supposed to be in `watg/source-code`

# File tree of necessary `watg` files

```
watg/
└─ src-tauri/                    # COMPILE HERE
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
