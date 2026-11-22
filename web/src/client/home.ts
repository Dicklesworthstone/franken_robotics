/** The home page's behaviour: entrances, the chapter marker, and figures that load as they come near. */

const figures: Record<string, () => Promise<{ default: (root: HTMLElement) => void }>> = {
  lineup: () => import("../figures/lineup"),
  stories: () => import("../figures/stories"),
  film: () => import("../figures/film"),
  stage: () => import("../figures/stage"),
  anatomy: () => import("../figures/anatomy"),
  preflight: () => import("../figures/preflight"),
  symmetry: () => import("../figures/symmetry"),
  batch: () => import("../figures/batch"),
  rooms: () => import("../figures/rooms"),
  corner: () => import("../figures/corner"),
  lip: () => import("../figures/lip"),
  region: () => import("../figures/region"),
  stance: () => import("../figures/stance"),
  docking: () => import("../figures/docking"),
  session: () => import("../figures/session"),
  shrink: () => import("../figures/shrink"),
  autopsy: () => import("../figures/autopsy"),
  retries: () => import("../figures/retries"),
  bench: () => import("../figures/bench"),
  constructions: () => import("../figures/constructions"),
};

async function mount(el: HTMLElement) {
  const load = figures[el.dataset.fig!];
  if (!load) return;
  try {
    (await load()).default(el);
    el.dataset.ready = "";
  } catch (error) {
    console.error(`Figure ${el.dataset.fig} did not start`, error);
    const note = document.createElement("p");
    note.className = "noscript";
    note.textContent = "This didn’t load. Reloading the page usually brings it back.";
    el.append(note);
  }
}

const near = new IntersectionObserver(
  (entries) => {
    for (const e of entries) {
      if (!e.isIntersecting) continue;
      near.unobserve(e.target);
      void mount(e.target as HTMLElement);
    }
  },
  { rootMargin: "420px 0px" },
);
for (const el of document.querySelectorAll<HTMLElement>("[data-fig]")) near.observe(el);

const shown = new IntersectionObserver(
  (entries) => {
    for (const e of entries) {
      if (!e.isIntersecting) continue;
      shown.unobserve(e.target);
      e.target.classList.add("seen");
    }
  },
  { rootMargin: "0px 0px -6% 0px", threshold: 0.02 },
);
for (const el of document.querySelectorAll(".reveal, .fig")) shown.observe(el);

// Marks the chapter being read, in the header and in the dock.
const spies = [...document.querySelectorAll<HTMLElement>("[data-spy]")];
const chapters = [...document.querySelectorAll<HTMLElement>("[data-chapter]")];
let current = "";
let queued = 0;
const spy = () => {
  queued = 0;
  const line = innerHeight * 0.42;
  const hit = chapters.find((c) => {
    const box = c.getBoundingClientRect();
    return box.top <= line && box.bottom > line;
  });
  const id = hit?.dataset.chapter ?? "";
  if (id === current) return;
  current = id;
  for (const a of spies) {
    if (a.dataset.spy === id) a.setAttribute("aria-current", "true");
    else a.removeAttribute("aria-current");
  }
};
addEventListener("scroll", () => (queued ||= requestAnimationFrame(spy)), { passive: true });
spy();
