import { icon } from "./parts";

export const origin = "https://frankenrobotics.com";
/** Where this build is served. Link previews are fetched from here; the canonical address is always `origin`. */
export const host = process.env.SITE_ORIGIN?.replace(/\/$/, "") || origin;
export const repo = "https://github.com/Dicklesworthstone/franken_robotics";
export const escape = (s: string) =>
  s
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");

/** The walking mark. site.js turns its legs as the page scrolls; without scripts it stands. */
export const mark = `<svg class="mark" viewBox="0 0 42 34" aria-hidden="true"><path class="leg far" data-leg="1" d="M13 14 9 21l3 8"/><path class="leg far" data-leg="2" d="M31 14l-4 7 3 8"/><rect class="body" x="8" y="7" width="27" height="9" rx="4"/><path class="body" d="M31 8.5 33.5 4h5.2A1.8 1.8 0 0 1 40.500 5.800V9a3 3 0 0 1-3 3H33z"/><circle class="eye" cx="36.800" cy="7.600" r="1.300"/><path class="leg" data-leg="0" d="M13 14 9 21l3 8"/><path class="leg" data-leg="3" d="M31 14l-4 7 3 8"/></svg>`;

export interface Shell {
  title: string;
  path: string;
  body: string;
  description?: string;
  /** The middle of the header: the chapters of the home page, or the site's sections. */
  nav?: string;
  dock?: string;
  head?: string;
  solid?: boolean;
  /** The page's share card, with its content hash, and what it shows. */
  share: { src: string; wide?: string; alt: string };
  version: (file: string) => string;
}

const site = (path: string) =>
  [
    ["/", "Overview"],
    ["/plan/", "The plan"],
    ["/search/", "Search"],
  ]
    .map(
      ([to, label]) =>
        `<a href="${to}"${path === to || (to === "/plan/" && path.startsWith("/plan/")) ? ' aria-current="page"' : ""}><span class="joint" aria-hidden="true"></span>${label}</a>`,
    )
    .join("");

const siteDock = (path: string) =>
  (
    [
      ["/", "home", "Overview"],
      ["/plan/", "book", "The plan"],
      ["/search/", "search", "Search"],
    ] as const
  )
    .map(
      ([to, name, label]) =>
        `<li><a href="${to}"${path === to ? ' aria-current="page"' : ""}>${icon(name)}${label}</a></li>`,
    )
    .join("");

export function shell(o: Shell) {
  const description =
    o.description ??
    "A robotics lab for AI agents. Describe a mission and get every robot that can do it, why the others fail, and a build packet for the one you pick.";
  const full = o.path === "/" ? o.title : `${o.title} · FrankenRobotics`;
  return `<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover"><meta name="theme-color" content="#030705"><title>${escape(full)}</title><meta name="description" content="${escape(description)}"><link rel="canonical" href="${origin + o.path}"><meta property="og:type" content="website"><meta property="og:site_name" content="FrankenRobotics"><meta property="og:title" content="${escape(full)}"><meta property="og:description" content="${escape(description)}"><meta property="og:url" content="${host + o.path}"><meta property="og:image" content="${host + o.share.src}"><meta property="og:image:type" content="image/jpeg"><meta property="og:image:width" content="1200"><meta property="og:image:height" content="630"><meta property="og:image:alt" content="${escape(o.share.alt)}"><meta name="twitter:card" content="summary_large_image"><meta name="twitter:title" content="${escape(full)}"><meta name="twitter:description" content="${escape(description)}"><meta name="twitter:image" content="${host + (o.share.wide ?? o.share.src)}"><meta name="twitter:image:alt" content="${escape(o.share.alt)}"><link rel="icon" href="/icon.svg" type="image/svg+xml"><link rel="preload" href="/fonts/inter-latin.woff2" as="font" type="font/woff2" crossorigin><link rel="stylesheet" href="${o.version("/site.css")}">${o.head ?? ""}<script type="module" src="${o.version("/site.js")}"></script></head>
<body${o.solid ? ' class="solid"' : ""}><a class="skip" href="#main">Skip to content</a><header class="top"><div class="bar"><a href="/" class="brand">${mark}FrankenRobotics</a><nav class="chapters" aria-label="${o.nav ? "Chapters" : "Site"}">${o.nav ?? site(o.path)}</nav><div class="tools"><a class="find" href="/search/" data-palette aria-label="Search the plan">${icon("search")}<span>Search</span><kbd>/</kbd></a><a class="btn go sm" href="${repo}">GitHub${icon("out")}</a></div><span class="progress" aria-hidden="true"><i></i></span></div></header>
<main id="main">${o.body}</main><footer class="end"><div class="wrap"><a href="/" class="brand">${mark}FrankenRobotics</a><span>In design, in the open.</span><a href="/plan/">The plan</a><a href="/plan.md" download>Markdown</a><a href="${repo}">GitHub</a></div></footer><nav class="dock" aria-label="Quick navigation"><ul>${o.dock ?? siteDock(o.path)}</ul></nav></body></html>`;
}
