// Count unread badge spans
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
  window.__TAURI__.core.invoke('report_title',{title: count.toString(), label: window.label});
}

// Inject responsive CSS for WhatsApp Web
function injectCSS() {
  const style = document.createElement('style');
  style.innerHTML = `

/*#######################################################################################
#  ELEMENTS            CSS selectors                                                    #
#---------------------------------------------------------------------------------------#
#  CONTAINER           #app>div>div>div:has(header) {min-width: fit-content;}           #
#    SIDEBAR           #app>div>div>div>header {                                        #
#    SPLASHSCREEN      #app>div>div>div>div:has(div>div>div>span[data-icon*="logo"])    #
#    LIST-OF-CHATS     #app>div>div>div>div:has(header>div>div>h1)                      #
#      CHAT            #app>div>div>div>div:has(div>header)                             #
#        CHAT-HEADER   #app>div>div>div>div>div>header                                  #
#        CHAT-CONTENT  #main>div>div>div>div                                            #
#        CHAT-FOOTER   #main>footer                                                     #
#          EMOJI-PANEL #expressions-panel-container>span>div                            #
#######################################################################################*/

@media (max-width: 747px) {
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
/*𝐄𝐌𝐎𝐉𝐈-𝐏𝐀𝐍𝐄𝐋*/
    #expressions-panel-container>span>div {left: 0px !important; max-width: 100% !important;}
/*𝐄𝐒𝐂-𝐁𝐔𝐓𝐓𝐎𝐍 (via js injection)*/
    .esc-button {
        margin-left: -8px !important;
        padding-right: 16px !important;
    }
}

`; document.head.appendChild(style);}

// Delay execution until DOM is ready and page is interactive
function ready(fn) {
  if (document.readyState !== 'loading') {fn();}
  else {document.addEventListener('DOMContentLoaded', fn);}
}

ready(()=>{injectCSS(); setTimeout(debugAndSend, 100); setInterval(debugAndSend, 500);});
