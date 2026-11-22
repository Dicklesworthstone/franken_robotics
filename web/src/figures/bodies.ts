/**
 * Robots as data. A body is a tree of links; each link has a joint, an offset from its parent and a few solids.
 * The renderer, the table beside it and the 256-body field all read the same tree. x is forward, y up, z right.
 */
export type V3 = [number, number, number];
export type Solid = "box" | "cyl" | "ball";
export type Mat = "shell" | "dark" | "steel" | "rubber" | "glow" | "amber";

export interface Part {
  s: Solid;
  /** box: full sizes. cyl: radius, length along its local y. ball: radius. */
  d: V3;
  p: V3;
  /** Euler rotation, XYZ order. */
  r?: V3;
  m: Mat;
}
export interface Link {
  name: string;
  parent: number;
  /** Offset of the joint in the parent's frame. */
  at: V3;
  /** A fixed rotation applied before the joint turns. */
  set?: V3;
  joint: "free" | "fixed" | "x" | "y" | "z";
  /** Joint angle at time t; `wheel` joints spin, the rest swing. */
  turn?: (t: number) => number;
  passive?: boolean;
  parts: Part[];
}
export interface Body {
  links: Link[];
  /** Height of the base above the floor, and how fast the floor passes underneath. */
  height: number;
  speed: number;
  /** Bob, sway or lean of the base. */
  base?: (t: number) => { y?: number; pitch?: number; roll?: number; yaw?: number; x?: number; z?: number };
  /** Radius that frames the whole body. */
  size: number;
}

const TAU = Math.PI * 2;
const ball = (r: number, p: V3, m: Mat): Part => ({ s: "ball", d: [r, r, r], p, m });
const box = (d: V3, p: V3, m: Mat, r?: V3): Part => ({ s: "box", d, p, m, r });
/** A cylinder of radius r and length l; axis picks the way it lies. */
const cyl = (r: number, l: number, p: V3, m: Mat, axis: "x" | "y" | "z" = "y"): Part => ({
  s: "cyl",
  d: [r, l, r],
  p,
  m,
  r: axis === "x" ? [0, 0, Math.PI / 2] : axis === "z" ? [Math.PI / 2, 0, 0] : undefined,
});

class Maker {
  links: Link[] = [];
  add(l: Link) {
    this.links.push(l);
    return this.links.length - 1;
  }
}

/** Joint angles that put a two-link leg's foot at (x, y) from the hip, y negative downwards. */
export function reach(l1: number, l2: number, x: number, y: number, bend: 1 | -1) {
  const d = Math.min(Math.hypot(x, y), (l1 + l2) * 0.9995);
  const c = (d * d - l1 * l1 - l2 * l2) / (2 * l1 * l2);
  const k = bend * Math.acos(Math.max(-1, Math.min(1, c)));
  const h = Math.atan2(x, -y) - Math.atan2(l2 * Math.sin(k), l1 + l2 * Math.cos(k));
  return [h, k] as const;
}

export interface Step {
  period: number;
  duty: number;
  stride: number;
  lift: number;
}
/** Where a foot is in its cycle: on the ground it moves back with the floor, in the air it swings forward. */
export function foot(phase: number, g: Step, lead = 0): [x: number, up: number] {
  const p = phase - Math.floor(phase);
  if (p < g.duty) return [lead + g.stride * (0.5 - p / g.duty), 0];
  const s = (p - g.duty) / (1 - g.duty);
  return [lead + g.stride * (-0.5 + (s - Math.sin(TAU * s) / TAU)), g.lift * Math.sin(Math.PI * s) ** 1.4];
}

interface Leg {
  name: string;
  parent: number;
  at: V3;
  l1: number;
  l2: number;
  hang: number;
  phase: number;
  gait: Step;
  bend: 1 | -1;
  thick?: number;
  lead?: number;
  wheel?: number;
  abduct?: number;
  side: 1 | -1;
}
/** A mammalian leg: hip roll, hip pitch, knee. Its foot follows the gait. */
function leg(mk: Maker, o: Leg) {
  const k = o.thick ?? 1;
  const solve = (t: number) => {
    const [x, up] = foot(t / o.gait.period + o.phase, o.gait, o.lead ?? 0);
    return reach(o.l1, o.l2, x, -o.hang + up, o.bend);
  };
  const hip = mk.add({
    name: `${o.name}.hip`,
    parent: o.parent,
    at: o.at,
    joint: "x",
    turn: () => (o.abduct ?? 0) * o.side,
    parts: [cyl(0.048 * k, 0.085 * k, [0, 0, 0], "dark", "x"), ball(0.052 * k, [0, 0, 0.055 * o.side * k], "steel")],
  });
  const thigh = mk.add({
    name: `${o.name}.thigh`,
    parent: hip,
    at: [0, 0, 0.075 * o.side * k],
    joint: "z",
    turn: (t) => solve(t)[0],
    parts: [
      cyl(0.058 * k, 0.06 * k, [0, 0, 0], "dark", "z"),
      cyl(0.03 * k, 0.066 * k, [0, 0, 0], "glow", "z"),
      box([0.062 * k, o.l1, 0.046 * k], [0, -o.l1 / 2, 0], "shell"),
    ],
  });
  const shank = mk.add({
    name: `${o.name}.shank`,
    parent: thigh,
    at: [0, -o.l1, 0],
    joint: "z",
    turn: (t) => solve(t)[1],
    parts: [
      cyl(0.042 * k, 0.056 * k, [0, 0, 0], "steel", "z"),
      box([0.036 * k, o.l2 * 0.96, 0.034 * k], [0, -o.l2 * 0.48, 0], "dark"),
      ...(o.wheel ? [] : [ball(0.03 * k, [0, -o.l2, 0], "rubber")]),
    ],
  });
  if (o.wheel) wheel(mk, `${o.name}.wheel`, shank, [0, -o.l2, 0.045 * o.side], o.wheel, 0.05, (t) => (-t * o.gait.stride) / o.wheel!);
  return hip;
}

