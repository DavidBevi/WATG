//,-------------------------------------------------------------------------------------,
//| wa.js - use this file to define:                                                    |
//| - UNREAD-COUNTER methods & parameters                                               |
//| - NOTIFICATION methods & parameters                                                 |
//| - STYLE mods to enable narrow-layout                                                |
//| - DEBUG-TOOLS                                                                       |
//| - CORE integration with Tauri backend (must be last)                                |
//'-------------------------------------------------------------------------------------'


// ==UNREAD-COUNTER== ===================================================================
// 🔔 Counts unread-badges, passes number to Tauri, returns list of badges (html elements)
// 🕒 Called every 0.5s (by extractNotif())
// ⚠️ Fragile: relies on exact color of unread-badges
function countUnreadMsg() {
  const pane = document.querySelector('#pane-side');
  if (!pane) {
    countUnreadMsg.i = (countUnreadMsg.i || 0) + 1;
    sendUnreadCountToWatg(['⏳', '⌛'][countUnreadMsg.i % 2]);
    return [];
  }
  let count = 0;  const result = [];
  for (const el of pane.querySelectorAll('span')) {
    const bg = window.getComputedStyle(el).backgroundColor;
    if (bg==='rgb(0, 168, 132)' || bg==='rgb(33, 192, 99)') {count++;result.push(el);}
  }
  sendUnreadCountToWatg(count > 0 ? String(count) : '_');
  return result; 
}

// ==NOTIFICATIONS== ====================================================================
// 🔔 Fetches msg-info from unread-badge-spans' relatives, builds+sends notifications
// 🕒 Called every 0.5s
// ⚠️ Fragile: relies on exact hierarchy of badges and other html-elements
function extractNotif(mode) {
  const badges = countUnreadMsg();
  const results = [];
  extractNotif.cache  = extractNotif.cache  ?? new Map(); // title → {msg, count}
  extractNotif.symbol = extractNotif.symbol ?? new Map([["status-check", "✔"],["status-dblcheck", "✔✔"],
    ["status-ptt", "🎙️"],["status-image", "📷"],["status-sticker", "📃"],["status-vcard", "👤"]]);

  for (let i = 0; i < badges.length; i++) {
    const badge = badges[i];

    // "aria-label" contains alt text of badges
    let badgeCount = parseInt(badge.getAttribute("aria-label")?.replace(/\D+/g, '')) || 0;
    if (badgeCount===0 && mode!=="log") continue;

    // rootT = root of TITLE = 5th-ancestor
    // rootM = root of MEX   = 4th-ancestor
    let rootT = badge.parentElement?.parentElement?.parentElement?.parentElement?.parentElement?.children[0];
    let rootM = badge.parentElement?.parentElement?.parentElement?.parentElement;

    // from rootT, select appropriate child (different between group-chat and normal-chat)
    let isGroup   = rootT?.children[0]?.children[0];
    let chatTitle = isGroup?.getAttribute("title") || 
                    rootT?.children[0]?.children[0]?.children[0]?.children[0]?.getAttribute("title");

    // from rootM, extract msg and eventual symbol
    var chatMsg   = rootM?.children[0]?.children[0]?.getAttribute("title");
    var chatInfo  = rootM?.children[0]?.children[0]?.children[0]?.children[0]?.children[0]?.textContent;

    // if there's a symbol before the msg: convert to emoji
    chatInfo = extractNotif.symbol.get(chatInfo) ?? (chatInfo ? `${chatInfo}:` : "");
    chatMsg = `${chatInfo??""} ${chatMsg}`.trim();
    let notif = [chatTitle, chatMsg];
    results.push(notif);

    // log
    if (mode==="log") {console.log(`(${badgeCount}) '${chatTitle}': '${chatMsg}'`);}

    let cached = extractNotif.cache.get(chatTitle);
    let isNew  = !cached || cached.msg !== chatMsg || cached.count < badgeCount;

    if (isNew) {
      extractNotif.cache.set(chatTitle, {msg: chatMsg, count: badgeCount});
      if (mode == null || mode === "sendIfNew") {
        new Notification(chatTitle, {body: chatMsg});
      }
    } else if (i === 0 && mode === "sendCached") {
      new Notification(chatTitle, {body: chatMsg});
    }
  }
}


