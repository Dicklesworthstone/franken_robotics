import { $, clamp, fit, fmt, loop, mix, reduced, smooth, type Loop } from "./lib";
import { ANGLE, FELL, FLIGHT, Field, REST, STANCE, STIFF } from "./slip";

const N = 120;
/** Seconds between releases. */
const CYCLE = 44;

/** Survivors by leg stiffness, stiff to soft. */
const ALIVE = [
  [0xd9, 0xf7, 0x5a],
  [0x86, 0xef, 0x8c],
  [0x2d, 0xd4, 0xb0],
  [0x5f, 0xc0, 0xf7],
] as const;
/** The fallen by how many steps they took, most recent last. */
const FALLEN = [
  [0x0a, 0x12, 0x0f],
  [0x14, 0x20, 0x1a],
  [0x3a, 0x28, 0x1b],
  [0x86, 0x40, 0x22],
  [0xdd, 0x72, 0x33],
  [0xff, 0xc0, 0x5a],
] as const;
const ramp = (stops: readonly (readonly [number, number, number])[], t: number) => {
  const p = clamp(t, 0, 1) * (stops.length - 1);
  const i = Math.min(stops.length - 2, Math.floor(p));
  const f = p - i;
  const a = stops[i]!;
  const b = stops[i + 1]!;
  return [mix(a[0], b[0], f), mix(a[1], b[1], f), mix(a[2], b[2], f)] as const;
};

