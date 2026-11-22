import { $, range } from "./lib";

const N = 200;
const LEVEL = 0.05 / 2;

const logs = [0];
for (let i = 1; i <= 2 * N; i++) logs.push(logs[i - 1]! + Math.log(i));
const choose = (n: number, k: number) => logs[n]! - logs[k]! - logs[n - k]!;
/** One-sided Fisher exact test that a of N rescues exceeds p of N placebo rescues. */
export function fisher(a: number, p: number) {
  const m = a + p;
  let sum = 0;
  for (let k = a; k <= Math.min(N, m); k++)
    if (m - k <= N) sum += Math.exp(choose(N, k) + choose(N, m - k) - choose(2 * N, m));
  return Math.min(1, sum);
}

export function label(n: number, placebo: number, flip: number) {
  const rho = placebo / N;
  const contrast = n / N - rho;
  const above = fisher(n, placebo) <= LEVEL && contrast > 0;
  if (rho > 0.2) return { contrast, tag: "chaotic: unattributable", tone: "open" };
  if (!above) return { contrast, tag: "not named", tone: "none" };
  if (contrast >= 0.5) return flip <= 0.1 ? { contrast, tag: "but-for", tone: "in" } : { contrast, tag: "flip rate too high for but-for", tone: "open" };
  return { contrast, tag: "contributing", tone: "open" };
}

export default function mount(root: HTMLElement) {
  const COLS = 40;
  const ROWS = 5;
  const grid = (count: number, rho: number, colour: string) => {
    let svg = "";
    // The first dots are what a placebo would have rescued anyway.
    for (let i = 0; i < N; i++) {
      const x = (i % COLS) * 10 + 5;
      const y = Math.floor(i / COLS) * 10 + 5;
      const on = i < count;
      const noise = i < rho;
      svg += on
        ? `<circle cx="${x}" cy="${y}" r="3.7" fill="${noise ? "#7c9086" : colour}"/>`
        : `<circle cx="${x}" cy="${y}" r="3" fill="none" stroke="rgb(190 240 212 / .22)"/>`;
    }
    return `<svg class="chart" viewBox="0 0 ${COLS * 10} ${ROWS * 10}" aria-hidden="true">${svg}</svg>`;
  };
  const draw = () => {
    const rho = placebo.value;
    const row = (name: string, count: number, colour: string) =>
      `<div><p><span>${name}</span><span class="num">${count} of ${N}</span></p>${grid(count, rho, colour)}</div>`;
    $(root, "[data-dots]").innerHTML =
      row("do(torque_limit := off)", torque.value, "#4ade80") +
      row("do(friction := nominal)", friction.value, "#6cc8f7") +
      row("placebo", rho, "#7c9086");
    const t = label(torque.value, rho, flip.value);
    const f = label(friction.value, rho, flip.value);
    if (t.tag === "but-for" && f.tag === "but-for") t.tag = f.tag = "alternative";
    const line = (name: string, v: ReturnType<typeof label>) =>
      `<div class="read" data-tone="${v.tone}"><small>${name}</small><b>${v.contrast.toFixed(2).replace("-", "−")}</b><span>${v.tag}</span></div>`;
    $(root, "[data-out]").innerHTML =
      line("Torque limit, net of placebo", t) +
      line("Friction, net of placebo", f) +
      `<p class="say">${
        rho / N > 0.2
          ? "The placebo alone changes the outcome in more than a fifth of the forks. The report names nobody."
          : t.tag === "alternative"
            ? "Either change alone would have saved it, so neither is the cause."
            : t.tag === "but-for" || f.tag === "but-for"
              ? "One factor is named. The other is listed beside it with its own number."
              : "No single change is enough to name a cause by itself."
      }</p>`;
  };
  const count = (v: number) => `${v}`;
  const torque = range(root, "torque", count, draw);
  const friction = range(root, "friction", count, draw);
  const placebo = range(root, "placebo", count, draw);
  const flip = range(root, "flip", (v) => v.toFixed(2), draw);
  draw();
}
