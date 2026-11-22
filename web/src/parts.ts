import { escape } from "./layout";

/** Where a section, appendix or construction of the plan is published. Filled in by the build. */
export type Refs = (id: string) => string;

/** What stands behind a figure. Every figure states one. */
export type Basis = "computed" | "toy" | "published" | "illustrative" | "drawing" | "plan";
const BASIS: Record<Basis, [label: string, meaning: string]> = {
  computed: ["Computed here", "The plan's own fixture, worked out in your browser"],
  toy: ["A toy, for intuition", "Runs in your browser; not FrankenRobotics code"],
  published: ["Arithmetic on published figures", "No measurement of this project"],
  illustrative: ["Illustrative numbers", "From a story in the plan; nothing was run"],
  drawing: ["Drawing, no physics", "Kinematics and rendering only"],
  plan: ["From the plan", "Quoted or counted from the plan"],
};

export const icon = (name: keyof typeof ICONS) =>
  `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">${ICONS[name]}</svg>`;
const ICONS = {
  search: '<circle cx="11" cy="11" r="7"/><path d="m20 20-3.6-3.6"/>',
  down: '<path d="M12 5v14M6 13l6 6 6-6"/>',
  right: '<path d="M5 12h14M13 6l6 6-6 6"/>',
  out: '<path d="M8 6h10v10M18 6 6 18"/>',
  pause: '<path d="M9 5v14M15 5v14"/>',
  play: '<path d="M8 5.5v13l10.5-6.5z"/>',
  redo: '<path d="M4 12a8 8 0 1 0 2.6-5.9M4 4v5h5"/>',
  question: '<path d="M12 3v3M12 18v3M3 12h3M18 12h3"/><circle cx="12" cy="12" r="5"/>',
  body: '<path d="M5 9h14v4H5zM7 13l-2 4 1.5 3.5M17 13l2 4-1.5 3.5M11 13l-1 7M14 13l1 7"/>',
  world: '<path d="M4 20V9l8-5 8 5v11M4 20h16M10 20v-6h4v6"/>',
  answer: '<path d="M4 4h16v16H4zM4 12h16M12 4v16"/><path d="m14.6 16.2 1.4 1.4 2.4-3"/>',
  evidence: '<path d="M6 3h9l4 4v14H6zM15 3v4h4M9 12h7M9 16h7"/>',
  test: '<path d="M12 21a8 8 0 1 0 0-16 8 8 0 0 0 0 16zM12 9v4l3 2M9 2h6"/>',
  book: '<path d="M4 5a2 2 0 0 1 2-2h13v15H6a2 2 0 0 0-2 2zM4 19a2 2 0 0 0 2 2h13v-3"/>',
  list: '<path d="M8 6h12M8 12h12M8 18h12M4 6h.01M4 12h.01M4 18h.01"/>',
  left: '<path d="M19 12H5M11 6l-6 6 6 6"/>',
  home: '<path d="m4 11 8-7 8 7v9H4zM10 20v-6h4v6"/>',
  check: '<path d="m5 12.5 4.5 4.5L19 7.5"/>',
  cross: '<path d="M6 6l12 12M18 6 6 18"/>',
} as const;

/** A heading with one word picked out: `A body is *data*.` */
export const title = (s: string) => escape(s).replace(/\*(.+?)\*/g, '<em class="g">$1</em>');

/** Inline copy: `code`, **bold** and [links](url). The text is ours, and is escaped all the same. */
export const copy = (s: string, ref?: Refs) =>
  escape(s)
    .replace(/`(.+?)`/g, "<code>$1</code>")
    .replace(/\*\*(.+?)\*\*/g, "<strong>$1</strong>")
    .replace(/\[(.+?)\]\((.+?)\)/g, (_, text, to) => {
      const href = to.startsWith("§") ? ref!(to.slice(1)) : to;
      return `<a class="txt" href="${href}">${text}</a>`;
    });

export interface Chapter {
  id: string;
  number: string;
  label: string;
  short: string;
  icon: keyof typeof ICONS;
  colour: string;
  title: string;
  thesis: string;
  sections: [id: string, label: string][];
}

