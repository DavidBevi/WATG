<p align="center">
  <img src="https://github.com/DavidBevi/WATG/blob/main/demo-pics/watg-title.png" width="330pt" align="center">
  <div align="center"><a href="https://github.com/DavidBevi/WATG/blob/main/executables/WATG-0.9.5-Beta.exe?raw=true"><b>v0.9.5-Beta</b></a> - for Windows 10/11</div>
</p>

<br/>

### ✅ WATG: Whatsapp+Telegram web in a super-light portable app:
- 📏 Resizable window with narrow-layout support
- 🔔 Toast-notifications for new messages
- 📥 Click-on-tray-icon to cycle between `WA` → `TG` → `Hidden`
- #️⃣ Unread count in tray-icon and taskbar / titlebar

<br/>

### ❌ But WATG is still in beta because:
- 🚧 Testing was very limited. [Any feedback is very welcome!](https://github.com/DavidBevi/WATG/issues/new)

<br/>

### ⚠️ You also should know:
Whatsapp is a _bitch_: it doesn't have a "narrow-layout", it doesn't expose notifications, and WATG has to workaround these issues with JS and CSS injection. Changes in Whatsapp code _will_ eventually break WATG, so I'm packing it with:
- ⚙️ **DevTools**, accessible (in tray menu) to inspect HTML/CSS/JS and troubleshoot.<br/>&nbsp; → _Help is available with command `help()` in Whatsapp's JS console_
- 🎨 **Custom mods**, via side-loading [**wa.css**](https://github.com/DavidBevi/WATG/blob/main/src-tauri/src/scripts/wa.css) and/or [**wa.js**](https://github.com/DavidBevi/WATG/blob/main/src-tauri/src/scripts/wa.js).<br/>&nbsp; → _❶ edit styles and/or funcs, ❷ save in the same folder of WATG.exe, ❸ relaunch WATG_

<br/>

🔧 WATG is a Tauri project, it should be relatively easy to build it for Linux and MacOS too. I'm focusing on **Windows** (at least for now), meanwhile you can check [**build tools and instructions**](https://github.com/DavidBevi/WATG/blob/main/src-tauri) and see if you can do it by yourself.

<br/>

<p align="center">
  <img src="https://github.com/DavidBevi/WATG/blob/main/demo-pics/example-toast.png">
</p>

<br/>
<br/>
