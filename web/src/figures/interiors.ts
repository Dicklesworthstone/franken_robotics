import { rng } from "./lib";

/** A small generator for the seven interiors of §19.7. Metres; x to the right, z towards the viewer, y up. */
export type Finish =
  | "floor"
  | "tile"
  | "wall"
  | "wood"
  | "fabric"
  | "steel"
  | "shelf"
  | "stock"
  | "stock2"
  | "stock3"
  | "dark"
  | "glass"
  | "green"
  | "belt"
  | "mat"
  | "hazard";
export interface Block {
  x: number;
  z: number;
  y?: number;
  w: number;
  d: number;
  h: number;
  f: Finish;
  turn?: number;
  /** Round things: a cylinder or a ball that fills the block's box. */
  s?: "cyl" | "ball";
}
export interface Hazard {
  name: string;
  size: string;
  effect: string;
  x: number;
  z: number;
  y?: number;
}
export interface Interior {
  w: number;
  d: number;
  blocks: Block[];
  hazards: Hazard[];
  /** What §19.7 says to expose when a robot fails here. */
  expose: string;
}
type U = () => number;
const WALL = 1.15;

class Plan {
  blocks: Block[] = [];
  hazards: Hazard[] = [];
  constructor(
    readonly w: number,
    readonly d: number,
    readonly u: U,
    floor: Finish = "floor",
  ) {
    this.add(0, 0, w + 0.24, d + 0.24, 0.1, floor, -0.1);
    // Only the two far walls are drawn at height, so the room reads from the front.
    this.add(0, -d / 2 - 0.06, w + 0.24, 0.12, WALL, "wall");
    this.add(-w / 2 - 0.06, 0, 0.12, d, WALL, "wall");
    this.add(0, d / 2 + 0.06, w + 0.24, 0.12, 0.12, "wall");
    this.add(w / 2 + 0.06, 0, 0.12, d, 0.12, "wall");
  }
  add(x: number, z: number, w: number, d: number, h: number, f: Finish, y = 0, turn = 0, s?: "cyl" | "ball") {
    this.blocks.push(s ? { x, z, w, d, h, f, y, turn, s } : { x, z, w, d, h, f, y, turn });
  }

  /* Furniture. None of it draws a random number, so a seed gives the same room it always gave. */

