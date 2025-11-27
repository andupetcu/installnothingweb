/**
 * Install Nothing - Terminal Simulator Application
 * Extracted from inline script for better maintainability
 */

const themes = {
  // Popular
  'dracula': { background: '#282a36', foreground: '#f8f8f2', cursor: '#f8f8f2',
    black:'#000000', red:'#ff5555', green:'#50fa7b', yellow:'#f1fa8c', blue:'#bd93f9', magenta:'#ff79c6', cyan:'#8be9fd', white:'#bbbbbb',
    brightBlack:'#555555', brightRed:'#ff6e6e', brightGreen:'#69ff94', brightYellow:'#ffffa5', brightBlue:'#d6acff', brightMagenta:'#ff92df', brightCyan:'#a4ffff', brightWhite:'#ffffff' },
  'one-dark': { background: '#282c34', foreground: '#abb2bf', cursor: '#528bff',
    black:'#000000', red:'#e06c75', green:'#98c379', yellow:'#e5c07b', blue:'#61afef', magenta:'#c678dd', cyan:'#56b6c2', white:'#dcdfe4',
    brightBlack:'#5c6370', brightRed:'#e06c75', brightGreen:'#98c379', brightYellow:'#e5c07b', brightBlue:'#61afef', brightMagenta:'#c678dd', brightCyan:'#56b6c2', brightWhite:'#ffffff' },
  'solarized-dark': { background: '#002b36', foreground: '#93a1a1', cursor: '#93a1a1',
    black:'#073642', red:'#dc322f', green:'#859900', yellow:'#b58900', blue:'#268bd2', magenta:'#d33682', cyan:'#2aa198', white:'#eee8d5',
    brightBlack:'#002b36', brightRed:'#cb4b16', brightGreen:'#586e75', brightYellow:'#657b83', brightBlue:'#839496', brightMagenta:'#6c71c4', brightCyan:'#93a1a1', brightWhite:'#fdf6e3' },
  'gruvbox-dark': { background: '#1d2021', foreground: '#ebdbb2', cursor: '#ebdbb2',
    black:'#282828', red:'#cc241d', green:'#98971a', yellow:'#d79921', blue:'#458588', magenta:'#b16286', cyan:'#689d6a', white:'#a89984',
    brightBlack:'#928374', brightRed:'#fb4934', brightGreen:'#b8bb26', brightYellow:'#fabd2f', brightBlue:'#83a598', brightMagenta:'#d3869b', brightCyan:'#8ec07c', brightWhite:'#ebdbb2' },
  'nord': { background: '#2e3440', foreground: '#d8dee9', cursor: '#88c0d0',
    black:'#3b4252', red:'#bf616a', green:'#a3be8c', yellow:'#ebcb8b', blue:'#81a1c1', magenta:'#b48ead', cyan:'#88c0d0', white:'#e5e9f0',
    brightBlack:'#4c566a', brightRed:'#bf616a', brightGreen:'#a3be8c', brightYellow:'#ebcb8b', brightBlue:'#81a1c1', brightMagenta:'#b48ead', brightCyan:'#8fbcbb', brightWhite:'#eceff4' },
  'monokai': { background: '#272822', foreground: '#f8f8f2', cursor: '#f8f8f2',
    black:'#272822', red:'#f92672', green:'#a6e22e', yellow:'#f4bf75', blue:'#66d9ef', magenta:'#ae81ff', cyan:'#a1efe4', white:'#f8f8f2',
    brightBlack:'#555555', brightRed:'#ff669d', brightGreen:'#c0ff6a', brightYellow:'#ffd866', brightBlue:'#a6e1fa', brightMagenta:'#d6baff', brightCyan:'#c7fffd', brightWhite:'#ffffff' },

  // macOS Terminal
  'macos-basic': { background: '#ffffff', foreground: '#000000', cursor: '#000000',
    black:'#000000', red:'#800000', green:'#008000', yellow:'#808000', blue:'#000080', magenta:'#800080', cyan:'#008080', white:'#c0c0c0',
    brightBlack:'#808080', brightRed:'#ff0000', brightGreen:'#00ff00', brightYellow:'#ffff00', brightBlue:'#0000ff', brightMagenta:'#ff00ff', brightCyan:'#00ffff', brightWhite:'#ffffff' },
  'macos-pro': { background: '#000000', foreground: '#e5e5e5', cursor: '#e5e5e5', selection: '#404040',
    black:'#000000', red:'#ff3b30', green:'#34c759', yellow:'#ffcc00', blue:'#0a84ff', magenta:'#ff2d55', cyan:'#5ac8fa', white:'#e5e5e5',
    brightBlack:'#808080', brightRed:'#ff453a', brightGreen:'#30d158', brightYellow:'#ffd60a', brightBlue:'#64d2ff', brightMagenta:'#ff375f', brightCyan:'#64d2ff', brightWhite:'#ffffff' },
  'macos-homebrew': { background: '#000000', foreground: '#00ff00', cursor: '#00ff00',
    black:'#001100', red:'#007700', green:'#00aa00', yellow:'#66ff66', blue:'#00dd00', magenta:'#00ff66', cyan:'#00ff99', white:'#66ff66',
    brightBlack:'#003300', brightRed:'#00aa00', brightGreen:'#00ff00', brightYellow:'#99ff99', brightBlue:'#00ff66', brightMagenta:'#66ff99', brightCyan:'#99ffcc', brightWhite:'#ccffcc' },

  // Windows
  'windows-cmd': { background: '#000000', foreground: '#C0C0C0', cursor: '#FFFFFF',
    black:'#000000', red:'#AA0000', green:'#00AA00', yellow:'#AA5500', blue:'#0000AA', magenta:'#AA00AA', cyan:'#00AAAA', white:'#AAAAAA',
    brightBlack:'#555555', brightRed:'#FF5555', brightGreen:'#55FF55', brightYellow:'#FFFF55', brightBlue:'#5555FF', brightMagenta:'#FF55FF', brightCyan:'#55FFFF', brightWhite:'#FFFFFF' },
  'windows-powershell': { background: '#012456', foreground: '#eeeeee', cursor: '#ffffff',
    black:'#0C0C0C', red:'#C50F1F', green:'#13A10E', yellow:'#C19C00', blue:'#0037DA', magenta:'#881798', cyan:'#3A96DD', white:'#CCCCCC',
    brightBlack:'#767676', brightRed:'#E74856', brightGreen:'#16C60C', brightYellow:'#F9F1A5', brightBlue:'#3B78FF', brightMagenta:'#B4009E', brightCyan:'#61D6D6', brightWhite:'#F2F2F2' },

  // Linux Distros
  'ubuntu': { background: '#2C001E', foreground: '#EEEEEC', cursor: '#FFFFFF',
    black:'#2E3436', red:'#CC0000', green:'#4E9A06', yellow:'#C4A000', blue:'#3465A4', magenta:'#75507B', cyan:'#06989A', white:'#D3D7CF',
    brightBlack:'#555753', brightRed:'#EF2929', brightGreen:'#8AE234', brightYellow:'#FCE94F', brightBlue:'#729FCF', brightMagenta:'#AD7FA8', brightCyan:'#34E2E2', brightWhite:'#EEEEEC' },
  'debian': { background: '#1B1B1B', foreground: '#EDEDED', cursor: '#FFFFFF',
    black:'#2E3436', red:'#CC0000', green:'#4E9A06', yellow:'#C4A000', blue:'#3465A4', magenta:'#A0006B', cyan:'#06989A', white:'#D3D7CF',
    brightBlack:'#555753', brightRed:'#EF2929', brightGreen:'#8AE234', brightYellow:'#FCE94F', brightBlue:'#729FCF', brightMagenta:'#FF1493', brightCyan:'#34E2E2', brightWhite:'#EEEEEC' },
  'redhat': { background: '#1E1E1E', foreground: '#E6E6E6', cursor: '#FFFFFF',
    black:'#1E1E1E', red:'#CC0000', green:'#2AA198', yellow:'#B58900', blue:'#268BD2', magenta:'#D33682', cyan:'#2AA198', white:'#E6E6E6',
    brightBlack:'#4D4D4D', brightRed:'#FF1A1A', brightGreen:'#33CC99', brightYellow:'#FFCC00', brightBlue:'#66B2FF', brightMagenta:'#FF66A3', brightCyan:'#66E0CC', brightWhite:'#FFFFFF' },
  'mint': { background: '#1b1d1e', foreground: '#c5c8c6', cursor: '#c5c8c6',
    black:'#1b1d1e', red:'#cc342b', green:'#89b482', yellow:'#fba922', blue:'#3971ed', magenta:'#a36ac7', cyan:'#70c0ba', white:'#c5c8c6',
    brightBlack:'#969896', brightRed:'#f96a38', brightGreen:'#a3d6a7', brightYellow:'#ffd75f', brightBlue:'#6cb6ff', brightMagenta:'#c7a0dc', brightCyan:'#9be3de', brightWhite:'#ffffff' },
};

