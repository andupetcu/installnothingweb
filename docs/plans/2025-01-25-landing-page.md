# Landing Page Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Create a marketing landing page at `/` with SEO optimization, moving the current app to `/app`.

**Architecture:** Static HTML landing page served at root, existing app moved to `/app/` subdirectory. ServeDir handles routing automatically. Landing page links to app with optional autostart parameter.

**Tech Stack:** HTML, CSS (dark terminal theme), existing Axum static file serving

---

## Task 1: Create Directory Structure and Move App

**Files:**
- Create: `web/app/` directory
- Move: `web/index.html` → `web/app/index.html`
- Move: `web/themes/` → `web/app/themes/`

**Step 1: Create app subdirectory**

```bash
mkdir -p web/app
```

**Step 2: Move current index.html to app directory**

```bash
mv web/index.html web/app/index.html
```

**Step 3: Move themes directory into app**

```bash
mv web/themes web/app/themes
```

**Step 4: Verify structure**

```bash
ls -la web/
ls -la web/app/
```

Expected:
```
web/
├── app/
│   ├── index.html
│   └── themes/
│       ├── dracula.png
│       ├── ... (15 theme PNGs)
```

**Step 5: Commit**

```bash
git add -A
git commit -m "refactor: move app to /app/ route for landing page"
```

---

## Task 2: Add Autostart Support to App

**Files:**
- Modify: `web/app/index.html`

**Step 1: Add autostart query param handling**

In `web/app/index.html`, find the `applyQueryParams` function (around line 617) and add autostart support at the end:

Find this code block:
```javascript
      // On page load, apply query params to prefill controls
      (function applyQueryParams() {
        const p = new URLSearchParams(window.location.search);
        const theme = p.get('theme');
        // ... existing code ...
        // reflect endless UI state on initial load
        updateEndlessUI();
      })();
```

Replace with:
```javascript
      // On page load, apply query params to prefill controls
      (function applyQueryParams() {
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
      })();
```

**Step 2: Test autostart locally**

```bash
cargo run --release --bin installer-web
# Open http://localhost:3000/app/?autostart=1
```

Expected: App should automatically start simulation without clicking "Start"

**Step 3: Commit**

```bash
git add web/app/index.html
git commit -m "feat: add autostart query param support"
```

---

## Task 3: Create SEO Assets

**Files:**
- Create: `web/favicon.ico`
- Create: `web/og-image.jpg`
- Create: `web/robots.txt`
- Create: `web/sitemap.xml`

**Step 1: Create robots.txt**

Create `web/robots.txt`:
```
User-agent: *
Allow: /

Sitemap: https://installnothing.com/sitemap.xml
```

**Step 2: Create sitemap.xml**

Create `web/sitemap.xml`:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://installnothing.com/</loc>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>https://installnothing.com/app/</loc>
    <changefreq>weekly</changefreq>
    <priority>0.9</priority>
  </url>
</urlset>
```

**Step 3: Create favicon**

Option A (quick): Use an emoji-based favicon generator or simple terminal icon
Option B: Create a simple 32x32 terminal icon (green `>_` on black background)

For now, use a placeholder SVG favicon. Create `web/favicon.svg`:
```svg
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32">
  <rect width="32" height="32" fill="#000"/>
  <text x="4" y="24" font-family="monospace" font-size="20" fill="#00ff00">&gt;_</text>
