import { $, fmt, line, loop, onWidth, reduced, rng, seen } from "./lib";

const SIDE = 20;
const CAMPAIGNS = SIDE * SIDE;
const TRIES = 500;
const ALPHA = 0.05;

/** For each campaign, the first submission each rule wrongly promotes, or 0. */
export function simulate(seed: number) {
  const u = rng(seed);
  const naive = new Uint16Array(CAMPAIGNS);
  const spend = new Uint16Array(CAMPAIGNS);
  for (let c = 0; c < CAMPAIGNS; c++)
    for (let i = 1; i <= TRIES; i++) {
      const p = u();
      if (!naive[c] && p <= ALPHA) naive[c] = i;
      if (!spend[c] && p <= ALPHA / (i * (i + 1))) spend[c] = i;
    }
  return { naive, spend };
}

export default function mount(root: HTMLElement) {
  const plot = $(root, "[data-plot]");
  const canvases = { naive: $<HTMLCanvasElement>(root, "[data-naive]"), spend: $<HTMLCanvasElement>(root, "[data-spend]") };
  const run = $<HTMLButtonElement>(root, "[data-run]");
  const label = run.innerHTML;
  let seed = 17;
  let data = simulate(seed);
  let at = 0;
  let going = false;
  let width = 500;
  const hits = (a: Uint16Array, n: number) => {
    let k = 0;
    for (const v of a) if (v && v <= n) k++;
    return k;
  };
  const paint = (which: "naive" | "spend") => {
    const c = canvases[which];
    const g = c.getContext("2d")!;
    g.clearRect(0, 0, 400, 400);
    for (let i = 0; i < CAMPAIGNS; i++) {
      const v = data[which][i]!;
      const bad = v > 0 && v <= at;
      const fresh = bad && at - v < Math.max(3, at * 0.06);
      g.fillStyle = bad ? (fresh ? "#ffd0d6" : "#fb7185") : at ? "#12301f" : "#0d1a14";
      g.fillRect((i % SIDE) * 20 + 1, Math.floor(i / SIDE) * 20 + 1, 18, 18);
    }
    $(root, `[data-${which}-n]`).textContent = `${hits(data[which], at)} of ${CAMPAIGNS}`;
  };
  const chart = () => {
    const W = width;
    const H = 240;
    const L = 44;
    const R = 12;
    const x = (i: number) => L + (Math.log(Math.max(1, i)) / Math.log(TRIES)) * (W - L - R);
    const y = (v: number) => 14 + (1 - v) * (H - 52);
    let svg = "";
    for (const v of [0, 0.5, 1])
      svg += `<path class="grid" d="M${L} ${y(v)}H${W - R}"/><text x="${L - 8}" y="${y(v) + 4}" text-anchor="end">${v * 100}%</text>`;
    for (const i of [1, 10, 100, 500])
      svg += `<path class="grid" d="M${x(i)} ${y(1)}V${y(0)}"/><text x="${x(i)}" y="${H - 18}" text-anchor="middle">${i}</text>`;
    svg += `<text x="${(L + W - R) / 2}" y="${H - 2}" text-anchor="middle">submissions</text>`;
    const theory: [number, number][] = [];
    for (let i = 1; i <= TRIES; i = Math.ceil(i * 1.06)) theory.push([x(i), y(1 - (1 - ALPHA) ** i)]);
    svg += `<path d="${line(theory)}" fill="none" stroke="#fb7185" stroke-opacity=".4" stroke-dasharray="4 5"/>`;
    svg += `<path class="ten" d="M${L} ${y(ALPHA)}H${W - R}"/><text x="${W - R}" y="${y(ALPHA) - 8}" text-anchor="end" style="fill:#fbbf24">5%</text>`;
    const draw = (a: Uint16Array, colour: string) => {
      if (!at) return;
      const pts: [number, number][] = [];
      for (let i = 1; i <= at; i = Math.max(i + 1, Math.floor(i * 1.03))) pts.push([x(i), y(hits(a, i) / CAMPAIGNS)]);
      pts.push([x(at), y(hits(a, at) / CAMPAIGNS)]);
      svg += `<path d="${line(pts)}" fill="none" stroke="${colour}" stroke-width="2.6" stroke-linejoin="round"/><circle cx="${pts.at(-1)![0]}" cy="${pts.at(-1)![1]}" r="5" fill="${colour}"/>`;
    };
    draw(data.naive, "#fb7185");
    draw(data.spend, "#4ade80");
    plot.innerHTML = `<svg class="chart" viewBox="0 0 ${W} ${H}" role="img" aria-label="Share of campaigns that have promoted something worthless, against submissions made">${svg}</svg>`;
  };
  const all = () => {
    paint("naive");
    paint("spend");
    chart();
    $(root, "[data-count]").textContent = `Submission ${fmt(Math.floor(at))} of ${TRIES}`;
  };
  const view = loop(
    canvases.naive,
    (dt) => {
      if (going) {
        // Logarithmic time, because the naive rule is finished by the hundredth try.
        at = Math.min(TRIES, Math.max(at + dt * 6, at * (1 + dt * 1.1)));
        if (at >= TRIES) {
          going = false;
          view.running = false;
          run.innerHTML = label;
        }
      }
      all();
    },
    false,
  );
  const start = () => {
    at = 0;
    if (reduced) {
      at = TRIES;
      return all();
    }
    going = true;
    view.running = true;
    run.textContent = "Submitting";
  };
  run.addEventListener("click", start);
  $(root, "[data-seed]").addEventListener("click", () => {
    data = simulate(++seed);
    start();
  });
  seen(root, () => void (!at && !going && start()));
  onWidth(plot, (w) => {
    width = w;
    all();
  });
}