  /** A top on four legs with an apron under it, and a low shelf if asked. */
  table(x: number, z: number, w: number, d: number, h: number, f: Finish = "wood", legs: Finish = f, shelf = false) {
    this.add(x, z, w, d, 0.04, f, h - 0.04);
    this.add(x, z, w - 0.12, d - 0.12, 0.07, f, h - 0.11);
    for (const sx of [-1, 1]) for (const sz of [-1, 1]) this.add(x + sx * (w / 2 - 0.06), z + sz * (d / 2 - 0.06), 0.05, 0.05, h - 0.04, legs);
    if (shelf) this.add(x, z, w - 0.1, d - 0.1, 0.025, legs, 0.16);
  }
  /** A chair whose back is on the side (bx, bz) points to. */
  chair(x: number, z: number, bx: number, bz: number) {
    this.add(x, z, 0.42, 0.42, 0.05, "fabric", 0.4);
    for (const sx of [-1, 1]) for (const sz of [-1, 1]) this.add(x + sx * 0.18, z + sz * 0.18, 0.035, 0.035, 0.4, "wood");
    if (Math.abs(bx) > Math.abs(bz)) this.add(x + Math.sign(bx) * 0.19, z, 0.04, 0.4, 0.42, "wood", 0.45);
    else this.add(x, z + Math.sign(bz) * 0.19, 0.4, 0.04, 0.42, "wood", 0.45);
  }
  /** A sofa with its back on the far side along z (−1) or the near side (1): arms, seat cushions and back cushions. */
  sofa(x: number, z: number, w: number, d: number, back: 1 | -1) {
    const arm = 0.16;
    const deep = 0.22;
    this.add(x, z, w - 0.08, d - 0.08, 0.06, "dark");
    this.add(x, z, w, d, 0.2, "fabric", 0.06);
    this.add(x, z + back * (d / 2 - deep / 2), w, deep, 0.56, "fabric", 0.26);
    for (const sx of [-1, 1]) this.add(x + sx * (w / 2 - arm / 2), z, arm, d, 0.34, "fabric", 0.26);
    const inner = w - 2 * arm;
    const n = inner > 1.3 ? 3 : 2;
    for (let k = 0; k < n; k++) {
      const cx = x - inner / 2 + (inner / n) * (k + 0.5);
      this.add(cx, z - (back * deep) / 2, inner / n - 0.025, d - deep - 0.02, 0.13, "stock2", 0.26);
      this.add(cx, z + back * (d / 2 - deep - 0.06), inner / n - 0.04, 0.12, 0.34, "stock2", 0.39);
    }
  }
  /** A bed with its head on the far side along z (−1) or the near side (1). */
  bed(x: number, z: number, w: number, d: number, head: 1 | -1) {
    this.add(x, z, w, d, 0.24, "wood");
    this.add(x, z, w - 0.06, d - 0.06, 0.18, "shelf", 0.24);
    this.add(x, z - head * d * 0.17, w - 0.02, d * 0.66, 0.05, "fabric", 0.41);
    this.add(x, z + head * (d * 0.16 - 0.05), w - 0.02, 0.12, 0.07, "stock2", 0.41);
    for (const sx of [-1, 1]) this.add(x + (sx * w) / 4, z + head * (d / 2 - 0.24), w / 2 - 0.12, 0.3, 0.1, "shelf", 0.42);
    this.add(x, z + head * (d / 2 + 0.04), w + 0.06, 0.08, 0.9, "wood");
  }
  /** A base cabinet on a recessed plinth, under a worktop. */
  counter(x: number, z: number, w: number, d: number, f: Finish) {
    this.add(x, z, w, d - 0.08, 0.08, "dark");
    this.add(x, z, w, d, 0.78, f, 0.08);
    this.add(x, z, w + 0.02, d + 0.02, 0.04, "dark", 0.86);
  }
  /** An office desk: a top on two end panels with a panel across the back. */
  desk(x: number, z: number, w: number, d: number, h: number) {
    this.add(x, z, w, d, 0.03, "wood", h - 0.03);
    for (const sx of [-1, 1]) this.add(x + sx * (w / 2 - 0.03), z, 0.04, d - 0.04, h - 0.03, "dark");
    this.add(x, z - d / 2 + 0.05, w - 0.1, 0.03, 0.36, "dark", h - 0.42);
  }
  /** A monitor on its stand, its lit face towards +z. */
  monitor(x: number, z: number, y: number) {
    this.add(x, z, 0.2, 0.14, 0.015, "steel", y);
    this.add(x, z - 0.03, 0.04, 0.03, 0.12, "steel", y);
    this.add(x, z - 0.02, 0.52, 0.03, 0.3, "dark", y + 0.1);
    this.add(x, z - 0.004, 0.48, 0.004, 0.26, "stock2", y + 0.12);
  }
  /** A television on a low cabinet, against a wall at +z. */
  tv(x: number, z: number) {
    this.add(x, z, 1.7, 0.4, 0.46, "wood");
    this.add(x, z, 1.72, 0.42, 0.04, "dark", 0.46);
    this.add(x, z, 0.4, 0.22, 0.02, "steel", 0.5);
    this.add(x, z + 0.02, 0.06, 0.04, 0.1, "steel", 0.5);
    this.add(x, z + 0.02, 1.35, 0.05, 0.75, "dark", 0.58);
  }
  /** A potted plant. */
  plant(x: number, z: number) {
    this.add(x, z, 0.36, 0.36, 0.38, "stock", 0, 0, "cyl");
    this.add(x, z, 0.62, 0.62, 0.52, "green", 0.32, 0, "ball");
    this.add(x + 0.08, z - 0.06, 0.42, 0.42, 0.4, "green", 0.62, 0, "ball");
  }
  /** A window in the far wall, centred at x. */
  window(x: number, w: number, sill = 0.5, head = 1) {
    this.windows.push([x, w, sill, head]);
  }
  windows: [x: number, w: number, sill: number, head: number][] = [];
  r(lo: number, hi: number) {
    return lo + this.u() * (hi - lo);
  }
  mm(lo: number, hi: number) {
    return `${Math.round(this.r(lo, hi))} mm`;
  }
  m(v: number) {
    return `${v.toFixed(2)} m`;
  }
  pick<T>(xs: T[]) {
    return xs[Math.floor(this.u() * xs.length)]!;
  }
  hazard(name: string, size: string, effect: string, x: number, z: number, y = 0.25) {
    this.hazards.push({ name, size, effect, x, z, y });
  }
  /** A partition along x (or z) with one doorway. */
  partition(axis: "x" | "z", at: number, from: number, to: number, door: number, gap: number, h = WALL) {
    const a = door - gap / 2;
    const b = door + gap / 2;
    const put = (s: number, e: number) => {
      if (e - s < 0.02) return;
      if (axis === "x") this.add((s + e) / 2, at, e - s, 0.1, h, "wall");
      else this.add(at, (s + e) / 2, 0.1, e - s, h, "wall");
    };
    put(from, a);
    put(b, to);
  }
  patch(x: number, z: number, w: number, d: number, turn = 0) {
    this.add(x, z, w, d, 0.012, "hazard", 0.002, turn);
  }
  done(expose: string): Interior {
    if (this.windows.length) {
      // Rebuild the far wall around its windows: piers between them, a wall under each sill and over each head.
      const z = -this.d / 2 - 0.06;
      const end = (this.w + 0.24) / 2;
      const pieces: Block[] = [];
      const wall = (a: number, b: number, y: number, h: number) => {
        if (b - a > 0.01 && h > 0.01) pieces.push({ x: (a + b) / 2, z, w: b - a, d: 0.12, h, f: "wall", y, turn: 0 });
      };
      let from = -end;
      for (const [x, w, sill, head] of [...this.windows].sort((a, b) => a[0] - b[0])) {
        wall(from, x - w / 2, 0, WALL);
        wall(x - w / 2, x + w / 2, 0, sill);
        wall(x - w / 2, x + w / 2, head, WALL - head);
        pieces.push({ x, z, w, d: 0.02, h: head - sill, f: "glass", y: sill, turn: 0 });
        pieces.push({ x, z: z + 0.05, w: w + 0.08, d: 0.2, h: 0.03, f: "shelf", y: sill - 0.03, turn: 0 });
        pieces.push({ x, z, w: 0.035, d: 0.06, h: head - sill, f: "shelf", y: sill, turn: 0 });
        pieces.push({ x, z, w: w, d: 0.06, h: 0.035, f: "shelf", y: head - 0.035, turn: 0 });
        from = x + w / 2;
      }
      wall(from, end, 0, WALL);
      this.blocks.splice(1, 1, ...pieces);
    }
    return { w: this.w, d: this.d, blocks: this.blocks, hazards: this.hazards, expose };
  }
}

