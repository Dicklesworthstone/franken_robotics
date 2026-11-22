import { $, $$, loop } from "./lib";

export default function mount(root: HTMLElement) {
  const plate = $(root, ".plate");
  const parts = $$<SVGGElement>(root, "g[data-part]");
  const items = $$(root, "[data-organ]");
  const now = $(root, "[data-now]");
  const tell = (item: HTMLElement) => {
    const name = $(item, "b").textContent!;
    const a = $<HTMLAnchorElement>(item, "a");
    now.innerHTML = `<p><b>${name}</b> ${$(item, "p").innerHTML}</p><a class="txt" href="${a.getAttribute("href")}">Read more</a>`;
  };
  tell(items[0]!);
  const show = (key?: string) => {
    const item = items.find((i) => i.dataset.organ === key);
    if (item) tell(item);
    plate.classList.toggle("focus", !!key);
    for (const p of parts) p.classList.toggle("on", p.dataset.part === key);
    for (const i of items) i.classList.toggle("on", i.dataset.organ === key);
  };
  // Until someone points at a part, it goes through them one by one.
  let clock = 0;
  let last = -1;
  const tour = loop(root, (dt) => {
    clock += dt;
    const i = Math.floor(clock / 2.6) % (items.length + 1);
    if (i === last) return;
    last = i;
    show(items[i]?.dataset.organ);
  });
  for (const type of ["pointerenter", "focusin", "pointerdown"] as const)
    root.addEventListener(
      type,
      () => {
        if (!tour.running) return;
        tour.running = false;
        show();
      },
      { once: true },
    );
  for (const item of items) {
    item.addEventListener("pointerenter", () => show(item.dataset.organ));
    item.addEventListener("focusin", () => show(item.dataset.organ));
    item.addEventListener("focusout", () => show());
    item.addEventListener("pointerleave", (e) => {
      if (e.pointerType === "mouse") show();
    });
    // Where the card stands in for the list's text, a tap chooses a part and the card carries its link.
    item.addEventListener("click", (e) => {
      if (getComputedStyle(now).display === "none" || e.metaKey || e.ctrlKey || e.shiftKey) return;
      e.preventDefault();
      show(item.dataset.organ);
    });
  }
  for (const part of parts) {
    // A wide invisible stroke makes the thin parts easy to point at.
    part.addEventListener("pointerenter", () => show(part.dataset.part));
    part.addEventListener("pointerleave", () => show());
    part.addEventListener("click", () => show(part.dataset.part));
  }
}
