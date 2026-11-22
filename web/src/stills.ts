/**
 * Renders the pictures that have to exist before a browser can draw anything: the hero's first frame and one share
 * card per page. The robots in them are the site's own figures, photographed in Chrome from a local build.
 *
 *   bun run build && bun run stills          # whatever is missing or whose text has changed
 *   bun run stills --all                     # everything
 */
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { chromium, type Page } from "playwright-core";
import { archetypes } from "./figures/archetypes";
import { makers } from "./figures/bodies";
import { envelope, verdict } from "./figures/envelope";
import { crowd as members, PLACES } from "./figures/lineup";
import { escape, mark } from "./layout";
import { entries, groups } from "./plan";

const root = resolve(import.meta.dir, "..");
const out = join(root, "out");
const media = join(root, "media");
const dir = join(media, "share");
const art = join(root, "out", ".art");
mkdirSync(dir, { recursive: true });
mkdirSync(art, { recursive: true });
const all = process.argv.includes("--all");

const server = Bun.serve({
  port: 0,
  hostname: "127.0.0.1",
  fetch(req) {
    let path = join(out, decodeURIComponent(new URL(req.url).pathname));
    if (statSync(path, { throwIfNoEntry: false })?.isDirectory()) path = join(path, "index.html");
    return statSync(path, { throwIfNoEntry: false })?.isFile()
      ? new Response(Bun.file(path), { headers: { "access-control-allow-origin": "*" } })
      : new Response("Not found", { status: 404 });
  },
});
const base = `http://127.0.0.1:${server.port}`;
const browser = await chromium.launch({
  channel: "chrome",
  args: ["--use-gl=angle", "--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"],
});

/** Opens the home page with everything but one figure's canvas out of the way. */
async function scene(w: number, h: number, scale: number, css: string) {
  const page = await browser.newPage({ viewport: { width: w, height: h }, deviceScaleFactor: scale });
  await page.goto(`${base}/`, { waitUntil: "load" });
  await page.addStyleTag({
    content: `header.top,.dock,.hero-copy,.console-wrap,.lineup::after,.skip{display:none!important}html{scroll-behavior:auto!important;scroll-padding:0!important}body::before,body::after{display:none!important}${css}`,
  });
  return page;
}
const webp = async (page: Page, png: Buffer, path: string, quality: number) => {
  const data = await page.evaluate(
    async ([src, q]) => {
      const img = new Image();
      img.src = `data:image/png;base64,${src}`;
      await img.decode();
      const c = document.createElement("canvas");
      c.width = img.width;
      c.height = img.height;
      c.getContext("2d")!.drawImage(img, 0, 0);
      return c.toDataURL("image/webp", q).split(",")[1]!;
    },
    [png.toString("base64"), quality] as const,
  );
  writeFileSync(path, Buffer.from(data, "base64"));
};

/* ---------- The hero's first frame ---------- */
for (const [name, w, h, scale] of [
  ["lineup", 1920, 1056, 1],
  ["lineup-tall", 540, 590, 2],
] as const) {
  const path = join(media, `${name}.webp`);
  if (!all && existsSync(path)) continue;
  const page = await scene(w, h, scale, `.hero{min-height:${h}px!important;padding:0!important}.lineup{position:absolute!important;inset:0!important;margin:0!important;aspect-ratio:auto!important}`);
  await page.waitForSelector("[data-live]");
  await page.waitForTimeout(700);
  await webp(page, await page.screenshot({ type: "png" }), path, 0.8);
  await page.close();
  console.log("poster", name, `${(statSync(path).size / 1024).toFixed(0)} KB`);
}