const home = (u: U) => {
  const p = new Plan(9.6, 7, u, "wood");
  const door = p.r(0.7, 0.9);
  const split = p.r(-0.8, 0.4);
  p.partition("z", split, -3.5, 3.5, p.r(0.6, 1.6), door);
  p.partition("x", -0.4, split, 4.8, p.r(split + 1, 3.6), 0.82);
  p.hazard("Interior door", `${p.m(door)} clear`, "The footprint passes at one heading, or not at all", split, 1.1, 0.5);
  p.add(split, 1.1, 0.16, door, 0.025, "hazard");
  // Living room
  const sx = split - 3.1;
  p.window(sx, 1.4);
  p.window(split + 1.6, 1, 0.6);
  p.sofa(sx, -2.75, 2.1, 0.94, -1);
  p.table(sx + p.r(-0.2, 0.3), -1.2, 1.1, 0.6, 0.4);
  p.add(sx, -1.2, 2.6, 1.9, 0.014, "mat");
  p.hazard("Rug edge", `${p.mm(5, 20)} thick`, "Snagged, pushed or slipped on", sx + 1.3, -0.8, 0.12);
  p.tv(sx, 2.95);
  p.patch(sx + 0.4, 1.3, 2, 0.04, p.r(-0.3, 0.3));
  p.hazard("Cord", `${p.mm(5, 12)} across`, "A wheel or foot snags", sx + 0.4, 1.3, 0.1);
  for (let i = 0; i < 6; i++) p.add(sx + p.r(-1.2, 1.4), p.r(0, 2.3), 0.12, 0.12, p.r(0.04, 0.14), p.pick<Finish>(["stock", "stock2", "stock3"]), 0, p.r(0, 3));
  p.hazard("Toys", `${p.mm(20, 150)} high`, "Ride-up, tip or a blocked caster", sx - 0.7, 0.8, 0.2);
  // Kitchen
  p.counter(3.9, 2.9, 1.5, 0.6, "steel");
  p.counter(split + 1.1, 2.95, 1.8, 0.6, "stock2");
  p.table(2.6, 1.2, 1.1, 0.8, 0.74);
  for (const [dx, dz] of [
    [-0.75, 0],
    [0.75, 0],
    [0, -0.62],
  ] as const)
    p.chair(2.6 + dx, 1.2 + dz, dx, dz);
  // Stairs
  const riser = p.r(0.15, 0.2);
  for (let i = 0; i < 6; i++) p.add(4.4, -3.2 + i * 0.27, 0.78, 0.28, riser * (6 - i), "wood");
  p.hazard("Stairs", `riser ${Math.round(riser * 1000)} mm`, "Step-height limit, toe strike, fall", 4.4, -1.6, 0.35);
  p.bed(split + 1.6, -2.35, 1.6, 1.9, -1);
  return p.done("Wheel slip at a rug edge, pitch on a threshold, strike of an underslung load");
};

