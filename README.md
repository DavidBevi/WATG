<p align="center">
  <img src="https://github.com/DavidBevi/WATG/blob/main/demo-pics/watg-title.png" width="330pt" align="center">
  <div align="center"><a href="https://github.com/DavidBevi/WATG/blob/main/executables/WATG-0.9.0-Beta.exe?raw=true"><b>v0.9.0-Beta</b></a> - for Windows 10/11</div>
</p>

### ✅ WATG: Whatsapp+Telegram web in a super-light portable app:
- ✅ Native window, reactive UI
- ✅ Native toast-notifications
- ✅ Click-on-tray-icon to cycle between `WA` → `TG` → `Hidden`
- ✅ Unread count in tray icon <img src="https://github.com/DavidBevi/WATG/blob/main/demo-pics/tray-all.png" height="13px"> and taskbar / titlebar (text)

### ❌ But WATG is still in beta because:
- ❌ My testing was very limited and I don't know if WATG is stable and functional. [Any feedback is very welcome!](https://github.com/DavidBevi/WATG/issues/new)

### ⚠️ You also should know:
Whatsapp is a _bitch_: it doesn't have a "narrow-layout", it doesn't expose the notifications, and WATG has to workaround these issues with JS injection. Changes in Whatsapp code _will_ eventually break WATG, so I'm packing it with:
- ⚙️ **DevTools**, accessible (in tray menu) to inspect HTML/CSS/JS and troubleshoot.<br/>&nbsp; → _Help is available with command `help()` in Whatsapp's JS console_
- 🎨 **Custom mods**, via side-load: ➊ download [**wa.js**](https://github.com/DavidBevi/WATG/blob/main/src-tauri/src/scripts/wa.js), ➋ edit as needed, ➌ keep it in the same folder of WATG.exe.<br/>&nbsp; → _Mods are loaded when WATG loads itself, restart it to apply changes_

<br/>

🔧 WATG is made with Tauri, but I'll make and share executables only for **Windows**. For other systems you can have a look at the [**build tools and instructions**](https://github.com/DavidBevi/WATG/blob/main/src-tauri) and see if you can do it by yourself.

<br/>

<p align="center">
  <img src="https://github.com/DavidBevi/WATG/blob/main/demo-pics/example-toast.png">
</p>

<br/>
<br/>