/* ---------- Pictures of the figures ---------- */
const shot = (name: string) => join(art, `${name}.png`);
async function crowd(name: string, w: number, h: number) {
  const page = await scene(w, h, 2, `.hero{min-height:${h}px!important;height:${h}px;padding:0!important}.lineup{position:absolute!important;inset:0!important;margin:0!important;aspect-ratio:auto!important}`);
  await page.waitForSelector("[data-live]");
  await page.waitForTimeout(900);
  await page.screenshot({ path: shot(name) });
  await page.close();
}
async function portraits() {
  const page = await scene(900, 900, 1.6, `#fig-stage .stage-view{position:fixed!important;inset:0;z-index:9999;width:900px;height:900px;min-height:0!important;aspect-ratio:auto;border:0;border-radius:0;background:#000}#fig-stage .stage-view>:not(canvas){display:none!important}.reveal,.fig{transform:none!important;filter:none!important;backdrop-filter:none!important;contain:none!important;overflow:visible!important}`);
  await page.evaluate(() => document.querySelector("#fig-stage")!.scrollIntoView());
  await page.waitForSelector("#fig-stage[data-ready]");
  for (const [i, a] of archetypes.entries()) {
    await page.evaluate((n) => document.querySelectorAll<HTMLElement>("#fig-stage .stage-list button")[n]!.click(), i);
    await page.waitForTimeout(1900);
    await page.screenshot({ path: shot(a.id) });
  }
  await page.close();
}
async function figure(name: string, id: string, w: number, h: number, wait = 2500, before?: (p: Page) => Promise<void>) {
  const page = await scene(w, h, 2, `#${id} canvas{position:fixed!important;inset:0;z-index:9999;width:${w}px!important;height:${h}px!important;border:0!important;border-radius:0!important;background:#030705}.reveal,.fig,.fig *{transform:none!important;filter:none!important;backdrop-filter:none!important;contain:none!important}.fig,.fig-body{overflow:visible!important}`);
  await page.evaluate((x) => document.getElementById(x)!.scrollIntoView(), id);
  await page.waitForSelector(`#${id}[data-ready]`);
  await before?.(page);
  await page.waitForTimeout(wait);
  await page.screenshot({ path: shot(name) });
  await page.close();
}

/* ---------- Cards ---------- */
const GROUP = ["#4ade80", "#86efac", "#fbbf24", "#b7a2fb", "#4ade80", "#6cc8f7", "#c6f25a", "#2dd4bf", "#fb7185", "#fbbf24", "#9fb3a8"];
/** Sections that have an obvious body. The rest take one in turn. */
const CAST: Record<string, string> = {
  introduction: "quad-mammal", "1": "wheel-leg", "2": "biped-point", "7": "humanoid", "10": "hexapod-radial", "13": "octopod", "14": "quad-mammal",
  "15": "wheel-leg", "16": "quad-sprawl", "17": "hexapod-radial", "18": "rocker-bogie", "19": "tri-wheel-stair", "20": "swerve-4", "21": "mecanum",
  "22": "biped-point", "23": "humanoid", "24": "balancer-2w", "25": "mobile-manipulator", "26": "octopod", "27": "quad-sprawl", "28": "ackermann",
  "29": "diff-drive", "30": "rocker-bogie", "31": "tracked-flipper", "32": "humanoid", "39": "quad-mammal", "appendix-g": "mobile-manipulator",
};
interface Card {
  id: string;
  eyebrow: string;
  title: string;
  sub: string;
  tag: string;
  numeral: string;
  colour: string;
  art: string;
}
// What the doorway in the home card says is counted, the way the page counts it.
const door = { fits: 0, wide: 0, tall: 0, long: 0 };
for (const m of members()) door[verdict(envelope(makers[m.id]!()), m.scale, PLACES.door!)]++;
const said = [`${door.fits} fit`, ...(["wide", "tall", "long"] as const).filter((k) => door[k]).map((k) => `${door[k]} too ${k}`)];
const counts = [`${archetypes.length} kinds of body`, "7 kinds of building", "256 per batch"];
const turn = archetypes.map((a) => a.id).filter((id) => id !== "snake-crawler" && id !== "omni-3");
const cards: Card[] = entries.map((e, i) => {
  const [, name, rest] = e.title.match(/^([A-Z][a-z]+): (.+)$/) ?? [, e.title, ""];
  const appendix = e.id.match(/^appendix-(.)$/)?.[1]?.toUpperCase();
  return {
    id: e.id,
    eyebrow: e.group === "Overview" ? "The plan" : e.group.split(",")[0]!,
    title: e.id === "introduction" ? "The whole plan" : name!.replace(/ \(.+\)$/, ""),
    sub: rest ? rest[0]!.toUpperCase() + rest.slice(1) : e.id === "introduction" ? "Contents, scope and how to read it" : "",
    tag: e.id === "introduction" ? "Introduction" : e.label,
    numeral: appendix ?? (e.id === "introduction" ? "0" : e.id),
    colour: GROUP[groups.indexOf(e.group)] ?? "#4ade80",
    art: CAST[e.id] ?? turn[i % turn.length]!,
  };
});
cards.push(
  { id: "library", eyebrow: "The plan", title: "The full design", sub: `${entries.length - 1} sections and appendices, cross-linked`, tag: "Library", numeral: "", colour: "#4ade80", art: "batch" },
  { id: "search", eyebrow: "Search", title: "Find anything", sub: "Bodies, buildings, missions, kernels and the math", tag: "Search", numeral: "", colour: "#6cc8f7", art: "rooms" },
);