const apartment = (u: U) => {
  const p = new Plan(8.2, 6.4, u, "wood");
  p.window(0, 1);
  const hall = p.r(0.9, 1.1);
  p.partition("x", -0.9, -4.1, 1.2, -1.6, 0.8);
  p.partition("x", -0.9 + hall, -4.1, 0.6, -2.8, 0.8);
  p.partition("z", 1.2, -3.2, -0.9, -2.2, 0.8);
  p.partition("z", 1.2 + hall, -3.2, 3.2, 1.4, 0.8);
  p.partition("z", 0.6, -0.9 + hall, 3.2, 1.7, 0.8);
  p.hazard("Corridor turn", `${p.m(hall)} wide`, "The body is too long to turn", 1.2 + hall / 2, -0.4, 0.15);
  p.patch(1.2 + hall / 2, -0.9 + hall / 2, hall, hall);
  p.hazard("Entry door closer", `${Math.round(p.r(10, 40))} N·m`, "Cannot be pushed, or closes on the robot", 1.2 + hall / 2, 2.9, 0.5);
  p.add(1.2 + hall / 2, 3.05, hall - 0.1, 0.06, WALL, "stock2", 0, 0.35);
  p.add(3.5, -2.4, 1, 1.3, 0.02, "steel");
  p.patch(2.95, -2.4, 0.05, 1.1);
  p.hazard("Lift sill gap", p.mm(10, 35), "A caster drops in", 2.95, -2.4, 0.12);
  p.counter(-2.7, -2.95, 2.4, 0.5, "steel");
  const galley = p.r(0.8, 1.0);
  p.counter(-2.7, -2.45 + galley, 2.4, 0.5, "stock2");
  p.hazard("Compact galley", `${p.m(galley)} aisle`, "No room to turn with a payload", -2.7, -2.7 + galley / 2 + 0.25, 0.1);
  p.sofa(-2.5, 2.55, 2, 1, 1);
  p.table(-2.5, 1.05, 1, 0.55, 0.4);
  p.patch(-1.3, 3.12, 2, 0.06);
  p.hazard("Balcony track", `${p.mm(15, 40)} high`, "The wheel stalls", -1.3, 3.1, 0.15);
  p.bed(3.4, 1.6, 1.1, 1.9, -1);
  return p.done("Clearance distribution, turning envelope, odometry drift");
};

