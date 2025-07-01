// Telegram unread count
function sendTitle() {
    const unreadChats = document.querySelectorAll('.dialog-subtitle-badge-unread').length;
    const count = unreadChats > 0 ? unreadChats.toString() : '_';
    window.__TAURI__.core.invoke('report_title',{title: count, label: window.label});
}
setTimeout(sendTitle, 100); setInterval(sendTitle, 500);