const css = `@font-face{font-family:Inter;src:url(${base}/fonts/inter-latin.woff2);font-weight:100 900}@font-face{font-family:Mono;src:url(${base}/fonts/mono-latin.woff2);font-weight:100 800}
*{box-sizing:border-box;margin:0}body{width:1200px;overflow:hidden;background:#030705;color:#fff;font-family:Inter;font-optical-sizing:auto;-webkit-font-smoothing:antialiased}
.card{position:relative;width:1200px;height:var(--h);overflow:hidden;background:radial-gradient(900px 620px at 84% 46%,color-mix(in srgb,var(--c) 24%,transparent),transparent 70%),radial-gradient(620px 420px at 0% 0%,color-mix(in srgb,var(--c) 11%,transparent),transparent 70%),#030705}
.card::before{content:"";position:absolute;inset:0;background:linear-gradient(rgb(255 255 255/.032) 1px,transparent 1px) 0 0/60px 60px,linear-gradient(90deg,rgb(255 255 255/.032) 1px,transparent 1px) 0 0/60px 60px;-webkit-mask:radial-gradient(800px 560px at 80% 45%,#000,transparent 75%)}
.brand{position:absolute;left:64px;top:50px;display:flex;align-items:center;gap:15px;font-size:29px;font-weight:800;letter-spacing:-.03em}
.mark{width:54px;height:44px;overflow:visible}.mark .body{fill:#4ade80}.mark .leg{fill:none;stroke:#4ade80;stroke-width:2.2;stroke-linecap:round;stroke-linejoin:round}.mark .far{opacity:.45}.mark .eye{fill:#02140a}
.where{position:absolute;right:54px;top:54px;font:500 19px Mono;color:rgb(255 255 255/.82);letter-spacing:-.01em;padding:8px 15px;border:1px solid rgb(255 255 255/.16);border-radius:12px;background:rgb(3 7 5/.6)}
.eyebrow{display:flex;align-items:center;gap:13px;font:600 18px/1.2 Mono;letter-spacing:.13em;text-transform:uppercase;color:var(--c)}
.eyebrow::before{content:"";width:11px;height:11px;border-radius:50%;background:var(--c);box-shadow:0 0 0 5px color-mix(in srgb,var(--c) 22%,transparent)}
.line{position:absolute;inset:auto 0 0;height:7px;background:linear-gradient(90deg,#fbbf24,#4ade80 22%,#6cc8f7 42%,#c6f25a 62%,#b7a2fb 82%,#fb7185)}
.tick{position:absolute;width:20px;height:20px;border:2px solid rgb(255 255 255/.2)}
.tick.a{left:24px;top:24px;border-right:0;border-bottom:0}.tick.b{right:24px;top:24px;border-left:0;border-bottom:0}`;

