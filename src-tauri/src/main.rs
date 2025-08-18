//,-------------------------------------------------------------------------------------,
//| main.rs - entry point for WATG Tauri app                                            |
//| - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - |
//| This file contains all cross-platform logic and delegates                           |
//| platform-specific code to for_[platform].rs                                         |
//'-------------------------------------------------------------------------------------'

// Disable terminal window on Windows ---------------------------------------------------
#![cfg_attr(target_os="windows", windows_subsystem="windows")] 
// DO NOT DELETE ------------------------------------------------------------------------

// Import core features, for every platform ---------------------------------------------
use std::{fs::File, io::Write, sync::Mutex};
use tauri::{
  AppHandle, Manager, LogicalPosition, LogicalSize, PhysicalSize, WebviewUrl,
  image::Image, webview::{Webview, WebviewBuilder}, tray::TrayIcon, Listener,
  window::{EffectsBuilder, Effect, Color}
};
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};

// Import platform specific logic -------------------------------------------------------
#[cfg(target_os="windows")] mod for_windows;
#[cfg(target_os="windows")] use for_windows as platform;
#[cfg(target_os="linux")] mod for_linux;
#[cfg(target_os="linux")] use for_linux as platform;
#[cfg(target_os="macos")] mod for_macos;
#[cfg(target_os="macos")] use for_macos as platform;

// Shared application state, accessible from commands and UI events ---------------------
struct AppState {
  state_index: Mutex<u8>,
  webview_wa: Mutex<Option<Webview>>,
  webview_tg: Mutex<Option<Webview>>,
  title_wa: Mutex<String>,
  title_tg: Mutex<String>,
  tray: Mutex<Option<TrayIcon>>,
  badge_wa: Mutex<u8>,
  badge_tg: Mutex<u8>,
  tray_icons: Vec<Image<'static>>,
}

// Called on panic: logs to file and shows an error dialog (only on Windows) ------------
fn setup_error_hook() {
  std::panic::set_hook(Box::new(|info| {
    let msg = if let Some(s) = info.payload().downcast_ref::<&str>() {*s}
         else if let Some(s) = info.payload().downcast_ref::<String>() {s.as_str()}
         else {"Unknown panic"};
    let log = format!("Panic: {}\n", msg);
    let _ = File::create("watg-error.log").and_then(|mut f| f.write_all(log.as_bytes()));
    #[cfg(target_os = "windows")]
    platform::show_dialog(msg, "error");
  }));
}

// Called from JS to report a new title and update tray icon accordingly ----------------
#[tauri::command]
fn report_title(app: AppHandle, title: String, label: String) {
  let state = app.state::<AppState>();
   {
    let mut wa = state.title_wa.lock().unwrap();
    let mut tg = state.title_tg.lock().unwrap();
    if label == "WA" { *wa = title.clone(); } else { *tg = title.clone(); }
  }
  if let Some(w) = app.get_window("main") {
    let wa = state.title_wa.lock().unwrap();
    let tg = state.title_tg.lock().unwrap();
    let full = format!("WATG {} {}", if wa.is_empty() {""} else {&*wa}, if tg.is_empty() {""} else {&*tg});
    let _ = w.set_title(&full);
  }
  let count = if title == "_" {0} else {title.parse::<u8>().unwrap_or(0)};
  {
    let mut badge_wa = state.badge_wa.lock().unwrap();
    let mut badge_tg = state.badge_tg.lock().unwrap();
    if label == "WA" { *badge_wa = count; } else { *badge_tg = count; }
    platform::update_tray_icon(&app, badge_wa.saturating_add(*badge_tg));
  }
}

// Called from JS to send a native system notification ----------------------------------
#[tauri::command]
fn notify(title: String, body: String) {
  print!("✉️  {} on TG: ", title);
  platform::send_notification(&title, &body);
}

// Switches between: WA view, TG view, and hidden state ---------------------------------
pub fn switch_view(app: &AppHandle, target: Option<u8>) {
  let w = app.get_window("main").unwrap();
  let state = app.state::<AppState>();
  let mut idx = state.state_index.lock().unwrap();
  let new_idx = target.unwrap_or_else(|| (*idx + 1) % 3);

  if new_idx == 0 { // Hide
      w.hide().unwrap();
      w.set_skip_taskbar(true).unwrap();
  } else { // Show
      #[cfg(target_os="windows")] w.set_effects(EffectsBuilder::new().effect(Effect::MicaDark)
        .build()).expect("Can't set theme");
      w.show().unwrap();
      w.set_skip_taskbar(false).unwrap();
      w.set_focus().unwrap();
      w.set_always_on_top(true).unwrap();
      w.set_always_on_top(false).unwrap();
      let width = w.inner_size().unwrap().width as f64;
      state.webview_wa.lock().unwrap().as_ref().unwrap().clone()
        .set_position(LogicalPosition::new((new_idx != 1) as u8 as f64 * width, 0.)).unwrap();
      state.webview_tg.lock().unwrap().as_ref().unwrap().clone()
        .set_position(LogicalPosition::new((new_idx == 1) as u8 as f64 * width, 0.)).unwrap();
  }

  *idx = new_idx;
}

