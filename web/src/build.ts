import { mkdirSync, readFileSync, writeFileSync, cpSync, rmSync } from "node:fs";
import { resolve, join, dirname } from "node:path";
import MarkdownIt from "markdown-it";
import { escape, origin, shell } from "./layout";
import { home } from "./home";

const root = resolve(import.meta.dir, ".."),
  out = join(root, "out");
const src = join(root, "../COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md");
const bytes = readFileSync(src),
  source = bytes.toString("utf8");
const md = new MarkdownIt({ html: false, linkify: true, typographer: false });
md.linkify.set({ fuzzyLink: false, fuzzyEmail: false });
// In C-32 the interval extension [c](B) is mathematical notation.
md.inline.ruler.before("link", "interval_enclosure", (state, silent) => {
  if (!state.src.startsWith("[c](B)", state.pos)) return false;
  if (!silent) state.pending += "[c](B)";
  state.pos += "[c](B)".length;
  return true;
});
type Section = {
  id: string;
  title: string;
  group: string;
  label: string;
  start: number;
  sub: { title: string; id: string }[];
  body: ReturnType<typeof md.parse>;
};
const sections: Section[] = [];
let group = "Overview";
function add(id: string, title: string, label: string, start: number) {
  sections.push({ id, title, label, start, group, sub: [], body: [] });
}
add("introduction", "Introduction and contents", "Edition 1.0", 0);
const tokens = md.parse(source, {});
for (let i = 0; i < tokens.length; i++) {
  const t = tokens[i];
  if (t.type !== "heading_open") continue;
  const heading = tokens[i + 1].content;
  if (t.tag === "h1" && (heading.startsWith("PART ") || heading === "APPENDICES")) {
    group =
      heading === "APPENDICES"
        ? "Appendices"
        : heading.replace(
            /^PART ([^.]+)\. (.+)$/,
            (s, n, name) => "Part " + n + " · " + name[0] + name.slice(1).toLowerCase(),
          );
  }
  if (t.tag !== "h2") continue;
  const m = heading.match(/^(\d+(?:\.\d+)?)\s*[.]? (.+)$/);
  const a = heading.match(/^Appendix ([A-O])\. (.+)$/);
  if (m) add(m[1], m[2], "§" + m[1], t.map[0]);
  else if (a) add("appendix-" + a[1].toLowerCase(), a[2], "Appendix " + a[1], t.map[0]);
}
const url = (p: Section) => "/plan/" + p.id + "/";
const lines = source.split("\n");
const refs = new Map<string, string>();
// Move each part divider to its opening chapter, preserving every intervening line.
for (const p of sections.slice(1)) {
  let i = p.start - 1;
  while (i > 0 && !lines[i].trim()) i--;
  if (/^# (PART |APPENDICES$)/.test(lines[i])) p.start = i;
}
if (sections.length !== 71 || new Set(sections.map((p) => p.id)).size !== sections.length)
  throw Error("Review changed plan structure");
const chunks = sections.map((p, i) =>
  lines.slice(p.start, sections[i + 1]?.start ?? lines.length).join("\n"),
);
if (chunks.join("\n") !== source) throw Error("Incomplete plan segmentation");
// Pagefind leaves content-addressed shards from previous runs in its output directory.
rmSync(out, { recursive: true, force: true });
for (const [i, p] of sections.entries()) {
  refs.set(p.id, url(p));
  refs.set(p.label, url(p));
  const block = md.parse(chunks[i], {}),
    ids = new Set<string>();
  for (let j = 0; j < block.length; j++) {
    const t = block[j];
    if (t.type !== "heading_open") continue;
    const title = block[j + 1].content;
    if (p.id !== "introduction" && (t.tag === "h1" || (t.tag === "h2" && title !== "Contents"))) {
      block.splice(j, 3);
      j--;
      continue;
    }
    if (t.tag === "h1") {
      t.tag = block[j + 2].tag = "p";
      t.attrSet("class", "source-heading");
      continue;
    }
    const match = title.match(/^(\d+(?:\.\d+)+|[A-O]\.\d+(?:\.\d+)*|C-\d+)\b/);
    let id = match ? "s-" + match[1].toLowerCase() : "heading-" + (p.sub.length + 1);
    if (ids.has(id)) id += "-" + j;
    ids.add(id);
    t.attrSet("id", id);
    p.sub.push({ title, id });
    if (match) refs.set(match[1], url(p) + "#" + id);
  }
  p.body = block;
}
md.renderer.rules.text = (ts, i) => {
  const s = escape(ts[i].content);
  if (
    ts
      .slice(0, i)
      .reduce((n, t) => n + (t.type === "link_open" ? 1 : t.type === "link_close" ? -1 : 0), 0)
  )
    return s;
  return s.replace(/§(\d+(?:\.\d+)*)|\bAppendix ([A-O])\b/g, (text, n, letter) => {
    const href = refs.get(n ?? "appendix-" + letter.toLowerCase());
    return href ? `<a class="xref" href="${href}">${text}</a>` : text;
  });
};
function emit(path: string, data: string | Buffer) {
  const dest = join(out, path);
  mkdirSync(dirname(dest), { recursive: true });
  writeFileSync(dest, data);
}
function page(path: string, title: string, content: string, description?: string) {
  emit(path + "index.html", shell(title, path, content, description));
}
const groups = [...new Set(sections.map((s) => s.group))];
const groupslug = (g: string) => "part-" + groups.indexOf(g);
const eyebrow = (s: string) => `<p class="eyebrow"><i></i>${escape(s)}</p>`;
const chapterLink = (p: Section) =>
  `<a href="${url(p)}"><small>${p.label}</small><span>${escape(p.title)}</span><b>↗</b></a>`;
for (const [i, p] of sections.entries()) {
  const neighbors = sections.slice(Math.max(0, i - 1), i + 2).filter((q) => q !== p);
  const outline = p.sub.map((h) => `<a href="#${h.id}">${escape(h.title)}</a>`).join("");
  const body = md.renderer.render(p.body, md.options, {});
  page(
    url(p),
    p.label + " · " + p.title,
    `<div class="wrap reader-layout"><aside class="reader-nav"><a class="back" href="/plan/#${groupslug(p.group)}">← Plan library</a>${eyebrow(p.group)}<details open><summary>In this part</summary><div class="part-links">${sections
      .filter((q) => q.group === p.group)
      .map(
        (q) =>
          `<a href="${url(q)}" ${p === q ? 'aria-current="page"' : ""}>${q.label} · ${escape(q.title)}</a>`,
      )
      .join(
        "",
      )}</div></details></aside><article class="reading" data-pagefind-body>${eyebrow(p.label + " / Hybrid plan v1.0")}<h1 data-pagefind-meta="title">${escape(p.title)}</h1>${outline ? `<details class="outline" data-pagefind-ignore><summary>On this page <small>${p.sub.length}</small></summary><div>${outline}</div></details>` : ""}<div class="prose">${body}</div><nav class="neighbors" aria-label="Adjacent chapters" data-pagefind-ignore>${neighbors.map((q) => `<a href="${url(q)}"><small>${q.start < p.start ? "← Previous" : "Next →"}</small>${q.label} · ${escape(q.title)}</a>`).join("")}</nav></article></div>`,
  );
}
page("/", "A proving ground for robots", home());
page(
  "/plan/",
  "Plan library",
  `<div class="wrap library">${eyebrow("Research library / Edition 1.0")}<h1>The hybrid <em>plan.</em></h1><p class="lede">From the algebra of a single joint to the experiment that would establish a ten-fold improvement. Read by subject or search across the entire document.</p><div class="actions"><a class="primary" href="/search/">Search the plan <b>↗</b></a><a class="secondary" href="/plan.md" download>Download Markdown ↓</a></div><nav class="part-jumps" aria-label="Jump to part">${groups.map((g) => `<a href="#${groupslug(g)}">${escape(g.split(" · ")[0])}</a>`).join("")}</nav>${groups
    .map(
      (g) =>
        `<section class="shelf" id="${groupslug(g)}"><h2>${escape(g)}</h2><div>${sections
          .filter((p) => p.group === g)
          .map(chapterLink)
          .join("")}</div></section>`,
    )
    .join("")}</div>`,
);
page(
  "/search/",
  "Search the plan",
  `<div class="wrap searchpage">${eyebrow("Plan library")}<h1>Search the <em>plan.</em></h1><form action="/search/" role="search"><label for="query">Search all sections and appendices</label><div><input type="search" id="query" name="q" placeholder="Try “contact”, “mission” or “M7”" maxlength="240" aria-describedby="query-help" autocomplete="off" autofocus><button class="primary">Search</button></div><small id="query-help">Use quotation marks for an exact phrase.</small></form><p id="status" role="status"></p><div id="results"></div><noscript>Full-text search uses JavaScript. <a href="/plan/">Browse the complete chapter index.</a></noscript></div>`,
);
emit(
  "404.html",
  shell(
    "Page not found",
    "/404/",
    `<div class="wrap library">${eyebrow("404")}<h1>Page not found.</h1><a class="primary" href="/plan/">Browse the plan →</a></div>`,
  ),
);
emit("plan.md", bytes);
cpSync(join(root, "static"), out, { recursive: true });
cpSync(join(root, "media"), join(out, "media"), { recursive: true });
for (const [font, file] of [
  ["manrope", "@fontsource-variable/manrope/files/manrope-latin-wght-normal.woff2"],
  ["mono", "@fontsource/ibm-plex-mono/files/ibm-plex-mono-latin-500-normal.woff2"],
]) {
  emit("fonts/" + font + ".woff2", readFileSync(join(root, "node_modules", file)));
  emit(
    "fonts/" + font + "-license.txt",
    readFileSync(join(root, "node_modules", file.split("/files/")[0], "LICENSE")),
  );
}
emit(
  "sitemap.xml",
  '<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">' +
    ["/", "/plan/", "/search/", ...sections.map(url)]
      .map((p) => "<url><loc>" + origin + p + "</loc></url>")
      .join("") +
    "</urlset>",
);
emit("robots.txt", "User-agent: *\nAllow: /\nSitemap: " + origin + "/sitemap.xml\n");
console.log(
  "Published " +
    sections.length +
    " reader entries and " +
    bytes.length +
    " original Markdown bytes.",
);