export const chapterOpen = (c: Chapter) => `
<div class="chapter" data-chapter="${c.id}" style="--chapter:${c.colour}">
<section class="opener" aria-labelledby="${c.id}-title"><div class="wrap" id="${c.id}"><div class="opener-grid reveal">
<div><p class="kick"><span class="joint on" aria-hidden="true"></span>Chapter ${c.number} · ${escape(c.label)}</p>
<h2 class="d2" id="${c.id}-title">${title(c.title)}</h2><p class="lede">${escape(c.thesis)}</p></div>
<div class="opener-index"><span class="numeral" aria-hidden="true">${c.number}</span><ol>${c.sections
  .map(([id, label], i) => `<li><a href="#${id}"><span class="num">${c.number}.${i + 1}</span>${escape(label)}</a></li>`)
  .join("")}</ol></div></div></div></section>`;
export const chapterClose = "</div>";

export function section(o: {
  id: string;
  index: string;
  eyebrow: string;
  title: string;
  kicker?: string;
  body: string;
  layout?: "wide" | "split";
  ref: Refs;
}) {
  const head = `<p class="kick small"><span class="num">${o.index}</span><i></i>${escape(o.eyebrow)}</p><h3 class="d3" id="${o.id}-title">${title(o.title)}</h3>`;
  const kicker = o.kicker ? `<p class="kicker">${copy(o.kicker, o.ref)}</p>` : "";
  return o.layout === "split"
    ? `<section class="sec split" id="${o.id}" aria-labelledby="${o.id}-title"><div class="wrap split-grid"><div class="sec-head reveal">${head}${kicker}</div><div class="sec-body">${o.body}</div></div></section>`
    : `<section class="sec" id="${o.id}" aria-labelledby="${o.id}-title"><div class="wrap"><div class="sec-head wide reveal"><div>${head}</div>${kicker}</div><div class="sec-body">${o.body}</div></div></section>`;
}

/** A figure, framed like an engineering drawing: its title block names its basis and its source. */
export function figure(o: {
  name: string;
  no: string;
  label: string;
  title: string;
  basis: Basis;
  source: [text: string, href: string][];
  note?: string;
  body: string;
  className?: string;
  ref?: Refs;
}) {
  const [basis, meaning] = BASIS[o.basis];
  return `<figure class="fig reveal ${o.className ?? ""}" id="fig-${o.name}" data-fig="${o.name}">
<header class="fig-head"><div><p class="micro${/\d|[{÷=]/.test(o.label) ? " asis" : ""}">${escape(o.label)}</p><h4>${title(o.title)}</h4></div><span class="basis ${o.basis}">${basis}</span></header>
<div class="fig-body">${o.body}</div>
<figcaption class="block"><span><small>Figure</small>${o.no}</span><span><small>Basis</small>${meaning}</span><span><small>Source</small>${o.source
    .map(([text, href]) => `<a href="${href}">${escape(text)}</a>`)
    .join(", ")}</span>${o.note ? `<span class="grow"><small>Note</small>${copy(o.note, o.ref)}</span>` : ""}</figcaption></figure>`;
}

export const seg = (name: string, label: string, options: [value: string, text: string][], on: string) =>
  `<div class="seg" role="group" aria-label="${escape(label)}" data-seg="${name}">${options
    .map(
      ([v, text]) =>
        `<button type="button" data-v="${v}" aria-pressed="${v === on}">${escape(text)}</button>`,
    )
    .join("")}</div>`;

export const range = (
  name: string,
  label: string,
  min: number,
  max: number,
  step: number,
  value: number,
  shown: string,
) =>
  `<label class="range" data-range="${name}"><span><b>${escape(label)}</b><output>${escape(shown)}</output></span><input type="range" min="${min}" max="${max}" step="${step}" value="${value}" style="--p:${((value - min) / (max - min)) * 100}%"></label>`;

export const toggle = (name: string, label: string, on = false) =>
  `<button type="button" class="toggle" data-toggle="${name}" aria-pressed="${on}"><i></i>${escape(label)}</button>`;

export const read = (name: string, label: string, value: string, note = "") =>
  `<div class="read" data-read="${name}"><small>${escape(label)}</small><b>${escape(value)}</b>${note || note === "" ? `<span>${escape(note)}</span>` : ""}</div>`;

const ONES = "zero one two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen".split(" ");
const TENS = "twenty thirty forty fifty sixty seventy eighty ninety".split(" ");
/** 24 as "twenty-four", so counts taken from the plan can open a heading. */
export function spell(n: number, capital = false) {
  const s = n < 20 ? ONES[n]! : `${TENS[Math.floor(n / 10) - 2]}${n % 10 ? `-${ONES[n % 10]}` : ""}`;
  return capital ? s[0]!.toUpperCase() + s.slice(1) : s;
}
