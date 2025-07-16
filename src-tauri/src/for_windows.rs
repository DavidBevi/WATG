//,-------------------------------------------------------------------------------------,
//| for_windows.rs                                                                      |
//| - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - |
//| This file contains code specific for the Windows' version of WATG,                  |
//| such as tray icon and toast notifications                                           |
//'-------------------------------------------------------------------------------------'

// Imports
use tauri::{Wry, AppHandle, tray::{TrayIcon, TrayIconBuilder, TrayIconEvent, MouseButton,
    MouseButtonState}, menu::{Menu, MenuItem}, image::Image, Manager};
use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_ICONINFORMATION, MB_OK};
use std::{io::Write, ffi::OsStr, iter::once, os::windows::ffi::OsStrExt, 
    process::Command, os::windows::process::CommandExt};
use crate::AppState;
use winrt_toast::{Toast, ToastManager, register};
use std::path::PathBuf;
use os_info::get;
use urlencoding::encode;

const VERSION: &str = env!("CARGO_PKG_VERSION"); 

// Show native dialog using WinAPI ------------------------------------------------------
// dialog_type expects "error" or "info", others show no icon
pub fn show_dialog(message: &str, dialog_type: &str) {
  let wide: Vec<u16> = OsStr::new(message).encode_wide().chain(once(0)).collect();
  let icon_flag = match dialog_type.to_lowercase().as_str() {
    "error" => MB_ICONERROR,
    "info" => MB_ICONINFORMATION,
    _ => 0,
  };
  unsafe {
    MessageBoxW(std::ptr::null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK | icon_flag);
  }
}

// Create tray icon and attach click + menu event handlers ------------------------------
pub fn create_tray_icon(app: &AppHandle<Wry>, menu: &Menu<Wry>) -> tauri::Result<TrayIcon> {
  TrayIconBuilder::new()
    .icon(app.default_window_icon().unwrap().clone())
    .menu(menu)
    .on_tray_icon_event(move |app, event| {
      if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
      } = event {
        crate::switch_view(&app.app_handle());
      }
    })
    .on_menu_event(|app, event| {
      match event.id.0.as_str() {
        "info" => {let _ = tauri_plugin_opener::open_url("https://github.com/DavidBevi/WATG", None::<&str>);}
        "switch" => crate::switch_view(app),
        "fix_toast_titles" => {
          match toast_cache_cleanup() {
            Ok(_) => show_dialog(&format!("𝐖𝐢𝐧𝐝𝐨𝐰𝐬 𝐏𝐮𝐬𝐡 𝐍𝐨𝐭𝐢𝐟𝐢𝐜𝐚𝐭𝐢𝐨𝐧 𝐝𝐚𝐭𝐚𝐛𝐚𝐬𝐞 cleaned, WATG notifications should now load the correct title (contact or group name).\n\nIf this doesn't happen try rebooting, and if it still fails please report it as a bug (use button in the tray-icon menu)."), "info"),
            Err(err) => show_dialog(&format!("Toast DB error: {}", err), "error"),
          }
        }
        "report_bug" => {let _ = tauri_plugin_opener::open_url(format!("https://github.com/DavidBevi/WATG/issues/new?body=WATG+{}+on+{}%0ADescription:+", VERSION, encode(&get().to_string())), None::<&str>);}
        "quit" => std::process::exit(0),
        _ => {}
      }
    })
    .show_menu_on_left_click(false)
    .build(app)
}

// Update tray icon image ---------------------------------------------------------------
pub fn set_tray_icon(tray: &TrayIcon, image: Image<'static>) {
  let _ = tray.set_icon(Some(image));
}

// Load embedded tray icons from disk and return as Image vector ------------------------
pub fn load_tray_icons() -> Vec<Image<'static>> {
  let mut icons = Vec::with_capacity(11);
  for i in 0..=10 {
    let bytes: &[u8] = match i {
      0 => include_bytes!("icons/tray-0.png"),
      1 => include_bytes!("icons/tray-1.png"),
      2 => include_bytes!("icons/tray-2.png"),
      3 => include_bytes!("icons/tray-3.png"),
      4 => include_bytes!("icons/tray-4.png"),
      5 => include_bytes!("icons/tray-5.png"),
      6 => include_bytes!("icons/tray-6.png"),
      7 => include_bytes!("icons/tray-7.png"),
      8 => include_bytes!("icons/tray-8.png"),
      9 => include_bytes!("icons/tray-9.png"),
      10 => include_bytes!("icons/tray-10.png"),
      _ => unreachable!(),
    };
    let image = Image::from_bytes(bytes).expect("Failed to create image");
    icons.push(image);
  }
  icons
}

