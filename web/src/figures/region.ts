import { $, clamp, fit, fmt, loop, range, reduced } from "./lib";

/** Footprints shown: half-length a and half-width b in metres. */
const AX = [0.2, 0.62] as const;
const BY = [0.08, 0.44] as const;
const PX = 0.15;
const PY = 0.075;

/** Necessary and sufficient, for equal ratings c and a payload W over the rectangle of §25.13. */
export function allocation(a: number, b: number, W: number, c: number) {
  const r = (W + (W * PX) / a) / 2;
  const s = (W + (W * PY) / b) / 2;
  const lo = Math.max(0, r - c, s - c, r + s - W);
  const hi = Math.min(c, r, s, c + r + s - W);
  return { r, s, lo, hi, ok: lo <= hi + 1e-9 };
}

interface Box {
  x: number;
  y: number;
  w: number;
  state: "in" | "out" | "open";
  born: number;
  depth: number;
}
const COLOUR = { in: "#4ade80", out: "#fb7185", open: "#fbbf24" };
const MAX = 9;
const L = 46;
const B = 40;

export default function mount(root: HTMLElement) {
  const canvas = $<HTMLCanvasElement>(root, "canvas");
  const g = canvas.getContext("2d")!;
  const info = $(root, "[data-info]");
  const tip = $(root, "[data-tip]");
  const refine = $<HTMLButtonElement>(root, "[data-refine]");
  let W = 1;
  let H = 1;
  let ratio = 1;
  let boxes: Box[] = [];
  let depth = 0;
  let picked: Box | undefined;
  let auto = 0;
  const at = (x: number, y: number) => [AX[0] + (AX[1] - AX[0]) * x, BY[0] + (BY[1] - BY[0]) * y] as const;

  const settle = (x: number, y: number, w: number, d: number): Box => {
    // A larger footprint only helps, so the two opposite corners decide the box.
    const [a0, b0] = at(x, y);
    const [a1, b1] = at(x + w, y + w);
    const state = allocation(a0, b0, load.value, cap.value).ok
      ? "in"
      : allocation(a1, b1, load.value, cap.value).ok
        ? "open"
        : "out";
    return { x, y, w, state, born: performance.now(), depth: d };
  };
  const totals = () => {
    const sum = { in: 0, out: 0, open: 0 };
    for (const b of boxes) sum[b.state] += b.w * b.w;
    $(root, "[data-in]").textContent = `${fmt(sum.in * 100, 1)}%`;
    $(root, "[data-out]").textContent = `${fmt(sum.out * 100, 1)}%`;
    $(root, "[data-open]").textContent = `${fmt(sum.open * 100, sum.open < 0.01 ? 2 : 1)}%`;
    refine.disabled = depth >= MAX || sum.open === 0;
  };
  const grow = () => {
    if (depth >= MAX) return false;
    const next: Box[] = [];
    let split = false;
    for (const b of boxes) {
      if (b.state !== "open") {
        next.push(b);
        continue;
      }
      split = true;
      const h = b.w / 2;
      for (const [dx, dy] of [
        [0, 0],
        [h, 0],
        [0, h],
        [h, h],
      ] as const)
        next.push(settle(b.x + dx, b.y + dy, h, b.depth + 1));
    }
    if (!split) return false;
    boxes = next;
    depth++;
    picked = undefined;
    totals();
    view.poke();
    return true;
  };
  const reset = (levels: number) => {
    boxes = [settle(0, 0, 1, 0)];
    depth = 0;
    picked = undefined;
    for (let i = 0; i < levels && grow(); i++);
    totals();
    view.poke();
  };
  const explain = (b: Box) => {
    const [a0, b0] = at(b.x, b.y);
    const [a1, b1] = at(b.x + b.w, b.y + b.w);
    const span = `a ${a0.toFixed(3)} to ${a1.toFixed(3)} m\nb ${b0.toFixed(3)} to ${b1.toFixed(3)} m`;
    info.className = `cellinfo ${b.state}`;
    if (b.state === "in") {
      const f = allocation(a0, b0, load.value, cap.value);
      const t = (f.lo + f.hi) / 2;
      const forces = [t, f.r - t, f.s - t, load.value - f.r - f.s + t].map((v) => v.toFixed(1)).join(", ");
      info.innerHTML = `<p class="micro">Works</p><pre>${span}\nhardest load, over one corner\nfeet carry ${forces} N\nlimit ${cap.value} N each</pre><p>Every stance in this box carries the load.</p>`;
    } else if (b.state === "out") {
      const f = allocation(a1, b1, load.value, cap.value);
      const need = f.r + f.s - load.value;
      const text =
        need > cap.value
          ? `one foot would have to carry ${need.toFixed(1)} N\nand its limit is ${cap.value} N`
          : f.r > 2 * cap.value
            ? `the front pair would have to carry ${f.r.toFixed(1)} N\nand their limit is ${cap.value} N each`
            : f.s > 2 * cap.value
              ? `one side would have to carry ${f.s.toFixed(1)} N\nand its limit is ${cap.value} N a foot`
              : "the load would be outside the feet";
      info.innerHTML = `<p class="micro">Can’t work</p><pre>${span}\neven at its best corner\n${text}</pre><p>No controller can balance any stance in this box.</p>`;
    } else {
      const [am, bm] = at(b.x + b.w / 2, b.y + b.w / 2);
      info.innerHTML = `<p class="micro">Too close to call</p><pre>${span}\nits best corner works, its worst doesn’t</pre><p>Cheapest next test: the center, a = ${am.toFixed(3)} m and b = ${bm.toFixed(3)} m. It settles at least a quarter of the box.</p>`;
    }
  };

  const draw = (_: number, now: number) => {
    const X = (x: number) => L + x * (W - L - 14);
    const Y = (y: number) => 14 + (1 - y) * (H - B - 14);
    g.setTransform(ratio, 0, 0, ratio, 0, 0);
    g.clearRect(0, 0, W, H);
    let moving = false;
    for (const b of boxes) {
      const age = reduced ? 1 : clamp((now - b.born) / 500, 0, 1);
      if (age < 1) moving = true;
      const x = X(b.x);
      const y = Y(b.y + b.w);
      const w = X(b.x + b.w) - x;
      const h = Y(b.y) - y;
      g.globalAlpha = (b.state === "open" ? 0.82 : 0.2) + (1 - age) * 0.5;
      g.fillStyle = COLOUR[b.state];
      g.fillRect(x, y, w, h);
      if (w > 5) {
        g.globalAlpha = 0.55;
        g.strokeStyle = b.state === "open" ? "#1b1300" : COLOUR[b.state];
        g.lineWidth = 0.8;
        g.strokeRect(x + 0.5, y + 0.5, w - 1, h - 1);
      }
    }
    g.globalAlpha = 1;
    g.font = "500 11px JetBrains Mono, monospace";
    g.fillStyle = "#98aba1";
    g.textAlign = "center";
    for (const a of [0.2, 0.3, 0.4, 0.5, 0.6]) g.fillText(a.toFixed(1), X((a - AX[0]) / (AX[1] - AX[0])), H - 22);
    g.fillText("half-length a, m", X(0.5), H - 6);
    g.textAlign = "right";
    for (const b of [0.1, 0.2, 0.3, 0.4]) g.fillText(b.toFixed(1), L - 8, Y((b - BY[0]) / (BY[1] - BY[0])) + 4);
    g.save();
    g.translate(12, Y(0.5));
    g.rotate(-Math.PI / 2);
    g.textAlign = "center";
    g.fillText("half-width b, m", 0, 0);
    g.restore();
    // The plan's own point, where it applies.
    if (load.value === 400 && cap.value === 160) {
      const x = X((1 / 3 - AX[0]) / (AX[1] - AX[0]));
      const y = Y((3 / 14 - BY[0]) / (BY[1] - BY[0]));
      g.strokeStyle = "#fff";
      g.lineWidth = 2;
      g.beginPath();
      g.arc(x, y, 6, 0, 7);
      g.stroke();
      g.fillStyle = "#fff";
      g.textAlign = "left";
      g.font = "600 12px Inter, sans-serif";
      g.fillText("a = 1/3, b = 3/14", x + 12, y + 4);
    }
    if (picked) {
      const x = X(picked.x);
      const y = Y(picked.y + picked.w);
      g.strokeStyle = "#fff";
      g.lineWidth = 2;
      g.strokeRect(x, y, Math.max(3, X(picked.x + picked.w) - x), Math.max(3, Y(picked.y) - y));
    }
    if (moving) view.poke();
  };
  const view = loop(canvas, draw, false);
  const cap = range(root, "c", (v) => `${v} N`, () => reset(Math.max(depth, 5)));
  const load = range(root, "w", (v) => `${v} N`, () => reset(Math.max(depth, 5)));
  fit(canvas, (w, h, r) => {
    W = w;
    H = h;
    ratio = r;
    view.poke();
  });
  refine.addEventListener("click", grow);
  $(root, "[data-reset]").addEventListener("click", () => {
    clearInterval(auto);
    reset(0);
  });
  $(root, "[data-auto]").addEventListener("click", () => {
    clearInterval(auto);
    if (depth >= MAX) reset(0);
    auto = window.setInterval(
      () => {
        if (!grow()) clearInterval(auto);
      },
      reduced ? 0 : 420,
    );
  });
  const point = (e: PointerEvent) => {
    const box = canvas.getBoundingClientRect();
    const x = (e.clientX - box.left - L) / (W - L - 14);
    const y = 1 - (e.clientY - box.top - 14) / (H - B - 14);
    if (x < 0 || x > 1 || y < 0 || y > 1) {
      tip.hidden = true;
      return;
    }
    const [a, b] = at(x, y);
    tip.hidden = false;
    tip.textContent = `a ${a.toFixed(3)}  b ${b.toFixed(3)}`;
    tip.style.left = `${clamp(e.clientX - box.left + 14, 4, W - tip.offsetWidth - 4)}px`;
    tip.style.top = `${clamp(e.clientY - box.top - 34, 4, H - 30)}px`;
    const hit = boxes.find((q) => x >= q.x && x <= q.x + q.w && y >= q.y && y <= q.y + q.w);
    if (hit && hit !== picked) {
      picked = hit;
      explain(hit);
      view.poke();
    }
  };
  canvas.addEventListener("pointermove", point);
  canvas.addEventListener("pointerdown", point);
  canvas.addEventListener("pointerleave", () => {
    tip.hidden = true;
  });
  reset(reduced ? 7 : 1);
  if (!reduced)
    new IntersectionObserver(
      (entries, io) => {
        if (!entries[0]!.isIntersecting) return;
        io.disconnect();
        const step = window.setInterval(() => {
          if (depth >= 5 || !grow()) clearInterval(step);
        }, 480);
      },
      { threshold: 0.35 },
    ).observe(canvas);
}
