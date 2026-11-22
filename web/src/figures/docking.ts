import { $, $$, loop, onWidth, range, reduced, seg } from "./lib";

/** The three pushes for a gain g that is known after the first one. */
export const pushes = (g: number, p: number) => [p, 1 / g - 2 * p, p - 1 / g];
/** Cart states at the ends of the three one-second pushes. */
export function ride(g: number, u: readonly number[]) {
  const path = [{ x: 0, v: 0 }];
  for (const k of u) {
    const { x, v } = path.at(-1)!;
    path.push({ x: x + v + (g * k) / 2, v: v + g * k });
  }
  return path;
}
const E = 1e-9;
export function verdict(U: number, eps: number, p: number, late: boolean) {
  if (late) return "late" as const;
  if (p < 1 - U - E || p > (U + 0.5) / 2 + E) return "authority" as const;
  // Closed reading intervals [p − ε, p + ε] and [2p − ε, 2p + ε] must be disjoint.
  if (p - 2 * eps <= E) return "alias" as const;
  return "ok" as const;
}

export default function mount(root: HTMLElement) {
  const carts = $(root, "[data-carts]");
  const chart = $(root, "[data-chart]");
  let width = 620;
  let clock = 0;
  const f3 = (v: number) => v.toFixed(3).replace("-", "−");
  const state = () => {
    const v = verdict(U.value, eps.value, p.value, when.value === "late");
    // Where the readings cannot be told apart, one command must serve both gains: take the middle.
    const own = [pushes(1, p.value), pushes(2, p.value)] as const;
    const shared = own[0].map((u, i) => (u + own[1][i]!) / 2);
    const lim = (u: number) => Math.max(-U.value, Math.min(U.value, u));
    const apart = v === "ok" || v === "authority";
    return [(apart ? own[0] : shared).map(lim), (apart ? own[1] : shared).map(lim)] as const;
  };
  const side = () => {
    const v = verdict(U.value, eps.value, p.value, when.value === "late");
    const top = (U.value + 0.5) / 4;
    const cert = $(root, ".cert");
    cert.classList.toggle("in", v === "ok");
    cert.classList.toggle("out", v !== "ok");
    $(root, "[data-verdict]").textContent = {
      ok: "Stops on the mark with either motor",
      late: "No controller can do it",
      authority: "That first push can’t be undone",
      alias: "The two motors look the same",
    }[v];
    $(root, "[data-why]").textContent = {
      ok: `A weak motor reads ${f3(p.value)} ± ${f3(eps.value)} and a strong one ${f3(2 * p.value)} ± ${f3(eps.value)}. They never overlap, so the cart knows which it has and corrects in time.`,
      late: "The reading lands after the last push. Both motors get the same three pushes and they need different ones, however good the sensor is.",
      authority: `The first push has to be between ${f3(1 - U.value)} and ${f3((U.value + 0.5) / 2)}, or a later one asks for more than the motor has.`,
      alias: `Both can report ${f3(2 * p.value - eps.value)}, and they need ${f3(1 - 2 * p.value)} and ${f3(0.5 - 2 * p.value)} next. ${
        eps.value < top - E
          ? `A harder first push, above ${f3(2 * eps.value)}, tells them apart.`
          : `No first push helps. With this motor the sensor error has to stay below ${f3(top)}.`
      }`,
    }[v];
  };
  const drawCarts = () => {
    const u = state();
    const W = width;
    const H = 214;
    const x0 = 46;
    const S = (W - x0 - 46) / 1.5;
    const X = (x: number) => x0 + x * S;
    const t = Math.min(3, clock);
    let svg = `<path d="M${X(1)} 10V${H - 22}" stroke="#4ade80" stroke-dasharray="4 5" fill="none"/><text x="${X(1)}" y="${H - 6}" text-anchor="middle" style="fill:#4ade80">stop here</text>`;
    for (const [i, g] of [1, 2].entries()) {
      const path = ride(g, u[i]!);
      const k = Math.min(2, Math.floor(t));
      const s = t - k;
      const { x, v } = path[k]!;
      const a = g * u[i]![k]!;
      const end = path[3]!;
      const pos = t >= 3 ? end.x : x + v * s + (a * s * s) / 2;
      const y = 22 + i * 86;
      const hit = Math.abs(end.x - 1) < 1e-6 && Math.abs(end.v) < 1e-6;
      const tone = t < 3 ? "#e9f2ec" : hit ? "#4ade80" : "#fb7185";
      svg += `<text x="${X(-0.06)}" y="${y + 64}">${g === 1 ? "weak" : "strong"} motor</text><path d="M${X(-0.06)} ${y + 46}H${W - 14}" stroke="rgb(190 240 212 / .25)" fill="none"/>`;
      for (const [j, q] of path.entries())
        if (j && j <= t + 1e-9) svg += `<circle cx="${X(q.x).toFixed(1)}" cy="${y + 46}" r="3" fill="#98aba1"/>`;
      const cx = X(pos);
      svg += `<g transform="translate(${cx.toFixed(2)} ${y})"><rect x="-27" y="12" width="54" height="24" rx="7" fill="${tone}" fill-opacity=".2" stroke="${tone}" stroke-width="2"/><circle cx="-15" cy="40" r="6" fill="#030705" stroke="${tone}" stroke-width="2"/><circle cx="15" cy="40" r="6" fill="#030705" stroke="${tone}" stroke-width="2"/>`;
      if (t < 3) {
        const push = u[i]![k]! * 62;
        svg += `<path d="M0 2h${push.toFixed(1)}m${push > 0 ? "-7 -5 7 5 -7 5" : "7 -5 -7 5 7 5"}" stroke="#c6f25a" stroke-width="2.4" fill="none" stroke-linejoin="round"/>`;
      }
      svg += "</g>";
      if (t >= 3) {
        const right = cx + 190 < W;
        svg += `<text x="${(right ? cx + 38 : cx - 38).toFixed(1)}" y="${y + 24}" text-anchor="${right ? "start" : "end"}" style="fill:${tone}">${hit ? "stopped at 1" : `x ${f3(end.x)}, v ${f3(end.v)}`}</text>`;
      }
    }
    carts.innerHTML = `<svg class="chart" viewBox="0 0 ${W} ${H}" role="img" aria-label="Two carts, one with a weak motor and one with a strong one, driven by the same controller">${svg}</svg>`;
  };
  const drawChart = () => {
    const W = width;
    const wide = W > 560;
    const pw = wide ? W * 0.46 : W;
    const H = wide ? 250 : 490;
    // Left: the readings on a line. Right: which (U, ε) allow any policy.
    const X = (v: number) => 12 + (Math.max(-0.45, v) + 0.45) * ((pw - 24) / 2.35);
    const late = when.value === "late";
    let svg = `<text class="label" x="12" y="18">${late ? "The reading comes too late to use" : "What the sensor can tell you"}</text>`;
    const band = (g: number, y: number, c: string) => {
      const m = g * p.value;
      const a = X(m - eps.value);
      const b = X(m + eps.value);
      svg += `<rect x="${a}" y="${y}" width="${Math.max(3, b - a)}" height="30" rx="6" fill="${c}" fill-opacity="${late ? 0.1 : 0.3}" stroke="${c}" stroke-opacity="${late ? 0.4 : 1}"/><circle cx="${X(m)}" cy="${y + 15}" r="4" fill="${c}"/><text x="${Math.min(b + 8, pw - 100)}" y="${y + 19}" style="fill:${c}">${g === 1 ? "weak" : "strong"} motor</text>`;
    };
    band(1, 46, "#6cc8f7");
    band(2, 92, "#fbbf24");
    if (!late) {
      const lo = X(2 * p.value - eps.value);
      const hi = X(p.value + eps.value);
      const mid = Math.max(64, Math.min((lo + hi) / 2, pw - 64));
      svg +=
        p.value - 2 * eps.value <= E
          ? `<rect x="${lo}" y="34" width="${Math.max(2, hi - lo)}" height="104" fill="#fb7185" fill-opacity=".2" stroke="#fb7185" stroke-dasharray="3 4"/><text x="${mid}" y="158" text-anchor="middle" style="fill:#fb7185">could be either</text>`
          : `<path d="M${hi} 142H${lo}M${hi} 137v10M${lo} 137v10" stroke="#4ade80" fill="none"/><text x="${mid}" y="162" text-anchor="middle" style="fill:#4ade80">gap ${(p.value - 2 * eps.value).toFixed(3)}</text>`;
    }
    svg += `<path class="axis" d="M12 182H${pw - 12}"/>`;
    for (const v of [0, 0.5, 1, 1.5])
      svg += `<path class="axis" d="M${X(v)} 182v5"/><text x="${X(v)}" y="201" text-anchor="middle">${v}</text>`;
    svg += `<text x="${pw / 2}" y="222" text-anchor="middle">speed reading</text>`;
    const ox = wide ? W * 0.56 : 42;
    const oy = wide ? 0 : 246;
    const mw = wide ? W * 0.42 : W - 60;
    const MX = (u: number) => ox + (u - 0.5) * 2 * mw;
    const MY = (e: number) => oy + 196 - (e / 0.45) * 164;
    svg += `<text class="label" x="${ox}" y="${oy + 18}">Where it works</text><path d="M${MX(0.5)} ${MY(0)}L${MX(0.5)} ${MY(0.25)}L${MX(1)} ${MY(0.375)}L${MX(1)} ${MY(0)}Z" fill="#4ade80" fill-opacity="${late ? 0.03 : 0.16}"/><path d="M${MX(0.5)} ${MY(0.25)}L${MX(1)} ${MY(0.375)}" stroke="${late ? "#7c9086" : "#4ade80"}" stroke-width="2" stroke-dasharray="6 4" fill="none"/><path class="axis" d="M${MX(0.5)} ${MY(0.45)}V${MY(0)}H${MX(1)}"/>`;
    for (const u of [0.5, 0.75, 1]) svg += `<text x="${MX(u)}" y="${MY(0) + 18}" text-anchor="middle">${u}</text>`;
    for (const e of [0.1, 0.2, 0.3, 0.4]) svg += `<text x="${ox - 8}" y="${MY(e) + 4}" text-anchor="end">${e}</text>`;
    const dead = late || eps.value >= (U.value + 0.5) / 4 - E;
    svg += `<text x="${ox + mw / 2}" y="${MY(0) + 37}" text-anchor="middle">motor strength across, sensor error up</text><circle cx="${MX(U.value)}" cy="${MY(eps.value)}" r="7" fill="${dead ? "#fb7185" : "#4ade80"}" stroke="#030705" stroke-width="3"/>`;
    chart.innerHTML = `<svg class="chart" viewBox="0 0 ${W} ${H}" role="img" aria-label="What the sensor reads for each motor, and the combinations of motor strength and sensor error that work">${svg}</svg>`;
  };
  const all = () => {
    side();
    drawChart();
    drawCarts();
  };
  const changed = () => {
    if (!reduced) clock = 0;
    all();
    view.poke();
  };
  const U = range(root, "U", (v) => v.toFixed(2), changed);
  const eps = range(root, "eps", (v) => v.toFixed(v === 0.3125 ? 4 : 3), changed);
  const p = range(root, "p", (v) => v.toFixed(3), changed);
  const when = seg(root, "when", changed);
  for (const b of $$<HTMLButtonElement>(root, "[data-set]"))
    b.addEventListener("click", () => {
      const [a, e, first] = b.dataset.set!.split(",").map(Number);
      U.set(a!);
      eps.set(e!);
      p.set(first!);
      when.set("early");
      changed();
    });
  let hold = 0;
  const view = loop(carts, (dt) => {
    if (view.running) {
      if (clock < 3) {
        clock = Math.min(3, clock + dt * 0.8);
        hold = 0;
      } else if ((hold += dt) > 2.6) clock = 0;
    }
    drawCarts();
  });
  if (reduced) clock = 3;
  onWidth(chart, (w) => {
    width = w;
    all();
  });
}