// ==STYLE== ============================================================================
// 🎨 CSS-mod to enable narrow-layout
// 🕒 Called ONCE, after WA loads
// ⚠️ Fragile: Whatsapp will eventually change so much that this breaks
function injectCustomCss() {
  if (document.getElementById('watg-css')) return;
  const style = document.createElement('style'); style.id = 'watg-css';
  style.innerHTML = `@media (max-width: 747px) {
/*𝐌𝐀𝐈𝐍 + 𝐌𝐀𝐈𝐍>𝐃𝐈𝐕 cap width*/
    #main, #main>div {max-width: 100vw !important; margin:0 !important; overflow-x:hidden;}
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
/*𝐋𝐈𝐒𝐓-𝐎𝐅-𝐂𝐇𝐀𝐓𝐒 needed to hide when narrow*/
    #app>div>div>div>div:has(header>div>div>h1){
        flex: 1 1 100% !important;
        max-width: none !important;
        min-width: 0 !important;
        overflow: hidden !important;
    }
/*𝐂𝐇𝐀𝐓 needed to show and fit-container when narrow*/
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

/*𝐎𝐏𝐓𝐈𝐎𝐍𝐀𝐋𝐒*/
/*𝐂𝐇𝐀𝐓-𝐓𝐀𝐁𝐋𝐈𝐒𝐓*/ #side>div[role="tablist"] {visibility:hidden;height:0px;padding:0px;}`;
  document.head.appendChild(style);
}
// 🎨 CSS+JS-mod to add ESC button in narrow-layout
// 🕒 Called every 0.5s
// ⚠️ Fragile: Whatsapp will eventually change so much that this breaks
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
    btn.onclick=()=>{document.dispatchEvent(new KeyboardEvent('keydown', {
        key:'Escape', keyCode:27, which:27, bubbles:true}));};
    target.insertBefore(btn, target.firstChild);
  }
}


// ==DEBUG-TOOLS== ======================================================================
// This section contains helper-functions to help maintain WATG, because Whatsapp changes
//   will eventually break "normal" functions
function help() {
  console.group("help() → Changes in Whatsapp code can break WATG functionality, use these functions to troubleshoot");

  console.group("extractNotif(mode)");
  console.info("extractNotif() looks for specific siblings of each numbered-badge to extract Title+Body and send notifications.",
    "\n\nExamples: extractNotif()             → IF there's a new or increased numbered-badge → toast",
    "\n          extractNotif('sendCached') → IF there's at least 1 cached numbered-badge → toast",
    "\n          extractNotif('log')        → prints Title + LastMessage for each chat with a numbered-badge",
    "\n\n❓  This function takes the output of countUnreadMsg(), which can become bad: use 𝐢𝐧𝐪𝐮𝐢𝐫𝐞𝐄𝐥𝐞𝐦𝐞𝐧𝐭𝐬() ↓ to troubleshoot."
  ); console.groupEnd();

  console.group("inquireElements(attribute, ancestor, [childrenNumbers])");
  console.info("inquireElements() is made to analyze the output of countUnreadMsg(), which should be a list of the unread-badges html-elements.",
    "\n    Use this function to check those elements and their relatives, to extract the unread-count and Title+Body for notifications.",
    "\n\nExamples: inquireElements('selectors')       → prints the selectors of the badges (their path in the HTML-hierarchy)",
    "\n          inquireElements('tag')             → prints element-type(s) of the badges",
    "\n          inquireElements('class', 1)        → prints class(es) of the badges' parents",
    "\n          inquireElements('title', 3, [2,1]) → prints titles of 1st-sons of 2nd-sons of 3rd-lvl-ancestors of the badges",
    "\n\nA quick way to troubleshoot is to run these:",
    "\n  - inquireElements('title', 4, [1,1])       → should print messages with unread-badge",
    "\n  - inquireElements('title', 5, [1,1,1])     → should print groups with unread-badge",
    "\n  - inquireElements('title', 5, [1,1,1,1,1]) → should print 1-on-1-chats with unread-badge" 
  ); console.groupEnd();

  console.groupEnd();
}
// DEBUG-HELPER
function inquireElements(p1, p2, p3) {
  const matchingSpans = countUnreadMsg(true);
  if (!matchingSpans) {
    inquireElements.i = (inquireElements.i || 0) + 1;
    return;
  }

  if (p1 === 'selectors') {for (const el of matchingSpans) {console.log(getUniqueSelector(el));} return;}

  for (const original of matchingSpans) {
    let el = original;
    // Climb up to the ancestor
    let level = parseInt(p2, 10) || 0;
    while (level-- > 0 && el) {el = el.parentElement;}
    if (!el) continue;
    // Descend into children (1-based indexing)
    if (Array.isArray(p3)) {
      for (const idx1 of p3) {
        const idx0 = idx1 - 1;
        if (!el || !el.children || el.children.length<=idx0 || idx0<0) {el=null; break;}
        el = el.children[idx0];
      }
    }
    if (!el) continue;
    if (!p1 || p1 === 'tag' || p1 === 'tagName') {console.log(el.tagName.toLowerCase());}
    else {console.log(el.getAttribute(p1));}
  }
}
// Returns a full DevTools-like CSS selector for a given element
function getUniqueSelector(el) {
  const path = [];
  while (el && el.nodeType === Node.ELEMENT_NODE) {
    let selector = el.nodeName.toLowerCase();
    if (el.id) {selector+= '#'+el.id; path.unshift(selector); break;}
    else {let sib=el, nth=1;
      while (sib=sib.previousElementSibling) {if (sib.nodeName.toLowerCase()===el.nodeName.toLowerCase()) nth++;}
      selector += `:nth-child(${nth})`;
    }
    path.unshift(selector); el = el.parentElement;
  }
  return path.join('> ');
}