/** A leg that reaches out sideways first: yaw at the body, then lift and knee. Used by sprawled and radial bodies. */
function sprawl(
  mk: Maker,
  o: { name: string; parent: number; at: V3; face: number; coxa: number; l1: number; l2: number; hang: number; out: number; phase: number; gait: Step; thick?: number },
) {
  const k = o.thick ?? 1;
  // The foot's path is along the body's x; the leg turns it into yaw and reach.
  const rest: [number, number] = [Math.cos(o.face) * o.out, Math.sin(o.face) * o.out];
  const solve = (t: number) => {
    const [x, up] = foot(t / o.gait.period + o.phase, o.gait);
    const fx = rest[0] + x;
    const fz = rest[1];
    const yaw = Math.atan2(fz, fx) - o.face;
    const r = Math.hypot(fx, fz) - o.coxa;
    const [h, kn] = reach(o.l1, o.l2, r, -o.hang + up, -1);
    return [yaw, h, kn] as const;
  };
  const coxa = mk.add({
    name: `${o.name}.coxa`,
    parent: o.parent,
    at: o.at,
    set: [0, -o.face, 0],
    joint: "y",
    turn: (t) => -solve(t)[0],
    parts: [
      cyl(0.05 * k, 0.07 * k, [0, 0, 0], "dark"),
      box([o.coxa, 0.05 * k, 0.055 * k], [o.coxa / 2, 0, 0], "shell"),
    ],
  });
  const femur = mk.add({
    name: `${o.name}.femur`,
    parent: coxa,
    at: [o.coxa, 0, 0],
    joint: "z",
    turn: (t) => solve(t)[1],
    parts: [
      cyl(0.044 * k, 0.07 * k, [0, 0, 0], "steel", "z"),
      cyl(0.022 * k, 0.076 * k, [0, 0, 0], "glow", "z"),
      box([0.05 * k, o.l1, 0.042 * k], [0, -o.l1 / 2, 0], "shell"),
    ],
  });
  mk.add({
    name: `${o.name}.tibia`,
    parent: femur,
    at: [0, -o.l1, 0],
    joint: "z",
    turn: (t) => solve(t)[2],
    parts: [
      cyl(0.036 * k, 0.06 * k, [0, 0, 0], "dark", "z"),
      box([0.034 * k, o.l2, 0.03 * k], [0, -o.l2 / 2, 0], "dark"),
      ball(0.026 * k, [0, -o.l2, 0], "rubber"),
    ],
  });
}

function wheel(
  mk: Maker,
  name: string,
  parent: number,
  at: V3,
  r: number,
  w: number,
  turn: (t: number) => number,
  o: { passive?: boolean; rollers?: number; set?: V3 } = {},
) {
  const parts: Part[] = [];
  if (o.rollers) {
    parts.push(cyl(r * 0.74, w * 0.9, [0, 0, 0], "dark", "z"));
    const n = 9;
    for (let i = 0; i < n; i++) {
      const a = (i / n) * TAU;
      // Barrel rollers around the rim, leaning by the roller angle.
      parts.push({
        s: "cyl",
        d: [r * 0.17, w * 1.25, r * 0.17],
        p: [Math.cos(a) * r * 0.84, Math.sin(a) * r * 0.84, 0],
        r: [Math.PI / 2 - o.rollers * Math.sin(a), o.rollers * Math.cos(a), 0],
        m: "rubber",
      });
    }
  } else {
    parts.push(cyl(r, w, [0, 0, 0], "rubber", "z"), cyl(r * 0.66, w * 1.06, [0, 0, 0], "dark", "z"));
    for (let i = 0; i < 5; i++) {
      const a = (i / 5) * TAU;
      parts.push(box([r * 0.56, r * 0.1, w * 1.12], [Math.cos(a) * r * 0.32, Math.sin(a) * r * 0.32, 0], "steel", [0, 0, a]));
    }
  }
  parts.push(cyl(r * 0.16, w * 1.2, [0, 0, 0], "glow", "z"));
  return mk.add({ name, parent, at, set: o.set, joint: "z", turn, passive: o.passive, parts });
}

const trunk = (parts: Part[]): Link => ({ name: "base", parent: -1, at: [0, 0, 0], joint: "free", parts });
const head = (x: number, y: number, k = 1): Part[] => [
  box([0.14 * k, 0.1 * k, 0.17 * k], [x, y, 0], "dark"),
  box([0.02 * k, 0.03 * k, 0.12 * k], [x + 0.068 * k, y + 0.012 * k, 0], "glow"),
];