</svg>
```

**Step 4: Create og-image placeholder**

This requires a screenshot of the app running. For now, use one of the existing theme PNGs as placeholder:

```bash
cp web/app/themes/dracula.png web/og-image.png
```

Note: Replace with proper 1200x630 OG image later.

**Step 5: Commit**

```bash
git add web/robots.txt web/sitemap.xml web/favicon.svg web/og-image.png
git commit -m "feat: add SEO assets (robots.txt, sitemap, favicon, og-image)"
```

---

## Task 4: Create Landing Page HTML

**Files:**
- Create: `web/index.html`

**Step 1: Create the landing page**

Create `web/index.html` with complete content:

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Install Nothing — Terminal Simulator for Demos & Presentations</title>
    <meta name="description" content="A full-screen, themeable terminal simulator for web and CLI. Pick macOS/Windows/Linux themes, choose a stack, share links, and run endless or timed demos — no real installs.">
    <meta name="keywords" content="terminal simulator, fake terminal, macOS Terminal, xterm.js, installer demo, endless mode, websocket, developer demo, conference demo">
    <link rel="canonical" href="https://installnothing.com/">
    <meta name="robots" content="index,follow,max-image-preview:large,max-snippet:-1,max-video-preview:-1">
    <meta name="theme-color" content="#000000">

    <!-- Open Graph -->
    <meta property="og:type" content="website">
    <meta property="og:url" content="https://installnothing.com/">
    <meta property="og:title" content="Install Nothing — Terminal Simulator">
    <meta property="og:description" content="Themeable terminal simulator for demos and presentations. Fake installs. Real impact.">
    <meta property="og:image" content="https://installnothing.com/og-image.png">

    <!-- Twitter -->
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:title" content="Install Nothing — Terminal Simulator">
    <meta name="twitter:description" content="Themeable terminal simulator for demos and presentations.">
    <meta name="twitter:image" content="https://installnothing.com/og-image.png">

    <!-- Favicon -->
    <link rel="icon" type="image/svg+xml" href="/favicon.svg">

    <!-- Structured Data -->
    <script type="application/ld+json">
    {
      "@context": "https://schema.org",
      "@type": "WebApplication",
      "name": "Install Nothing",
      "url": "https://installnothing.com/",
      "applicationCategory": "DeveloperApplication",
      "operatingSystem": "Any",
      "description": "A full-screen, themeable terminal simulator for web and CLI.",
      "offers": { "@type": "Offer", "price": "0", "priceCurrency": "USD" }
    }
    </script>

    <!-- Analytics -->
    <script defer src="https://analytics.noru1.ro/script.js" data-website-id="eeab6188-5279-49d1-9d42-f22780168378"></script>

    <style>
      * { box-sizing: border-box; margin: 0; padding: 0; }

      :root {
        --bg: #0a0a0a;
        --bg-alt: #111111;
        --fg: #e6e6e6;
        --fg-muted: #888888;
        --accent: #3d8bfd;
        --accent-hover: #5a9eff;
        --green: #34c759;
        --border: #222222;
      }

      html { scroll-behavior: smooth; }

      body {
        min-height: 100vh;
        background: var(--bg);
        color: var(--fg);
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
        line-height: 1.6;
      }

      a { color: var(--accent); text-decoration: none; }
      a:hover { color: var(--accent-hover); }

      .container {
        max-width: 1100px;
        margin: 0 auto;
        padding: 0 24px;
      }

      /* Header */
      header {
        padding: 16px 0;
        border-bottom: 1px solid var(--border);
      }

      .header-inner {
        display: flex;
        justify-content: space-between;
        align-items: center;
      }

      .logo {
        font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
        font-size: 18px;
        font-weight: 600;
        color: var(--fg);
      }

      .logo span { color: var(--green); }

      nav a {
        margin-left: 24px;
        color: var(--fg-muted);
        font-size: 14px;
      }

      nav a:hover { color: var(--fg); }

      /* Hero */
      .hero {
        padding: 80px 0 60px;
        text-align: center;
      }

      .hero h1 {
        font-size: clamp(2rem, 5vw, 3rem);
        font-weight: 700;
        margin-bottom: 16px;
        line-height: 1.2;
      }

      .hero .tagline {
        font-size: clamp(1.1rem, 2.5vw, 1.4rem);
        color: var(--green);
        font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
        margin-bottom: 32px;
      }

      .hero p {
        font-size: 1.1rem;
        color: var(--fg-muted);
        max-width: 600px;
        margin: 0 auto 40px;
      }

      .cta-btn {
        display: inline-block;
        padding: 16px 32px;
        background: var(--accent);
        color: #fff;
        font-size: 1.1rem;
        font-weight: 600;
        border-radius: 8px;
        transition: background 0.2s, transform 0.2s;
      }

      .cta-btn:hover {
        background: var(--accent-hover);
        color: #fff;
        transform: translateY(-2px);
      }

      /* Section styling */
      section {
        padding: 60px 0;
        border-top: 1px solid var(--border);
      }

      section h2 {
        font-size: 1.8rem;
        margin-bottom: 40px;
        text-align: center;
      }

      /* Features */
      .features-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
        gap: 24px;
      }

      .feature-card {
        background: var(--bg-alt);
        border: 1px solid var(--border);
        border-radius: 12px;
        padding: 24px;
      }

      .feature-card .icon {
        font-size: 2rem;
        margin-bottom: 12px;
      }

      .feature-card h3 {
        font-size: 1.1rem;
        margin-bottom: 8px;
      }

      .feature-card p {
        color: var(--fg-muted);
        font-size: 0.95rem;
      }

      /* Theme Gallery */
      .themes-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
        gap: 16px;
      }

      .theme-card {
        background: var(--bg-alt);
        border: 1px solid var(--border);
        border-radius: 8px;
        overflow: hidden;
        transition: transform 0.2s, border-color 0.2s;
      }

      .theme-card:hover {
        transform: translateY(-4px);
        border-color: var(--accent);
      }

      .theme-card img {
        width: 100%;
        height: auto;
        display: block;
      }

      .theme-card .name {
        padding: 10px 12px;
        font-size: 0.9rem;
        color: var(--fg-muted);
        border-top: 1px solid var(--border);
      }

      /* Use Cases */
      .use-cases-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
        gap: 20px;
      }

      .use-case {
        background: var(--bg-alt);
        border: 1px solid var(--border);
        border-radius: 12px;
        padding: 24px;
        text-align: center;
      }

      .use-case .icon {
        font-size: 2.5rem;
        margin-bottom: 16px;
      }

      .use-case h3 {
        font-size: 1rem;
        margin-bottom: 8px;
      }

      .use-case p {
        color: var(--fg-muted);
        font-size: 0.9rem;
      }

      /* How It Works */
      .steps {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
        gap: 32px;
        max-width: 900px;
        margin: 0 auto;
      }

      .step {
        text-align: center;
      }

      .step-number {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: 48px;
        height: 48px;
        background: var(--accent);
        color: #fff;
        font-size: 1.4rem;
        font-weight: 700;
        border-radius: 50%;
        margin-bottom: 16px;
      }

      .step h3 {
        font-size: 1.1rem;
        margin-bottom: 8px;
      }

      .step p {
        color: var(--fg-muted);
        font-size: 0.95rem;
      }

      /* FAQ */
      .faq-list {
        max-width: 700px;
        margin: 0 auto;
      }

      .faq-item {
        border-bottom: 1px solid var(--border);
        padding: 20px 0;
      }

      .faq-item:last-child { border-bottom: none; }

      .faq-item h3 {
        font-size: 1rem;
        margin-bottom: 8px;
        color: var(--fg);
      }

      .faq-item p {
        color: var(--fg-muted);
        font-size: 0.95rem;
      }

      /* Footer */
      footer {
        padding: 40px 0;
        border-top: 1px solid var(--border);
        text-align: center;
        color: var(--fg-muted);
        font-size: 0.9rem;
      }

      footer a { color: var(--fg-muted); }
      footer a:hover { color: var(--fg); }

      .footer-links {
        margin-bottom: 16px;
      }

      .footer-links a {
        margin: 0 12px;
      }

      /* Responsive */
      @media (max-width: 600px) {
        .hero { padding: 50px 0 40px; }
        section { padding: 40px 0; }
        .header-inner { flex-direction: column; gap: 12px; }
        nav a { margin: 0 12px; }
      }
    </style>
  </head>
  <body>
    <header>
      <div class="container header-inner">
        <div class="logo"><span>&gt;_</span> Install Nothing</div>
        <nav>
          <a href="#features">Features</a>
          <a href="#themes">Themes</a>
          <a href="#use-cases">Use Cases</a>
          <a href="#faq">FAQ</a>
          <a href="https://github.com/andrei/install-nothing" target="_blank">GitHub</a>
        </nav>
      </div>
    </header>

    <main>
      <section class="hero">
        <div class="container">
          <h1>Terminal Simulator for Demos & Presentations</h1>
          <p class="tagline">Fake Installs. Real Impact.</p>
          <p>A full-screen, themeable terminal that simulates realistic installation sequences. Perfect for conference booths, product demos, and developer presentations — without installing anything.</p>
          <a href="/app/?autostart=1" class="cta-btn">Start Demo</a>
        </div>
      </section>

      <section id="features">
        <div class="container">
          <h2>Features</h2>
          <div class="features-grid">
            <div class="feature-card">
              <div class="icon">🎨</div>
              <h3>15+ Themes</h3>
              <p>macOS, Windows, Ubuntu, Dracula, Nord, and more. Match your brand or presentation style.</p>
            </div>
            <div class="feature-card">
              <div class="icon">⚡</div>
              <h3>20 Simulation Stages</h3>
              <p>BIOS boot, kernel compilation, Docker containers, AI model loading, cloud provisioning, and more.</p>
            </div>
            <div class="feature-card">
              <div class="icon">🔗</div>
              <h3>Shareable URLs</h3>
              <p>Configure once, share the link. Theme, duration, and settings are encoded in the URL.</p>
            </div>
            <div class="feature-card">
              <div class="icon">⏱️</div>
              <h3>Timed or Endless</h3>
              <p>Set a specific duration or run forever. Perfect countdown timer in the overlay.</p>
            </div>
            <div class="feature-card">
              <div class="icon">📱</div>
              <h3>Fully Responsive</h3>
              <p>Works on desktop, tablet, and mobile. Touch-friendly controls with double-tap to reveal.</p>
            </div>
            <div class="feature-card">
              <div class="icon">🖥️</div>
              <h3>CLI & Web</h3>
              <p>Run in your terminal or in the browser. Same simulation engine, your choice of interface.</p>
            </div>
          </div>
        </div>
      </section>

      <section id="themes">
        <div class="container">
          <h2>Theme Gallery</h2>
          <div class="themes-grid">
            <div class="theme-card">
              <img src="/app/themes/dracula.png" alt="Dracula theme" loading="lazy">
              <div class="name">Dracula</div>
            </div>
            <div class="theme-card">
              <img src="/app/themes/one-dark.png" alt="One Dark theme" loading="lazy">
              <div class="name">One Dark</div>
            </div>
            <div class="theme-card">
              <img src="/app/themes/nord.png" alt="Nord theme" loading="lazy">
              <div class="name">Nord</div>
            </div>
            <div class="theme-card">
              <img src="/app/themes/macos-pro.png" alt="macOS Pro theme" loading="lazy">
              <div class="name">macOS Pro</div>
            </div>
            <div class="theme-card">
              <img src="/app/themes/ubuntu.png" alt="Ubuntu theme" loading="lazy">
              <div class="name">Ubuntu</div>
            </div>
            <div class="theme-card">
              <img src="/app/themes/windows-powershell.png" alt="Windows PowerShell theme" loading="lazy">
              <div class="name">PowerShell</div>
            </div>
          </div>
          <p style="text-align: center; margin-top: 24px; color: var(--fg-muted);">
            <a href="/app/">View all 15 themes →</a>
          </p>
        </div>
      </section>

      <section id="use-cases">
        <div class="container">
          <h2>Use Cases</h2>
          <div class="use-cases-grid">
            <div class="use-case">
              <div class="icon">🎪</div>
              <h3>Conference Booths</h3>
              <p>Eye-catching terminal visuals without risk of crashes or exposing real systems.</p>
            </div>
            <div class="use-case">
              <div class="icon">🎬</div>
              <h3>Product Demos</h3>
              <p>Show realistic installation flows for marketing videos and presentations.</p>
            </div>
            <div class="use-case">
              <div class="icon">📺</div>
              <h3>Stream Backgrounds</h3>
              <p>Animated terminal overlay for coding streams and developer content.</p>
            </div>
            <div class="use-case">
              <div class="icon">💼</div>
              <h3>Portfolio Showcase</h3>
              <p>Demonstrate terminal UI skills on your developer portfolio.</p>
            </div>
          </div>
        </div>
      </section>

      <section id="how-it-works">
        <div class="container">
          <h2>How It Works</h2>
          <div class="steps">
            <div class="step">
              <div class="step-number">1</div>
              <h3>Pick a Theme</h3>
              <p>Choose from 15+ terminal themes including macOS, Windows, Linux distros, and popular color schemes.</p>
            </div>
            <div class="step">
              <div class="step-number">2</div>
              <h3>Configure</h3>
              <p>Select simulation stages, set duration (or go endless), and customize font settings.</p>
            </div>
            <div class="step">
              <div class="step-number">3</div>
              <h3>Run & Share</h3>
              <p>Start the simulation and share the URL. Your configuration is encoded in the link.</p>
            </div>
          </div>
        </div>
      </section>

      <section id="faq">
        <div class="container">
          <h2>FAQ</h2>
          <div class="faq-list">
            <div class="faq-item">
              <h3>Does this actually install anything?</h3>
              <p>No. It's a simulation that displays realistic-looking terminal output. Nothing is installed, modified, or executed on your system.</p>
            </div>
            <div class="faq-item">
              <h3>Can I use this for my conference booth?</h3>
              <p>Absolutely! That's one of the main use cases. Set it to endless mode, pick a theme, and let it run on a big screen.</p>
            </div>
            <div class="faq-item">
              <h3>Is there a CLI version?</h3>
              <p>Yes! Install via cargo and run <code>install-nothing --all</code> in your terminal. Same simulation engine, native experience.</p>
            </div>
            <div class="faq-item">
              <h3>Is it free?</h3>
              <p>Yes, completely free and open source. Check out the <a href="https://github.com/andrei/install-nothing">GitHub repo</a>.</p>
            </div>
            <div class="faq-item">
              <h3>Can I contribute themes or stages?</h3>
              <p>Pull requests welcome! See AGENTS.md in the repo for contribution guidelines.</p>
            </div>
          </div>
        </div>
      </section>
    </main>

    <footer>
      <div class="container">
        <div class="footer-links">
          <a href="/app/">Launch App</a>
          <a href="https://github.com/andrei/install-nothing">GitHub</a>
          <a href="https://github.com/andrei/install-nothing/issues">Report Issue</a>
        </div>
        <p>Made with fun by <a href="https://github.com/andrei">Kerim Buyukakyuz</a> · MIT License</p>
      </div>
    </footer>
  </body>
</html>
```

