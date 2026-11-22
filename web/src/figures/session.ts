import { $, $$, reduced, seen } from "./lib";

export default function mount(root: HTMLElement) {
  const lines = $$(root, "[data-step]");
  const buttons = $$<HTMLButtonElement>(root, ".session-steps button");
  const term = $(root, "[data-term]");
  const gists = JSON.parse($<HTMLTemplateElement>(root, "[data-gists]").content.textContent!) as [string, string][];
  const play = $<HTMLButtonElement>(root, "[data-play]");
  const label = play.innerHTML;
  const behavior: ScrollBehavior = reduced ? "auto" : "smooth";
  let i = 0;
  let timer = 0;
  const show = (n: number, how: ScrollBehavior = behavior) => {
    i = Math.max(0, Math.min(lines.length - 1, n));
    lines.forEach((l, j) => {
      l.classList.toggle("later", j > i);
      l.classList.toggle("now", j === i);
    });
    buttons.forEach((b, j) => b.setAttribute("aria-pressed", String(j === i)));
    const list = buttons[i]!.closest("ol")!;
    if (list.scrollWidth > list.clientWidth + 4)
      list.scrollTo({ left: buttons[i]!.offsetLeft - list.clientWidth / 2 + 70, behavior: how });
    term.scrollTo({ top: term.scrollHeight, behavior: how });
    $(root, "[data-n]").textContent = `Step ${i + 1} of ${lines.length}`;
    $(root, "[data-t]").textContent = gists[i]![0];
    $(root, "[data-d]").textContent = gists[i]![1];
    $<HTMLButtonElement>(root, "[data-prev]").disabled = i === 0;
    $<HTMLButtonElement>(root, "[data-next]").disabled = i === lines.length - 1;
  };
  const stop = () => {
    clearInterval(timer);
    timer = 0;
    play.innerHTML = label;
  };
  const go = (n: number) => {
    stop();
    show(n);
  };
  buttons.forEach((b, j) => b.addEventListener("click", () => go(j)));
  $(root, "[data-prev]").addEventListener("click", () => go(i - 1));
  $(root, "[data-next]").addEventListener("click", () => go(i + 1));
  const start = () => {
    if (i === lines.length - 1) show(0);
    play.textContent = "Stop";
    timer = window.setInterval(() => {
      if (i >= lines.length - 1) stop();
      else show(i + 1);
    }, 2400);
  };
  play.addEventListener("click", () => (timer ? stop() : start()));
  show(0, "auto");
  // It plays by itself the first time it is seen, until someone takes over, and rests while it is off screen.
  if (!reduced) seen(root, () => void (!timer && i === 0 && start()));
  new IntersectionObserver((list) => void (!list.at(-1)!.isIntersecting && stop())).observe(root);
}
