<p align="center">
  <img src="https://github.com/DavidBevi/WATG/blob/main/demo-pics/watg-title.png" width="330pt" align="center">
  <div align="center"><a href="https://github.com/DavidBevi/WATG/blob/main/executables/WATG-0.8.3-Beta.exe?raw=true"><b>v0.8.3-Beta</b></a> - for Windows 10/11</div>
</p>

### ✅ WATG combines WhatsApp Web & Telegram Web in a super-light app, with:
- ✅ Native window with reactive UI
- ✅ Native toast-notifications for Telegram <sup>(Todo: Whatsapp)</sup>
- ✅ Click-on-tray-icon to cycle between `WA` → `TG` → `Hidden`
- ✅ Unread count in tray icon <img src="https://github.com/DavidBevi/WATG/blob/main/demo-pics/tray-all.png" height="13px"> and taskbar / titlebar (text)

### ❌ But WATG is still in beta because:
- ❌ Whatsapp notifications are broken, they used to work but a change in WA code broke them. I'm not even sure I can intercept them anymore 😶
- ❌ My testing was very limited and I don't know if WATG is stable and functional. [Any feedback is very welcome!](https://github.com/DavidBevi/WATG/issues/new)

### ⚠️ You also should know:
- ⚠️ Whatsapp can change its code at any time, breaking *any* mod. It already happened with the unread-badge-count (fixed) and notifications (still broken).
- ⚠️ Whatsapp doesn't have a "narrow-layout", I made a userstyle to enable it **but** it's very partial. You can cope with it and enlarge the window when needed, or **mod my mod**: ➊ download [**wa.js**](https://github.com/DavidBevi/WATG/blob/main/src-tauri/src/scripts/wa.js), ➋ edit as needed, ➌ keep it in the same folder of the WATG executable. WATG will load your `wa.js` when it loads itself.
- 🫥 Really, Whatsapp is a bitch, please tell everybody to use *any* alternative!

<br/>

🔧 WATG is made with Tauri, but I'll make and share executables only for **Windows**. For other systems you can have a look at the [**build tools and instructions**](https://github.com/DavidBevi/WATG/blob/main/src-tauri) and see if you can do it by yourself.

<br/>

<p align="center">
  <img src="https://github.com/DavidBevi/WATG/blob/main/demo-pics/example-toast.png">
</p>

<br/>
<br/>