// Main ---------------------------------------------------------------------------------
fn main() {
  setup_error_hook();
  println!("Running from: {:?}", std::env::current_exe());  // debug

  let icons = platform::load_tray_icons();

  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .plugin(tauri_plugin_opener::init())
    .manage(AppState {
      state_index: Mutex::new(0),
      webview_wa: Mutex::new(None),
      webview_tg: Mutex::new(None),
      title_wa: Mutex::new(String::new()),
      title_tg: Mutex::new(String::new()),
      tray: Mutex::new(None),
      badge_wa: Mutex::new(0),
      badge_tg: Mutex::new(0),
      tray_icons: icons,
    })
    .invoke_handler(tauri::generate_handler![report_title, notify])
    .setup(move |app| {
      let menu = platform::build_tray_menu(&app.handle());
      let tray = platform::create_tray_icon(&app.handle(), &menu)?;
      app.state::<AppState>().tray.lock().unwrap().replace(tray);

      let window = tauri::window::WindowBuilder::new(app, "main").build()?;
      window.restore_state(StateFlags::all()).unwrap();
      window.set_title("WATG: --").unwrap();
      window.set_background_color(Some(Color(50,50,50,1))).unwrap();
      {
        let app_handle = app.handle().clone();
        window.on_window_event(move |event| {
          if matches!(event, tauri::WindowEvent::Moved(_) | tauri::WindowEvent::Resized(_)) {
            let _ = app_handle.save_window_state(StateFlags::all());
          }
        });
      }

      let size = window.inner_size().unwrap();
      let (width, height) = (size.width as f64, size.height as f64);

      let wv1 = window.add_child(
        WebviewBuilder::new("WA", WebviewUrl::External("https://web.whatsapp.com".parse().unwrap()))
          .zoom_hotkeys_enabled(true)
          .auto_resize()
          .initialization_script(&format!(r#"{internal}{external}"#,
            internal = include_str!("scripts/wa.js"),
            external = std::env::current_exe().ok()
              .and_then(|p| std::fs::read_to_string(p.with_file_name("wa.js")).ok()).unwrap_or_default()
          )),
        LogicalPosition::new(0., 0.),
        LogicalSize::new(width, height),
      )?;
      wv1.set_size(PhysicalSize::new(width, height)).unwrap();
      wv1.set_zoom(0.75)?;

      wv1.listen("tauri://message", move |event| {
        let payload = event.payload();
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(payload) {
          if value.get("_watg") == Some(&serde_json::Value::Bool(true)) {
            if let (Some(title), Some(body)) = (
              value.get("title").and_then(|v| v.as_str()),
              value.get("body").and_then(|v| v.as_str()),
            ) {
              print!("✉️  {} on WA: ", title);
              platform::send_notification(title, body);
            }
          }
        }
      });

      let wv2 = window.add_child(
        WebviewBuilder::new("TG", WebviewUrl::External("https://web.telegram.org/k/".parse().unwrap()))
          .zoom_hotkeys_enabled(true)
          .auto_resize()
          .initialization_script(&format!(r#"{internal}{external}"#,
            internal = include_str!("scripts/tg.js"),
            external = std::env::current_exe().ok()
              .and_then(|p| std::fs::read_to_string(p.with_file_name("tg.js")).ok()).unwrap_or_default()
          )),
        LogicalPosition::new(width, 0.),
        LogicalSize::new(width, height),
      )?;
      wv2.set_size(PhysicalSize::new(width, height)).unwrap();
      wv2.set_zoom(0.75)?;

      let wv1_handle = wv1.clone();
      let wv2_handle = wv2.clone();

      let state = app.state::<AppState>();
      *state.webview_wa.lock().unwrap() = Some(wv1);
      *state.webview_tg.lock().unwrap() = Some(wv2);
      *state.state_index.lock().unwrap() = 2;

      switch_view(&app.handle(), Some(1));

      // Set MicaDark theme - Calling it immediately after the creation of window kept failing, here it works.
      #[cfg(target_os="windows")] window.set_effects(EffectsBuilder::new().effect(Effect::MicaDark)
        .build()).expect("Can't set Mica dark theme");

      if let Some(w) = app.get_window("main") {
        w.listen("open-devtools", move |event| {
          let payload: String = event.payload().to_string();
          if payload=="\"WA\"" {let _ = wv1_handle.open_devtools();}
          else if payload=="\"TG\"" {let _ = wv2_handle.open_devtools();}
          else {println!("⚙️  [main.rs] received open-devtools request for {:?}", event.payload())};
        });
      }

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
