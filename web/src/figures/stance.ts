import { $, $$, Q, clamp, q, range, seg } from "./lib";

const W = q(400);
const A = q(1, 3);
const RATING = q(160);
const PX = q(60);
const PY = q(30);
const ZERO = q(0);
const max = (...xs: Q[]) => xs.reduce((a, b) => (a.cmp(b) >= 0 ? a : b));
const min = (...xs: Q[]) => xs.reduce((a, b) => (a.cmp(b) <= 0 ? a : b));

/** Forces [++, +−, −+, −−] for a moment (px, py), or the interval that came out empty. */
export function balance(b: Q, px: Q, py: Q) {
  const r = W.add(px.div(A)).div(q(2));
  const s = W.add(py.div(b)).div(q(2));
  const lo = max(ZERO, r.sub(RATING), s.sub(RATING), r.add(s).sub(W));
  const hi = min(RATING, r, s, RATING.add(r).add(s).sub(W));
  const t = lo.add(hi).div(q(2));
  return { r, s, lo, hi, ok: lo.cmp(hi) <= 0, f: [t, r.sub(t), s.sub(t), W.sub(r).sub(s).add(t)] };
}
/** Four equal springs on rigid ground leave nothing to choose. */
export function springs(b: Q, px: Q, py: Q) {
  const x = px.div(A).div(q(4));
  const y = py.div(b).div(q(4));
  const m = q(100);
  return [m.add(x).add(y), m.add(x).sub(y), m.sub(x).add(y), m.sub(x).sub(y)];
}
export const EDGE = q(3, 14);
const EDGE_MM = (3 / 14) * 1000;
const isEdge = (mm: number) => Math.abs(mm - EDGE_MM) < 0.01;
export const width = (mm: number) => (isEdge(mm) ? EDGE : q(Math.round(mm), 1000));

/** The lines of the argument, as the plan writes them. */
export function argue(b: Q, passive: boolean): { covers: boolean; proof: string[] } {
  const corner = balance(b, PX, PY);
  const sc = springs(b, PX, PY);
  const proof = ["corner (+, +): W = 400, P = (60, 30)"];
  if (passive) {
    const covers = sc[0]!.cmp(RATING) <= 0 && sc[3]!.cmp(ZERO) >= 0;
    proof.push("equal springs: 100 ± 60/4a ± 30/4b", `f = (${sc.join(", ")}) N`);
    proof.push(covers ? `largest ${sc[0]} N, rating 160 N` : `${sc[0]} N on one foot, rating 160 N`);
    if (corner.ok && !covers) proof.push("commanded feet pass this body,", "so that verdict is withdrawn and", "the edited assumption is named");
    return { covers, proof };
  }
  if (corner.ok) {
    proof.push(
      `r = ${corner.r}, s = 200 + 15/b = ${corner.s}`,
      `${corner.lo} ≤ t ≤ ${corner.hi}`,
      `f = (${corner.f.join(", ")}) N`,
      "sum 400, every foot in [0, 160]",
      corner.lo.cmp(corner.hi) === 0 ? "zero reserve: the exact edge" : `t may vary by ${corner.hi.sub(corner.lo)} N`,
    );
    return { covers: true, proof };
  }
  proof.push(
    `r = ${corner.r}, s = 200 + 15/b = ${corner.s}`,
    `last foot ≥ 0 needs t ≥ ${corner.lo}`,
    "first foot allows t ≤ 160",
    `dual (0, ½, ½): ${corner.lo} > 160`,
  );
  return { covers: false, proof };
}