// Theme gallery configuration: id maps to themes/{id}.png
const themeGalleryItems = [
  { id: 'dracula', name: 'Dracula' },
  { id: 'one-dark', name: 'One Dark' },
  { id: 'solarized-dark', name: 'Solarized Dark' },
  { id: 'gruvbox-dark', name: 'Gruvbox Dark' },
  { id: 'nord', name: 'Nord' },
  { id: 'monokai', name: 'Monokai' },
  { id: 'macos-basic', name: 'macOS Basic' },
  { id: 'macos-pro', name: 'macOS Pro' },
  { id: 'macos-homebrew', name: 'macOS Homebrew' },
  { id: 'windows-cmd', name: 'Windows CMD' },
  { id: 'windows-powershell', name: 'Windows PowerShell' },
  { id: 'ubuntu', name: 'Ubuntu' },
  { id: 'debian', name: 'Debian' },
  { id: 'redhat', name: 'Red Hat' },
  { id: 'mint', name: 'Linux Mint' },
];

// State variables
let term, socket, fitAddon, onResize;
let paused = false;
let buffer = [];
let lastConfig = null;
let hideTimer = null;
let lastTapTime = 0;
let countdownInterval = null;
let remainingSeconds = 0;
let reconnectAttempts = 0;
const maxReconnectAttempts = 5;

