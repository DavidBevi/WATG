//,-------------------------------------------------------------------------------------,
//| tg.js - injected in Telegram Web webview to enable some features                    |
//'-------------------------------------------------------------------------------------'

// Telegram unread count
function sendTitle() {
  const unreadChats = document.querySelectorAll('.dialog-subtitle-badge-unread').length;
  const count = unreadChats > 0 ? unreadChats.toString() : '_';
  window.__TAURI__.core.invoke('report_title', { title: count, label: window.label });
}
setTimeout(sendTitle, 100);
setInterval(sendTitle, 500);

// Open URLs in default browser
document.addEventListener('click', (event) => {
  const anchor = event.target.closest('a');
  if (anchor && anchor.href) {
    event.preventDefault();
    window.__TAURI__.opener.openUrl(anchor.href);
  }
});

// Enable notification permission
if (Notification.permission !== "granted") {
  Object.defineProperty(window.Notification, 'permission', { get: () => "granted" });
}

// Patch global Notification to intercept direct uses (only injected on Windows)
console.log('🔔 notification-shim injected');
const _Native = window.Notification;
window.Notification = function (title, opts) {
  console.log('→ window.Notification invoked:', title, opts?.body);
  window.__TAURI__.core.invoke('notify', { title, body: opts?.body ?? '' });
  return new _Native(title, opts);
};
window.Notification.requestPermission = _Native.requestPermission.bind(_Native);
Object.defineProperty(window.Notification, 'permission', { get() { return 'granted'; } });
