//,-------------------------------------------------------------------------------------,
//| for_windows.rs                                                                      |
//| - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - |
//| This file contains code specific for the Windows' version of WATG,                  |
//| such as tray icon and toast notifications                                           |
//'-------------------------------------------------------------------------------------'

// Imports
use tauri::{Wry, AppHandle, tray::{TrayIcon, TrayIconBuilder, TrayIconEvent, MouseButton,
   MouseButtonState}, menu::Menu, image::Image};
use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK};
use std::{ffi::OsStr, iter::once, os::windows::ffi::OsStrExt};

// Show native error dialog using WinAPI ------------------------------------------------
pub fn show_error_dialog(message: &str) {
  let wide: Vec<u16> = OsStr::new(message).encode_wide().chain(once(0)).collect();
  unsafe {
    MessageBoxW(std::ptr::null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK | MB_ICONERROR);
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
      if event.id.as_ref() == "switch" {
        crate::switch_view(app);
      }
    })
    .show_menu_on_left_click(false)
    .build(app)
}

// Update tray icon image ---------------------------------------------------------------
pub fn set_tray_icon(tray: &TrayIcon, image: Image<'static>) {
  let _ = tray.set_icon(Some(image));
}

// Send native Windows notification (can be extended to use winrt-toast) ----------------
pub fn send_notification(title: &str, body: &str) {
  let _ = toast_notification::Toast::new(title, body).show();
}

// Minimal wrapper for Windows toast notifications --------------------------------------
mod toast_notification {
  pub struct Toast<'a> {
    pub title: &'a str,
    pub body: &'a str,
  }

  impl<'a> Toast<'a> {
    pub fn new(title: &'a str, body: &'a str) -> Self {
      Self { title, body }
    }

    pub fn show(&self) -> Result<(), ()> {
      println!("🔥 for_windows.rs → toast_notification(): title='{}' body='{}'", self.title, self.body);
      Ok(())
    }
  }
}