export interface QuadSpec {
  legs: 4 | 6 | 8;
  femur: number;
  tibia: number;
  length: number;
  width: number;
  thick: number;
  gait: Step;
  bend: 1 | -1;
  wheel?: number;
  pattern?: "trot" | "wave";
}
/** A walker with two, three or four pairs of legs. */
export function walker(q: QuadSpec): Body {
  const mk = new Maker();
  const pairs = q.legs / 2;
  const hang = (q.femur + q.tibia) * 0.82;
  const parts: Part[] = [
    box([q.length * 0.98, 0.13 * q.thick, q.width * 1.18], [0, 0.02, 0], "shell"),
    box([q.length * 0.74, 0.05, q.width * 0.86], [0, 0.1 * q.thick, 0], "dark"),
    box([q.length * 0.3, 0.012, q.width * 0.3], [-q.length * 0.1, 0.128 * q.thick + 0.004, 0], "glow"),
    // Both ends carry the same block; only the sensor strip tells front from back.
    box([0.13, 0.095, 0.16], [q.length / 2 + 0.05, 0.035, 0], "dark"),
    box([0.13, 0.095, 0.16], [-q.length / 2 - 0.05, 0.035, 0], "dark"),
    box([0.012, 0.03, 0.12], [q.length / 2 + 0.118, 0.045, 0], "glow"),
  ];
  mk.add(trunk(parts));
  for (let i = 0; i < pairs; i++) {
    const x = q.length * (0.43 - (0.86 * i) / (pairs - 1));
    for (const side of [-1, 1] as const) {
      const diagonal = (i + (side === 1 ? 1 : 0)) % 2;
      const phase = q.pattern === "wave" ? i / pairs + (side === 1 ? 0.5 : 0) : diagonal * 0.5;
      leg(mk, {
        name: `${pairs === 2 ? (i ? "hind" : "fore") : `l${i + 1}`}.${side < 0 ? "left" : "right"}`,
        parent: 0,
        at: [x, 0, side * q.width * 0.5],
        l1: q.femur,
        l2: q.tibia,
        hang,
        phase,
        gait: q.gait,
        // Front and hind knees mirror each other, so the body has both of its symmetries.
        bend: (x < 0 ? q.bend : -q.bend) as 1 | -1,
        side,
        thick: q.thick,
        wheel: q.wheel,
      });
    }
  }
  const bob = q.gait.lift * 0.12;
  return {
    links: mk.links,
    height: hang + (q.wheel ?? 0.03 * q.thick),
    speed: q.gait.stride / (q.gait.duty * q.gait.period),
    base: (t) => ({ y: bob * Math.cos((2 * TAU * t) / q.gait.period), roll: 0.012 * Math.sin((TAU * t) / q.gait.period) }),
    size: 0.62 + q.length * 0.5,
  };
}

const TROT: Step = { period: 0.62, duty: 0.56, stride: 0.24, lift: 0.085 };

function radial(legs: number, o: { r: number; coxa: number; l1: number; l2: number; hang: number; out: number; gait: Step; tripod: boolean }): Body {
  const mk = new Maker();
  mk.add(
    trunk([
      cyl(o.r, 0.1, [0, 0.01, 0], "shell"),
      cyl(o.r * 0.72, 0.04, [0, 0.075, 0], "dark"),
      ball(o.r * 0.32, [0, 0.09, 0], "steel"),
      cyl(o.r * 0.9, 0.012, [0, 0.062, 0], "glow"),
    ]),
  );
  for (let i = 0; i < legs; i++) {
    const a = ((i + 0.5) / legs) * TAU;
    sprawl(mk, {
      name: `leg${i + 1}`,
      parent: 0,
      at: [Math.cos(a) * o.r * 0.94, 0, Math.sin(a) * o.r * 0.94],
      face: a,
      coxa: o.coxa,
      l1: o.l1,
      l2: o.l2,
      hang: o.hang,
      out: o.out,
      phase: o.tripod ? (i % 2) * 0.5 : (i % 2) * 0.5 + (i % 4 > 1 ? 0.25 : 0),
      gait: o.gait,
      thick: 0.85,
    });
  }
  return { links: mk.links, height: o.hang + 0.026, speed: o.gait.stride / (o.gait.duty * o.gait.period), size: o.r + o.out + 0.12 };
}

function biped(): Body {
  const mk = new Maker();
  const g: Step = { period: 0.82, duty: 0.6, stride: 0.34, lift: 0.1 };
  mk.add(trunk([box([0.2, 0.2, 0.24], [0, 0.12, 0], "shell"), box([0.14, 0.05, 0.28], [0, 0, 0], "dark"), ...head(0.05, 0.28, 0.9)]));
  for (const side of [-1, 1] as const)
    leg(mk, { name: side < 0 ? "left" : "right", parent: 0, at: [0, 0, side * 0.09], l1: 0.36, l2: 0.4, hang: 0.66, phase: side < 0 ? 0 : 0.5, gait: g, bend: 1, side, thick: 1.05 });
  return { links: mk.links, height: 0.69, speed: g.stride / (g.duty * g.period), base: (t) => ({ y: 0.012 * Math.cos((2 * TAU * t) / g.period), z: 0.02 * Math.sin((TAU * t) / g.period), roll: 0.03 * Math.sin((TAU * t) / g.period) }), size: 0.86 };
}