// DOM references
const startBtn = document.getElementById('startBtn');
const playPauseBtn = document.getElementById('playPauseBtn');
const restartBtn = document.getElementById('restartBtn');
const stopBtn = document.getElementById('stopBtn');
const shareBtn = document.getElementById('shareBtn');
const terminalWrap = document.getElementById('terminalWrap');
const topbar = document.getElementById('topbar');
const status = document.getElementById('status');
const simVal = document.getElementById('simval');
const themeVal = document.getElementById('themeval');
const countdownEl = document.getElementById('countdown');
const themeGallery = document.getElementById('themeGallery');

function start() {
  const theme = document.getElementById('theme').value;
  const stack = document.getElementById('stack').value;
  const endless = document.getElementById('endless').checked;
  const h = parseInt(document.getElementById('durationH').value || '0', 10);
  const m = parseInt(document.getElementById('durationM').value || '0', 10);
  const s = parseInt(document.getElementById('durationS').value || '0', 10);
  let duration = endless ? 0 : (h * 3600 + m * 60 + s);
  if (!endless) {
    // enforce sane bounds: min 5s, max 6h
    if (duration < 5) duration = 5;
    if (duration > 21600) duration = 21600;
  }
  const fontSize = parseInt(document.getElementById('fontsize').value || '10', 10);
  const fontFamily = document.getElementById('fontfamily').value;

  lastConfig = { theme, stack, duration, fontSize, fontFamily, h, m, s, endless };

  terminalWrap.style.display = 'block';
  document.getElementById('app').style.display = 'none';

  term = new window.Terminal({
    convertEol: true,
    disableStdin: true,
    fontFamily,
    fontSize,
    theme: themes[theme] || themes['dracula'],
  });
  term.open(document.getElementById('terminal'));
  // Ensure terminal uses 100% of the container
  fitAddon = new window.FitAddon.FitAddon();
  term.loadAddon(fitAddon);
  fitAddon.fit();
  onResize = () => fitAddon.fit();
  window.addEventListener('resize', onResize);
  connectWS(lastConfig);

  // Controls are hidden by default; reveal on double click/tap
  enableOverlayControls();
}

