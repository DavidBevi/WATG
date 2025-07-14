//,-------------------------------------------------------------------------------------,
//| wa.js - injected in WhatsApp Web webview to enable some features                    |
//'-------------------------------------------------------------------------------------'

function debugAndSend() {
  const parentElement = document.querySelector('#pane-side');
  if (!parentElement) { console.error('Parent element #pane-side not found.'); return; }
  const spans = Array.from(parentElement.querySelectorAll('span'));
  const unreadBadges = spans.filter(el => {
    const style = window.getComputedStyle(el);
    const bg = style.backgroundColor;
    const name = (el.getAttribute('aria-label') || el.getAttribute('name') || '').toLowerCase().trim();
    return (bg === 'rgb(0, 168, 132)' || bg === 'rgb(37, 211, 102)') &&
           (name === 'da leggere' || name.includes('non lett') ||
            name === 'unread' || name.includes('unread'));
  });
  const count = unreadBadges.length <= 0 ? '_' : unreadBadges.length;
  window.__TAURI__.core.invoke('report_title', { title: count.toString(), label: window.label });
}

function injectCSS() {
  const style = document.createElement('style');
  style.innerHTML = `@media (max-width: 747px) {
/*𝐂𝐎𝐍𝐓𝐀𝐈𝐍𝐄𝐑*/
    #app>div>div>div:has(header) {
        display: flex! important;
        overflow: hidden! important;
        min-width: fit-content !important;
        max-width: 100vw;
    }
/*𝐒𝐏𝐋𝐀𝐒𝐇𝐒𝐂𝐑𝐄𝐄𝐍*/
    #app>div>div>div>div:has(div>div>div>span[data-icon*="logo"]) {max-width: 0%;}
/*𝐒𝐈𝐃𝐄𝐁𝐀𝐑*/
    #app>div>div>div>header, #app>div>div>div>header * {
        flex: 0 0 0 !important;
        width: 0 !important;
        max-width: 0 !important;
        min-width: 0 !important;
        padding: 0 !important;
        margin: 0 !important;
        border: 0 !important;
        overflow: hidden !important;
    }
/*𝐋𝐈𝐒𝐓-𝐎𝐅-𝐂𝐇𝐀𝐓𝐒*/
    #app>div>div>div>div:has(header>div>div>h1){
        flex: 1 1 100% !important;
        max-width: none !important;
        min-width: 0 !important;
        overflow: hidden !important;
    }
/*𝐂𝐇𝐀𝐓-𝐇𝐄𝐀𝐃𝐄𝐑 + 𝐂𝐇𝐀𝐓-𝐂𝐎𝐍𝐓𝐄𝐍𝐓 + 𝐂𝐇𝐀𝐓-𝐅𝐎𝐎𝐓𝐄𝐑*/
    #app>div>div>div>div>div>header, #main>div>div>div>div, #main>footer {max-width: 100vw;}
/*𝐂𝐇𝐀𝐓*/
    #app>div>div>div>div:has(div>header){
        flex: 0 0 100% !important;
        max-width: 100% !important;
        min-width: 0% !important;
        overflow: hidden! important;
    }
/*𝐂𝐇𝐀𝐓-𝐈𝐍𝐅𝐎 + 𝐂𝐇𝐀𝐓-𝐌𝐄𝐃𝐈𝐀-𝐒𝐄𝐍𝐃 + 𝐂𝐇𝐀𝐓-𝐌𝐄𝐃𝐈𝐀-𝐑𝐎𝐋𝐋*/
    #app>div>div>div>div:has(span>div>span>div>div>header),
    #app>div>div>div>div>div:has(span>div>div>div>div>div>input),
    #app>div>div>div>div:has(span>div>span>div>header) {
        position: absolute;
        width: 99vw;
    }
/*𝐄𝐌𝐎𝐉𝐈-𝐏𝐀𝐍𝐄𝐋*/
    #expressions-panel-container>span>div {left: 0px !important; max-width: 100% !important;}
/*𝐄𝐒𝐂-𝐁𝐔𝐓𝐓𝐎𝐍 (via js injection)*/
    .esc-button {
        margin-left: -8px !important;
        padding-right: 16px !important;
    }
}
/*𝐂𝐇𝐀𝐓-𝐓𝐀𝐁𝐋𝐈𝐒𝐓*/ #side>div[role="tablist"] {visibility:hidden;height:0px;padding:0px;}`;
  document.head.appendChild(style);
}

function injectEscButton() {
  if (window.innerWidth >= 748) return;

  const target = document.querySelector('#app>div>div>div>div>div>header');
  if (target && !target.querySelector('.esc-button')) {
    const btn = document.createElement('button');

    const nativeBtn = document.querySelector('button[aria-label="Menu"], button[aria-label="Cerca…"]');
    btn.className = (nativeBtn ? nativeBtn.className : '') + ' esc-button';

    const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    svg.setAttribute('viewBox', '0 0 56 56');
    svg.setAttribute('height', '24');
    svg.setAttribute('width', '24');
    svg.setAttribute('fill', 'currentColor');
    svg.setAttribute('preserveAspectRatio', 'xMidYMid meet');

    const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
    path.setAttribute('d', 'M48 23H15Q12 23 14 21L23 12Q25 10 23 8L23 8Q21 6 19 8L4 24Q2 26 4 28L19 44Q21 46 23 44L23 44Q25 42 23 40L14 31Q12 29 15 29H48Q50 29 50 27V25Q50 23 48 23Z');
    svg.appendChild(path);
    btn.appendChild(svg);

    btn.onclick = () => {
      document.dispatchEvent(new KeyboardEvent('keydown', {
        key: 'Escape', keyCode: 27, which: 27, bubbles: true
      }));
    };

    target.insertBefore(btn, target.firstChild);
  }
}

function ready(fn) {
  if (document.readyState !== 'loading') { fn(); }
  else { document.addEventListener('DOMContentLoaded', fn); }
}

ready(() => {
  injectCSS();
  setTimeout(() => {
    debugAndSend();
    setInterval(debugAndSend, 500);
    setInterval(injectEscButton, 1000);
  }, 3000);
});


// Open URLs in default browser
document.addEventListener('click', (event) => {
    // Check if the clicked element is an anchor tag or is inside one
    const anchor = event.target.closest('a');
    if (anchor && anchor.href) {
        // Prevent default, invoke opener installed in main.rs
        event.preventDefault(); window.__TAURI__.opener.openUrl(anchor.href);
    }
});

// Enable notification permission
if (Notification.permission!=="granted") { Object.defineProperty(window.Notification,'permission',{get:()=>"granted"}); }

// Only injected on Windows to reroute new Notification(...) calls
;(function () {
  const _Native = window.Notification;
  window.Notification = function (title, opts) {window.__TAURI__.invoke('notify', {title, body: opts?.body ?? ''});
    // still fire the in‐page Notification to keep any page‐side logic happy
    return new _Native(title, opts);};
  window.Notification.requestPermission = _Native.requestPermission.bind(_Native);
  Object.defineProperty(window.Notification, 'permission', {get() { return 'granted'; }});})();

  