const portraitCard = (c: Card, h: number) => `<div class="card" style="--c:${c.colour};--h:${h}px"><style>
.num{position:absolute;right:${c.numeral.length > 2 ? 44 : 56}px;bottom:-34px;font-size:${c.numeral.length > 3 ? 300 : c.numeral.length > 2 ? 350 : 430}px;line-height:1;font-weight:800;letter-spacing:-.07em;color:transparent;-webkit-text-stroke:2px color-mix(in srgb,var(--c) 30%,transparent)}
.art{position:absolute;mix-blend-mode:screen;${c.art === "batch" || c.art === "rooms" ? "inset:0;width:1200px;height:100%;object-fit:cover;-webkit-mask:linear-gradient(90deg,transparent 16%,#000 62%)" : "right:-92px;top:-34px;width:700px;height:700px;-webkit-mask:radial-gradient(closest-side,#000 76%,transparent 100%)"}}
.veil{position:absolute;inset:0;background:linear-gradient(90deg,#030705 8%,rgb(3 7 5/.84) 38%,transparent 62%),linear-gradient(0deg,rgb(3 7 5/.72),transparent 34%)}
.copy{position:absolute;left:64px;top:122px;bottom:118px;width:640px;display:flex;flex-direction:column;justify-content:center}h1 span.k{white-space:nowrap}
h1{margin-top:22px;font-weight:800;letter-spacing:-.052em;line-height:.99;text-wrap:balance;padding-bottom:.08em}
h1 em{font-style:normal;background:linear-gradient(100deg,color-mix(in srgb,var(--c) 70%,#fff),var(--c) 60%);-webkit-background-clip:text;color:transparent;padding-right:.06em;margin-right:-.06em}
.sub{margin-top:20px;font-size:29px;line-height:1.34;font-weight:450;letter-spacing:-.018em;color:#c3d1c8;text-wrap:balance}
.foot{position:absolute;left:64px;bottom:46px;display:flex;gap:12px}
.pill{font:600 19px/1 Mono;padding:12px 17px;border-radius:13px;border:1px solid color-mix(in srgb,var(--c) 44%,transparent);background:color-mix(in srgb,var(--c) 13%,#030705);color:color-mix(in srgb,var(--c) 55%,#fff)}
.pill.b{border-color:rgb(255 255 255/.14);background:rgb(255 255 255/.05);color:#b9c7be;font-weight:500}
</style>${c.numeral ? `<span class="num">${c.numeral}</span>` : ""}<img class="art" src="${base}/.art/${c.art}.png"><div class="veil"></div><i class="tick a"></i><i class="tick b"></i><div class="brand">${mark}FrankenRobotics</div><div class="copy"><p class="eyebrow">${escape(c.eyebrow)}</p><h1 data-fit>${escape(c.title).replace(/(\S+)$/, "<em>$1</em>").replace(/(?<![<\/\w])\w+-\w+(?![^<]*>)/g, '<span class="k">$&</span>')}<span style="margin-left:-.02em">.</span></h1>${c.sub ? `<p class="sub">${escape(c.sub)}</p>` : ""}</div><div class="foot"><span class="pill">${escape(c.tag)}</span><span class="pill b">Robotics lab for AI agents</span><span class="pill b">frankenrobotics.com</span></div><i class="line"></i></div>`;

const homeCard = (h: number) => `<div class="card" style="--c:#4ade80;--h:${h}px"><style>
.art{position:absolute;right:-60px;top:-58px;width:1420px;height:auto;filter:brightness(1.42) contrast(1.05) saturate(1.16)}
.veil{position:absolute;inset:0;background:linear-gradient(90deg,#030705 2%,rgb(3 7 5/.95) 26%,rgb(3 7 5/.46) 44%,transparent 57%),linear-gradient(0deg,rgb(3 7 5/.9),transparent 32%),linear-gradient(180deg,rgb(3 7 5/.62),transparent 20%),radial-gradient(560px 360px at 77% 50%,rgb(74 222 128/.13),transparent 70%)}
.copy{position:absolute;left:64px;top:${h > 615 ? 140 : 126}px;width:720px}
h1{margin-top:24px;font-size:86px;font-weight:800;letter-spacing:-.058em;line-height:.94;text-shadow:0 6px 40px rgb(3 7 5/.8)}
h1 em{font-style:normal;background:linear-gradient(100deg,#d9f75a,#4ade80 52%,#2dd4bf);-webkit-background-clip:text;color:transparent;padding:0 .06em .12em 0;margin-right:-.06em;text-shadow:none}
.sub{margin-top:20px;width:500px;font-size:27px;line-height:1.36;font-weight:450;letter-spacing:-.018em;color:#cfdcd4}
.foot{position:absolute;left:64px;bottom:${h > 615 ? 50 : 40}px;display:flex;gap:12px}
.pill{font:600 18px/1 Mono;padding:12px 17px;border-radius:13px;border:1px solid rgb(74 222 128/.36);background:rgb(8 30 18/.86);color:#b9f7cf}
.hud{position:absolute;right:54px;bottom:${h > 615 ? 48 : 38}px;font:500 18px/1.35 Mono;color:#e6f2ea;padding:11px 16px;border:1px solid rgb(255 255 255/.16);border-radius:13px;background:rgb(3 7 5/.78)}
.hud b{color:#4ade80;font-weight:700}.hud i{font-style:normal;color:#fbbf24}
</style><img class="art" src="${base}/.art/crowd-${h}.png"><div class="veil"></div><div class="brand">${mark}FrankenRobotics</div><span class="where">frankenrobotics.com</span><div class="copy"><p class="eyebrow">Robotics lab for AI agents</p><h1>Name the job.<br>Get <em>every robot</em><br>that can do it.</h1></div><div class="foot">${counts.map((c) => `<span class="pill">${c}</span>`).join("")}</div><span class="hud">${PLACES.door!.width.toFixed(2)} m door · <b>${said[0]}</b>${said.slice(1).map((s) => ` · <i>${s}</i>`).join("")}</span><i class="line"></i></div>`;