export default function mount(root: HTMLElement) {
  const view = $(root, "[data-view]");
  let px = 150;
  let py = 75;
  const draw = () => {
    const b = width(slider.value);
    const passive = mode.value === "springs";
    // A payload offset of x millimetres on 400 N is a moment of 0.4·x N·m.
    const mx = q(px * 2, 5);
    const my = q(py * 2, 5);
    const here = balance(b, mx, my);
    const f = passive ? springs(b, mx, my) : here.f;
    const { covers, proof } = argue(b, passive);
    const cert = $(root, ".cert");
    cert.classList.toggle("in", covers);
    cert.classList.toggle("out", !covers);
    $(root, "[data-head]").textContent = `b = ${b} m, payload anywhere in the rectangle`;
    $(root, "[data-verdict]").textContent = covers
      ? "Covers the whole payload rectangle"
      : passive
        ? "The springs overload a foot"
        : "Excluded for every controller";
    $(root, "[data-proof]").textContent = proof.join("\n");
    for (const p of $$<HTMLButtonElement>(root, "[data-b]"))
      p.setAttribute("aria-pressed", String(Math.abs(Number(p.dataset.b) - slider.value) < 0.01));

    const bm = b.value * 1000;
    const am = 1000 / 3;
    const feet = [
      [am, -bm],
      [am, bm],
      [-am, -bm],
      [-am, bm],
    ] as const;
    const at = (i: number) => feet[i]!.join(" ");
    const k = 960 / Math.max(280, view.clientWidth);
    const small = Math.max(15, 11.5 * k).toFixed(1);
    const large = Math.max(19, 12.5 * k);
    let svg = `<rect x="${-am - 10}" y="${-bm + 30}" width="${2 * am + 20}" height="${2 * bm - 60}" rx="46" fill="#0e1b15" stroke="rgb(190 240 212 / .3)"/><path d="M${at(0)}L${at(1)}L${at(3)}L${at(2)}Z" fill="none" stroke="rgb(190 240 212 / .25)" stroke-dasharray="5 6"/>`;
    svg += `<rect x="-150" y="-75" width="300" height="150" rx="5" fill="rgb(198 242 90 / .07)" stroke="#c6f25a" stroke-opacity=".6"/><text x="0" y="62" text-anchor="middle" style="font-size:${small}px">${k > 1.7 ? "payload" : "payload rectangle"}</text>`;
    const ringR = (v: number) => 12 + Math.sqrt(Math.max(0, v)) * 2.5;
    for (const [i, [x, y]] of feet.entries()) {
      const v = f[i]!;
      const bad = v.cmp(RATING) > 0 || v.cmp(ZERO) < 0 || (!passive && !here.ok);
      const tone = bad ? "#fb7185" : "#4ade80";
      svg += `<circle cx="${x}" cy="${y}" r="${ringR(160)}" fill="none" stroke="rgb(190 240 212 / .22)" stroke-dasharray="2 4"/><circle cx="${x}" cy="${y}" r="${ringR(v.value)}" fill="${tone}" fill-opacity=".24" stroke="${tone}" stroke-width="2"/><text class="label" x="${x}" y="${(y + large * 0.34).toFixed(1)}" text-anchor="middle" style="font-size:${large.toFixed(1)}px">${v.d === 1n ? v.toString() : v.value.toFixed(1)}</text>`;
    }
    svg += `<path d="M0 ${-py}H${px}M${px} 0V${-py}" stroke="#c6f25a" stroke-opacity=".4" fill="none"/><circle data-com cx="${px}" cy="${-py}" r="17" fill="#c6f25a" stroke="#030705" stroke-width="4" tabindex="0" role="slider" aria-label="Payload position" aria-valuenow="${px}" aria-valuemin="-150" aria-valuemax="150" aria-valuetext="${px} mm along, ${py} mm across" style="cursor:grab"/>`;
    svg += `<path d="M${am + 88} ${-bm}V${bm}m-7 0h14M${am + 81} ${-bm}h14" stroke="#98aba1" fill="none"/><text x="526" y="${-bm - 16}" text-anchor="end" style="font-size:${small}px">${(2 * bm).toFixed(0)} mm</text>`;
    const focused = document.activeElement?.hasAttribute("data-com") && root.contains(document.activeElement);
    view.innerHTML = `<svg class="chart" viewBox="-430 -375 960 750" role="group" aria-label="Robot from above, with the force on each foot in newtons">${svg}</svg>`;
    if (focused) $<SVGElement>(view, "[data-com]").focus({ preventScroll: true });
  };
  const place = (x: number, y: number) => {
    px = clamp(Math.round(x), -150, 150);
    py = clamp(Math.round(y), -75, 75);
    draw();
  };
  const where = (e: PointerEvent) => {
    const box = $<SVGSVGElement>(view, "svg").getBoundingClientRect();
    const k = 960 / box.width;
    return [(e.clientX - box.left) * k - 430, -((e.clientY - box.top) * k - 375)] as const;
  };
  let held = false;
  view.addEventListener("pointerdown", (e) => {
    const [x, y] = where(e);
    if (Math.abs(x) > 190 || Math.abs(y) > 115) return;
    e.preventDefault();
    held = true;
    view.setPointerCapture(e.pointerId);
    place(x, y);
  });
  view.addEventListener("pointermove", (e) => {
    if (held) place(...where(e));
  });
  for (const t of ["pointerup", "pointercancel"] as const)
    view.addEventListener(t, () => {
      held = false;
    });
  view.addEventListener("keydown", (e) => {
    const d = { ArrowLeft: [-10, 0], ArrowRight: [10, 0], ArrowUp: [0, 5], ArrowDown: [0, -5] }[e.key];
    if (!d || !(e.target as Element).hasAttribute("data-com")) return;
    e.preventDefault();
    place(px + d[0]!, py + d[1]!);
  });
  const slider = range(root, "b", (v) => (isEdge(v) ? "3/14 m" : `${Math.round(v)} mm`), draw);
  const mode = seg(root, "share", draw);
  for (const p of $$<HTMLButtonElement>(root, "[data-b]"))
    p.addEventListener("click", () => {
      slider.set(Number(p.dataset.b));
      draw();
    });
  draw();
}