// Build theme gallery
function buildGallery() {
  if (!themeGallery) return;
  themeGalleryItems.forEach(item => {
    const card = document.createElement('div');
    card.className = 'card';
    const img = document.createElement('img');
    img.src = `/app/themes/${item.id}.png`;
    img.alt = `${item.name} preview`;
    img.loading = 'lazy';
    const titleRow = document.createElement('div');
    titleRow.className = 'title';
    const nameEl = document.createElement('span');
    nameEl.textContent = item.name;
    const btn = document.createElement('button');
    btn.className = 'use-btn';
    btn.type = 'button';
    btn.textContent = 'Use this theme';
    btn.addEventListener('click', (ev) => {
      ev.stopPropagation();
      const sel = document.getElementById('theme');
      if (sel) sel.value = item.id;
    });
    titleRow.appendChild(nameEl);
    titleRow.appendChild(btn);
    card.appendChild(img);
    card.appendChild(titleRow);
    card.addEventListener('click', () => openLightbox(img.src, item.name));
    themeGallery.appendChild(card);
  });
}

function connectWS(config, isReconnect = false) {
  // reset pause state and buffer
  paused = false;
  buffer = [];
  playPauseBtn.textContent = 'Pause';

  // Reset reconnect counter on fresh connection
  if (!isReconnect) {
    reconnectAttempts = 0;
  }

  // start countdown or show infinity
  if (config.endless) {
    stopCountdown(false);
    countdownEl.textContent = '\u221E';
  } else {
    startCountdown(config.duration);
  }

  if (socket) {
    try { socket.close(); } catch {}
  }

  const url = new URL(window.location.href);
  url.protocol = url.protocol.replace('http', 'ws');
  url.pathname = '/ws';
  const mode = config.endless ? 'endless' : 'duration';
  url.search = `?theme=${encodeURIComponent(config.theme)}&stack=${encodeURIComponent(config.stack)}&duration=${config.duration}&mode=${mode}`;

  socket = new WebSocket(url.toString());
  setStatus('Connecting');
  socket.addEventListener('open', () => {
    setStatus('Connected');
    simVal.textContent = config.stack;
    themeVal.textContent = config.theme;
    reconnectAttempts = 0; // Reset on successful connection
  });
  socket.addEventListener('message', (ev) => {
    if (paused) {
      buffer.push(ev.data);
    } else {
      term.writeln(ev.data);
    }
  });
  socket.addEventListener('close', () => {
    setStatus('Disconnected');
    stopCountdown(true);
    // Auto-reconnect with exponential backoff
    if (term && reconnectAttempts < maxReconnectAttempts) {
      const delay = Math.min(1000 * Math.pow(2, reconnectAttempts), 10000);
      reconnectAttempts++;
      setStatus(`Reconnecting (${reconnectAttempts}/${maxReconnectAttempts})...`);
      setTimeout(() => {
        if (term && lastConfig) {
          connectWS(lastConfig, true);
        }
      }, delay);
    }
  });
  socket.addEventListener('error', () => { setStatus('Error'); });
}

function stop() {
  if (socket) socket.close();
  if (onResize) window.removeEventListener('resize', onResize);
  terminalWrap.style.display = 'none';
  document.getElementById('app').style.display = 'block';
  status.textContent = '';
  simVal.textContent = '-';
  themeVal.textContent = '-';
  term = null;
  hideControls();
  stopCountdown(false);
}

function showControls() {
  topbar.classList.add('visible');
  if (hideTimer) clearTimeout(hideTimer);
  hideTimer = setTimeout(hideControls, 5000);
}

function hideControls() {
  if (hideTimer) { clearTimeout(hideTimer); hideTimer = null; }
  topbar.classList.remove('visible');
}

function enableOverlayControls() {
  // Mouse double-click
  terminalWrap.addEventListener('dblclick', showControls);
  // Touch double-tap
  terminalWrap.addEventListener('touchend', (e) => {
    const now = Date.now();
    if (now - lastTapTime < 350) {
      showControls();
      lastTapTime = 0;
    } else {
      lastTapTime = now;
    }
  });
}

// Simple lightbox overlay
function createLightbox() {
  const wrap = document.createElement('div');
  wrap.className = 'lightbox';
  wrap.innerHTML = `
    <button class="lightbox-close" aria-label="Close">Close</button>
    <div class="lightbox-content">
      <img id="lightboxImg" alt="Theme preview" />
      <div class="lightbox-caption" id="lightboxCaption"></div>
    </div>
  `;
  document.body.appendChild(wrap);
  wrap.addEventListener('click', (e) => { if (e.target === wrap) closeLightbox(); });
  wrap.querySelector('.lightbox-close').addEventListener('click', closeLightbox);
  return wrap;
}

