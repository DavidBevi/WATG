#![windows_subsystem = "windows"]  // Avoid showing console (Windows only)

use std::{sync::Mutex, fs::File, io::Write};
use tauri::{
  LogicalPosition, LogicalSize, PhysicalSize, WebviewUrl,
  AppHandle, Manager,
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent, TrayIcon},
  menu::{Menu, MenuItem},
  webview::Webview,
};
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};
use tauri::image::Image;

#[cfg(target_os = "windows")]
fn show_error_dialog(message: &str) {
  use std::ffi::OsStr;
  use std::iter::once;
  use std::os::windows::ffi::OsStrExt;
  use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK};

  let wide: Vec<u16> = OsStr::new(message).encode_wide().chain(once(0)).collect();
  unsafe {
    MessageBoxW(std::ptr::null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK | MB_ICONERROR);
  }
}

fn setup_error_hook() {
  std::panic::set_hook(Box::new(|info| {
    let msg = if let Some(s) = info.payload().downcast_ref::<&str>() {
      *s
    } else if let Some(s) = info.payload().downcast_ref::<String>() {
      s.as_str()
    } else {
      "Unknown panic"
    };

    let log = format!("Panic: {}\n", msg);
    let _ = File::create("watg-error.log").and_then(|mut f| f.write_all(log.as_bytes()));

    #[cfg(target_os = "windows")]
    show_error_dialog(msg);
  }));
}

// Application state holds both WebView handles and their last-reported titles
struct AppState {
  state_index: Mutex<u8>,     // 0=WA, 1=TG, 2=Hidden
  webview_wa: Mutex<Option<Webview>>,
  webview_tg: Mutex<Option<Webview>>,
  title_wa: Mutex<String>,
  title_tg: Mutex<String>,
  tray: Mutex<Option<TrayIcon>>,
  badge_wa: Mutex<u8>,
  badge_tg: Mutex<u8>,
  tray_icons: Vec<Image<'static>>,
}

#[tauri::command]
fn report_title(app: AppHandle, title: String, label: String) {
  let state = app.state::<AppState>();

  {
    let mut wa = state.title_wa.lock().unwrap();
    let mut tg = state.title_tg.lock().unwrap();
    if label == "WA" {
      *wa = title.clone();
    } else {
      *tg = title.clone();
    }
  }

  if let Some(w) = app.get_window("main") {
    let wa = state.title_wa.lock().unwrap();
    let tg = state.title_tg.lock().unwrap();
    let full = format!(
      "WATG {} {}",
      if wa.is_empty() { "" } else { &*wa },
      if tg.is_empty() { "" } else { &*tg }
    );
    let _ = w.set_title(&full);
  }

  let count = if title == "_" { 0 } else { title.parse::<u8>().unwrap_or(0) };

  {
    let mut badge_wa = state.badge_wa.lock().unwrap();
    let mut badge_tg = state.badge_tg.lock().unwrap();
    if label == "WA" {
      *badge_wa = count;
    } else {
      *badge_tg = count;
    }

    let total = badge_wa.saturating_add(*badge_tg);
    update_tray_icon(&app, total);
  }
}

#[tauri::command]
fn report_badges(app: AppHandle, count: String, label: String) {
  let n = count.parse::<u8>().unwrap_or(0);
  let state = app.state::<AppState>();
  match label.as_str() {
    "WA" => *state.badge_wa.lock().unwrap() = n,
    "TG" => *state.badge_tg.lock().unwrap() = n,
    _ => (),
  }

  let wa = *state.badge_wa.lock().unwrap();
  let tg = *state.badge_tg.lock().unwrap();
  update_tray_icon(&app, wa.saturating_add(tg));
}