function humanoid(): Body {
  const mk = new Maker();
  const g: Step = { period: 1.0, duty: 0.6, stride: 0.42, lift: 0.08 };
  const beat = (t: number, side = 0) => (TAU * t) / g.period + (side < 0 ? Math.PI : 0);
  mk.add(trunk([box([0.15, 0.12, 0.25], [0, 0, 0], "dark"), box([0.13, 0.05, 0.28], [0, 0.04, 0], "steel")]));
  const yaw = mk.add({ name: "waist.yaw", parent: 0, at: [0, 0.08, 0], joint: "y", turn: (t) => 0.12 * Math.sin(beat(t)), parts: [cyl(0.07, 0.05, [0, 0, 0], "dark")] });
  const roll = mk.add({ name: "waist.roll", parent: yaw, at: [0, 0.04, 0], joint: "x", turn: (t) => 0.02 * Math.sin(beat(t)), parts: [cyl(0.045, 0.13, [0, 0, 0], "steel", "x")] });
  const chest = mk.add({
    name: "waist.pitch",
    parent: roll,
    at: [0, 0.03, 0],
    joint: "z",
    turn: () => -0.04,
    parts: [box([0.2, 0.28, 0.34], [0, 0.17, 0], "shell"), box([0.03, 0.15, 0.22], [0.1, 0.18, 0], "dark"), cyl(0.035, 0.014, [0.12, 0.22, 0], "glow", "x"), box([0.16, 0.06, 0.4], [0, 0.31, 0], "dark")],
  });
  mk.add({ name: "head", parent: chest, at: [0, 0.38, 0], joint: "fixed", parts: [cyl(0.04, 0.08, [0, 0, 0], "dark"), ball(0.1, [0.01, 0.11, 0], "shell"), box([0.04, 0.05, 0.14], [0.085, 0.12, 0], "dark"), box([0.012, 0.016, 0.11], [0.108, 0.126, 0], "glow")] });
  for (const side of [-1, 1] as const) {
    const n = side < 0 ? "left" : "right";
    let p = mk.add({ name: `${n}.shoulder.pitch`, parent: chest, at: [0, 0.29, side * 0.22], joint: "z", turn: (t) => 0.42 * Math.sin(beat(t, side) + 0.4), parts: [ball(0.058, [0, 0, 0], "steel"), cyl(0.034, 0.12, [0, 0, 0], "glow", "z")] });
    p = mk.add({ name: `${n}.shoulder.roll`, parent: p, at: [0, 0, side * 0.04], joint: "x", turn: () => -side * 0.1, parts: [cyl(0.045, 0.07, [0, 0, 0], "dark", "x")] });
    p = mk.add({ name: `${n}.shoulder.yaw`, parent: p, at: [0, -0.04, 0], joint: "y", turn: () => side * 0.06, parts: [box([0.07, 0.2, 0.07], [0, -0.11, 0], "shell")] });
    p = mk.add({ name: `${n}.elbow`, parent: p, at: [0, -0.24, 0], joint: "z", turn: (t) => 0.5 + 0.18 * Math.sin(beat(t, side)), parts: [ball(0.042, [0, 0, 0], "steel"), box([0.056, 0.13, 0.056], [0, -0.08, 0], "dark")] });
    p = mk.add({ name: `${n}.wrist.yaw`, parent: p, at: [0, -0.15, 0], joint: "y", turn: () => 0, parts: [cyl(0.03, 0.08, [0, -0.02, 0], "shell")] });
    p = mk.add({ name: `${n}.wrist.pitch`, parent: p, at: [0, -0.08, 0], joint: "z", turn: () => 0.1, parts: [ball(0.027, [0, 0, 0], "steel")] });
    mk.add({ name: `${n}.wrist.roll`, parent: p, at: [0, -0.02, 0], joint: "x", turn: () => 0, parts: [box([0.065, 0.085, 0.035], [0, -0.045, 0], "steel")] });
    const solve = (t: number) => {
      const [x, up] = foot(t / g.period + (side < 0 ? 0 : 0.5), g, -0.02);
      return reach(0.4, 0.4, x, -0.74 + up, -1);
    };
    p = mk.add({ name: `${n}.hip.yaw`, parent: 0, at: [0, -0.04, side * 0.1], joint: "y", turn: () => 0, parts: [cyl(0.06, 0.05, [0, 0.01, 0], "steel")] });
    p = mk.add({ name: `${n}.hip.roll`, parent: p, at: [0, 0, 0], joint: "x", turn: () => 0, parts: [ball(0.065, [0, 0, 0], "dark")] });
    p = mk.add({ name: `${n}.hip.pitch`, parent: p, at: [0, 0, 0], joint: "z", turn: (t) => solve(t)[0], parts: [cyl(0.045, 0.16, [0, 0, 0], "glow", "z"), box([0.11, 0.36, 0.1], [0, -0.2, 0], "shell")] });
    p = mk.add({ name: `${n}.knee`, parent: p, at: [0, -0.4, 0], joint: "z", turn: (t) => solve(t)[1], parts: [cyl(0.052, 0.1, [0, 0, 0], "steel", "z"), box([0.085, 0.34, 0.08], [0, -0.2, 0], "dark")] });
    p = mk.add({ name: `${n}.ankle.pitch`, parent: p, at: [0, -0.4, 0], joint: "z", turn: (t) => -(solve(t)[0] + solve(t)[1]), parts: [ball(0.04, [0, 0, 0], "steel")] });
    mk.add({ name: `${n}.ankle.roll`, parent: p, at: [0, 0, 0], joint: "x", turn: () => 0, parts: [box([0.23, 0.05, 0.1], [0.05, -0.04, 0], "rubber")] });
  }
  return { links: mk.links, height: 0.845, speed: g.stride / (g.duty * g.period), base: (t) => ({ y: 0.012 * Math.cos(2 * beat(t)), z: 0.018 * Math.sin(beat(t)) }), size: 1.02 };
}

