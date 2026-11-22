import { escape } from "./layout";
import { icon, title as heading } from "./parts";
import { entries, groups, groupSlug, ref, version, words, type Entry } from "./plan";

const n = (v: number) => v.toLocaleString("en-US");
const short = (g: string) => g.split(" · ")[0]!;

/** Subsections for the contents lists. Long chapters list their top level only. */
function outline(entry: Entry) {
  const top = entry.sub.filter((s) => s.level === 2);
  const two = entry.sub.filter((s) => s.level <= 3);
  return two.length <= 60 ? two : top;
}
const contents = (entry: Entry) =>
  outline(entry)
    .map((s) => `<a href="#${s.id}"${s.level === 3 ? ' class="h3"' : ""}>${escape(s.title)}</a>`)
    .join("");

export function reader(entry: Entry, html: string) {
  const i = entries.indexOf(entry);
  const before = entries[i - 1];
  const after = entries[i + 1];
  const part = entries.filter((e) => e.group === entry.group);
  const facts = [
    [entry.words, "words", "word"],
    [entry.sub.length, "subsections", "subsection"],
    [entry.tables, "tables", "table"],
    [entry.figures, "listings", "listing"],
  ] as const;
  const toc = contents(entry);
  const name = `${entry.label} · ${entry.title}`;
  return `<div class="reader"><aside class="rail part" data-pagefind-ignore><p class="micro">${escape(entry.group)}</p><nav aria-label="Sections in this part">${part
    .map(
      (e) =>
        `<a href="${e.url}"${e === entry ? ' aria-current="page"' : ""}><b>${escape(e.label.replace("Appendix ", ""))}</b>${escape(e.title)}</a>`,
    )
    .join("")}</nav></aside>
<article class="reading" data-pagefind-body><header><nav class="crumbs" aria-label="Breadcrumb" data-pagefind-ignore><a href="/plan/">Plan library</a><span aria-hidden="true">/</span><a href="/plan/#${groupSlug(entry.group)}">${escape(short(entry.group))}</a><span aria-hidden="true">/</span><span>${escape(entry.label)}</span></nav><h1 data-pagefind-meta="title">${escape(entry.title)}</h1><p class="facts" data-pagefind-ignore>${facts
    .filter(([v]) => v > 0)
    .map(([v, many, one]) => `<span><b>${n(v)}</b> ${v === 1 ? one : many}</span>`)
    .join("")}<span>Plan version ${version}</span></p></header>${
    toc
      ? `<details class="inline-toc" id="contents" data-pagefind-ignore><summary>On this page<small>${outline(entry).length}</small></summary><div>${toc}</div></details>`
      : `<span id="contents"></span>`
  }<div class="prose">${html}</div><nav class="neighbors" aria-label="Previous and next" data-pagefind-ignore>${
    before ? `<a class="prev" href="${before.url}" rel="prev"><small>Previous · ${escape(before.label)}</small>${escape(before.title)}</a>` : ""
  }${
    after ? `<a class="next" href="${after.url}" rel="next"><small>Next · ${escape(after.label)}</small>${escape(after.title)}</a>` : ""
  }</nav></article>${
    toc
      ? `<aside class="rail toc" data-pagefind-ignore><p class="micro">On this page</p><nav aria-label="On this page: ${escape(name)}">${toc}</nav></aside>`
      : ""
  }</div>`;
}

export function readerDock(entry: Entry) {
  const i = entries.indexOf(entry);
  const before = entries[i - 1];
  const after = entries[i + 1];
  const step = (e: Entry | undefined, glyph: "left" | "right", label: string) =>
    e
      ? `<li><a href="${e.url}" aria-label="${label}: ${escape(`${e.label} ${e.title}`)}">${icon(glyph)}${label}</a></li>`
      : `<li><a href="/plan/">${icon("book")}Library</a></li>`;
  return `<li><a href="#contents" data-sheet>${icon("list")}Contents</a></li><li><a href="/search/" data-palette>${icon("search")}Search</a></li>${step(before, "left", "Previous")}${step(after, "right", "Next")}`;
}

