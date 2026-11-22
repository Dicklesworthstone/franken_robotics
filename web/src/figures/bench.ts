import { $, fmt, seg, marks } from "./lib";

type Answer = [label: string, value: string, note: string];
interface Fixture {
  text: string;
  svg: () => string;
  right: Answer;
  wrong: Answer;
  slider?: { name: string; min: number; max: number; step: number; start: number; shown: (v: number) => string };
}
let knob = 0;
const G = "#4ade80";
const C = "#fb7185";
const A = "#fbbf24";
const minus = (v: string) => v.replace("-", "−");

/** Probability that a majority of n independent reads is right, each right with probability 3/4. */
export function majority(n: number) {
  let sum = 0;
  for (let k = Math.ceil(n / 2); k <= n; k++) {
    let c = 1;
    for (let i = 0; i < k; i++) c = (c * (n - i)) / (i + 1);
    sum += c * 3 ** k;
  }
  return [Math.round(sum), 4 ** n] as const;
}
const gcd = (a: number, b: number): number => (b ? gcd(b, a % b) : a);
const ratio = ([n, d]: readonly [number, number]) => {
  const g = gcd(n, d);
  return `${fmt(n / g)}/${fmt(d / g)}`;
};
export const shell = (t: number) => {
  const full = 0.4 * 0.3 * 0.2;
  return (full - (0.4 - 2 * t) * (0.3 - 2 * t) * (0.2 - 2 * t)) * 2700;
};

