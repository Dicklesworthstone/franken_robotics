import { $, clamp, fmt, line, onWidth, range, seg } from "./lib";

interface Curve {
  name: string;
  colour: string;
  f: (x: number) => number;
}
interface Tab {
  text: string;
  label: string;
  /** The slider runs 0 to 100; this turns it into the quantity on the axis. */
  at: (p: number) => number;
  shown: (x: number) => string;
  start: number;
  lo: number;
  hi: number;
  log: boolean;
  ticks: [number, string][];
  top: number;
  curves: Curve[];
  reads: (x: number) => [label: string, value: string, note: string][];
}

const pow = (p: number, hi: number) => hi ** (p / 100);
const tabs: Record<string, Tab> = {
  engine: {
    text: "The environment step is 87% of training for a rough-terrain humanoid, 85% for a dexterous hand and 46% for a cart-pole. However fast the step becomes, the other share of training stays.",
    label: "Engine speed-up",
    at: (p) => pow(p, 1000),
    shown: (x) => `× ${fmt(x, x < 10 ? 1 : 0)}`,
    start: 34,
    lo: 1,
    hi: 1000,
    log: true,
    ticks: [
      [1, "1×"],
      [10, "10×"],
      [100, "100×"],
      [1000, "1,000×"],
    ],
    top: 12,
    curves: [
      { name: "humanoid, 87%", colour: "#fbbf24", f: (x) => 1 / (0.13 + 0.87 / x) },
      { name: "hand, 85%", colour: "#fb923c", f: (x) => 1 / (0.15 + 0.85 / x) },
      { name: "cart-pole, 46%", colour: "#6cc8f7", f: (x) => 1 / (0.54 + 0.46 / x) },
      { name: "physics only, 29%", colour: "#b7a2fb", f: (x) => 1 / (0.71 + 0.29 / x) },
    ],
    reads: (x) => [
      ["Humanoid training", `${fmt(1 / (0.13 + 0.87 / x), 2)}×`, "stops at 7.7×"],
      ["Dexterous hand", `${fmt(1 / (0.15 + 0.85 / x), 2)}×`, "stops at 6.7×"],
      ["Cart-pole", `${fmt(1 / (0.54 + 0.46 / x), 2)}×`, "stops at 1.9×"],
    ],
  },
  turn: {
    text: "Suppose FrankenRobotics spent no compute at all, and an agent still decided one design per turn. Both arms wait for the agent, so the ratio is (training + turn) ÷ turn.",
    label: "Agent turn",
    at: (p) => 30 + (p / 100) * 150,
    shown: (x) => `${fmt(x)} s`,
    start: 40,
    lo: 30,
    hi: 180,
    log: false,
    ticks: [
      [30, "30 s"],
      [60, "60 s"],
      [120, "120 s"],
      [180, "180 s"],
    ],
    top: 16,
    curves: [
      { name: "training 410 s, one card", colour: "#fbbf24", f: (t) => (410 + t) / t },
      { name: "training 300 s", colour: "#6cc8f7", f: (t) => (300 + t) / t },
    ],
    reads: (t) => [
      ["Against 410 s of training", `${fmt((410 + t) / t, 2)}×`, "4.4 to 7.8× for turns of 120 to 60 s"],
      ["Against 300 s", `${fmt((300 + t) / t, 2)}×`, "3.5 to 6× for the same turns"],
      ["What lifts it", "many per turn", "One query, a whole family of designs"],
    ],
  },
  cost: {
    text: "Simulate every design cheaply at c GPU-seconds, and train only a fraction f of them at 410. The compute ratio is 410 ÷ (c + 410·f).",
    label: "Cheap rung c",
    at: (p) => 5 + (p / 100) * 55,
    shown: (x) => `${fmt(x)} GPU-s`,
    start: 60,
    lo: 5,
    hi: 60,
    log: false,
    ticks: [
      [5, "5"],
      [20, "20"],
      [41, "41"],
      [60, "60"],
    ],
    top: 40,
    curves: [
      { name: "train none", colour: "#fbbf24", f: (c) => 410 / c },
      { name: "train 1%", colour: "#fb923c", f: (c) => 410 / (c + 4.1) },
      { name: "train 5%", colour: "#6cc8f7", f: (c) => 410 / (c + 20.5) },
    ],
    reads: (c) => {
      const f = (41 - c) / 410;
      return [
        ["Train none of them", `${fmt(410 / c, 1)}×`, "the ceiling at this c"],
        ["Share you may train for ten", f > 0 ? `${fmt(f * 100, 1)}%` : "none", f > 0 ? "any more and the ratio is below ten" : "above 41 GPU-seconds no share reaches ten"],
        ["At the plan's c = 38", "10.8×", "ten needs at most 0.7% trained"],
      ];
    },
  },
  miss: {
    text: "The score is a restricted mean, so a mission left unresolved counts as the full deadline. If a share q is missed, no speed on the rest lifts the ratio above 1/q.",
    label: "Missions missed",
    at: (p) => 1 + (p / 100) * 39,
    shown: (x) => `${fmt(x)}%`,
    start: 28,
    lo: 1,
    hi: 40,
    log: false,
    ticks: [
      [1, "1%"],
      [10, "10%"],
      [20, "20%"],
      [40, "40%"],
    ],
    top: 30,
    curves: [{ name: "1 ÷ q", colour: "#fbbf24", f: (q) => 100 / q }],
    reads: (q) => [
      ["Best possible ratio", `${fmt(100 / q, 2)}×`, "if everything else took no time"],
      ["One mission in eight", "8×", "cannot show ten-fold"],
      ["To reach ten", "q ≤ 10%", "and then only with instant answers"],
    ],
  },
};