const manifestPath = join(dir, "manifest.json");
const manifest: Record<string, string> = existsSync(manifestPath) ? JSON.parse(readFileSync(manifestPath, "utf8")) : {};
// A card is redrawn when its markup changes. A change to the figures themselves needs --all.
const key = (html: string) => Bun.hash((css + html).replaceAll(base, "")).toString(16);
const html: Record<string, string> = { home: homeCard(630), "home-wide": homeCard(600) };
for (const c of cards) html[c.id] = portraitCard(c, 630);
const stale = (id: string) => all || manifest[id] !== key(html[id]!) || !existsSync(join(dir, `${id}.jpg`));
const due = cards.filter((c) => stale(c.id));
const homeDue = stale("home") || stale("home-wide");

if (due.length || homeDue) {
  if (homeDue) for (const h of [630, 600]) await crowd(`crowd-${h}`, 1200, h);
  if (due.some((c) => turn.includes(c.art) || c.art === "snake-crawler")) await portraits();
  if (due.some((c) => c.art === "batch")) await figure("batch", "fig-batch", 1200, 630);
  if (due.some((c) => c.art === "rooms"))
    await figure("rooms", "fig-rooms", 1200, 630, 2500, async (p) => {
      await p.evaluate(() => document.querySelector<HTMLElement>("#fig-rooms [data-seg] button:nth-child(2)")!.click());
    });
  const page = await browser.newPage({ viewport: { width: 1200, height: 630 }, deviceScaleFactor: 1 });
  const draw = async (name: string, h: number) => {
    await page.setViewportSize({ width: 1200, height: h });
    await page.setContent(`<!doctype html><meta charset="utf-8"><style>${css}</style>${html[name]}`, { waitUntil: "load" });
    await page.evaluate(async () => {
      await document.fonts.ready;
      await Promise.all([...document.images].map((i) => i.decode()));
      // The largest title that keeps its longest word whole and the block inside its column.
      const t = document.querySelector<HTMLElement>("[data-fit]");
      const box = t?.parentElement;
      if (t && box)
        for (let s = 132; s >= 46; s -= 2) {
          t.style.fontSize = `${s}px`;
          if (t.scrollWidth <= box.clientWidth && box.scrollHeight <= box.clientHeight) break;
        }
    });
    await page.waitForTimeout(60);
    await page.screenshot({ path: join(dir, `${name}.jpg`), type: "jpeg", quality: name.startsWith("home") ? 90 : 84 });
    manifest[name] = key(html[name]!);
  };
  if (homeDue) {
    await draw("home", 630);
    await draw("home-wide", 600);
  }
  for (const c of due) await draw(c.id, 630);
  writeFileSync(manifestPath, JSON.stringify(Object.fromEntries(Object.entries(manifest).sort()), null, 1) + "\n");
}
const size = [...cards.map((c) => c.id), "home", "home-wide"].reduce((n, id) => n + statSync(join(dir, `${id}.jpg`)).size, 0);
console.log(`${due.length + (homeDue ? 2 : 0)} cards drawn; ${cards.length + 2} in media/share, ${(size / 1e6).toFixed(1)} MB.`);
await browser.close();
server.stop(true);
