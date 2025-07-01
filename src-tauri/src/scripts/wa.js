function debugAndSend() {
  const spans = Array.from(document.querySelectorAll('span'));
  const unreadBadges = spans.filter(el => {
    const style = window.getComputedStyle(el);
    const bg = style.backgroundColor;
    const name = (el.getAttribute('aria-label') || el.getAttribute('name') || '').toLowerCase().trim();
    return (bg === 'rgb(0, 168, 132)' || bg === 'rgb(37, 211, 102)') &&
           (name === 'da leggere' || name.includes('non lett') ||
            name === 'unread' || name.includes('unread'));
  });
  const count = unreadBadges.length > 0 ? unreadBadges.length : '_';
  window.__TAURI__.core.invoke('report_title', { title: count.toString(), label: window.label });
}

function injectCSS() {
  const style = document.createElement('style');
  style.innerHTML = `@media (max-width: 747px) {
    #app>div>div>div:has(header) {
        display: flex !important;
        overflow: hidden !important;
        min-width: fit-content !important;
        max-width: 100vw;
    }
    #app>div>div>div>div:has(div>div>div>span[data-icon*="logo"]) {max-width: 0%;}
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
    #app>div>div>div>div:has(header>div>div>h1){
        flex: 1 1 100% !important;
        max-width: none !important;
        min-width: 0 !important;
        overflow: hidden !important;
    }
    #app>div>div>div>div>div>header, #main>div>div>div>div, #main>footer {max-width: 100vw;}
    #app>div>div>div>div:has(div>header){
        flex: 0 0 100% !important;
        max-width: 100% !important;
        min-width: 0% !important;
        overflow: hidden !important;
    }
    #expressions-panel-container>span>div {
        left: 0px !important;
        max-width: 100% !important;
    }
    .esc-button {
        margin-left: -8px !important;
        padding-right: 16px !important;
    }
  }`;
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
    path.setAttribute('d', 'M 48 23 H 15 C 14 23 13 22 14 21 L 23 12 C 25 10 25 10 23 8 L 23 8 C 21 6 21 6 19 8 L 4 24 C 2 26 2 26 4 28 L 19 44 C 21 46 21 46 23 44 L 23 44 C 25 42 25 42 23 40 L 14 31 C 13 30 14 29 15 29 H 48 C 50 29 50 29 50 27 V 25 C 50 23 50 23 48 23 Z');
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