const lightbox = createLightbox();

function openLightbox(src, caption) {
  const img = document.getElementById('lightboxImg');
  const cap = document.getElementById('lightboxCaption');
  img.src = src; cap.textContent = caption;
  lightbox.classList.add('visible');
}

function closeLightbox() {
  lightbox.classList.remove('visible');
}

function startCountdown(totalSeconds) {
  remainingSeconds = totalSeconds || 0;
  updateCountdown();
  if (countdownInterval) clearInterval(countdownInterval);
  countdownInterval = setInterval(() => {
    if (!paused && remainingSeconds > 0) {
      remainingSeconds -= 1;
      updateCountdown();
    }
    if (remainingSeconds <= 0) {
      clearInterval(countdownInterval);
      countdownInterval = null;
    }
  }, 1000);
}

function stopCountdown(toZero) {
  if (countdownInterval) { clearInterval(countdownInterval); countdownInterval = null; }
  if (toZero) { remainingSeconds = 0; updateCountdown(); }
}

function updateCountdown() {
  countdownEl.textContent = formatHHMMSS(remainingSeconds);
}

function formatHHMMSS(total) {
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  const s = Math.floor(total % 60);
  return `${String(h).padStart(2,'0')}:${String(m).padStart(2,'0')}:${String(s).padStart(2,'0')}`;
}

function buildShareUrl(cfg) {
  const url = new URL(window.location.href);
  url.searchParams.set('theme', cfg.theme);
  url.searchParams.set('stack', cfg.stack);
  // prefer original h/m/s if present, else derive
  const total = cfg.duration || 0;
  const h = (cfg.h ?? Math.floor(total / 3600));
  const m = (cfg.m ?? Math.floor((total % 3600) / 60));
  const s = (cfg.s ?? Math.floor(total % 60));
  url.searchParams.set('h', String(h));
  url.searchParams.set('m', String(m));
  url.searchParams.set('s', String(s));
  // include font prefs for completeness
  url.searchParams.set('fontsize', String(cfg.fontSize));
  url.searchParams.set('fontfamily', cfg.fontFamily);
  return url.toString();
}

function flashStatus(message) {
  const prev = status.textContent;
  status.textContent = `${prev ? prev + ' \u00B7 ' : ''}${message}`;
  setTimeout(() => {
    // remove the added message, keep earlier status
    status.textContent = prev;
  }, 2000);
}

function setStatus(state) {
  status.textContent = state;
  status.classList.remove('ok','warn','err');
  if (state === 'Connected') status.classList.add('ok');
  else if (state === 'Connecting') status.classList.add('warn');
  else status.classList.add('err');
}

// Toast notification for user feedback
function showToast(message, duration = 2500) {
  // Remove existing toast if any
  const existing = document.getElementById('toast');
  if (existing) existing.remove();

  const toast = document.createElement('div');
  toast.id = 'toast';
  toast.textContent = message;
  toast.style.cssText = `
    position: fixed;
    bottom: 24px;
    left: 50%;
    transform: translateX(-50%);
    background: rgba(0, 0, 0, 0.85);
    color: #fff;
    padding: 12px 24px;
    border-radius: 8px;
    font-size: 14px;
    z-index: 1000;
    animation: toast-fade 0.3s ease;
    border: 1px solid rgba(255, 255, 255, 0.1);
  `;
  document.body.appendChild(toast);
  setTimeout(() => {
    toast.style.opacity = '0';
    toast.style.transition = 'opacity 0.3s ease';
    setTimeout(() => toast.remove(), 300);
  }, duration);
}

// Toggle enabled/disabled state of time inputs based on Endless
function updateEndlessUI() {
  const isEndless = document.getElementById('endless').checked;
  for (const id of ['durationH','durationM','durationS']) {
    const el = document.getElementById(id);
    el.disabled = isEndless;
    el.setAttribute('aria-disabled', String(isEndless));
  }
}