function car(kind: "diff" | "ackermann" | "mecanum"): Body {
  const mk = new Maker();
  const r = kind === "diff" ? 0.11 : 0.1;
  const v = 0.6;
  const spin = (t: number) => (-t * v) / r;
  if (kind === "diff") {
    mk.add(trunk([cyl(0.2, 0.16, [0, 0.06, 0], "shell"), cyl(0.205, 0.02, [0, 0.0, 0], "dark"), cyl(0.15, 0.03, [0, 0.155, 0], "dark"), cyl(0.06, 0.08, [0.05, 0.2, 0], "steel"), cyl(0.062, 0.014, [0.05, 0.21, 0], "glow")]));
    wheel(mk, "left.wheel", 0, [0, 0, -0.2], r, 0.05, spin);
    wheel(mk, "right.wheel", 0, [0, 0, 0.2], r, 0.05, spin);
    const yoke = mk.add({ name: "caster.yoke", parent: 0, at: [-0.15, -0.03, 0], joint: "y", passive: true, turn: (t) => 0.12 * Math.sin(t), parts: [cyl(0.02, 0.05, [0, 0, 0], "steel")] });
    mk.add({ name: "caster.ball", parent: yoke, at: [-0.02, -0.045, 0], joint: "z", passive: true, turn: spin, parts: [ball(0.035, [0, 0, 0], "rubber")] });
    return { links: mk.links, height: r, speed: v, size: 0.46 };
  }
  const L = 0.5,
    W = kind === "ackermann" ? 0.34 : 0.36;
  mk.add(trunk([box([0.66, 0.11, 0.3], [0, 0.08, 0], "shell"), box([0.36, 0.1, 0.24], [-0.06, 0.17, 0], "dark"), box([0.02, 0.03, 0.2], [0.332, 0.09, 0], "glow"), box([0.5, 0.05, 0.22], [0, 0.0, 0], "dark")]));
  const steer = (t: number) => 0.32 * Math.sin(t * 0.8);
  for (const [i, x] of [L / 2, -L / 2].entries())
    for (const side of [-1, 1] as const) {
      const name = `${i ? "rear" : "front"}.${side < 0 ? "left" : "right"}`;
      if (kind === "ackermann" && !i) {
        const k = mk.add({ name: `${name}.knuckle`, parent: 0, at: [x, 0, side * (W / 2 - 0.03)], joint: "y", passive: side > 0, turn: steer, parts: [cyl(0.026, 0.09, [0, 0, 0], "steel"), box([0.1, 0.014, 0.016], [-0.06, 0.02, 0], "amber")] });
        wheel(mk, `${name}.wheel`, k, [0, 0, side * 0.05], r, 0.07, spin, { passive: true });
      } else wheel(mk, `${name}.wheel`, 0, [x, 0, side * W * 0.56], r, kind === "mecanum" ? 0.085 : 0.07, spin, kind === "mecanum" ? { rollers: (i === 0) === side > 0 ? 0.78 : -0.78 } : {});
    }
  return { links: mk.links, height: r, speed: v, size: 0.62 };
}

function omni(): Body {
  const mk = new Maker();
  mk.add(trunk([cyl(0.21, 0.09, [0, 0.07, 0], "shell"), cyl(0.13, 0.05, [0, 0.14, 0], "dark"), cyl(0.215, 0.014, [0, 0.07, 0], "glow"), cyl(0.18, 0.03, [0, 0.0, 0], "dark")]));
  for (let i = 0; i < 3; i++) {
    const a = (i / 3) * TAU;
    wheel(mk, `wheel${i + 1}`, 0, [Math.cos(a) * 0.23, 0, Math.sin(a) * 0.23], 0.075, 0.05, (t) => 3.2 * Math.sin(t * 0.7 + (i * TAU) / 3), { rollers: 0.0001, set: [0, -a + Math.PI / 2, 0] });
  }
  return { links: mk.links, height: 0.075, speed: 0, base: (t) => ({ x: 0.12 * Math.cos(t * 0.7), z: 0.12 * Math.sin(t * 0.7), yaw: 0.2 * Math.sin(t * 0.35) }), size: 0.5 };
}

