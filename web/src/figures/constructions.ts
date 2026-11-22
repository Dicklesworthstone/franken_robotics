import { $, $$, seg } from "./lib";

interface Card {
  replaces: string;
  payoff: string;
  test: string;
  crates: string;
}
const TIER: Record<string, string> = { S: "Solid", F: "Frontier", M: "Moonshot" };

export default function mount(root: HTMLElement) {
  const links = $$<HTMLAnchorElement>(root, "[data-i]");
  const card = $(root, "[data-card]");
  const rest = card.innerHTML;
  let cards: Promise<Card[]> | undefined;
  let current = -1;
  // Nothing is fetched until a reader shows interest in one.
  const load = () =>
    (cards ??= fetch("/constructions.json").then((r) => {
      if (!r.ok) throw Error(String(r.status));
      return r.json() as Promise<Card[]>;
    }));

  const show = async (i: number) => {
    current = i;
    for (const [j, a] of links.entries()) {
      if (j === i) a.setAttribute("aria-current", "true");
      else a.removeAttribute("aria-current");
    }
    const a = links[i]!;
    const tiers = a.closest<HTMLElement>("li")!.dataset.tiers!;
    let c: Card;
    try {
      c = (await load())[i]!;
    } catch {
      cards = undefined;
      location.href = a.href;
      return;
    }
    if (current !== i) return;
    const row = (name: string, html: string) => (html ? `<div><p class="micro">${name}</p><p>${html}</p></div>` : "");
    card.innerHTML = `<p class="kick small"><span class="num">C-${$(a, ".num").textContent}</span><i></i>${[...tiers].map((t) => TIER[t]).join(" and ")}</p><h4>${$(a, "b").innerHTML}</h4>${row("Replaces", c.replaces)}${row("Payoff", c.payoff)}${row("Test", c.test)}<p class="where"><span class="num">${c.crates}</span><a class="txt" href="${a.href}">Read it in full</a></p>`;
    card.scrollTop = 0;
    card.classList.add("open");
    if (innerWidth < 1000) card.scrollIntoView({ block: "nearest" });
  };
  root.addEventListener("click", (e) => {
    const a = (e.target as Element).closest<HTMLAnchorElement>("[data-i]");
    if (!a || e.metaKey || e.ctrlKey || e.shiftKey || e.button) return;
    e.preventDefault();
    void show(Number(a.dataset.i));
  });
  root.addEventListener("pointerover", (e) => {
    if ((e.target as Element).closest("[data-i]")) void load().catch(() => (cards = undefined));
  });
  root.addEventListener("keydown", (e) => {
    const from = (e.target as Element).closest<HTMLAnchorElement>("[data-i]");
    const d = { ArrowRight: 1, ArrowLeft: -1 }[e.key];
    if (!from || !d) return;
    let j = Number(from.dataset.i) + d;
    while (links[j]?.closest("li")?.classList.contains("dim")) j += d;
    if (!links[j]) return;
    e.preventDefault();
    links[j]!.focus();
    void show(j);
  });
  // One is open from the start, so that the panel shows what it is for.
  if (innerWidth >= 1000) void show(32);
  seg(root, "tier", (t) => {
    for (const a of links) {
      const li = a.closest<HTMLElement>("li")!;
      li.classList.toggle("dim", !!t && !li.dataset.tiers!.includes(t));
    }
    if (current >= 0 && links[current]!.closest("li")!.classList.contains("dim")) {
      links[current]!.removeAttribute("aria-current");
      card.innerHTML = rest;
      current = -1;
    }
  });
}
