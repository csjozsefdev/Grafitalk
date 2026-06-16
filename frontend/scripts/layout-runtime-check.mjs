import { mkdir, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { chromium } from "playwright";

const BASE_URL = process.env.LAYOUT_CHECK_URL ?? "http://localhost:5173/";
const OUT_DIR = join(process.cwd(), "docs", "qa-screenshots", "layout-runtime-check");

function extractGridFromCss(css, label) {
  const layoutIdx = css.indexOf(".gt-layout:not(.gt-layout--settings)");
  const fallbackIdx = css.indexOf(".gt-layout{");
  const idx = layoutIdx >= 0 ? layoutIdx : fallbackIdx;
  if (idx < 0) return { label, found: false };

  const slice = css.slice(idx, idx + 600);
  const gridMatch = slice.match(/grid-template-columns:([^;}{]+)/);
  return {
    label,
    found: true,
    gridRule: gridMatch ? gridMatch[1].trim() : null,
    snippet: slice.slice(0, 280),
  };
}

async function measurePage(page) {
  await page.addInitScript(() => {
    window.__TAURI_INTERNALS__ = {
      metadata: { currentWindow: { label: "main" } },
      invoke: async (cmd) => (String(cmd).includes("list") ? [] : null),
      transformCallback: () => 0,
      unregisterCallback: () => {},
      convertFileSrc: (path) => path,
    };
  });

  await page.goto(BASE_URL, { waitUntil: "domcontentloaded", timeout: 60000 });
  await page.waitForSelector(".gt-layout", { timeout: 45000 });
  await page.waitForFunction(() => !document.querySelector(".grafi-splash--grafitalk"), {
    timeout: 45000,
  });
  await page.waitForTimeout(400);

  return page.evaluate(() => {
    const stylesheets = [...document.styleSheets].map((sheet) => {
      try {
        return sheet.href ?? "[inline]";
      } catch {
        return "[blocked]";
      }
    });

    const layout = document.querySelector(".gt-layout");
    const sidebar = document.querySelector(".gt-sidebar");
    const main = document.querySelector(".gt-main");
    const context = document.querySelector(".gt-context");

    const cs = (el) => (el ? getComputedStyle(el) : null);
    const rect = (el) => (el ? el.getBoundingClientRect() : null);

    return {
      url: location.href,
      viewport: { width: window.innerWidth, height: window.innerHeight },
      documentScrollWidth: document.documentElement.scrollWidth,
      stylesheets,
      layout: {
        className: layout?.className ?? null,
        display: cs(layout)?.display ?? null,
        gridTemplateColumns: cs(layout)?.gridTemplateColumns ?? null,
        width: rect(layout)?.width ?? null,
      },
      sidebar: {
        width: rect(sidebar)?.width ?? null,
        computedWidth: cs(sidebar)?.width ?? null,
      },
      main: {
        width: rect(main)?.width ?? null,
        computedWidth: cs(main)?.width ?? null,
      },
      context: {
        width: rect(context)?.width ?? null,
        computedWidth: cs(context)?.width ?? null,
        maxWidth: cs(context)?.maxWidth ?? null,
        display: cs(context)?.display ?? null,
      },
    };
  });
}

async function main() {
  await mkdir(OUT_DIR, { recursive: true });

  const distDir = join(process.cwd(), "dist", "assets");
  const distFiles = await import("node:fs/promises").then((fs) =>
    fs.readdir(distDir).catch(() => [])
  );

  const distCssContents = [];
  for (const file of distFiles.filter((f) => f.endsWith(".css"))) {
    const content = await readFile(join(distDir, file), "utf8");
    distCssContents.push(extractGridFromCss(content, `dist/${file}`));
  }

  const sourceCss = await readFile(join(process.cwd(), "src", "styles", "app.css"), "utf8");
  const sourceExtract = extractGridFromCss(sourceCss, "source/app.css");

  const browser = await chromium.launch();
  const page = await browser.newPage({ viewport: { width: 1280, height: 820 } });

  let runtime;
  try {
    runtime = await measurePage(page);
    await page.screenshot({
      path: join(OUT_DIR, "workbench-1280-after-check.png"),
      fullPage: false,
    });
  } finally {
    await browser.close();
  }

  const report = {
    checkedAt: new Date().toISOString(),
    launchMode: {
      inferred: "Tauri dev (devUrl http://localhost:5173) loads Vite dev server, NOT frontend/dist",
      tauriConf: {
        beforeDevCommand: "npm run dev --prefix ../frontend",
        devUrl: "http://localhost:5173",
        frontendDist: "../frontend/dist",
      },
    },
    sourceCss: sourceExtract,
    distCssBeforeOrAfterBuild: distCssContents,
    runtime,
  };

  await writeFile(join(OUT_DIR, "runtime-report.json"), JSON.stringify(report, null, 2));
  console.log(JSON.stringify(report, null, 2));
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