function swerve(): Body {
  const mk = new Maker();
  mk.add(trunk([box([0.5, 0.08, 0.5], [0, 0.17, 0], "shell"), box([0.32, 0.05, 0.32], [0, 0.23, 0], "dark"), box([0.51, 0.012, 0.51], [0, 0.17, 0], "glow")]));
  for (const x of [1, -1])
    for (const z of [-1, 1]) {
      const n = `${x > 0 ? "front" : "rear"}.${z < 0 ? "left" : "right"}`;
      const m = mk.add({ name: `${n}.steer`, parent: 0, at: [x * 0.2, 0.12, z * 0.2], joint: "y", turn: (t) => 0.9 * Math.sin(t * 0.6), parts: [cyl(0.055, 0.06, [0, 0, 0], "dark"), box([0.03, 0.12, 0.09], [0, -0.06, 0], "steel")] });
      wheel(mk, `${n}.drive`, m, [0, -0.12, 0], 0.075, 0.045, (t) => -t * 7);
    }
  return { links: mk.links, height: 0.075, speed: 0.5, base: (t) => ({ yaw: 0.0 * t }), size: 0.56 };
}

function tracked(): Body {
  const mk = new Maker();
  mk.add(trunk([box([0.56, 0.13, 0.28], [0, 0.08, 0], "shell"), box([0.3, 0.06, 0.2], [0, 0.17, 0], "dark"), cyl(0.04, 0.1, [0, 0.23, 0], "steel"), ball(0.045, [0, 0.29, 0], "dark"), box([0.02, 0.04, 0.16], [0.285, 0.08, 0], "glow")]));
  const shoes = (n: string, parent: number, at: V3, len: number, r0: number, r1: number, w: number) => {
    mk.add({ name: n, parent, at, joint: "z", turn: (t) => -t * 6, parts: [cyl(r0, w, [0, 0, 0], "dark", "z"), cyl(r0 * 0.3, w * 1.08, [0, 0, 0], "glow", "z")] });
    const belt: Part[] = [box([len, 0.022, w], [len / 2, r0 * 0.98, 0], "rubber"), box([len, 0.022, w], [len / 2, -r0 * 0.98, 0], "rubber"), cyl(r1, w, [len, 0, 0], "rubber", "z"), cyl(r0 * 1.06, w * 0.98, [0, 0, 0], "rubber", "z"), box([len, r0 * 1.4, w * 0.6], [len / 2, 0, 0], "dark")];
    for (let i = 0; i < 7; i++) belt.push(box([0.016, r0 * 2.28, w * 1.02], [(len * (i + 0.5)) / 7, 0, 0], "rubber"));
    return belt;
  };
  for (const side of [-1, 1] as const) {
    const n = side < 0 ? "left" : "right";
    const belt = shoes(`${n}.sprocket`, 0, [-0.24, 0, side * 0.2], 0.48, 0.095, 0.1, 0.1);
    mk.links[0]!.parts.push(...belt.map((p) => ({ ...p, p: [p.p[0] - 0.24, p.p[1], p.p[2] + side * 0.2] as V3 })));
    for (const [f, x] of [["front", 0.24], ["rear", -0.24]] as const) {
      const dir = x > 0 ? 1 : -1;
      mk.add({ name: `${n}.${f}.flipper`, parent: 0, at: [x, 0, side * 0.29], joint: "z", turn: (t) => dir * (0.35 + 0.32 * Math.sin(t * 0.8 + (x > 0 ? 0 : 1.6))), parts: [box([0.27, 0.1, 0.055], [dir * 0.13, 0, 0], "dark"), cyl(0.088, 0.06, [0, 0, 0], "rubber", "z"), cyl(0.05, 0.06, [dir * 0.27, 0, 0], "rubber", "z"), box([0.27, 0.02, 0.06], [dir * 0.135, 0.066, 0], "rubber", [0, 0, -dir * 0.14]), box([0.27, 0.02, 0.06], [dir * 0.135, -0.066, 0], "rubber", [0, 0, dir * 0.14]), cyl(0.02, 0.07, [0, 0, 0], "glow", "z")] });
    }
  }
  return { links: mk.links, height: 0.108, speed: 0.5, size: 0.72 };
}

function triwheel(): Body {
  const mk = new Maker();
  mk.add(trunk([box([0.34, 0.4, 0.3], [0, 0.36, 0], "shell"), box([0.36, 0.05, 0.32], [0, 0.14, 0], "dark"), box([0.02, 0.2, 0.18], [0.175, 0.38, 0], "dark"), box([0.01, 0.03, 0.16], [0.19, 0.46, 0], "glow"), cyl(0.02, 0.36, [-0.18, 0.68, 0.12], "steel"), cyl(0.02, 0.36, [-0.18, 0.68, -0.12], "steel"), cyl(0.018, 0.28, [-0.18, 0.86, 0], "dark", "z")]));
  for (const side of [-1, 1] as const) {
    const n = side < 0 ? "left" : "right";
    const c = mk.add({ name: `${n}.cluster`, parent: 0, at: [0, 0, side * 0.22], joint: "z", turn: (t) => -TAU / 12 + -t * 0.5, parts: [cyl(0.05, 0.05, [0, 0, 0], "dark", "z"), ...[0, 1, 2].map((i) => box([0.17, 0.045, 0.03], [Math.cos((i / 3) * TAU) * 0.085, Math.sin((i / 3) * TAU) * 0.085, 0], "steel", [0, 0, (i / 3) * TAU]))] });
    for (let i = 0; i < 3; i++) {
      const a = (i / 3) * TAU;
      wheel(mk, `${n}.wheel${i + 1}`, c, [Math.cos(a) * 0.17, Math.sin(a) * 0.17, side * 0.04], 0.095, 0.05, (t) => -t * 5, { passive: i > 0 });
    }
  }
  return { links: mk.links, height: 0.2, speed: 0.4, base: (t) => ({ y: 0.034 * Math.abs(Math.cos(((t * 0.5) * 3) / 2 + 0.2)) - 0.015 }), size: 0.82 };
}