// Update tray icon based on total badge count ------------------------------------------
pub fn update_tray_icon(app: &AppHandle<Wry>, count: u8) {
  let state = app.state::<AppState>();
  let tray = state.tray.lock().unwrap();
  if let Some(tray) = tray.as_ref() {
    let icon_index = count.min(10);
    let image = state.tray_icons[icon_index as usize].clone();
    set_tray_icon(tray, image);
  }
}

// Embedded app icon (required by toast notifications) ----------------------------------
static ICON_WATG: &[u8] = include_bytes!("icons/icon-watg.ico");

// Send native Windows notifications ----------------------------------------------------
pub fn send_notification(title: &str, body: &str) {
  use std::sync::OnceLock;

  static ICON_PATH: OnceLock<PathBuf> = OnceLock::new();

  let icon_path = ICON_PATH.get_or_init(|| {
    let path = std::env::temp_dir().join("watg-icon.ico");
    std::fs::write(&path, ICON_WATG).expect("Failed to write embedded icon to file");
    path
  });

  let aumid = format!("davidbevi.watg.{}", title.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>());

  register(&aumid, title, Some(icon_path)).expect("Failed to register toast shortcut");

  let mut binding = Toast::new();
  let toast = binding.text1(body);
  let _ = ToastManager::new(&aumid).show(&toast);

  let script = r#"@echo off
setlocal enabledelayedexpansion
set basekey=HKCU\Software\Classes\AppUserModelId
for /f "tokens=*" %%K in ('reg query "%basekey%"') do (
    set "keyname=%%~nxK"
    if /i "!keyname:~0,14!"=="davidbevi.watg" (reg delete "%basekey%\!keyname!" /f >nul 2>&1))
endlocal"#;

  if let Ok(mut file) = std::fs::File::create(std::env::temp_dir().join("watg-registry-cleaner.bat")) {
    if file.write_all(script.as_bytes()).is_ok() {
      let bat_path = std::env::temp_dir().join("watg-registry-cleaner.bat");
      let output = Command::new("cmd")
        .args(&["/C", bat_path.to_str().unwrap()])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output();

      if let Ok(output) = output {
        if !output.status.success() {
          eprintln!("Batch execution failed:\n{}", String::from_utf8_lossy(&output.stderr));
        }
      } else {
        eprintln!("Failed to launch batch script.");
      }
    }
  }

  println!("«{}» → 🔔", body.replace(['\n', '\r'], " ").chars().take(60).collect::<String>());
}

// Fix for corrupted toast notification titles ------------------------------------------
fn toast_cache_cleanup() -> Result<usize, String> {
  use rusqlite::Connection;

  let path = dirs::data_local_dir()
    .ok_or("Cannot resolve %LocalAppData%")?
    .join("Microsoft\\Windows\\Notifications\\wpndatabase.db");

  if !path.exists() {
    return Err("Database file not found".into());
  }

  let conn = Connection::open(&path).map_err(|e| e.to_string())?;

  conn.execute("DELETE FROM HandlerSettings;", []).ok();
  let count = conn.execute(
    "DELETE FROM NotificationHandler WHERE PrimaryId LIKE 'davidbevi%';",
    [],
  ).map_err(|e| e.to_string())?;

  Ok(count)
}

// Create tray menu with all options ----------------------------------------------------
pub fn build_tray_menu(app: &AppHandle<Wry>) -> Menu<Wry> {
  let info_i = MenuItem::with_id(app, "info", format!("ℹ️  𝐖𝐀𝐓𝐆 {} (about)", VERSION), true, None::<&str>).unwrap();
  let switch_i = MenuItem::with_id(app, "switch", "🔃  Switch view", true, None::<&str>).unwrap();
  let fix_i = MenuItem::with_id(app, "fix_toast_titles", "🗑️  Toast-cache clean-up", true, None::<&str>).unwrap();
  let report_i = MenuItem::with_id(app, "report_bug", "🪲  Report a bug", true, None::<&str>).unwrap();
  let quit_i = MenuItem::with_id(app, "quit", "❌  Quit WATG", true, None::<&str>).unwrap();

  Menu::with_items(app, &[&info_i, &switch_i, &fix_i, &report_i, &quit_i]).unwrap()
}