const office = (u: U) => {
  const p = new Plan(11, 7.6, u, "mat");
  for (const x of [-4.2, -1.8, 0.6]) p.window(x, 1.3);
  p.window(4.75, 1);
  for (const z of [-2.5, -0.4])
    for (let i = 0; i < 4; i++) {
      const x = -4.5 + i * 1.7;
      p.desk(x, z, 1.5, 0.7, 0.72);
      p.monitor(x, z - 0.1, 0.72);
      const cx = x + p.r(-0.3, 0.3);
      const cz = z + 0.75 + p.r(-0.1, 0.2);
      p.add(cx, cz, 0.46, 0.46, 0.08, "dark", 0.42, p.r(0, 3));
      p.add(cx, cz, 0.06, 0.06, 0.42, "steel");
      p.add(cx, cz, 0.5, 0.08, 0.03, "dark", 0.02, p.r(0, 3));
      p.add(cx, cz, 0.08, 0.5, 0.03, "dark", 0.02, p.r(0, 3));
      p.add(cx, cz + 0.2, 0.42, 0.06, 0.5, "dark", 0.5);
    }
  p.hazard("Castor chair", `base ${p.m(p.r(0.3, 0.35))}`, "Pushed into the path, legs under the scan plane", -2.6, 0.6, 0.6);
  p.add(3.9, -1.4, 0.04, 4.6, WALL, "glass");
  p.add(4.7, 0.9, 1.6, 0.04, WALL, "glass");
  p.hazard("Glass partition", "full height", "Optical ranging sees nothing", 3.9, -0.6, 0.9);
  p.table(4.75, -1.8, 1.1, 2, 0.74);
  p.partition("x", 1.9, -5.5, 1.6, -0.6, 0.85);
  p.add(-0.6, 1.95, 0.85, 0.05, WALL, "stock2", 0, 0.5);
  p.hazard("Door closer", `${Math.round(p.r(10, 40))} N·m`, "More push than the body has", -0.6, 1.9, 0.8);
  p.patch(0.8, 0.95, 3, 0.1, 0.1);
  p.hazard("Cable cover", `${p.mm(10, 20)} high`, "A small wheel stalls", 1.2, 0.95, 0.12);
  for (let i = 0; i < 4; i++) p.partition("z", -4.4 + i * 1.8, 1.9, 3.8, 4, 0.1);
  p.hazard("Repeated rooms", "identical modules", "It does not know which door this is", -3.5, 2.9, 0.4);
  p.add(3.8, 2.6, 3, 2, 0.006, "steel");
  p.hazard("Glossy lobby", "low roughness", "Phantom depth below the floor", 3.8, 2.8, 0.1);
  p.plant(4.9, 3.2);
  return p.done("Depth dropout on glass, localization ambiguity, chair contacts");
};

