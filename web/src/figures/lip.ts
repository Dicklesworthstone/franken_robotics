import { $, $$, onWidth, range } from "./lib";

const LOAD = 25;
const WHEELS = [
  { r: 60, colour: "#6cc8f7" },
  { r: 75, colour: "#fbbf24" },
] as const;

/** Quasi-static climb of a sharp lip: lever arm, lift arm, torque and the friction it needs. */
export function climb(r: number, h: number) {
  const a = Math.sqrt(2 * r * h - h * h);
  const b = r - h;
  return { a, b, torque: (LOAD * a) / 1000, mu: a / b };
}

export default function mount(root: HTMLElement) {
  const view = $(root, "[data-view]");
  const rows = $(root, "[data-rows]");
  let width = 800;
  const draw = () => {
    const lip = h.value;
    const S = Math.min(2.1, width / 400);
    const ground = 210 * S * 0.9;
    const H = ground + 52;
    let svg = `<defs><linearGradient id="lip-f" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#16303d"/><stop offset="1" stop-color="#0a161d"/></linearGradient></defs>`;
    rows.innerHTML = "";
    for (const [i, { r, colour }] of WHEELS.entries()) {
      const { a, b, torque, mu } = climb(r, lip);
      const torqueOk = torque <= cap.value + 1e-12;
      const grip = muS.value >= mu - 1e-12;
      const ok = torqueOk && grip;
      const x0 = (width * (i + 0.5)) / 2;
      const cx = x0 - 24 * S;
      const cy = ground - r * S;
      const ex = cx + a * S;
      const ey = ground - lip * S;
      const left = (width * i) / 2 + 8;
      const right = (width * (i + 1)) / 2 - 8;
      svg += `<path d="M${left} ${ground}H${ex}V${ey}H${right}V${ground + 22}H${left}Z" fill="url(#lip-f)"/><path d="M${left} ${ground}H${ex}V${ey}H${right}" fill="none" stroke="#c7dbe6" stroke-width="2"/>`;
      svg += `<circle cx="${cx}" cy="${cy}" r="${r * S}" fill="#0c171c" stroke="${colour}" stroke-width="3"/><circle cx="${cx}" cy="${cy}" r="${r * S * 0.8}" fill="none" stroke="${colour}" stroke-opacity=".28"/>`;
      // The right triangle that decides it.
      svg += `<path d="M${cx} ${cy}L${ex} ${ey}L${cx} ${ey}Z" fill="${colour}" fill-opacity=".13" stroke="${colour}" stroke-opacity=".55"/>`;
      svg += `<path d="M${cx} ${ey}H${ex}" stroke="${colour}" stroke-width="3"/><path d="M${cx} ${cy}V${ey}" stroke="#fff" stroke-opacity=".75" stroke-width="2.5"/>`;
      svg += `<text x="${(cx + ex) / 2}" y="${ey - 7}" text-anchor="middle" style="fill:${colour}">a ${a.toFixed(1)}</text><text x="${cx - 8}" y="${(cy + ey) / 2 + 4}" text-anchor="end">r−h ${b.toFixed(0)}</text>`;
      svg += `<path d="M${cx} ${cy}V${cy + 34 * S}m-5 -9 5 9 5 -9" fill="none" stroke="#fff" stroke-width="2"/><circle cx="${cx}" cy="${cy}" r="5" fill="#fff"/><circle cx="${ex}" cy="${ey}" r="4.5" fill="${ok ? "#4ade80" : "#fb7185"}"/>`;
      svg += `<text class="label" x="${x0}" y="${H - 22}" text-anchor="middle">${r} mm wheel</text><text x="${x0}" y="${H - 5}" text-anchor="middle" style="fill:${ok ? "#4ade80" : "#fb7185"}">${ok ? "gets over" : !grip && !torqueOk ? "slips, and stalls" : !grip ? "slips on the edge" : "stalls"}</text>`;
      rows.innerHTML += `<tr><td style="color:${colour}">${r} mm</td><td>${a.toFixed(1)} mm</td><td class="${torqueOk ? "yes" : "no"}">${torque.toFixed(3)} N·m</td><td class="${grip ? "yes" : "no"}">${mu.toFixed(4)}</td><td class="${ok ? "yes" : "no"}">${ok ? "gets over" : "stuck"}</td></tr>`;
    }
    view.innerHTML = `<svg class="chart" viewBox="0 0 ${width} ${H}" role="img" aria-label="Two wheels against the same lip, each with the triangle that sets its torque and friction">${svg}</svg>`;
  };
  const muS = range(root, "mu", (v) => v.toFixed(2), draw);
  const cap = range(root, "cap", (v) => `${v.toFixed(2)} N·m`, draw);
  const h = range(root, "h", (v) => `${v} mm`, draw);
  for (const b of $$<HTMLButtonElement>(root, "[data-set]"))
    b.addEventListener("click", () => {
      const [m, c] = b.dataset.set!.split(",").map(Number);
      muS.set(m!);
      cap.set(c!);
      h.set(12);
      draw();
    });
  onWidth(view, (w) => {
    width = w;
    draw();
  });
}
