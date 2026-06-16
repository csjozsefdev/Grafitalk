import { mkdir, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { chromium } from "playwright";

const OUT_DIR = join(process.cwd(), "docs", "qa-screenshots", "layout-root-cause");

async function extractDistGrid() {
  const distDir = join(process.cwd(), "dist", "assets");
  const { readdir } = await import("node:fs/promises");
  const files = (await readdir(distDir)).filter((f) => f.endsWith(".css"));
  const results = [];

  for (const file of files) {
    const css = await readFile(join(distDir, file), "utf8");
    const grids = css.match(/grid-template-columns:[^;}{]+/g) ?? [];
    const layoutIdx = css.indexOf(".gt-layout");
    results.push({
      file: `dist/assets/${file}`,
      hasGtLayout: layoutIdx >= 0,
      gridRules: grids.filter((g) => g.includes("280px") || g.includes("248px") || g.includes("520px")),
      editorMaxWidth: css.includes("max-width:var(--gt-measure)") || css.includes("max-width: 68ch"),
    });
  }

  const source = await readFile(join(process.cwd(), "src", "styles", "app.css"), "utf8");
  const sourceGrid = source.match(
    /\.gt-layout:not\(\.gt-layout--settings\)[^{]*\{[^}]*grid-template-columns:\s*([^;]+)/s
  );

  return {
    sourceFile: "src/styles/app.css",
    sourceGrid: sourceGrid?.[1]?.trim() ?? null,
    distFiles: results,
  };
}

async function measureAt(url, label) {
  const browser = await chromium.launch();
  const page = await browser.newPage({ viewport: { width: 1280, height: 820 } });

  await page.addInitScript(() => {
    window.__TAURI_INTERNALS__ = {
      metadata: { currentWindow: { label: "main" } },
      invoke: async (cmd) => (String(cmd).includes("list") ? [] : null),
      transformCallback: () => 0,
      unregisterCallback: () => {},
      convertFileSrc: (path) => path,
    };
  });

  try {
    await page.goto(url, { waitUntil: "domcontentloaded", timeout: 60000 });
    await page.waitForSelector(".gt-layout", { timeout: 45000 });
    await page.waitForFunction(() => !document.querySelector(".grafi-splash--grafitalk"), {
      timeout: 45000,
    });

    const data = await page.evaluate(() => {
      const pick = (selector) => {
        const el = document.querySelector(selector);
        if (!el) return null;
        const cs = getComputedStyle(el);
        const rect = el.getBoundingClientRect();
        return {
          width: rect.width,
          height: rect.height,
          computedWidth: cs.width,
          computedMaxWidth: cs.maxWidth,
          computedFlex: cs.flex,
          display: cs.display,
        };
      };

      const layout = document.querySelector(".gt-layout");
      const layoutCs = layout ? getComputedStyle(layout) : null;

      const stylesheets = [];
      for (const sheet of document.styleSheets) {
        try {
          if (sheet.href) stylesheets.push(sheet.href);
        } catch {
          // ignore
        }
      }

      const mediaMatches = {
        maxWidth1120: window.matchMedia("(max-width: 1120px)").matches,
      };

      return {
        url: location.href,
        viewport: {
          innerWidth: window.innerWidth,
          innerHeight: window.innerHeight,
          devicePixelRatio: window.devicePixelRatio,
        },
        documentScrollWidth: document.documentElement.scrollWidth,
        stylesheets,
        inlineStyleSheetCount: document.styleSheets.length - stylesheets.length,
        layoutClassName: layout?.className ?? null,
        gridTemplateColumns: layoutCs?.gridTemplateColumns ?? null,
        mediaMatches,
        elements: {
          sidebar: pick(".gt-sidebar"),
          main: pick(".gt-main"),
          context: pick(".gt-context"),
          documentSurface: pick(".gt-document__surface"),
          documentEditor: pick(".gt-document__editor"),
          contextSection: pick(".gt-context__section"),
          contextEditor: pick(".gt-context__editor"),
        },
      };
    });

    await page.screenshot({
      path: join(OUT_DIR, `workbench-${label}.png`),
      fullPage: false,
    });

    return { label, ...data };
  } finally {
    await browser.close();
  }
}

async function main() {
  await mkdir(OUT_DIR, { recursive: true });

  const cssCompare = await extractDistGrid();
  const dev = await measureAt("http://localhost:5173/", "vite-dev-5173");

  let dist = null;
  try {
    dist = await measureAt("http://localhost:4173/", "dist-preview-4173");
  } catch {
    dist = { label: "dist-preview-4173", error: "Vite preview not running on :4173" };
  }

  const report = {
    checkedAt: new Date().toISOString(),
    launchMode: {
      terminalShows: "npx @tauri-apps/cli dev",
      viteDev: "http://localhost:5173",
      tauriUsesDevUrl: true,
      distUsedInDev: false,
      tauriWindow: { width: 1280, height: 820, minWidth: 960, minHeight: 640 },
    },
    cssCompare,
    runtime: { dev, dist },
  };

  await writeFile(join(OUT_DIR, "root-cause-report.json"), JSON.stringify(report, null, 2));
  console.log(JSON.stringify(report, null, 2));
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
