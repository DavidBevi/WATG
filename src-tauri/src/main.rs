use std::sync::Mutex;
use tauri::{
  LogicalPosition, LogicalSize, PhysicalSize, WebviewUrl,
  AppHandle, Manager,
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  menu::{Menu, MenuItem},
  webview::Webview,
};

struct AppState {
  state_index: Mutex<u8>, // 0: WA, 1: TG, 2: hidden
  webview1: Mutex<Option<Webview>>,
  webview2: Mutex<Option<Webview>>,
}

fn main() {
  tauri::Builder::default()
    .manage(AppState {
      state_index: Mutex::new(0),
      webview1: Mutex::new(None),
      webview2: Mutex::new(None),
    })
    .setup(|app| {
      let width = 725.;
      let height = 860.;

      let switch_i = MenuItem::with_id(app, "switch", "Switch", true, None::<&str>)?;
      let menu = Menu::with_items(app, &[&switch_i])?;

      let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|app, event| {
          match event {
            TrayIconEvent::Click {
              button: MouseButton::Left,
              button_state: MouseButtonState::Up,
              ..
            } => switch_view(&app.app_handle()),
            _ => println!("event not handled"),
          }
        })
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
          "switch" => switch_view(app),
          _ => println!("menu item {:?} not handled", event.id),
        })
        .build(app)?;

      let window = tauri::window::WindowBuilder::new(app, "main")
        .inner_size(width, height)
        .position(930., 10.)
        .build()?;
      window.set_title("WATG").unwrap();

      let wv1 = window.add_child(
        tauri::webview::WebviewBuilder::new("WA", WebviewUrl::External("https://web.whatsapp.com".parse().unwrap()))
          .auto_resize(),
        LogicalPosition::new(0., 0.),
        LogicalSize::new(width, height),
      )?;
      wv1.set_zoom(0.75)?;

      let wv2 = window.add_child(
        tauri::webview::WebviewBuilder::new("TG", WebviewUrl::External("https://web.telegram.org".parse().unwrap()))
          .auto_resize(),
        LogicalPosition::new(width, 0.),
        LogicalSize::new(0., height),
      )?;
      wv2.set_zoom(0.75)?;

      let state = app.state::<AppState>();
      *state.webview1.lock().unwrap() = Some(wv1);
      *state.webview2.lock().unwrap() = Some(wv2);

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn switch_view(app: &AppHandle) {
  let state = app.state::<AppState>();
  let mut index = state.state_index.lock().unwrap();

  let wv1_opt = state.webview1.lock().unwrap();
  let wv2_opt = state.webview2.lock().unwrap();
  if wv1_opt.is_none() || wv2_opt.is_none() {
    println!("Webviews not initialized");
    return;
  }
  let wv1 = wv1_opt.as_ref().unwrap();
  let wv2 = wv2_opt.as_ref().unwrap();

  let main_window = app.get_window("main").expect("main window not found");
  let size = main_window.inner_size().unwrap();
  let width = size.width as f64;
  let height = size.height as f64;

  match *index {
    0 => {
      // State 0 → State 1 (Telegram)
      // main_window.set_title("Telegram").unwrap();

      main_window.show().unwrap();
      main_window.set_skip_taskbar(false).unwrap();
      main_window.set_focus().unwrap();
      main_window.set_always_on_top(true).unwrap();
      main_window.set_always_on_top(false).unwrap();

      wv1.set_position(LogicalPosition::new(width, 0.)).unwrap();
      wv1.set_size(PhysicalSize::new(0, height as u32)).unwrap();

      wv2.set_position(LogicalPosition::new(0., 0.)).unwrap();
      wv2.set_size(PhysicalSize::new(width, height)).unwrap();
    }
    1 => {
      // State 1 → State 2 (hidden)
      main_window.set_title("WATG").unwrap();

      main_window.hide().unwrap();
      main_window.set_skip_taskbar(true).unwrap();
    }
    _ => {
      // State 2 → State 0 (WhatsApp)
      // main_window.set_title("Whatsapp").unwrap();

      main_window.show().unwrap();
      main_window.set_skip_taskbar(false).unwrap();
      main_window.set_focus().unwrap();
      main_window.set_always_on_top(true).unwrap();
      main_window.set_always_on_top(false).unwrap();

      wv1.set_position(LogicalPosition::new(0., 0.)).unwrap();
      wv1.set_size(PhysicalSize::new(width, height)).unwrap();

      wv2.set_position(LogicalPosition::new(width, 0.)).unwrap();
      wv2.set_size(PhysicalSize::new(0, height as u32)).unwrap();
    }
  }

  *index = (*index + 1) % 3;
}
