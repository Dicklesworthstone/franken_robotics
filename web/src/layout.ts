export const origin = "https://frankenrobotics.com";
export const repo = "https://github.com/Dicklesworthstone/franken_robotics";
export const escape = (s: string) =>
  s
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
export function shell(
  title: string,
  path: string,
  body: string,
  description = "Explore the FrankenRobotics research plan for physics-based robot design, mission queries, and verifiable simulation.",
) {
  const nav = (url: string, label: string) =>
    `<a href="${url}"${path === url ? ' aria-current="page"' : ""}>${label}</a>`;
  return `<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><meta name="theme-color" content="#090e0c"><title>${escape(title)} · FrankenRobotics</title><meta name="description" content="${escape(description)}"><link rel="canonical" href="${origin + path}"><meta property="og:type" content="website"><meta property="og:title" content="${escape(title)} · FrankenRobotics"><meta property="og:description" content="${escape(description)}"><meta property="og:url" content="${origin + path}"><meta property="og:image" content="${origin}/media/quadruped.webp"><meta name="twitter:card" content="summary_large_image"><link rel="icon" href="/icon.svg"><link rel="stylesheet" href="/site.css"><link rel="preload" href="/fonts/manrope.woff2" as="font" type="font/woff2" crossorigin><script type="module" src="/site.js"></script></head>
<body><a class="skip" href="#main">Skip to content</a><header class="top"><div class="wrap bar"><a href="/" class="brand"><img src="/icon.svg" alt="" width="33" height="33"><span>Franken<strong>Robotics</strong></span></a><nav aria-label="Main navigation">${nav("/", "Overview")}${nav("/#field-guide", "Field guide")}${nav("/plan/", "Plan library")}</nav><a class="searchlink" href="/search/" aria-label="Search the plan"><span class="lens"></span><span>Search</span><kbd>/</kbd></a></div></header>
<main id="main">${body}</main><footer><div class="wrap foot"><a href="/" class="brand"><img src="/icon.svg" width="25" height="25" alt="">FrankenRobotics</a><span>Research plan · Edition 1.0</span><a href="/plan/">Plan library</a><a href="${repo}">GitHub ↗</a><a href="/plan.md" download>Markdown ↓</a></div></footer></body></html>`;
}
