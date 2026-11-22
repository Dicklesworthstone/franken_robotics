/**
 * Measures how tall each figure is once it has drawn itself, at a phone's, a tablet's and a desktop's width, and writes
 * heights.json. The page keeps that much room free, so nothing jumps when a figure loads: Safari does not hold the
 * reader's place when something above it grows.
 *
 *   bun src/heights.ts http://127.0.0.1:8787     # a preview of the built site
 */
import { chromium } from "playwright-core";

const base = process.argv[2] ?? "http://127.0.0.1:8787";
const browser = await chromium.launch({ channel: "chrome", headless: true, args: ["--use-gl=angle", "--use-angle=metal", "--enable-gpu"] });
const heights: Record<string, number[]> = {};
for (const [i, [width, height]] of ([[393, 800], [780, 1000], [1440, 900]] as const).entries()) {
  const page = await browser.newPage({ viewport: { width, height } });
  await page.goto(base, { waitUntil: "load" });
  const ids = await page.locator("figure.fig[data-fig]:not([data-fig=loop])").evaluateAll((all) => all.map((e) => e.id));
  for (const id of ids) {
    await page.evaluate((x) => document.getElementById(x)!.scrollIntoView({ block: "start", behavior: "instant" }), id);
    await page.waitForTimeout(1600);
    const h = await page.evaluate((x) => document.querySelector(`#${x} > .fig-body`)?.getBoundingClientRect().height, id);
    if (h) (heights[id.slice(4)] ??= [])[i] = Math.round(h);
  }
  await page.close();
}
await browser.close();
const lines = Object.entries(heights).map(([name, all]) => ` "${name}": ${JSON.stringify(all).replaceAll(",", ", ")}`);
await Bun.write(new URL("heights.json", import.meta.url), `{\n${lines.join(",\n")}\n}\n`);
console.log(heights);