fn main() {
  setup_error_hook(); // Must be the first line inside main()

  let js_wa = include_str!("scripts/wa.js").to_string();
  let js_tg = include_str!("scripts/tg.js").to_string();

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

  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
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
    .invoke_handler(tauri::generate_handler![report_title, report_badges])
    .setup(move |app| {
      let switch_i = MenuItem::with_id(app, "switch", "Switch", true, None::<&str>)?;
      let menu = Menu::with_items(app, &[&switch_i])?;
      let tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|app, event| {
          if let TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
          } = event {
            switch_view(&app.app_handle());
          }
        })
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
          if event.id.as_ref() == "switch" {
            switch_view(app);
          }
        })
        .build(app)?;

      app.state::<AppState>().tray.lock().unwrap().replace(tray);

      let window = tauri::window::WindowBuilder::new(app, "main").build()?;
      window.restore_state(StateFlags::all()).unwrap();
      window.set_title("WATG: --").unwrap();

      {
        use tauri::WindowEvent;
        let app_handle = app.handle().clone();
        window.on_window_event(move |event| {
          if matches!(event, WindowEvent::Moved(_) | WindowEvent::Resized(_)) {
            let _ = app_handle.save_window_state(StateFlags::all());
          }
        });
      }

      let size = window.inner_size().unwrap();
      let width = size.width as f64;
      let height = size.height as f64;

      let wv1 = window.add_child(
        tauri::webview::WebviewBuilder::new("WA", WebviewUrl::External("https://web.whatsapp.com".parse().unwrap()))
          .initialization_script(&format!("window.label='WA';{}", js_wa))
          .auto_resize(),
        LogicalPosition::new(0., 0.),
        LogicalSize::new(width, height),
      )?;
      wv1.set_zoom(0.75)?;

      let wv2 = window.add_child(
        tauri::webview::WebviewBuilder::new("TG", WebviewUrl::External("https://web.telegram.org/k/".parse().unwrap()))
          .initialization_script(&format!("window.label='TG';{}", js_tg))
          .auto_resize(),
        LogicalPosition::new(width, 0.),
        LogicalSize::new(width, height),
      )?;
      wv2.set_zoom(0.75)?;

      let state = app.state::<AppState>();
      *state.webview_wa.lock().unwrap() = Some(wv1);
      *state.webview_tg.lock().unwrap() = Some(wv2);

      *app.state::<AppState>().state_index.lock().unwrap() = 2;
      switch_view(&app.handle());

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn update_tray_icon(app: &AppHandle, count: u8) {
  let icon_index = count.min(10);
  let state = app.state::<AppState>();
  let tray_lock = state.tray.lock().unwrap();
  if let Some(tray) = tray_lock.as_ref() {
    let image = &state.tray_icons[icon_index as usize];
    let _ = tray.set_icon(Some(image.clone()));
  }
}

fn switch_view(app: &AppHandle) {
  let state = app.state::<AppState>();
  let mut idx = state.state_index.lock().unwrap();
  let wv1_guard = state.webview_wa.lock().unwrap();
  let wv1 = wv1_guard.as_ref().unwrap();
  let wv2_guard = state.webview_tg.lock().unwrap();
  let wv2 = wv2_guard.as_ref().unwrap();
  let w = app.get_window("main").unwrap();
  let size = w.inner_size().unwrap();
  let (width, height) = (size.width as f64, size.height as f64);

  match *idx {
    0 => { // 0→1 TG
      w.show().unwrap();
      w.set_skip_taskbar(false).unwrap();
      w.set_focus().unwrap();
      w.set_always_on_top(true).unwrap();
      w.set_always_on_top(false).unwrap();
      wv1.set_position(LogicalPosition::new(width, 0.)).unwrap();
      wv1.set_size(PhysicalSize::new(0, height as u32)).unwrap();
      wv2.set_position(LogicalPosition::new(0., 0.)).unwrap();
      wv2.set_size(PhysicalSize::new(width, height)).unwrap();
    }
    1 => { // 1→2 Hide
      w.hide().unwrap();
      w.set_skip_taskbar(true).unwrap();
    }
    _ => { // 2→0 WA
      w.show().unwrap();
      w.set_skip_taskbar(false).unwrap();
      w.set_focus().unwrap();
      w.set_always_on_top(true).unwrap();
      w.set_always_on_top(false).unwrap();
      wv1.set_position(LogicalPosition::new(0., 0.)).unwrap();
      wv1.set_size(PhysicalSize::new(width, height)).unwrap();
      wv2.set_position(LogicalPosition::new(width, 0.)).unwrap();
      wv2.set_size(PhysicalSize::new(width, height)).unwrap();
    }
  }

  *idx = (*idx + 1) % 3;
}
