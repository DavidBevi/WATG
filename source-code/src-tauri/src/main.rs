use std::sync::Mutex;
use tauri::{
  LogicalPosition, LogicalSize, PhysicalSize, WebviewUrl,
  AppHandle, Manager,
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  menu::{Menu, MenuItem},
  webview::Webview,
};

// Application state holds both WebView handles and their last-reported titles
struct AppState {
  state_index: Mutex<u8>,     // 0: WA, 1: TG, 2: hidden
  webview1: Mutex<Option<Webview>>,
  webview2: Mutex<Option<Webview>>,
  title_wa: Mutex<String>,
  title_tg: Mutex<String>,
}

// This command is invoked from the injected JS to update titles
#[tauri::command]
fn report_title(app: AppHandle, title: String, label: String) {
  let state = app.state::<AppState>();
  {
    let mut wa = state.title_wa.lock().unwrap();
    let mut tg = state.title_tg.lock().unwrap();
    if label == "WA" {
      *wa = title;
    } else {
      *tg = title;
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
}

fn main() {
  // WhatsApp: regex out "(N)" from title or "_" if none
    // let js_wa = r#"
    //   function sendTitle() {
    //     const m = document.title.match(/\((\d+)\)/);
    //     const count = m ? m[1] : '_';
    //     window.__TAURI__.core.invoke('report_title', {
    //       title: count, label: window.label
    //     });
    //   }
    //   setTimeout(sendTitle, 100); setInterval(sendTitle, 500);
    // "#;

    let js_wa = r#"
      function debugAndSend() {
        // get all spans
        const spans = Array.from(document.querySelectorAll('span'));
        // filter by computed background color and accessibility name
        const unreadBadges = spans.filter(el => {
          const style = window.getComputedStyle(el);
          const bg = style.backgroundColor;
          const name = el.getAttribute('aria-label') || el.getAttribute('name') || '';
          return (bg === 'rgb(0, 168, 132)' || bg === 'rgb(37, 211, 102)')
              && (name.trim() === 'Da leggere' || name.includes('non lett') 
                  || name === 'unread'         || name.includes('unread') )
        });
        console.log('DEBUG unread badges:', unreadBadges.length, unreadBadges);

        const count = unreadBadges.length > 0 ? unreadBadges.length : '_';
        window.__TAURI__.core.invoke('report_title', {
          title: count.toString(),
          label: window.label
        });
      }

      setTimeout(debugAndSend, 100);
      setInterval(debugAndSend, 1000);
    "#;

    // Telegram: count conversations with unread badges or "_" if zero
    let js_tg = r#"
      function sendTitle() {
        const unreadChats = document.querySelectorAll('.dialog-subtitle-badge-unread').length;
        const count = unreadChats > 0 ? unreadChats.toString() : '_';
        window.__TAURI__.core.invoke('report_title', {
          title: count, label: window.label
        });
      }
      setTimeout(sendTitle, 100); setInterval(sendTitle, 500);
    "#;

  tauri::Builder::default()
    .manage(AppState {
      state_index: Mutex::new(0),
      webview1: Mutex::new(None),
      webview2: Mutex::new(None),
      title_wa: Mutex::new(String::new()),
      title_tg: Mutex::new(String::new()),
    })
    .invoke_handler(tauri::generate_handler![report_title])
    .setup(move |app| {
      let width = 725.;
      let height = 860.;

      // build tray icon and menu
      let switch_i = MenuItem::with_id(app, "switch", "Switch", true, None::<&str>)?;
      let menu = Menu::with_items(app, &[&switch_i])?;
      let _tray = TrayIconBuilder::new()
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

      // main window
      let window = tauri::window::WindowBuilder::new(app, "main")
        .inner_size(width, height)
        .position(930., 10.)
        .build()?;
      window.set_title("WATG: --").unwrap();

      // WhatsApp webview
      let wv1 = window.add_child(
        tauri::webview::WebviewBuilder::new(
          "WA",
          WebviewUrl::External("https://web.whatsapp.com".parse().unwrap())
        )
        .initialization_script(&format!("window.label='WA';{}", js_wa))
        .auto_resize(),
        LogicalPosition::new(0., 0.),
        LogicalSize::new(width, height),
      )?;
      wv1.set_zoom(0.75)?;

      // Telegram (Web K) webview with unread-chat counting
      let wv2 = window.add_child(
        tauri::webview::WebviewBuilder::new(
          "TG",
          WebviewUrl::External("https://web.telegram.org/k/".parse().unwrap())
        )
        .initialization_script(&format!("window.label='TG';{}", js_tg))
        .auto_resize(),
        LogicalPosition::new(width, 0.),
        LogicalSize::new(width, height),
      )?;
      wv2.set_zoom(0.75)?;

      // store webview handles
      let state = app.state::<AppState>();
      *state.webview1.lock().unwrap() = Some(wv1);
      *state.webview2.lock().unwrap() = Some(wv2);

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// switch_view toggles WA → hidden → TG, keeping your auto-resize logic intact
fn switch_view(app: &AppHandle) {
  let state = app.state::<AppState>();
  let mut idx = state.state_index.lock().unwrap();
  let wv1_guard = state.webview1.lock().unwrap();
  let wv1 = wv1_guard.as_ref().unwrap();
  let wv2_guard = state.webview2.lock().unwrap();
  let wv2 = wv2_guard.as_ref().unwrap();
  let w = app.get_window("main").unwrap();
  let size = w.inner_size().unwrap();
  let (width, height) = (size.width as f64, size.height as f64);

  match *idx {
    0 => {
      // show TG
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
    1 => {
      // hidden (do not reset title)
      w.hide().unwrap();
      w.set_skip_taskbar(true).unwrap();
    }
    _ => {
      // show WA
      w.show().unwrap();
      w.set_skip_taskbar(false).unwrap();
      w.set_focus().unwrap();
      w.set_always_on_top(true).unwrap();
      w.set_always_on_top(false).unwrap();
      wv1.set_position(LogicalPosition::new(0., 0.)).unwrap();
      wv1.set_size(PhysicalSize::new(width, height)).unwrap();
      wv2.set_position(LogicalPosition::new(width, 0.)).unwrap();
      wv2.set_size(PhysicalSize::new(0, height as u32)).unwrap();
    }
  }

  *idx = (*idx + 1) % 3;
}