const fixtures: Record<string, Fixture> = {
  arm: {
    text: "Two links of 0.4 and 0.3 m, the second standing up. A force of 10 N pushes the tool along x while the joints turn at 0.2 and −0.1 rad/s.",
    svg: () =>
      `<defs><marker id="b-a" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0 0 10 5 0 10Z" fill="${A}"/></marker><marker id="b-g" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0 0 10 5 0 10Z" fill="${G}"/></marker></defs><path class="grid" d="M40 250H500M60 20V270"/><path d="M100 250H300V100" fill="none" stroke="#22372c" stroke-width="22" stroke-linecap="round" stroke-linejoin="round"/><path d="M100 250H300V100" fill="none" stroke="#dfe8e2" stroke-width="13" stroke-linecap="round" stroke-linejoin="round"/><circle cx="100" cy="250" r="14" fill="#030705" stroke="${G}" stroke-width="4"/><circle cx="300" cy="250" r="14" fill="#030705" stroke="${G}" stroke-width="4"/><circle cx="300" cy="100" r="9" fill="${G}"/><path d="M312 100H410" stroke="${A}" stroke-width="3" marker-end="url(#b-a)"/><text x="362" y="90" style="fill:${A}" text-anchor="middle">10 N</text><path d="M300 100 270 30" stroke="${G}" stroke-width="3" marker-end="url(#b-g)"/><text x="258" y="46" text-anchor="end" style="fill:${G}">(−0.03, 0.08) m/s</text><text x="200" y="284" text-anchor="middle">0.4 m</text><text x="322" y="182">0.3 m</text><text x="322" y="262">−3 N·m</text><text x="84" y="222" text-anchor="end">−3 N·m</text>`,
    right: ["Joint torques Jᵀf", "(−3, −3) N·m", "Both routes to power agree: f·v and τ·q̇ are each −0.3 W."],
    wrong: ["Rows of a space screw", "(0, 0) N·m", "What you get by taking the lower three rows of a space screw Jacobian for a point Jacobian."],
  },
  shell: {
    text: "An aluminium enclosure of 0.4 × 0.3 × 0.2 m at 2,700 kg/m³. Its inertia decides how the whole robot moves, so it matters whether the file said there is a wall.",
    svg: () => {
      const t = knob / 1000;
      const box = (s: number, x: number, y: number) => {
        const w = 400 * s;
        const h = 200 * s;
        const k = 90 * s;
        return { front: `M${x} ${y}h${w}v${h}h${-w}Z`, top: `M${x} ${y}l${k} ${-k}h${w}l${-k} ${k}Z`, side: `M${x + w} ${y}l${k} ${-k}v${h}l${-k} ${k}Z` };
      };
      const o = box(0.78, 60, 110);
      const i = t * 1000 * 0.78;
      return `<path d="${o.top}" fill="#1b2b24" stroke="#bfe9d0" stroke-opacity=".5"/><path d="${o.side}" fill="#122019" stroke="#bfe9d0" stroke-opacity=".5"/><path d="${o.front}" fill="${G}" fill-opacity=".3" stroke="${G}"/><rect x="${60 + i}" y="${110 + i}" width="${Math.max(0, 312 - 2 * i)}" height="${Math.max(0, 156 - 2 * i)}" fill="#040a07" stroke="${G}" stroke-opacity=".6"/><text x="216" y="194" text-anchor="middle">${t >= 0.1 ? "solid" : "air"}</text><text x="216" y="290" text-anchor="middle">0.4 m</text><text x="452" y="160">0.2 m</text>`;
    },
    right: ["With this wall", "", "The 2.5 mm wall of the fixture gives 3.45 kg."],
    wrong: ["Filled solid", "64.8 kg", "What a mass calculation returns when it treats the closed surface as solid metal. The plan raises a density warning."],
    slider: { name: "Wall", min: 1, max: 100, step: 0.5, start: 2.5, shown: (v) => `${v.toFixed(1)} mm` },
  },
  mid: {
    text: "Four feet rated at 0.5 N, standing at (±1 − e, ±1) m, with a load W at the origin. The pair on the right must carry W(1 + e)/2 and can carry 1 N.",
    svg: () => {
      const X = (e: number) => 70 + e * 400;
      const Y = (w: number) => 260 - (w - 0.8) * 180;
      let edge = "";
      for (let i = 0; i <= 60; i++) {
        const e = i / 60;
        edge += `${i ? "L" : "M"}${X(e).toFixed(1)} ${Y(2 / (1 + e)).toFixed(1)}`;
      }
      return `<path d="${edge}L${X(1)} ${Y(0.8)}L${X(0)} ${Y(0.8)}Z" fill="${G}" fill-opacity=".13"/><path d="${edge}" fill="none" stroke="${G}" stroke-width="2.4"/><path class="axis" d="M${X(0)} 20V${Y(0.8)}H${X(1.02)}"/><path d="M${X(0)} ${Y(1.9)}L${X(0.9)} ${Y(1)}" stroke="${C}" stroke-dasharray="5 5"/><circle cx="${X(0)}" cy="${Y(1.9)}" r="7" fill="${G}"/><circle cx="${X(0.9)}" cy="${Y(1)}" r="7" fill="${G}"/><circle cx="${X(0.45)}" cy="${Y(1.45)}" r="8" fill="${C}"/><text x="${X(0) + 14}" y="${Y(1.9) - 6}">(0, 1.9) needs 0.95 N</text><text x="${X(0.9) + 8}" y="${Y(1) - 14}" text-anchor="end">(0.9, 1) needs 0.95 N</text><text x="${X(0.45) + 16}" y="${Y(1.45) - 8}" style="fill:${C}">midpoint needs 841/800 N</text><text x="${X(0.5)}" y="284" text-anchor="middle">offset e across, load W up</text>`;
    },
    right: ["The midpoint (0.45, 1.45)", "841/800 N", "More than the 1 N the pair can carry, so it is excluded."],
    wrong: ["Convex hull of two witnesses", "admitted", "Two good designs do not vouch for the one between them."],
  },
  stage: {
    text: "Twenty equally likely worlds and two stages of a task. Each stage passes in 19 of the 20. How often does the whole task pass?",
    svg: () => {
      let s = "";
      for (const [r, [title, bad1, bad2]] of (
        [
          ["different worlds fail", 4, 13],
          ["the same world fails", 9, 9],
        ] as const
      ).entries()) {
        const y = 60 + r * 120;
        s += `<text x="20" y="${y - 22}">${title}</text>`;
        for (let i = 0; i < 20; i++) {
          const x = 24 + i * 24.5;
          for (const [k, bad] of [bad1, bad2].entries())
            s += `<rect x="${x}" y="${y + k * 24}" width="19" height="19" rx="4" fill="${i === bad ? C : G}" fill-opacity="${i === bad ? 1 : 0.28}"/>`;
          if (i === bad1 || i === bad2) s += `<path d="M${x - 2} ${y - 4}h23v51h-23Z" fill="none" stroke="${C}" stroke-opacity=".6"/>`;
        }
      }
      return s;
    },
    right: ["Joint count", "18 or 19 of 20", "It depends on whether the failing worlds coincide, so the stages are scored together."],
    wrong: ["Multiply the rates", "0.9025", "0.95 × 0.95 is neither of the two possible answers here. It assumes independence."],
  },
  cayley: {
    text: "The batched solver turns a body with the Cayley map, which needs only arithmetic and lands exactly on a rotation. The angle it turns is 2·atan(ωh/2).",
    svg: () => {
      const a = knob;
      const c = 2 * Math.atan(a / 2);
      const p = (t: number, r: number) => `${(150 + r * Math.cos(t)).toFixed(2)} ${(240 - r * Math.sin(t)).toFixed(2)}`;
      const arm = (t: number, colour: string, dash = "") =>
        `<path d="M150 240L${p(t, 230)}" stroke="${colour}" stroke-width="3"${dash ? ` stroke-dasharray="${dash}"` : ""} stroke-linecap="round"/><circle cx="${p(t, 230).split(" ")[0]}" cy="${p(t, 230).split(" ")[1]}" r="6" fill="${colour}"/>`;
      return `<path d="M${p(0, 230)}A230 230 0 0 0 ${p(1.6, 230)}" fill="none" stroke="rgb(190 240 212 / .2)"/><path class="axis" d="M150 240H420"/>${arm(a, A, "6 5")}${arm(c, G)}<circle cx="150" cy="240" r="9" fill="#030705" stroke="#dfe8e2" stroke-width="3"/><text x="395" y="60" text-anchor="start" style="fill:${A}">ωh ${a.toFixed(3)}</text><text x="395" y="84" style="fill:${G}">Cayley ${c.toFixed(3)}</text>${a > 0.2 ? `<text x="395" y="116" style="fill:${C}">FROB-REF-212</text><text x="395" y="134" style="fill:${C}">refused</text>` : ""}`;
    },
    right: ["Cayley angle", "", "Above 0.2 rad per substep the step is refused, and the refusal states the substep count that admits it."],
    wrong: ["Read as ωh", "", "The label of an exact retraction on a second-order one. The error grows with the cube of the step."],
    slider: { name: "Turn per step, ωh", min: 0.02, max: 1.4, step: 0.02, start: 0.9, shown: (v) => `${v.toFixed(2)} rad` },
  },
  reads: {
    text: "A sensor that is right three times in four, read again and again, and the majority believed. That helps while the mistakes are independent.",
    svg: () => {
      const n = Math.round(knob);
      const X = (k: number) => 70 + ((k - 1) / 14) * 400;
      const Y = (v: number) => 250 - ((v - 0.6) / 0.4) * 210;
      let path = "";
      let dots = "";
      for (let k = 1; k <= 15; k += 2) {
        const [a, b] = majority(k);
        path += `${k > 1 ? "L" : "M"}${X(k)} ${Y(a / b)}`;
        dots += `<circle cx="${X(k)}" cy="${Y(a / b)}" r="${k === n ? 8 : 4}" fill="${G}"/>`;
      }
      return `<path class="axis" d="M70 30V250H480"/>${[0.75, 0.9, 1].map((v) => `<path class="grid" d="M70 ${Y(v)}H480"/><text x="62" y="${Y(v) + 4}" text-anchor="end">${v * 100}%</text>`).join("")}<path d="${path}" fill="none" stroke="${G}" stroke-width="2.4"/>${dots}<path d="M70 ${Y(0.75)}H480" stroke="${C}" stroke-width="2.4"/><circle cx="${X(n)}" cy="${Y(0.75)}" r="7" fill="${C}"/><text x="478" y="${Y(0.75) + 24}" text-anchor="end" style="fill:${C}">one fault shared by every read</text>${[1, 3, 5, 7, 11, 15].map((k) => `<text x="${X(k)}" y="270" text-anchor="middle">${k}</text>`).join("")}<text x="270" y="290" text-anchor="middle">reads</text>`;
    },
    right: ["Independent reads", "", "27/32 with three, 459/512 with five, 3,807/4,096 with seven, the first above 90%."],
    wrong: ["One shared fault", "3/4", "A common cause makes every later read a copy of the first. No number of reads helps."],
    slider: { name: "Reads", min: 1, max: 15, step: 2, start: 7, shown: (v) => `${v}` },
  },
};