// ==CORE== KEEP LAST ===================================================================
// ⚙️ This defines the method to pass the unread count to WATG
// 🕒 Called: by other functions
// 💪 Robust: Whatsapp changes can't break this
function sendUnreadCountToWatg(count) {
  window.__TAURI__.core.invoke('report_title', { title: count.toString(), label: "WA" });
}
// ⚙️ Force-enable notification-permission, reroute notifications via Rust
// 🕒 Called: once, when loading 
// 💪 Robust: Whatsapp changes can't break this
(function () {
  Object.defineProperty(window.Notification, 'permission', {get() {return 'granted';},});
  const Native = window.Notification;
  window.Notification = function (title, opts) {
    console.log('→ window.Notification invoked:', title, opts?.body);
    window.__TAURI__.event.emit('tauri://message', {_watg: true, title, body: opts?.body ?? '',});
    return new Native(title, opts);
  };
  window.Notification.requestPermission = Native.requestPermission.bind(Native);
  // ⚙️ Intercept notifications dispatched from service workers to the main thread
  // 🕒 Called: whenever Notification is constructed
  // 💪 Robust: logs stack trace to trace the source
  try {
    const originalShow = Native.prototype.show;
    Native.prototype.show = function () {
      console.log('→ Native Notification.prototype.show called:', this.title);
      return originalShow.apply(this, arguments);
    };
  } catch (e) {
    console.warn('Notification.prototype.show interception failed:', e);
  }
})();
// ⚙️ Add listener to reroute link-opening in default browser
// 🕒 Called: once, when loading 
// 💪 Robust: Whatsapp changes can't break this
(function () {
  document.addEventListener('click', (event) => {const anchor = event.target.closest('a');
    if (anchor?.href) {event.preventDefault();window.__TAURI__.opener.openUrl(anchor.href);}});
})();
// ⚙️ injectCustomCss (once), start polling for extractNotif + injectEscButton (every 0.5s)
// 🕒 Called: once, when loading 
// 💪 Robust: Whatsapp changes can't break this
(function () {
  document.addEventListener('DOMContentLoaded', () => {
    help(); 
    try { setInterval(() => extractNotif?.(), 500); } catch(e) { console.warn('extractNotif failed: ', e); }
    try { injectCustomCss?.(); } catch(e) { console.warn('injectCustomCss failed: ', e); }
    setTimeout(() => {
      try { setInterval(() => injectEscButton?.(), 500); } catch(e) { console.warn('injectEscButton failed: ', e); }
    }, 3000);
  });
})();