**Step 2: Test landing page locally**

```bash
cargo run --release --bin installer-web
# Open http://localhost:3000/
```

Expected:
- Landing page displays at `/`
- "Start Demo" button links to `/app/?autostart=1`
- All sections render correctly
- Theme images load from `/app/themes/`

**Step 3: Commit**

```bash
git add web/index.html
git commit -m "feat: add landing page with SEO and marketing content"
```

---

## Task 5: Update Theme Image Paths in App

**Files:**
- Modify: `web/app/index.html`

Since themes moved to `/app/themes/`, update the relative paths in the app.

**Step 1: Check current path**

In `web/app/index.html`, find the gallery image path (around line 371):
```javascript
img.src = `themes/${item.id}.png`;
```

Since the app is now at `/app/index.html`, the relative path `themes/` should still work (resolves to `/app/themes/`).

**Step 2: Verify paths work**

```bash
cargo run --release --bin installer-web
# Open http://localhost:3000/app/
```

Expected: Theme preview images should load correctly.

If images don't load, update to absolute paths:
```javascript
img.src = `/app/themes/${item.id}.png`;
```

**Step 3: Commit if changes needed**

```bash
git add web/app/index.html
git commit -m "fix: update theme image paths for /app/ route"
```

---

## Task 6: Final Verification

