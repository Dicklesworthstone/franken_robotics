import { $, clamp, loop, mix, range, seg } from "./lib";

/** Corridor width. The walls are x = 0, y = 0 and the block x > A, y > A. */
export const A = 0.9;
/** The square of the plane that is searched, and the corners of the rectangle in body axes. */
const SPAN = 2.4;

/**
 * Signed clearance of a rectangle (length L, width b, centre x, y, heading c = cos, s = sin) in the L-shaped
 * corridor. Positive outside every wall, negative inside one.
 */
export function clearance(x: number, y: number, c: number, s: number, L: number, b: number) {
  const ex = Math.abs(c) * (L / 2) + Math.abs(s) * (b / 2);
  const ey = Math.abs(s) * (L / 2) + Math.abs(c) * (b / 2);
  const walls = Math.min(x - ex, y - ey);
  // Separation from the block along a direction n in the first quadrant is min over corners of n·(A − p).
  const hx = L / 2;
  const hy = b / 2;
  const qx = [
    A - x - c * hx + s * hy,
    A - x - c * hx - s * hy,
    A - x + c * hx + s * hy,
    A - x + c * hx - s * hy,
  ];
  const qy = [
    A - y - s * hx - c * hy,
    A - y - s * hx + c * hy,
    A - y + s * hx - c * hy,
    A - y + s * hx + c * hy,
  ];
  const along = (nx: number, ny: number) =>
    Math.min(
      nx * qx[0]! + ny * qy[0]!,
      nx * qx[1]! + ny * qy[1]!,
      nx * qx[2]! + ny * qy[2]!,
      nx * qx[3]! + ny * qy[3]!,
    );
  let best = Math.max(along(1, 0), along(0, 1));
  const edge = (nx: number, ny: number) => {
    if (nx < 0 || ny < 0) [nx, ny] = [-nx, -ny];
    if (nx >= 0 && ny >= 0) best = Math.max(best, along(nx, ny));
  };
  edge(c, s);
  edge(-s, c);
  for (let i = 0; i < 4; i++) {
    const n = Math.hypot(qx[i]!, qy[i]!);
    if (n > 1e-9 && qx[i]! >= 0 && qy[i]! >= 0) best = Math.max(best, along(qx[i]! / n, qy[i]! / n));
  }
  return Math.min(walls, best);
}

const FREE = 1;
const OPEN = 2;
const BLOCKED = 3;
export interface Complex {
  n: number;
  m: number;
  cells: Uint8Array;
  verdict: "fits" | "blocked" | "open";
}

/** Classifies every cell of (x, y, heading) by the bound of §19.11 and looks for a path, or a cut. */
export function complex(L: number, b: number, level: number): Complex {
  const n = 24 << level;
  const m = 16 << level;
  const d = SPAN / n;
  const turn = Math.PI / m;
  const rho = Math.hypot(L, b) / 2;
  const cells = new Uint8Array(n * n * m);
  // Coarse blocks settle most of the space at once; only blocks near a wall are opened.
  const fill = (x0: number, y0: number, k0: number, w: number) => {
    const th = (k0 + w / 2) * turn;
    const c0 = clearance((x0 + w / 2) * d, (y0 + w / 2) * d, Math.cos(th), Math.sin(th), L, b);
    const e = ((w * d) / 2) * Math.SQRT2 + rho * ((w * turn) / 2);
    const state = c0 > e ? FREE : c0 < -e ? BLOCKED : OPEN;
    if (w === 1 || state !== OPEN) {
      for (let k = k0; k < k0 + w; k++)
        for (let y = y0; y < y0 + w; y++) {
          const row = (k * n + y) * n;
          cells.fill(state, row + x0, row + x0 + w);
        }
      return;
    }
    const h = w >> 1;
    for (let i = 0; i < 8; i++) fill(x0 + (i & 1) * h, y0 + ((i >> 1) & 1) * h, k0 + ((i >> 2) & 1) * h, h);
  };
  for (let k = 0; k < m; k += 8)
    for (let y = 0; y < n; y += 8) for (let x = 0; x < n; x += 8) fill(x, y, k, 8);
  // Start: far up the vertical corridor, lengthwise. Goal: far along the horizontal one, lengthwise.
  const cell = (x: number, y: number, th: number) => {
    const k = ((Math.floor(th / turn) % m) + m) % m;
    return (k * n + Math.min(n - 1, Math.floor(y / d))) * n + Math.min(n - 1, Math.floor(x / d));
  };
  const far = SPAN - 0.06;
  const start = cell(A / 2, far, Math.PI / 2);
  const goal = cell(far, A / 2, 1e-9);
  const reach = (allow: (s: number) => boolean) => {
    if (!allow(cells[start]!) || !allow(cells[goal]!)) return false;
    const seen = new Uint8Array(cells.length);
    const queue = new Int32Array(cells.length);
    let head = 0;
    let tail = 0;
    queue[tail++] = start;
    seen[start] = 1;
    const nn = n * n;
    while (head < tail) {
      const i = queue[head++]!;
      if (i === goal) return true;
      const x = i % n;
      const y = Math.floor(i / n) % n;
      const k = Math.floor(i / nn);
      const go = (j: number) => {
        if (!seen[j] && allow(cells[j]!)) {
          seen[j] = 1;
          queue[tail++] = j;
        }
      };
      if (x > 0) go(i - 1);
      if (x < n - 1) go(i + 1);
      if (y > 0) go(i - n);
      if (y < n - 1) go(i + n);
      go(k > 0 ? i - nn : i + (m - 1) * nn);
      go(k < m - 1 ? i + nn : i - (m - 1) * nn);
    }
    return false;
  };
  const verdict = reach((s) => s === FREE) ? "fits" : reach((s) => s !== BLOCKED) ? "open" : "blocked";
  return { n, m, cells, verdict };
}

