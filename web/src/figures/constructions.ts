import { $, $$, reduced, seg } from "./lib";

const TIER: Record<string, string> = { S: "Core", F: "Frontier", M: "Moonshot" };

export default function mount(root: HTMLElement) {
  const links = $$<HTMLAnchorElement>(root, "[data-i]");
  const card = $(root, "[data-card]");
  const rest = card.innerHTML;
  let current = -1;
  const home = card.parentElement!;
  const holder = document.createElement("li");
  holder.className = "inline";
  const close = () => {
    links[current]?.removeAttribute("aria-current");
    card.innerHTML = rest;
    card.classList.remove("open");
    home.append(card);
    holder.remove();
    current = -1;
  };
  const show = (i: number) => {
    current = i;
    for (const [j, a] of links.entries()) {
      if (j === i) a.setAttribute("aria-current", "true");
      else a.removeAttribute("aria-current");
    }
    const a = links[i]!;
    const li = a.closest<HTMLElement>("li")!;
    card.innerHTML = `<p class="kick small"><span class="num">${$(a, ".num").textContent}</span><i></i>${[...li.dataset.tiers!].map((t) => TIER[t]).join(" and ")}</p><h4>${$(a, "b").innerHTML}</h4><p class="good">${li.dataset.good}</p><a class="more" href="${a.href}">Read the math</a>`;
    card.scrollTop = 0;
    card.classList.add("open");
    if (innerWidth < 1000) {
      a.closest("li")!.after(holder);
      holder.append(card);
      card.scrollIntoView({ block: "nearest", behavior: reduced ? "auto" : "smooth" });
    } else if (card.parentElement !== home) {
      home.append(card);
      holder.remove();
    }
  };
  root.addEventListener("click", (e) => {
    const a = (e.target as Element).closest<HTMLAnchorElement>("[data-i]");
    if (!a || e.metaKey || e.ctrlKey || e.shiftKey || e.button) return;
    e.preventDefault();
    // A second tap on the open tile closes it.
    if (innerWidth < 1000 && Number(a.dataset.i) === current) return close();
    show(Number(a.dataset.i));
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
    show(j);
  });
  // One is open from the start, so that the panel shows what it is for.
  if (innerWidth >= 1000) show(32);
  seg(root, "tier", (t) => {
    for (const a of links) {
      const li = a.closest<HTMLElement>("li")!;
      li.classList.toggle("dim", !!t && !li.dataset.tiers!.includes(t));
    }
    if (current >= 0 && links[current]!.closest("li")!.classList.contains("dim")) {
      close();
    }
  });
}
