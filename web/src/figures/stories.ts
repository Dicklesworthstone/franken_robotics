import { $, $$, reduced } from "./lib";

export default function mount(root: HTMLElement) {
  const links = $$<HTMLAnchorElement>(root, "[data-story]");
  const panels = $$(root, "[data-panel]");
  const items = links.map((a) => a.closest<HTMLElement>("li")!);
  const list = $(root, ".story-list");
  const at = $(root, "[data-at]");
  const prev = $<HTMLButtonElement>(root, "[data-prev]");
  const next = $<HTMLButtonElement>(root, "[data-next]");
  let current = 0;

  const show = (i: number, follow = true) => {
    current = i;
    for (const [j, p] of panels.entries()) p.classList.toggle("on", j === i);
    for (const [j, a] of links.entries()) {
      if (j === i) a.setAttribute("aria-current", "true");
      else a.removeAttribute("aria-current");
    }
    at.textContent = `${i + 1} of ${links.length}`;
    prev.disabled = i === 0;
    next.disabled = i === links.length - 1;
    if (!follow) return;
    const behavior = reduced ? "auto" : "smooth";
    const item = items[i]!;
    // Scrolls the list only; the page stays where the reader left it.
    if (list.scrollWidth > list.clientWidth + 4) list.scrollTo({ left: item.offsetLeft - list.clientWidth / 2 + item.clientWidth / 2, behavior });
    else list.scrollTo({ top: item.offsetTop - list.clientHeight / 2 + item.clientHeight / 2, behavior });
  };
  list.addEventListener("click", (e) => {
    const a = (e.target as Element).closest<HTMLAnchorElement>("[data-story]");
    if (!a || e.metaKey || e.ctrlKey || e.shiftKey || e.button) return;
    e.preventDefault();
    show(Number(a.dataset.story));
  });
  const step = (d: number) => {
    const to = current + d;
    if (!links[to]) return;
    show(to);
    return to;
  };
  prev.addEventListener("click", () => step(-1));
  next.addEventListener("click", () => step(1));
  list.addEventListener("keydown", (e) => {
    const d = { ArrowDown: 1, ArrowRight: 1, ArrowUp: -1, ArrowLeft: -1 }[e.key];
    if (!d) return;
    e.preventDefault();
    const to = step(d);
    if (to !== undefined) links[to]!.focus({ preventScroll: true });
  });
  show(0, false);
}