function rocker(): Body {
  const mk = new Maker();
  const spin = (t: number) => (-t * 0.35) / 0.1;
  const rock = (t: number, s: number) => 0.14 * Math.sin(t * 1.1 + (s > 0 ? 0 : Math.PI));
  mk.add(trunk([box([0.46, 0.14, 0.34], [0, 0.24, 0], "shell"), box([0.5, 0.02, 0.5], [-0.04, 0.33, 0], "dark"), cyl(0.016, 0.26, [0.18, 0.45, 0], "steel"), box([0.09, 0.07, 0.15], [0.2, 0.6, 0], "shell"), box([0.012, 0.03, 0.11], [0.248, 0.6, 0], "glow")]));
  for (const side of [-1, 1] as const) {
    const n = side < 0 ? "left" : "right";
    const rk = mk.add({ name: `${n}.rocker`, parent: 0, at: [0.0, 0.2, side * 0.22], joint: "z", passive: true, turn: (t) => rock(t, side), parts: [cyl(0.04, 0.05, [0, 0, 0], "steel", "z"), box([0.38, 0.035, 0.035], [0.15, -0.1, 0], "dark", [0, 0, -0.58]), box([0.24, 0.035, 0.035], [-0.1, -0.06, 0], "dark", [0, 0, 0.56])] });
    wheel(mk, `${n}.front`, rk, [0.32, -0.2, side * 0.06], 0.1, 0.08, spin);
    const bg = mk.add({ name: `${n}.bogie`, parent: rk, at: [-0.2, -0.12, 0], joint: "z", passive: true, turn: (t) => -rock(t, side) * 1.5, parts: [cyl(0.03, 0.05, [0, 0, 0], "steel", "z"), box([0.34, 0.03, 0.03], [0, -0.04, 0], "dark")] });
    wheel(mk, `${n}.middle`, bg, [0.15, -0.08, side * 0.06], 0.1, 0.08, spin);
    wheel(mk, `${n}.rear`, bg, [-0.17, -0.08, side * 0.06], 0.1, 0.08, spin);
  }
  return { links: mk.links, height: 0.1, speed: 0.35, size: 0.76 };
}

function manipulator(): Body {
  const mk = new Maker();
  const r = 0.1;
  mk.add(trunk([box([0.58, 0.16, 0.42], [0, 0.1, 0], "shell"), box([0.6, 0.03, 0.44], [0, 0.02, 0], "dark"), box([0.02, 0.04, 0.3], [0.3, 0.1, 0], "glow"), box([0.5, 0.02, 0.36], [0, 0.19, 0], "dark")]));
  wheel(mk, "left.wheel", 0, [0, 0, -0.22], r, 0.05, (t) => (-t * 0.25) / r);
  wheel(mk, "right.wheel", 0, [0, 0, 0.22], r, 0.05, (t) => (-t * 0.25) / r);
  mk.add({ name: "caster", parent: 0, at: [-0.23, -0.06, 0], joint: "y", passive: true, turn: () => 0, parts: [ball(0.04, [0, 0, 0], "rubber")] });
  const l = [0.14, 0.2, 0.16, 0.18, 0.13, 0.1, 0.07];
  const axes = ["y", "z", "y", "z", "y", "z", "y"] as const;
  const rest = [0.4, -0.5, 0.2, -1.1, 0.1, -0.9, 0];
  let p = 0;
  let at: V3 = [0.14, 0.2, 0.1];
  for (let i = 0; i < 7; i++) {
    const w = 0.062 - i * 0.005;
    p = mk.add({ name: `arm${i + 1}`, parent: p, at, joint: axes[i]!, turn: (t) => rest[i]! + 0.22 * Math.sin(t * 0.7 + i * 1.3), parts: [axes[i] === "y" ? cyl(w, 0.07, [0, 0.02, 0], "dark") : cyl(w * 0.95, w * 2.2, [0, 0, 0], "steel", "z"), box([w * 1.45, l[i]!, w * 1.45], [0, l[i]! / 2 + 0.02, 0], i % 2 ? "shell" : "dark"), ...(axes[i] === "z" ? [cyl(w * 0.4, w * 2.3, [0, 0, 0], "glow", "z")] : [])] });
    at = [0, l[i]! + 0.03, 0];
  }
  mk.add({ name: "gripper", parent: p, at, joint: "fixed", parts: [box([0.05, 0.03, 0.11], [0, 0.01, 0], "dark"), box([0.03, 0.08, 0.016], [0, 0.06, 0.045], "steel"), box([0.03, 0.08, 0.016], [0, 0.06, -0.045], "steel")] });
  return { links: mk.links, height: r, speed: 0.25, size: 0.9 };
}