const warehouse = (u: U) => {
  const p = new Plan(13, 8, u, "floor");
  const aisle = p.r(1.5, 2.6);
  const beam = p.r(1.2, 1.5);
  for (let row = 0; row < 3; row++) {
    const z = -3.2 + row * (aisle + 0.95);
    for (let bay = 0; bay < 4; bay++) {
      const x = -5.1 + bay * 2.2;
      for (const dx of [-1.05, 1.05]) for (const dz of [-0.4, 0.4]) p.add(x + dx, z + dz, 0.08, 0.08, 2.3, "stock2");
      for (const y of [0.06, beam * 0.62, beam * 1.24]) {
        p.add(x, z, 2.1, 0.86, 0.06, "stock3", y);
        for (let k = 0; k < 3; k++)
          if (u() < 0.86) p.add(x - 0.68 + k * 0.68, z, 0.58, 0.68, p.r(0.26, 0.56), p.pick<Finish>(["stock", "wood", "shelf"]), y + 0.06);
      }
    }
  }
  p.hazard("Aisle", `${p.m(aisle)} wide`, "Turning and passing limits", -3, -3.2 + (aisle + 0.95) / 2, 0.1);
  p.hazard("First rack beam", `${p.m(beam)} up`, "Strikes a tall body or a raised load", -0.6, -3.2 + 0.4 + (aisle + 0.95), beam * 0.62);
  for (let i = 0; i < 3; i++) {
    const x = 4.4 + p.r(-0.4, 1.2);
    const z = -2.6 + i * 1.5;
    p.add(x, z, 1.2, 0.9, 0.15, "wood", 0, p.r(-0.3, 0.3));
    if (u() < 0.6) p.add(x, z, 0.9, 0.7, p.r(0.3, 0.8), "stock", 0.15, p.r(-0.2, 0.2));
  }
  p.hazard("Pallet", "1.2 m × 0.14 to 0.16 m", "A wheel catches in a fork opening", 4.6, -1.1, 0.5);
  for (let i = 0; i < 5; i++) p.add(5 + i * 0.3, 2.9, 0.31, 1.7, 0.05 * (i + 1), "floor");
  p.patch(4.82, 2.9, 0.08, 1.7);
  p.hazard("Dock plate", `lip ${p.mm(10, 30)}`, "Traction limit with payload, pitch at the lip", 4.82, 2.9, 0.15);
  p.patch(0.8, 3.4, 11, 0.04);
  p.hazard("Floor joint", `${p.mm(10, 25)} wide`, "A caster drops, vibration", 1.2, 3.4, 0.1);
  p.patch(2.9, -0.6, 0.9, 0.5, 0.4);
  p.hazard("Stretch wrap", "thin film", "Slip, or wrapped round an axle", 2.9, -0.6, 0.12);
  return p.done("Rack strikes, load sway, cycle time, charge consumed");
};

const plant = (u: U) => {
  const p = new Plan(12, 8, u, "floor");
  const belt = p.r(0.6, 0.9);
  p.add(-1.2, -1.2, 7.2, 0.8, 0.12, "belt", belt);
  for (let i = 0; i < 5; i++) for (const dz of [-0.35, 0.35]) p.add(-4.4 + i * 1.6, -1.2 + dz, 0.08, 0.08, belt, "steel");
  for (let i = 0; i < 6; i++) p.add(-4.4 + i * 1.2 + p.r(-0.2, 0.2), -1.2, 0.4, 0.4, 0.3, p.pick<Finish>(["stock", "stock3"]), belt + 0.12);
  p.hazard("Conveyor", `${p.r(0.1, 1).toFixed(1)} m/s at ${belt.toFixed(2)} m`, "Placement error on a moving target", 0.6, -1.2, belt + 0.5);
  for (const [x, z] of [
    [-4, -3],
    [-1, -3.1],
    [2.2, -3],
  ] as const) {
    p.add(x, z, 1.7, 1.1, 1.1, "steel");
    p.add(x, z + 0.5, 0.9, 0.12, 0.5, "dark", 0.5);
    p.add(x + 0.6, z, 0.3, 0.3, 0.4, "stock3", 1.1);
  }
  for (let i = 0; i < 9; i++) p.add(-5 + i * 0.9, 0.9, 0.05, 0.05, 0.95, "stock3");
  for (const y of [0.35, 0.7, 0.92]) p.add(-1.4, 0.9, 7.3, 0.03, 0.03, "stock3", y);
  p.hazard("Fence", "declared region", "Keep-out zone violated", -3, 0.9, 0.9);
  for (let i = 0; i < 14; i++) p.add(3.6 + i * 0.12, 1.7, 0.05, 1.8, 0.02, "dark");
  p.add(4.4, 1.7, 1.75, 1.85, 0.008, "hazard");
  p.hazard("Floor grate", `slot ${p.mm(15, 30)}`, "Foot or caster snag, twist at the contact", 4.4, 1.7, 0.15);
  p.patch(0.7, 2.4, 1.6, 0.9, 0.2);
  p.hazard("Oil film", `${p.r(0.05, 1).toFixed(2)} mm, ${Math.round(p.r(20, 100))} mPa·s`, "Slip that depends on dwell time and load", 0.7, 2.4, 0.12);
  p.patch(-3.2, 2.6, 3, 0.18);
  p.hazard("Cable cover", `${p.mm(15, 40)} high`, "Stall, strike", -3.2, 2.6, 0.14);
  p.add(4.8, -2.6, 1.6, 1.8, 1.15, "steel");
  p.hazard("Polished metal", "mirror-like", "Phantom points, glare", 4.0, -2.2, 0.9);
  p.add(-5.2, 3.2, 0.9, 0.7, 0.8, "stock");
  return p.done("Contact timing on a moving belt, boundary work, placement error");
};