const H = 330;
const L = 34;
const T = 18;
const B = 34;

export default function mount(root: HTMLElement) {
  const plot = $(root, "[data-plot]");
  const reads = $(root, "[data-reads]");
  const text = $(root, "[data-text]");
  const label = $(root, '[data-range="x"] b');
  let tab = tabs.engine!;
  let W = 900;
  const draw = () => {
    // Labels sit beside the curves where there is room, and in a key below where there is not.
    const narrow = W < 620;
    const R = narrow ? 10 : 178;
    const x = tab.at(slider.value);
    const sx = (v: number) =>
      L +
      (W - L - R) *
        (tab.log ? Math.log(v / tab.lo) / Math.log(tab.hi / tab.lo) : (v - tab.lo) / (tab.hi - tab.lo));
    const sy = (v: number) => T + (H - T - B) * (1 - clamp(v, 0, tab.top) / tab.top);
    let svg = "";
    for (const [v, name] of tab.ticks)
      svg += `<path class="grid" d="M${sx(v)} ${T}V${H - B}"/><text x="${sx(v)}" y="${H - 12}" text-anchor="middle">${name}</text>`;
    const step = tab.top > 24 ? 10 : tab.top > 13 ? 5 : 2;
    for (let v = 0; v <= tab.top; v += step)
      if (v !== 10) svg += `<path class="grid" d="M${L} ${sy(v)}H${W - R}"/><text x="${L - 9}" y="${sy(v) + 4}" text-anchor="end">${v}</text>`;
    svg += `<path class="ten" d="M${L} ${sy(10)}H${W - R}"/><text x="${L - 9}" y="${sy(10) + 4}" text-anchor="end" style="fill:#fbbf24">10</text>`;
    // Labels at the right edge, nudged apart where curves end close together.
    const ends = tab.curves
      .map((c) => ({ c, y: sy(c.f(tab.hi)) }))
      .sort((a, b) => a.y - b.y);
    for (let i = 1; i < ends.length; i++) ends[i]!.y = Math.max(ends[i]!.y, ends[i - 1]!.y + 17);
    for (const { c, y } of ends) {
      const pts: [number, number][] = [];
      for (let i = 0; i <= 160; i++) {
        const v = tab.log ? tab.lo * (tab.hi / tab.lo) ** (i / 160) : tab.lo + ((tab.hi - tab.lo) * i) / 160;
        if (c.f(v) <= tab.top * 1.001) pts.push([sx(v), sy(c.f(v))]);
      }
      svg += `<path d="${line(pts)}" fill="none" stroke="${c.colour}" stroke-width="2.4" stroke-linejoin="round"/>`;
      if (!narrow) svg += `<text x="${W - R + 12}" y="${y + 4}" style="fill:${c.colour}">${c.name}</text>`;
      if (c.f(x) <= tab.top)
        svg += `<circle cx="${sx(x)}" cy="${sy(c.f(x))}" r="5.5" fill="${c.colour}" stroke="#030705" stroke-width="2.5"/>`;
    }
    svg += `<path d="M${sx(x)} ${T}V${H - B}" stroke="#fff" stroke-opacity=".35" fill="none"/>`;
    plot.innerHTML = `<svg class="chart" viewBox="0 0 ${W} ${H}" role="img" aria-label="Speed-up of the whole loop against ${tab.label.toLowerCase()}">${svg}</svg>${narrow ? `<ul class="legend">${tab.curves.map((c) => `<li style="--k:${c.colour}">${c.name}</li>`).join("")}</ul>` : ""}`;
    reads.innerHTML = tab
      .reads(x)
      .map(([a, b, c]) => `<div class="read"><small>${a}</small><b>${b}</b><span>${c}</span></div>`)
      .join("");
  };
  const slider = range(root, "x", (v) => tab.shown(tab.at(v)), draw);
  seg(root, "tab", (v) => {
    tab = tabs[v]!;
    text.textContent = tab.text;
    label.textContent = tab.label;
    slider.set(tab.start);
    draw();
  });
  text.textContent = tab.text;
  onWidth(plot, (w) => {
    W = w;
    draw();
  });
}