export default function mount(root: HTMLElement) {
  const map = $<HTMLCanvasElement>(root, "[data-map]");
  const close = $<HTMLCanvasElement>(root, "[data-close]");
  const m = map.getContext("2d", { alpha: false })!;
  const g = close.getContext("2d", { alpha: false })!;
  const time = $(root, "[data-time]");
  const standing = $(root, "[data-standing]");
  const where = $(root, "[data-view]");
  const probe = $(root, "[data-probe]");
  const pause = $<HTMLButtonElement>(root, "[data-pause]");
  const field = new Field(N, N);
  const tile = document.createElement("canvas");
  tile.width = tile.height = N;
  const tg = tile.getContext("2d")!;
  const img = tg.createImageData(N, N);
  const size = { m: [1, 1, 1], g: [1, 1, 1] } as Record<"m" | "g", [number, number, number]>;
  let clock = 0;
  let pointer: [number, number] | undefined;
  /** The close-up's centre on the map, in cells. */
  let cx = N * 0.45;
  let cy = N * 0.4;

  const aliveColour = Array.from({ length: N }, (_, row) => ramp(ALIVE, row / (N - 1)));
  const paintTile = () => {
    const d = img.data;
    const { state, y, steps, fellAt } = field;
    for (let i = 0, p = 0; i < field.count; i++, p += 4) {
      let c: readonly [number, number, number];
      let k = 1;
      if (state[i] === FELL) {
        const age = field.time - fellAt[i]!;
        c = ramp(FALLEN, Math.log2(1 + steps[i]!) / 5);
        const flash = Math.max(0, 1 - age / 0.35);
        d[p] = mix(c[0], 255, flash * 0.75);
        d[p + 1] = mix(c[1], 240, flash * 0.75);
        d[p + 2] = mix(c[2], 220, flash * 0.75);
      } else {
        c = aliveColour[(i / N) | 0]!;
        // The height of the mass makes the crescent shimmer with its own gait.
        k = clamp(0.55 + (y[i]! - 0.86) * 3.1, 0.42, 1.12);
        d[p] = c[0] * k;
        d[p + 1] = c[1] * k;
        d[p + 2] = c[2] * k;
      }
      d[p + 3] = 255;
    }
    tg.putImageData(img, 0, 0);
  };

  const runner = (i: number, x: number, y: number, s: number) => {
    const { state, foot, cos, sin, x: px, y: py, vy } = field;
    const st = state[i]!;
    const c = st === FELL ? ramp(FALLEN, Math.log2(1 + field.steps[i]!) / 5) : aliveColour[(i / N) | 0]!;
    const rgb = `rgb(${c[0] | 0} ${c[1] | 0} ${c[2] | 0})`;
    const ground = y + s * 0.82;
    const unit = s * 0.52;
    g.fillStyle = st === FELL ? "rgb(255 255 255 / .012)" : "rgb(120 255 170 / .035)";
    g.fillRect(x + 1, y + 1, s - 2, s - 2);
    // The ground scrolls at the runner's own speed.
    g.strokeStyle = "rgb(190 240 212 / .16)";
    g.lineWidth = 1;
    g.beginPath();
    g.moveTo(x + s * 0.08, ground);
    g.lineTo(x + s * 0.92, ground);
    const off = st === FELL ? 0 : ((px[i]! * unit) % (s * 0.14)) as number;
    for (let m = 0; m < 7; m++) {
      const gx = x + s * 0.1 + ((m * s * 0.14 - off + s * 2) % (s * 0.82));
      g.moveTo(gx, ground);
      g.lineTo(gx - s * 0.04, ground + s * 0.05);
    }
    g.stroke();
    if (st === FELL) {
      g.strokeStyle = rgb;
      g.lineWidth = Math.max(1.4, s * 0.03);
      g.beginPath();
      g.moveTo(x + s * 0.36, ground - s * 0.05);
      g.lineTo(x + s * 0.64, ground - s * 0.05);
      g.stroke();
      g.fillStyle = rgb;
      g.beginPath();
      g.arc(x + s * 0.66, ground - s * 0.08, s * 0.07, 0, 7);
      g.fill();
      return;
    }
    const bx = x + s * 0.5;
    const by = ground - py[i]! * unit;
    let fx = bx + REST * cos[i]! * unit;
    let fy = by + REST * sin[i]! * unit;
    if (st === STANCE) {
      fx = bx + (foot[i]! - px[i]!) * unit;
      fy = ground;
    } else if (st === FLIGHT) {
      // The leg swings from its take-off pose to its landing pose on the way over the top.
      const lead = smooth(clamp(0.5 - vy[i]! / 4, 0, 1));
      const a = mix(Math.PI - Math.atan2(sin[i]!, cos[i]!), Math.atan2(sin[i]!, cos[i]!), lead);
      const len = REST * (1 - 0.22 * Math.sin(Math.PI * lead));
      fx = bx + Math.cos(a) * len * unit;
      fy = by + Math.sin(a) * len * unit;
    }
    // A spring drawn as a spring, so its compression shows.
    const dx = fx - bx;
    const dy = fy - by;
    const l = Math.hypot(dx, dy) || 1;
    const nx = -dy / l;
    const ny = dx / l;
    g.strokeStyle = rgb;
    g.lineWidth = Math.max(1.2, s * 0.024);
    g.lineJoin = "round";
    g.beginPath();
    g.moveTo(bx, by);
    const turns = 9;
    for (let j = 1; j < turns; j++) {
      const f = 0.16 + (0.68 * j) / turns;
      const side = (j % 2 ? 1 : -1) * s * 0.034;
      g.lineTo(bx + dx * f + nx * side, by + dy * f + ny * side);
    }
    g.lineTo(fx, fy);
    g.stroke();
    g.fillStyle = rgb;
    g.beginPath();
    g.arc(fx, fy, s * 0.028, 0, 7);
    g.fill();
    g.fillStyle = "#f4fff8";
    g.beginPath();
    g.arc(bx, by, s * 0.088, 0, 7);
    g.fill();
    g.fillStyle = rgb;
    g.beginPath();
    g.arc(bx, by, s * 0.062, 0, 7);
    g.fill();
  };

  /** The surviving column range of a row, once enough runners have fallen to show it. */
  const edges = (r: number) => {
    let first = -1;
    let last = -1;
    for (let c = 0; c < N; c++)
      if (field.state[r * N + c] !== FELL) {
        if (first < 0) first = c;
        last = c;
      }
    return [first, last] as const;
  };
  /** Unattended, the close-up follows the edge of the crescent, where both outcomes are in view. */
  const wander = (dt: number) => {
    let tx = N * 0.5;
    let ty = N * 0.42;
    if (pointer) [tx, ty] = pointer;
    else if (clock > 3) {
      const u = 0.5 - 0.5 * Math.cos((clock - 3) * 0.16);
      ty = mix(N * 0.2, N * 0.74, u);
      const [first, last] = edges(Math.round(ty));
      tx = first < 0 ? tx : Math.floor((clock - 3) / 19.6) % 2 ? last : first;
    }
    const k = reduced ? 1 : 1 - Math.exp(-dt * (pointer ? 9 : 1.1));
    cx = mix(cx, tx, k);
    cy = mix(cy, ty, k);
  };

  const draw = () => {
    paintTile();
    // The close-up.
    const [W, H, R] = size.g;
    const across = W < 420 ? 5 : W < 640 ? 6 : 7;
    const cell = W / across;
    const down = H / cell;
    const x = clamp(cx + 0.5 - across / 2, 0, N - across);
    const y = clamp(cy + 0.5 - down / 2, 0, N - down);
    g.setTransform(R, 0, 0, R, 0, 0);
    g.fillStyle = "#040806";
    g.fillRect(0, 0, W, H);
    for (let r = Math.floor(y); r < Math.min(N, y + down); r++)
      for (let c = Math.floor(x); c < Math.min(N, x + across); c++) runner(r * N + c, (c - x) * cell, (r - y) * cell, cell);
    // The whole family.
    const [MW, MH, MR] = size.m;
    m.setTransform(MR, 0, 0, MR, 0, 0);
    m.imageSmoothingEnabled = false;
    m.drawImage(tile, 0, 0, MW, MH);
    const px = MW / N;
    m.lineWidth = 3.5;
    m.strokeStyle = "rgb(3 7 5 / .7)";
    m.strokeRect(x * px, y * px, across * px, down * px);
    m.lineWidth = 1.5;
    m.strokeStyle = "#fff";
    m.strokeRect(x * px, y * px, across * px, down * px);

    time.textContent = `${field.time.toFixed(1)} s`;
    standing.textContent = fmt(field.standing);
    const angle = (c: number) => ANGLE[0] + ((ANGLE[1] - ANGLE[0]) * (c + 0.5)) / N;
    const stiff = (r: number) => STIFF[1] - ((STIFF[1] - STIFF[0]) * (r + 0.5)) / N;
    where.textContent = `${angle(x + across / 2 - 0.5).toFixed(0)}°, ${stiff(y + down / 2 - 0.5).toFixed(0)} kN/m`;
    probe.hidden = !pointer;
    if (pointer) {
      const c = clamp(Math.floor(pointer[0]), 0, N - 1);
      const r = clamp(Math.floor(pointer[1]), 0, N - 1);
      const i = r * N + c;
      const n = field.steps[i]!;
      probe.textContent = `angle  ${angle(c).toFixed(1)}°\nspring ${stiff(r).toFixed(1)} kN/m\n${
        field.state[i] === FELL ? `fell after ${n} ${n === 1 ? "step" : "steps"}` : `running, ${n} steps`
      }`;
      probe.style.left = `${clamp(pointer[0] * px + 16, 6, Math.max(6, MW - probe.offsetWidth - 6))}px`;
      probe.style.top = `${clamp(pointer[1] * px + 16, 6, MH - probe.offsetHeight - 6)}px`;
    }
  };

  const step = (dt: number) => {
    clock += dt;
    const rate = clock < 6 ? 0.8 : 0.5;
    field.advance(dt * rate, Math.max(4, Math.round(dt * rate * 900)));
  };
  const again = () => {
    clock = 0;
    field.release();
  };

  const film: Loop = loop(root, (dt) => {
    if (film.running) {
      step(dt);
      if (clock >= CYCLE) again();
    }
    wander(dt);
    draw();
  });
  fit(map, (w, h, r) => {
    size.m = [w, h, r];
    film.poke();
  });
  fit(close, (w, h, r) => {
    size.g = [w, h, r];
    film.poke();
  });
  if (reduced) {
    for (let i = 0; i < 260; i++) field.advance(0.05, 45);
    clock = 13;
    pause.setAttribute("aria-pressed", "true");
    film.poke();
  }
  pause.addEventListener("click", () => {
    film.running = !film.running;
    pause.setAttribute("aria-pressed", String(!film.running));
    pause.setAttribute("aria-label", film.running ? "Pause" : "Play");
  });
  $(root, "[data-again]").addEventListener("click", () => {
    again();
    film.running = true;
    pause.setAttribute("aria-pressed", "false");
  });
  const at = (event: PointerEvent) => {
    const box = map.getBoundingClientRect();
    pointer = [((event.clientX - box.left) / box.width) * N, ((event.clientY - box.top) / box.height) * N];
    film.poke();
  };
  map.addEventListener("pointermove", at);
  map.addEventListener("pointerdown", at);
  for (const type of ["pointerleave", "pointercancel"] as const)
    map.addEventListener(type, () => {
      pointer = undefined;
      film.poke();
    });
}