export default function mount(root: HTMLElement) {
  const bench = $(root, "[data-bench]");
  let key = "arm";
  const answers = () => {
    const f = fixtures[key]!;
    const right: Answer = [...f.right];
    const wrong: Answer = [...f.wrong];
    if (key === "shell") right[1] = `${shell(knob / 1000).toFixed(2)} kg`;
    if (key === "cayley") {
      const c = 2 * Math.atan(knob / 2);
      right[1] = `${c.toFixed(4)} rad`;
      wrong[1] = minus(`${knob.toFixed(4)} rad`);
      wrong[2] = `${f.wrong[2]} Here it is off by ${(((knob - c) / knob) * 100).toFixed(2)}%.`;
    }
    if (key === "reads") {
      const m = majority(Math.round(knob));
      right[1] = `${ratio(m)} = ${((m[0] / m[1]) * 100).toFixed(1)}%`;
    }
    return { right, wrong };
  };
  const paint = () => {
    const f = fixtures[key]!;
    $(bench, "[data-art]").innerHTML = `<svg class="chart" viewBox="0 0 560 300" role="img" aria-label="${f.text}">${f.svg()}</svg>`;
    const { right, wrong } = answers();
    const card = (cls: string, [a, b, c]: Answer) => `<div class="${cls}"><small>${a}</small><b>${b}</b><p>${c}</p></div>`;
    $(bench, "[data-pair]").innerHTML = marks(card("in", right) + card("out", wrong));
  };
  const build = () => {
    const f = fixtures[key]!;
    const s = f.slider;
    knob = s?.start ?? 0;
    bench.innerHTML = `<div data-art></div><div class="bench-side"><p>${f.text}</p>${
      s
        ? `<label class="range"><span><b>${s.name}</b><output aria-live="off"></output></span><input type="range" aria-label="${s.name}" min="${s.min}" max="${s.max}" step="${s.step}" value="${s.start}"></label>`
        : ""
    }<div class="pair" data-pair aria-live="polite"></div></div>`;
    if (s) {
      const input = $<HTMLInputElement>(bench, "input");
      const sync = () => {
        knob = Number(input.value);
        input.style.setProperty("--p", `${((knob - s.min) / (s.max - s.min)) * 100}%`);
        $(bench, "output").textContent = s.shown(knob);
        paint();
      };
      input.addEventListener("input", sync);
      sync();
    } else paint();
  };
  seg(root, "bench", (v) => {
    key = v;
    build();
  });
  build();
}