function snake(): Body {
  const mk = new Maker();
  const n = 9,
    len = 0.17;
  let parent = -1;
  for (let i = 0; i < n; i++) {
    const at: V3 = i ? [-len, 0, 0] : [0, 0, 0];
    parent = mk.add({ name: i ? `link${i}` : "head", parent, at, joint: i ? "y" : "free", turn: (t) => 0.62 * Math.sin(t * 2.2 - i * 0.78), parts: [box([len * 0.82, 0.075, 0.085], [-len / 2, 0.02, 0], i ? (i % 2 ? "dark" : "shell") : "shell"), cyl(0.034, 0.09, [0, 0.02, 0], "steel"), cyl(0.012, 0.094, [0, 0.02, 0], "glow"), ...(i ? [] : [box([0.05, 0.03, 0.06], [0.03, 0.03, 0], "dark"), box([0.01, 0.014, 0.05], [0.056, 0.034, 0], "glow")])] });
    for (const s of [-1, 1] as const) mk.add({ name: `${i ? `link${i}` : "head"}.wheel.${s < 0 ? "left" : "right"}`, parent, at: [-len / 2, -0.01, s * 0.055], joint: "z", passive: true, turn: (t) => -t * 9, parts: [cyl(0.032, 0.018, [0, 0, 0], "rubber", "z")] });
  }
  return { links: mk.links, height: 0.042, speed: 0.3, base: (t) => ({ x: 0.62, yaw: -0.3 + 0.24 * Math.sin(t * 2.2 + 1.2) }), size: 0.94 };
}

function balancer(): Body {
  const mk = new Maker();
  const r = 0.14;
  const lean = (t: number) => 0.07 * Math.sin(t * 1.5) + 0.025 * Math.sin(t * 3.7);
  mk.add(trunk([box([0.1, 0.46, 0.24], [0, 0.3, 0], "shell"), box([0.11, 0.08, 0.26], [0, 0.02, 0], "dark"), box([0.02, 0.06, 0.16], [0.05, 0.42, 0], "dark"), box([0.006, 0.018, 0.13], [0.062, 0.42, 0], "glow"), cyl(0.014, 0.3, [0, 0.53, 0], "steel", "z")]));
  for (const side of [-1, 1] as const) wheel(mk, `${side < 0 ? "left" : "right"}.wheel`, 0, [0, 0, side * 0.17], r, 0.05, (t) => -lean(t - 0.18) * 5 - lean(t));
  return { links: mk.links, height: r, speed: 0, base: (t) => ({ pitch: -lean(t), x: 0.1 * Math.sin(t * 1.5 - 0.5) }), size: 0.74 };
}

const WIDE: Step = { period: 1.0, duty: 0.62, stride: 0.16, lift: 0.07 };
export const makers: Record<string, () => Body> = {
  "quad-mammal": () => walker({ legs: 4, femur: 0.25, tibia: 0.25, length: 0.56, width: 0.2, thick: 1, gait: TROT, bend: 1 }),
  "quad-sprawl": () => {
    const mk = new Maker();
    mk.add(trunk([box([0.52, 0.09, 0.24], [0, 0.01, 0], "shell"), box([0.36, 0.05, 0.17], [0, 0.07, 0], "dark"), box([0.1, 0.07, 0.13], [0.3, 0.01, 0], "dark"), box([0.1, 0.07, 0.13], [-0.3, 0.01, 0], "dark"), box([0.2, 0.012, 0.08], [0, 0.098, 0], "glow")]));
    for (const x of [1, -1])
      for (const z of [-1, 1])
        sprawl(mk, { name: `${x > 0 ? "fore" : "hind"}.${z < 0 ? "left" : "right"}`, parent: 0, at: [x * 0.22, 0, z * 0.13], face: Math.atan2(z * 0.86, x * 0.5), coxa: 0.07, l1: 0.21, l2: 0.27, hang: 0.17, out: 0.36, phase: x * z > 0 ? 0 : 0.5, gait: WIDE });
    return { links: mk.links, height: 0.196, speed: WIDE.stride / (WIDE.duty * WIDE.period), size: 0.68 };
  },
  "hexapod-radial": () => radial(6, { r: 0.2, coxa: 0.07, l1: 0.16, l2: 0.28, hang: 0.2, out: 0.3, gait: { ...WIDE, duty: 0.55 }, tripod: true }),
  octopod: () => radial(8, { r: 0.22, coxa: 0.07, l1: 0.19, l2: 0.3, hang: 0.2, out: 0.34, gait: { ...WIDE, duty: 0.66, period: 1.2 }, tripod: false }),
  "biped-point": biped,
  humanoid,
  "diff-drive": () => car("diff"),
  ackermann: () => car("ackermann"),
  mecanum: () => car("mecanum"),
  "omni-3": omni,
  "swerve-4": swerve,
  "tracked-flipper": tracked,
  "wheel-leg": () => walker({ legs: 4, femur: 0.24, tibia: 0.24, length: 0.56, width: 0.22, thick: 1, gait: { period: 3.4, duty: 0.78, stride: 0.1, lift: 0.08 }, bend: 1, wheel: 0.085, pattern: "wave" }),
  "tri-wheel-stair": triwheel,
  "rocker-bogie": rocker,
  "mobile-manipulator": manipulator,
  "snake-crawler": snake,
  "balancer-2w": balancer,
};
