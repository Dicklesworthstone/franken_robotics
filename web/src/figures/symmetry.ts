import { $, toggle } from "./lib";

interface Feature {
  kind: string;
  x: number;
  y: number;
  /** A direction attached to the feature, as a vector. */
  dx?: number;
  dy?: number;
  /** Counts only for what the robot senses, carries and is told to do. */
  observed?: boolean;
}
const ISO = [
  { name: "identity", f: (x: number, y: number) => [x, y] },
  { name: "left to right", f: (x: number, y: number) => [x, -y] },
  { name: "front to back", f: (x: number, y: number) => [-x, y] },
  { name: "half turn", f: (x: number, y: number) => [-x, -y] },
] as const;
const LEGS = [
  [1, -1],
  [1, 1],
  [-1, -1],
  [-1, 1],
] as const;

/** The isometries that carry every feature to one of the same kind, place and direction. */
export function stabilizer(features: Feature[]) {
  const near = (a: number, b: number) => Math.abs(a - b) < 1e-9;
  return ISO.filter(({ f }) =>
    features.every((p) => {
      const [x, y] = f(p.x, p.y);
      const [dx, dy] = f(p.dx ?? 0, p.dy ?? 0);
      return features.some(
        (o) => o.kind === p.kind && near(o.x, x!) && near(o.y, y!) && near(o.dx ?? 0, dx!) && near(o.dy ?? 0, dy!),
      );
    }),
  ).map((g) => g.name);
}
const groupName = (elements: string[]) => ["", "1", "C₂", "", "C₂ × C₂"][elements.length]!;

export default function mount(root: HTMLElement) {
  const view = $(root, "[data-view]");
  const draw = () => {
    const features: Feature[] = [];
    for (const [x, y] of LEGS) {
      features.push({ kind: "leg", x: x * 100, y: y * 62, dx: knees.on ? -1 : -x });
      features.push({ kind: gear.on && x === 1 && y === 1 ? "gear 7:1" : "gear 9:1", x: x * 100, y: y * 62 });
    }
    const body = [...features];
    if (lidar.on) features.push({ kind: "lidar", x: 112, y: 0, dx: 1, observed: true });
    if (payload.on) features.push({ kind: "payload", x: -10, y: 26, observed: true });
    if (goal.on) features.push({ kind: "goal", x: 0, y: 0, dx: 1, observed: true });
    const mech = stabilizer(body);
    const obs = stabilizer(features);
    const say =
      obs.length === 4
        ? "Nothing it senses tells front from back or left from right. Weights may be tied four ways."
        : obs.length === 2
          ? `One symmetry survives: ${obs[1]}. Tests may still use ${mech.length === 4 ? "all four" : "both"} on the mechanism; a policy may tie weights only along this one.`
          : mech.length > 1
            ? "Nothing survives in what it observes, though the mechanism itself is still symmetric. Mirrored tests are fine and tied weights are not."
            : "Neither group is left. A symmetry declared by the author would be refused at stage 5 of preflight.";
    $(root, "[data-mech]").textContent = groupName(mech);
    $(root, "[data-obs]").textContent = groupName(obs);
    $(root, "[data-mech-n]").textContent = mech.length === 1 ? "identity only" : mech.slice(1).join(", ");
    $(root, "[data-obs-n]").textContent = obs.length === 1 ? "identity only" : obs.slice(1).join(", ");
    $(root, "[data-say]").textContent = say;

    let svg = "";
    const axis = (d: string, name: string) => {
      const on = obs.includes(name as (typeof obs)[number]);
      const mechOn = mech.includes(name as (typeof mech)[number]);
      svg += `<path d="${d}" fill="none" stroke="${on ? "#4ade80" : mechOn ? "#6cc8f7" : "#fb7185"}" stroke-width="${on ? 2 : 1.2}" stroke-dasharray="${on ? "none" : "4 5"}" opacity="${on ? 1 : mechOn ? 0.75 : 0.3}"/>`;
    };
    axis("M-190 0H190", "left to right");
    axis("M0 -135V135", "front to back");
    svg += `<rect x="-112" y="-40" width="224" height="80" rx="26" fill="#0e1b15" stroke="rgb(190 240 212 / .4)"/><rect x="-70" y="-24" width="140" height="48" rx="14" fill="none" stroke="rgb(190 240 212 / .16)"/>`;
    if (goal.on) svg += `<path d="M-40 0H44M28 -11 46 0 28 11" fill="none" stroke="#c6f25a" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>`;
    for (const f of features) {
      if (f.kind === "leg") {
        const out = Math.sign(f.y);
        const kx = f.x + f.dx! * 22;
        svg += `<path d="M${f.x} ${f.y - out * 20}V${f.y + out * 12}L${kx} ${f.y + out * 30}L${f.x} ${f.y + out * 48}" fill="none" stroke="#c9d8cf" stroke-width="7" stroke-linejoin="round" stroke-linecap="round"/><circle cx="${f.x}" cy="${f.y + out * 48}" r="7" fill="#030705" stroke="#c9d8cf" stroke-width="2"/>`;
      } else if (f.kind.startsWith("gear")) {
        const odd = f.kind !== "gear 9:1";
        svg += `<circle cx="${f.x}" cy="${f.y - Math.sign(f.y) * 4}" r="12" fill="${odd ? "#fbbf24" : "#1b2621"}" stroke="${odd ? "#fbbf24" : "#9aa9a1"}" stroke-width="2"/>`;
        if (odd) svg += `<text x="${f.x + 22}" y="${f.y - 4}" style="fill:#fbbf24">${f.kind.slice(5)}</text>`;
      } else if (f.kind === "lidar")
        svg += `<path d="M112 0 182 -50A86 86 0 0 1 182 50Z" fill="rgb(108 200 247 / .14)" stroke="rgb(108 200 247 / .55)"/><circle cx="112" cy="0" r="9" fill="#6cc8f7"/>`;
      else if (f.kind === "payload")
        svg += `<rect x="-42" y="6" width="64" height="40" rx="7" fill="#b7a2fb" opacity=".9"/><text x="-10" y="31" text-anchor="middle" style="fill:#150c2e;font-weight:700">payload</text>`;
    }
    view.innerHTML = `<svg class="chart" viewBox="-200 -150 400 300" role="img" aria-label="Top view of the quadruped with its mirror lines">${svg}</svg>`;
  };
  const lidar = toggle(root, "lidar", draw);
  const payload = toggle(root, "payload", draw);
  const goal = toggle(root, "goal", draw);
  const knees = toggle(root, "knees", draw);
  const gear = toggle(root, "gear", draw);
  draw();
}