// Populate time selects with best-practice ranges and sensible default (00:00:45)
function initTimeInputs() {
  const hSel = document.getElementById('durationH');
  const mSel = document.getElementById('durationM');
  const sSel = document.getElementById('durationS');
  for (let h = 0; h <= 23; h++) {
    const o = document.createElement('option'); o.value = String(h); o.textContent = String(h).padStart(2,'0'); hSel.appendChild(o);
  }
  for (let i = 0; i <= 59; i++) {
    const om = document.createElement('option'); om.value = String(i); om.textContent = String(i).padStart(2,'0'); mSel.appendChild(om);
    const os = document.createElement('option'); os.value = String(i); os.textContent = String(i).padStart(2,'0'); sSel.appendChild(os);
  }
  // default 00:00:45
  hSel.value = '0'; mSel.value = '0'; sSel.value = '45';
}

// On page load, apply query params to prefill controls
function applyQueryParams() {
  const p = new URLSearchParams(window.location.search);
  const theme = p.get('theme');
  const stack = p.get('stack');
  const h = p.get('h');
  const m = p.get('m');
  const s = p.get('s');
  const mode = p.get('mode');
  const fontsize = p.get('fontsize');
  const fontfamily = p.get('fontfamily');
  const autostart = p.get('autostart');
  if (theme) document.getElementById('theme').value = theme;
  if (stack) document.getElementById('stack').value = stack;
  if (h) document.getElementById('durationH').value = String(Math.max(0, Math.min(23, parseInt(h, 10) || 0)));
  if (m) document.getElementById('durationM').value = String(Math.max(0, Math.min(59, parseInt(m, 10) || 0)));
  if (s) document.getElementById('durationS').value = String(Math.max(0, Math.min(59, parseInt(s, 10) || 0)));
  if (mode === 'endless') document.getElementById('endless').checked = true;
  if (fontsize) document.getElementById('fontsize').value = String(Math.max(8, Math.min(28, parseInt(fontsize, 10) || 10)));
  if (fontfamily) document.getElementById('fontfamily').value = fontfamily;
  // reflect endless UI state on initial load
  updateEndlessUI();
  // Auto-start if requested
  if (autostart === '1' || autostart === 'true') {
    setTimeout(() => start(), 100);
  }
}

// Initialize event listeners
function initEventListeners() {
  startBtn.addEventListener('click', start);
  stopBtn.addEventListener('click', stop);

  playPauseBtn.addEventListener('click', () => {
    if (!term) return;
    paused = !paused;
    playPauseBtn.textContent = paused ? 'Play' : 'Pause';
    if (!paused && buffer.length) {
      for (const line of buffer) term.writeln(line);
      buffer = [];
    }
  });

  restartBtn.addEventListener('click', () => {
    if (!term || !lastConfig) return;
    term.reset();
    connectWS(lastConfig);
  });

  // Keyboard shortcuts
  document.addEventListener('keydown', (e) => {
    // Only handle when terminal is active
    if (!term) return;

    // Don't intercept if user is typing in an input
    const tag = e.target.tagName.toLowerCase();
    const isInput = tag === 'input' || tag === 'textarea' || tag === 'select';

    // Escape to stop (always works)
    if (e.key === 'Escape') {
      stop();
      return;
    }

    // Space to pause/play (when not in an input)
    if (e.key === ' ' && !isInput) {
      e.preventDefault();
      playPauseBtn.click();
      return;
    }

    // R to restart (when not in an input)
    if (e.key === 'r' && !e.ctrlKey && !e.metaKey && !isInput) {
      restartBtn.click();
      return;
    }
  });

  // Close lightbox on Escape (when not in terminal mode)
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape' && !term) closeLightbox();
  });

  shareBtn.addEventListener('click', async () => {
    if (!lastConfig) return;
    const url = buildShareUrl(lastConfig);
    try {
      await navigator.clipboard.writeText(url);
      showToast('Link copied to clipboard!');
    } catch (err) {
      // Fallback for browsers without clipboard API
      prompt('Copy this URL:', url);
    }
  });

  document.addEventListener('change', (e) => {
    if (e.target && e.target.id === 'endless') updateEndlessUI();
  });
}

// Initialize application
function init() {
  initTimeInputs();
  buildGallery();
  initEventListeners();
  applyQueryParams();
}

// Run initialization when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}