const retail = (u: U) => {
  const p = new Plan(11, 7.6, u, "tile");
  const aisle = p.r(0.9, 1.5);
  for (let i = 0; i < 4; i++) {
    const x = -4.6 + i * (aisle + 0.62);
    p.add(x, -0.6, 0.5, 4.2, 0.1, "shelf");
    for (const y of [0.35, 0.72, 1.08]) p.add(x, -0.6, 0.5, 4.2, 0.03, "shelf", y);
    p.add(x, -0.6, 0.04, 4.2, 1.35, "shelf");
    for (let k = 0; k < 11; k++)
      for (const y of [0.1, 0.38, 0.75])
        for (const s of [-0.14, 0.14])
          if (u() < 0.85) p.add(x + s, -2.5 + k * 0.38, 0.2, 0.32, p.r(0.12, 0.26), p.pick<Finish>(["stock", "stock2", "stock3", "green"]), y);
  }
  p.hazard("Shelving aisle", `${p.m(aisle)} wide`, "Shelf strike, reach limit", -4.6 + (aisle + 0.62) / 2, -0.4, 0.1);
  const ex = -4.6 + 1 * (aisle + 0.62);
  p.add(ex, 1.85, 0.9, 0.5, 0.8, "stock3");
  p.hazard("End cap", "tight turn", "Turning envelope with a payload", ex, 1.9, 1);
  const cx = p.r(1.6, 2.4);
  p.add(cx, 1.7, 0.6, 0.95, 0.5, "steel", 0.35, p.r(-0.4, 0.4));
  p.add(cx, 1.7, 0.5, 0.85, 0.05, "dark", 0.14);
  p.hazard("Shopping cart", `${Math.round(p.r(15, 30))} kg`, "Blocks the aisle, or is pushed into a display", cx, 1.7, 1.1);
  p.add(3.6, -3.4, 3, 0.7, 1.3, "glass");
  p.add(3.6, -3.5, 2.9, 0.5, 0.9, "stock2", 0.1);
  p.hazard("Glass case", "with condensation", "Depth dropout or a false return", 3.4, -3.05, 0.9);
  const lane = p.r(0.6, 0.9);
  for (let i = 0; i < 2; i++) {
    p.counter(4.6, 0.6 + i * (lane + 0.62), 1.5, 0.6, "wood");
    p.add(4.2, 0.6 + i * (lane + 0.62), 0.3, 0.3, 0.3, "dark", 0.9);
  }
  p.hazard("Checkout lane", `${p.m(lane)} wide`, "The footprint does not pass", 4.6, 0.6 + (lane + 0.62) / 2, 0.15);
  p.patch(0.2, 2.9, 1.1, 0.7, 0.5);
  p.hazard("Spill", `${p.r(0.1, 2).toFixed(1)} mm film`, "Slip", 0.2, 2.9, 0.12);
  p.add(-3.4, 3.1, 2.2, 1, 0.012, "mat");
  p.hazard("Entrance mat", `edge ${p.mm(5, 12)}`, "Edge snag", -2.3, 3.1, 0.1);
  return p.done("Occluded obstacles, shelf strikes, stopping distance");
};