**Step 1: Full test of all routes**

```bash
cargo run --release --bin installer-web
```

Test checklist:
- [ ] `http://localhost:3000/` — Landing page loads
- [ ] `http://localhost:3000/app/` — App loads
- [ ] `http://localhost:3000/app/?autostart=1` — App auto-starts
- [ ] `http://localhost:3000/robots.txt` — Returns robots.txt
- [ ] `http://localhost:3000/sitemap.xml` — Returns sitemap
- [ ] `http://localhost:3000/favicon.svg` — Returns favicon
- [ ] Landing page "Start Demo" button works
- [ ] Theme gallery images load on landing page
- [ ] Theme gallery images load in app
- [ ] WebSocket simulation works

**Step 2: Verify SEO meta tags**

Open landing page, view source, confirm:
- `<title>` tag present
- `<meta name="description">` present
- Open Graph tags present
- JSON-LD structured data present

**Step 3: Final commit**

```bash
git add -A
git commit -m "chore: final landing page verification"
```

---

## Summary of File Changes

| Action | Path |
|--------|------|
| Create | `web/app/` directory |
| Move | `web/index.html` → `web/app/index.html` |
| Move | `web/themes/` → `web/app/themes/` |
| Create | `web/index.html` (landing page) |
| Create | `web/robots.txt` |
| Create | `web/sitemap.xml` |
| Create | `web/favicon.svg` |
| Create | `web/og-image.png` (placeholder) |
| Modify | `web/app/index.html` (autostart param) |

## Post-Implementation Notes

1. **OG Image**: Replace `og-image.png` with a proper 1200x630 screenshot
2. **GitHub URL**: Update the GitHub URLs in landing page to your actual repo
3. **Domain**: Verify `installnothing.com` canonical URLs match your deployment
4. **Analytics**: Analytics script is already included from existing setup