export const longest = (b: number) => 2 * Math.SQRT2 * A - 2 * b;

export default function mount(root: HTMLElement) {
  const plan = $(root, "[data-plan]");
  const canvas = $<HTMLCanvasElement>(root, ".corner-map canvas");
  const g = canvas.getContext("2d")!;
  const set = (k: string, text: string, tone: string) => {
    const p = $(root, `[data-v="${k}"]`);
    p.classList.remove("in", "out", "open");
    p.classList.add(tone);
    $(p, "b").textContent = text;
  };
  let current: Complex;
  let pending = 0;
  /** The map shows the heading the body has now, until a reader picks one. */
  let follow = true;
  let centre: readonly [number, number] = [0, 0];
  const size = canvas.width;
  const tile = document.createElement("canvas");
  const tg = tile.getContext("2d")!;
  const COLOUR = {
    [FREE]: [38, 148, 88],
    [OPEN]: [232, 170, 36],
    [BLOCKED]: [46, 17, 26],
  } as Record<number, number[]>;
  const slice = () => {
    const { n, m, cells } = current;
    // The body starts lengthwise in the upright corridor, at a heading of 90 degrees.
    const deg = psi.value;
    const k = ((Math.floor(((90 + deg) / 180) * m) % m) + m) % m;
    if (tile.width !== n) tile.width = tile.height = n;
    const img = tg.createImageData(n, n);
    for (let y = 0; y < n; y++)
      for (let x = 0; x < n; x++) {
        const c = COLOUR[cells[(k * n + (n - 1 - y)) * n + x]!]!;
        const p = (y * n + x) * 4;
        img.data[p] = c[0]!;
        img.data[p + 1] = c[1]!;
        img.data[p + 2] = c[2]!;
        img.data[p + 3] = 255;
      }
    tg.putImageData(img, 0, 0);
    g.imageSmoothingEnabled = false;
    g.drawImage(tile, 0, 0, size, size);
    if (n <= 96) {
      g.strokeStyle = `rgba(3,7,5,${n <= 48 ? 0.5 : 0.28})`;
      g.lineWidth = 1;
      g.beginPath();
      for (let i = 1; i < n; i++) {
        g.moveTo((i / n) * size, 0);
        g.lineTo((i / n) * size, size);
        g.moveTo(0, (i / n) * size);
        g.lineTo(size, (i / n) * size);
      }
      g.stroke();
    }
    const S = size / SPAN;
    g.strokeStyle = "rgba(255,255,255,.9)";
    g.lineWidth = 4;
    g.beginPath();
    g.moveTo(2, 0);
    g.lineTo(2, size - 2);
    g.lineTo(size, size - 2);
    g.moveTo(A * S, 0);
    g.lineTo(A * S, size - A * S);
    g.lineTo(size, size - A * S);
    g.stroke();
    g.font = "500 24px JetBrains Mono, monospace";
    g.fillStyle = "rgba(255,255,255,.9)";
    g.fillText("where its center can be,", A * S + 30, 50);
    g.fillText(`turned ${Math.round(deg)}°`, A * S + 30, 84);
    if (follow) {
      const x = centre[0] * S;
      const y = size - centre[1] * S;
      g.beginPath();
      g.arc(x, y, 13, 0, Math.PI * 2);
      g.fillStyle = "#fff";
      g.fill();
      g.lineWidth = 7;
      g.strokeStyle = "rgba(3,7,5,.85)";
      g.stroke();
    }
  };
  const solve = () => {
    const L = len.value;
    const b = wide.value;
    current = complex(L, b, Number(res.value));
    set("inner", b <= A ? "fits" : "won’t fit", b <= A ? "in" : "out");
    const diag = Math.hypot(L, b);
    set("outer", diag <= A ? "fits" : "won’t fit", diag <= A ? "in" : "out");
    if (current.verdict === "fits") set("cell", "It fits, and there’s the path", "in");
    else if (current.verdict === "blocked") set("cell", "It can’t fit. No path exists", "out");
    else set("cell", "Too close to call at this detail", "open");
    const top = longest(b);
    set("truth", `${L <= top ? "fits" : "won’t fit"}, up to ${top.toFixed(3)} m`, L <= top ? "in" : "out");
    slice();
  };
  const later = () => {
    cancelAnimationFrame(pending);
    pending = requestAnimationFrame(solve);
  };

  let t = 0;
  const frame = (dt: number) => {
    if (view.running) t = (t + dt * 0.17) % 1;
    const L = len.value;
    const b = wide.value;
    const top = longest(b);
    const ok = L <= top + 1e-9;
    // Down the corridor, round the corner on the outer walls, and away. A body that is too long stops where it jams.
    const swing = (th: number) => {
      const c = Math.cos(th);
      const s = Math.sin(th);
      const slack = Math.max(0, (A * (s + c) - L * s * c - b) / 2);
      return [(L * c) / 2 + s * (b / 2 + slack), (L * s) / 2 + c * (b / 2 + slack), Math.PI - th] as const;
    };
    let jam = 0;
    if (!ok) {
      for (let th = Math.PI / 2; th > 0; th -= 0.004)
        if (A * (Math.sin(th) + Math.cos(th)) - L * Math.sin(th) * Math.cos(th) - b < 0) {
          jam = th;
          break;
        }
    }
    const u = t < 0.5 || ok ? t : 1 - t;
    let pose: readonly [number, number, number];
    const in0 = swing(Math.PI / 2);
    const out0 = swing(0);
    if (u < 0.26) pose = [in0[0], mix(SPAN - 0.05, in0[1], u / 0.26), Math.PI / 2];
    else if (ok && u > 0.74) pose = [mix(out0[0], SPAN - 0.05, (u - 0.74) / 0.26), out0[1], Math.PI];
    else {
      const k = ok ? (u - 0.26) / 0.48 : (u - 0.26) / 0.24;
      pose = swing(mix(Math.PI / 2, ok ? 0 : jam, clamp(k, 0, 1)));
    }
    const stuck = !ok && u > 0.485;
    centre = [pose[0], pose[1]];
    if (follow) {
      psi.set(Math.round(clamp(((pose[2] * 180) / Math.PI) - 90, 0, 90)));
      slice();
    }
    const S = 100;
    const w = SPAN * S;
    const tone = ok ? "#4ade80" : "#fb7185";
    const diag = Math.hypot(L, b) / 2;
    const deg = (-pose[2] * 180) / Math.PI;
    plan.innerHTML = `<svg class="chart" viewBox="-6 -6 ${w + 12} ${w + 12}" role="img" aria-label="Plan of the corner with the body turning through it"><path d="M0 -6V${w}H${w + 6}V${w - A * S}H${A * S}V-6Z" fill="#0c1f29"/><path d="M${A * S} -6H${w + 6}V${w - A * S}H${A * S}Z" fill="rgb(190 220 240 / .045)"/><path d="M0 -6V${w}H${w + 6}M${A * S} -6V${w - A * S}H${w + 6}" fill="none" stroke="#dceaf2" stroke-width="2.4"/><g transform="translate(${(pose[0] * S).toFixed(2)} ${(w - pose[1] * S).toFixed(2)})"><circle r="${diag * S}" fill="none" stroke="#fb7185" stroke-dasharray="3 5" opacity=".55"/><circle r="${(b / 2) * S}" fill="none" stroke="#6cc8f7" stroke-dasharray="3 5" opacity=".8"/><g transform="rotate(${deg.toFixed(2)})"><rect x="${(-L / 2) * S}" y="${(-b / 2) * S}" width="${L * S}" height="${b * S}" rx="4" fill="${tone}" fill-opacity="${stuck ? 0.42 : 0.24}" stroke="${tone}" stroke-width="2"/><path d="M${(L / 2) * S - 15} -5l8 5-8 5" fill="none" stroke="${tone}" stroke-width="1.8"/></g></g><text x="${A * S + 8}" y="10" style="font-size:7px">corridors 0.90 m</text>${stuck ? `<text x="${A * S + 8}" y="${w - A * S - 9}" style="fill:#fb7185;font-size:8px">jammed on the inner corner</text>` : ""}</svg>`;
  };
  const view = loop(plan, frame);
  const metres = (v: number) => `${v.toFixed(2)} m`;
  const moved = () => {
    later();
    view.poke();
  };
  const len = range(root, "L", metres, moved);
  const wide = range(root, "b", metres, moved);
  const psi = range(root, "psi", (v) => `${v}°`, () => {
    setFollow(false);
    slice();
  });
  const toggle = $<HTMLButtonElement>(root, "[data-toggle=follow]");
  const setFollow = (on: boolean) => {
    follow = on;
    toggle.setAttribute("aria-pressed", String(on));
  };
  toggle.addEventListener("click", () => {
    setFollow(!follow);
    slice();
    view.poke();
  });
  const res = seg(root, "res", later);
  solve();
  t = 0.45;
  view.poke();
}
