/**
 * Grafi Advisor runtime audit collector.
 * Targets the Vite dev server (same bundle as Tauri WebView in dev).
 * Does not modify application source.
 */
import { mkdir, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { chromium } from "playwright";

const BASE_URL = process.env.GRAFI_AUDIT_URL ?? "http://localhost:5173/";
const OUT_DIR = join(process.cwd(), "docs", "qa-screenshots", "grafi-advisor-audit");
const REPORT_JSON = join(OUT_DIR, "runtime-measurements.json");

function overflow(rect) {
  return {
    leftOverflow: Math.max(0, -rect.left),
    topOverflow: Math.max(0, -rect.top),
    rightOverflow: Math.max(0, rect.right - window.innerWidth),
    bottomOverflow: Math.max(0, rect.bottom - window.innerHeight),
  };
}

function measureEl(el) {
  if (!el) return null;
  const rect = el.getBoundingClientRect();
  const cs = getComputedStyle(el);
  return {
    rect: {
      x: rect.x,
      y: rect.y,
      width: rect.width,
      height: rect.height,
      top: rect.top,
      right: rect.right,
      bottom: rect.bottom,
      left: rect.left,
    },
    overflow: overflow(rect),
    computed: {
      position: cs.position,
      display: cs.display,
      width: cs.width,
      height: cs.height,
      margin: cs.margin,
      padding: cs.padding,
      overflow: cs.overflow,
      transform: cs.transform,
      objectFit: cs.objectFit,
      objectPosition: cs.objectPosition,
      zIndex: cs.zIndex,
      pointerEvents: cs.pointerEvents,
    },
  };
}

async function main() {
  await mkdir(OUT_DIR, { recursive: true });

  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({
    viewport: { width: 1280, height: 820 },
    deviceScaleFactor: 1.25,
  });

  const consoleLogs = [];
  page.on("console", (msg) => consoleLogs.push(`[${msg.type()}] ${msg.text()}`));
  page.on("pageerror", (err) => consoleLogs.push(`[pageerror] ${err.message}`));

  await page.addInitScript(() => {
    window.__TAURI_INTERNALS__ = {
      metadata: { currentWindow: { label: "main" } },
      invoke: async (cmd) => {
        if (cmd === "list_projects" || cmd === "list_archived_projects") {
          return [];
        }
        return null;
      },
      transformCallback: () => 0,
      unregisterCallback: () => {},
      convertFileSrc: (path) => path,
    };
  });

  try {
    await page.goto(BASE_URL, { waitUntil: "domcontentloaded", timeout: 60000 });

    // Splash blocks pointer events; host may exist before splash unmounts.
    await page.waitForSelector(".gt-layout", { timeout: 45000 });
    await page.waitForFunction(
      () => !document.querySelector(".grafi-splash--grafitalk"),
      { timeout: 45000 }
    );
    await page.waitForSelector(".gt-grafi-host", {
      state: "attached",
      timeout: 15000,
    });
    await page.waitForTimeout(500);
  } catch (waitErr) {
    await page.screenshot({
      path: join(OUT_DIR, "00-failure-state.png"),
      fullPage: true,
    });
    const bodyHtml = await page.content();
    await writeFile(
      join(OUT_DIR, "failure-diagnostics.json"),
      JSON.stringify(
        {
          error: String(waitErr),
          consoleLogs,
          title: await page.title(),
          bodySnippet: bodyHtml.slice(0, 4000),
        },
        null,
        2
      ),
      "utf8"
    );
    throw waitErr;
  }

  const data = await page.evaluate(() => {
    const sel = (s) => document.querySelector(s);
    const img = sel(".grafi-figure__img");

    return {
      viewport: {
        innerWidth: window.innerWidth,
        innerHeight: window.innerHeight,
        devicePixelRatio: window.devicePixelRatio,
      },
      document: {
        scrollWidth: document.documentElement.scrollWidth,
        scrollHeight: document.documentElement.scrollHeight,
        bodyScrollWidth: document.body.scrollWidth,
        bodyScrollHeight: document.body.scrollHeight,
      },
      elements: {
        host: measureEl(sel(".gt-grafi-host")),
        advisor: measureEl(sel(".gt-grafi-host .grafi-advisor")),
        bubble: measureEl(sel(".grafi-bubble")),
        figureColumn: measureEl(sel(".grafi-bubble__figure-column")),
        figureWrap: measureEl(sel(".grafi-bubble__figure-wrap")),
        figure: measureEl(sel(".grafi-figure")),
        figureInner: measureEl(sel(".grafi-figure__inner")),
        image: measureEl(img),
        panel: measureEl(sel(".grafi-bubble__panel")),
        app: measureEl(sel(".gt-app")),
        layout: measureEl(sel(".gt-layout")),
        sidebar: measureEl(sel(".gt-sidebar")),
      },
      image: img
        ? {
            src: img.currentSrc || img.src,
            naturalWidth: img.naturalWidth,
            naturalHeight: img.naturalHeight,
            complete: img.complete,
          }
        : null,
      hitTest: (() => {
        const host = sel(".gt-grafi-host");
        const panel = sel(".grafi-bubble__panel");
        const figure = sel(".grafi-figure");
        if (!host) return null;
        const hr = host.getBoundingClientRect();
        const points = {
          hostCenter: document.elementsFromPoint(
            hr.left + hr.width / 2,
            hr.top + hr.height / 2
          ).map((e) => e.className?.toString?.() ?? e.tagName),
          figureCenter: figure
            ? (() => {
                const fr = figure.getBoundingClientRect();
                return document
                  .elementsFromPoint(
                    fr.left + fr.width / 2,
                    fr.top + fr.height / 2
                  )
                  .map((e) => e.className?.toString?.() ?? e.tagName);
              })()
            : null,
          panelCenter: panel
            ? (() => {
                const pr = panel.getBoundingClientRect();
                return document
                  .elementsFromPoint(
                    pr.left + pr.width / 2,
                    pr.top + pr.height / 2
                  )
                  .map((e) => e.className?.toString?.() ?? e.tagName);
              })()
            : null,
        };
        return points;
      })(),
      domPath: (() => {
        const host = sel(".gt-grafi-host");
        if (!host) return null;
        const parts = [];
        let node = host;
        while (node) {
          parts.push(
            node.tagName.toLowerCase() +
              (node.className ? `.${String(node.className).trim().split(/\s+/).join(".")}` : "")
          );
          node = node.firstElementChild;
          if (parts.length > 12) break;
        }
        return parts;
      })(),
    };

    function measureEl(el) {
      if (!el) return null;
      const rect = el.getBoundingClientRect();
      const cs = getComputedStyle(el);
      return {
        rect: {
          x: rect.x,
          y: rect.y,
          width: rect.width,
          height: rect.height,
          top: rect.top,
          right: rect.right,
          bottom: rect.bottom,
          left: rect.left,
        },
        overflow: {
          leftOverflow: Math.max(0, -rect.left),
          topOverflow: Math.max(0, -rect.top),
          rightOverflow: Math.max(0, rect.right - window.innerWidth),
          bottomOverflow: Math.max(0, rect.bottom - window.innerHeight),
        },
        computed: {
          position: cs.position,
          display: cs.display,
          width: cs.width,
          height: cs.height,
          margin: cs.margin,
          padding: cs.padding,
          overflow: cs.overflow,
          transform: cs.transform,
          objectFit: cs.objectFit,
          objectPosition: cs.objectPosition,
          zIndex: cs.zIndex,
          pointerEvents: cs.pointerEvents,
          backgroundColor: cs.backgroundColor,
        },
      };
    }
  });

  await page.screenshot({
    path: join(OUT_DIR, "01-baseline-workbench.png"),
    fullPage: false,
  });

  await page.evaluate(() => {
    for (const sel of [
      ".gt-grafi-host",
      ".grafi-figure",
      ".grafi-figure__img",
      ".grafi-bubble__panel",
    ]) {
      const el = document.querySelector(sel);
      if (el) {
        el.style.outline = "2px solid #e11d48";
        el.style.outlineOffset = "1px";
      }
    }
  });

  await page.screenshot({
    path: join(OUT_DIR, "02-debug-outlines.png"),
    fullPage: false,
  });

  await writeFile(REPORT_JSON, JSON.stringify({ ...data, consoleLogs }, null, 2), "utf8");
  console.log("Wrote", REPORT_JSON);
  console.log(JSON.stringify(data, null, 2));

  await browser.close();
}

main().catch(async (err) => {
  console.error(err);
  process.exit(1);
});