const kitchen = (u: U) => {
  const p = new Plan(9, 6.6, u, "tile");
  const aisle = p.r(0.9, 1.2);
  const z1 = -0.9;
  p.add(-1, -2.85, 6, 0.75, 0.9, "steel");
  for (let i = 0; i < 4; i++) {
    const x = -3.2 + i * 1.3;
    p.add(x, -2.85, 1, 0.6, 0.06, "dark", 0.9);
    if (u() < 0.8) {
      p.add(x, -2.85, 0.4, 0.4, p.r(0.2, 0.34), "steel", 0.96);
    }
  }
  p.add(-1, -3.15, 6, 0.3, 0.1, "steel", 1.7);
  p.add(-0.4, z1 + aisle - 0.2, 4.8, 0.8, 0.06, "steel", 0.86);
  p.add(-0.4, z1 + aisle - 0.2, 4.7, 0.7, 0.04, "steel", 0.22);
  for (const dx of [-2.3, 2.3]) for (const dz of [-0.35, 0.35]) p.add(-0.4 + dx, z1 + aisle - 0.2 + dz, 0.07, 0.07, 0.86, "steel");
  for (let i = 0; i < 5; i++) p.add(-2.2 + i * 0.85, z1 + aisle - 0.2, 0.4, 0.3, 0.12, p.pick<Finish>(["stock", "steel", "shelf"]), 0.92);
  p.hazard("Galley aisle", `${p.m(aisle)} wide`, "A turn with a carried container", -2.6, -1.6, 0.1);
  p.hazard("Low shelf", `${p.m(p.r(0.15, 0.3))} clearance`, "Strikes a tall chassis, hides the far side", 1.9, z1 + aisle + 0.15, 0.3);
  p.patch(0.5, -1.7, 1.5, 0.8, 0.1);
  p.hazard("Greasy floor", `${p.r(0.05, 1).toFixed(2)} mm, ${Math.round(p.r(30, 100))} mPa·s`, "Slip that lasts through a stance", 0.5, -1.7, 0.1);
  p.patch(-2.5, 1.9, 1.3, 0.9, 0.3);
  p.hazard("Wet floor", `${p.r(0.1, 2).toFixed(1)} mm water`, "Slip, and phantom depth", -2.5, 1.9, 0.1);
  for (let i = 0; i < 9; i++) p.add(0.4 + i * 0.1, 2.1, 0.04, 0.7, 0.02, "dark");
  p.add(0.8, 2.1, 1, 0.8, 0.008, "hazard");
  p.hazard("Floor drain", `slot ${p.mm(8, 15)}`, "A caster drops, and the floor tips towards it", 0.8, 2.1, 0.12);
  p.add(-0.6, -2.1, 2.2, 0.65, 0.016, "mat");
  p.hazard("Anti-fatigue mat", `${p.mm(10, 20)} thick`, "Edge snag, and it slides when wet", -1.7, -2.05, 0.14);
  p.partition("z", 3, -3.3, 3.3, 0.6, 0.95);
  p.add(3.05, 0.6, 0.05, 0.9, WALL, "stock2", 0, 0.5);
  p.hazard("Swing door", `${Math.round(p.r(5, 20))} N·m`, "Returns onto the robot or its load", 3, 0.6, 0.8);
  p.add(3.75, -2.2, 1.2, 1.6, 1.15, "steel");
  p.patch(3.75, -1.3, 1, 0.1);
  p.hazard("Cold-room threshold", p.mm(10, 40), "Stall, and strips that hide the way", 3.75, -1.3, 0.2);
  p.table(3.8, 2.2, 1, 1.4, 0.9, "wood", "steel", true);
  return p.done("Low-traction turn, depth artifacts, acceleration of a carried container");
};

export const interiors: Record<string, (u: U) => Interior> = {
  home,
  apartment,
  office,
  warehouse,
  plant,
  retail,
  kitchen,
};
export const generate = (place: string, seed: number) => interiors[place]!(rng(seed * 7919 + place.length * 104729));