const paths: [name: string, text: string, steps: [label: string, to: string][]][] = [
  [
    "New here",
    "What it is, what it’s built to do, and the scenarios that drive it.",
    [["Overview", "1"], ["Principles", "3"], ["Goals", "6"], ["Scenarios", "7"]],
  ],
  [
    "Robot design",
    "Bodies, motors, buildings, missions and the workshop.",
    [["Skeleton", "14"], ["Sinew", "15"], ["Habitat", "19"], ["Augur", "25"], ["Foundry", "31"]],
  ],
  [
    "Simulation",
    "Dynamics, contact, GPU kernels and performance.",
    [["Kinesis", "16"], ["Phylum", "17"], ["Traction", "18"], ["Spark", "21"], ["Speed", "36"]],
  ],
  [
    "Agents",
    "Commands, budgets, control and the sealed exam.",
    [["Reflex", "22"], ["Arena", "24"], ["Atelier", "28"], ["Tether", "32"], ["Safety", "37"]],
  ],
  [
    "Testing",
    "Adversarial worlds, failure analysis, replay and the test program.",
    [["Nemesis", "26"], ["Autopsy", "27"], ["Chronicle", "29"], ["Gauntlet", "38"]],
  ],
  [
    "The math",
    "Forty constructions, the algebra core, proofs and worked answers.",
    [["Constructions", "10"], ["Marrow", "13"], ["Lemma", "12"], ["Compendium", "appendix-d"], ["Answers", "appendix-m"]],
  ],
];

export function library() {
  return `<section class="lib-head"><div class="lattice" aria-hidden="true"></div><span class="glow" style="width:40rem;height:40rem;left:-12rem;top:-14rem;background:#1d7a48" aria-hidden="true"></span><div class="wrap"><p class="kick"><span class="joint on" aria-hidden="true"></span>Version ${version}</p><h1 class="d2">${heading("The *full design*.")}</h1><p class="lede">Everything behind FrankenRobotics in ${entries.length - 1} sections and appendices, from the algebra of a single joint to the packet that goes to the workshop.</p><div class="cta"><a class="btn go" href="/search/" data-palette>${icon("search")}Search the text</a><a class="btn" href="/plan.md" download>Download Markdown${icon("down")}</a></div></div></section>
<div class="wrap"><h2 class="sr">Where to start</h2><div class="paths">${paths
    .map(
      ([name, text, steps]) =>
        `<div class="path"><h3>${name}</h3><p>${text}</p><p class="steps">${steps.map(([l, to]) => `<a class="tag" href="${ref(to)}">${l}</a>`).join("")}</p></div>`,
    )
    .join("")}</div>
<label class="filter" data-filter hidden>${icon("search")}<span class="sr">Filter sections by title</span><input type="search" placeholder="Filter by title or topic" autocomplete="off" spellcheck="false"><output aria-live="polite">${entries.length}</output></label>
<nav class="jump" aria-label="Parts">${groups.map((g) => `<a class="tag" href="#${groupSlug(g)}">${escape(short(g))}</a>`).join("")}</nav>${groups
    .map((g) => {
      const list = entries.filter((e) => e.group === g);
      const [part, name] = g.split(" · ");
      return `<section class="shelf" id="${groupSlug(g)}"><header><h2>${escape(name ?? part!)}</h2><span>${name ? `${escape(part!)} · ` : ""}${list.length} ${list.length === 1 ? "entry" : "entries"}</span></header><div class="cards">${list
        .map(
          (e) =>
            `<a class="card" href="${e.url}"><span class="row"><b>${escape(e.label)}</b><span>${n(e.words)} words</span></span><strong>${escape(e.title)}</strong><p>${escape(e.summary)}</p></a>`,
        )
        .join("")}</div></section>`;
    })
    .join("")}</div>`;
}

export const search = `<div class="wrap searchpage"><h1 class="d2">${heading("Search *everything*.")}</h1><form action="/search/" role="search"><div class="q">${icon("search")}<label class="sr" for="query">Search all sections and appendices</label><input type="search" id="query" name="q" placeholder="contact, mission, kitchen, M7" maxlength="240" aria-describedby="query-help" autocomplete="off" spellcheck="false" autofocus></div><small id="query-help">Put quotation marks around an exact phrase.</small></form><p id="status" role="status"></p><div id="results"></div><noscript><p class="lede">Full-text search needs JavaScript. <a class="txt" href="/plan/">Browse the library instead.</a></p></noscript></div>`;

export const missing = `<div class="wrap simple"><p class="kick"><span class="joint" aria-hidden="true"></span>404</p><h1 class="d2">${heading("No such *page*.")}</h1><p class="lede">This page may have moved. Search for it, or browse the full design.</p><div class="cta"><a class="btn go" href="/plan/">Open the library</a><a class="btn" href="/search/" data-palette>${icon("search")}Search</a></div></div>`;
