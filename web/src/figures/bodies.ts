/**
 * Robots as data. A body is a tree of links; each link has a joint, an offset from its parent and a few solids.
 * The stage, the table beside it, the crowd and the door test all read the same tree. x is forward, y up, z right.
 *
 * The motion comes from the mechanism, not from the eye. A foot on the ground stays where it landed. A wheel turns
 * by the distance its hub travels over its radius, along whatever direction its rollers allow. A steered wheel
 * points along its hub's velocity, and the floor moves the way the body drives (`travel`). A body made `idle`
 * waits on the spot: legs mark time and wheels hold still, which is how the crowd stands in front of the door.
 */
export type V3 = [number, number, number];
export type Solid = "box" | "cyl" | "ball" | "cap" | "tire" | "ring" | "barrel";
export type Mat = "shell" | "dark" | "steel" | "rubber" | "glow" | "amber" | "lens";

export interface Part {
  s: Solid;
  /**
   * box: full sizes. ball: radius. The round solids lie along their local y. cyl: radius at +y, length, radius at −y.
   * cap: radius, length between its two half-spheres, radius. tire: outer radius, width, rim radius.
   * ring: radius, tube radius, radius. barrel: middle radius, length, end radius.
   */
  d: V3;
  p: V3;
  /** Euler rotation, XYZ order. */
  r?: V3;
  m: Mat;
}
/** A track: copies of the link's parts, `pitch` apart, round a loop from x = 0 to x = len with ends of radius r. */
export interface Belt {
  len: number;
  r: number;
  pitch: number;
}
export interface Link {
  name: string;
  parent: number;
  /** Offset of the joint in the parent's frame. */
  at: V3;
  /** A fixed rotation applied before the joint turns. */
  set?: V3;
  joint: "free" | "fixed" | "x" | "y" | "z";
  /** Joint angle at time t. For a belt, how far the track has run. */
  turn?: (t: number) => number;
  passive?: boolean;
  belt?: Belt;
  parts: Part[];
}
/** A place on the floor: metres forward and to the right, and a heading that turns left when positive. */
export interface Pose {
  x: number;
  z: number;
  yaw: number;
}
export interface Body {
  links: Link[];
  /** Height of the base above the floor. */
  height: number;
  /** Mean forward speed, metres per second. */
  speed: number;
  /** Bob, sway or lean of the base. */
  base?: (t: number) => { y?: number; pitch?: number; roll?: number; yaw?: number; x?: number; z?: number };
  /** Where the body has driven to by time t. The stage moves the floor the other way. */
  travel?: (t: number) => Pose;
  /** Seconds after which the motion repeats, so that a measurement can cover all of it. */
  cycle?: number;
  /** Radius that frames the whole body. */
  size: number;
}

const TAU = Math.PI * 2;
const ease = (v: number) => {
  const c = Math.min(1, Math.max(0, v));
  return c * c * (3 - 2 * c);
};
const mod = (v: number, m: number) => ((v % m) + m) % m;
/** 1 inside [a, b], 0 outside, with smooth ramps of the given length. */
const gate = (u: number, a: number, b: number, ramp: number) => ease((u - a) / ramp) * ease((b - u) / ramp);

/* ---------- Solids ---------- */

type Axis = "x" | "y" | "z";
const lay = (axis: Axis): V3 | undefined =>
  axis === "x" ? [0, 0, Math.PI / 2] : axis === "z" ? [Math.PI / 2, 0, 0] : undefined;
const ball = (r: number, p: V3, m: Mat): Part => ({ s: "ball", d: [r, r, r], p, m });
const box = (d: V3, p: V3, m: Mat, r?: V3): Part => ({ s: "box", d, p, m, r });
/** A cylinder of radius r and length l; axis picks the way it lies. */
const cyl = (r: number, l: number, p: V3, m: Mat, axis: Axis = "y"): Part => ({ s: "cyl", d: [r, l, r], p, m, r: lay(axis) });
const cap = (r: number, l: number, p: V3, m: Mat, axis: Axis = "y"): Part => ({ s: "cap", d: [r, l, r], p, m, r: lay(axis) });
const ring = (r: number, tube: number, p: V3, m: Mat, axis: Axis = "y"): Part => ({ s: "ring", d: [r, tube, r], p, m, r: lay(axis) });
const tire = (r: number, w: number, rim: number, p: V3, m: Mat, axis: Axis = "z"): Part => ({ s: "tire", d: [r, w, rim], p, m, r: lay(axis) });

/** XYZ Euler angles of a rotation matrix given by rows. */
function euler(m: number[]): V3 {
  const y = Math.asin(Math.max(-1, Math.min(1, m[2]!)));
  if (Math.abs(m[2]!) < 0.9999999) return [Math.atan2(-m[5]!, m[8]!), y, Math.atan2(-m[1]!, m[0]!)];
  return [Math.atan2(m[7]!, m[4]!), y, 0];
}
/** The rotation that turns local y towards `d` by the shortest way, then turns `twist` about it. */
export function aim(d: V3, twist = 0): V3 {
  const n = Math.hypot(...d);
  const [x, y, z] = [d[0] / n, d[1] / n, d[2] / n];
  const s = Math.hypot(x, z);
  let m: number[];
  if (s < 1e-9) m = y > 0 ? [1, 0, 0, 0, 1, 0, 0, 0, 1] : [1, 0, 0, 0, -1, 0, 0, 0, -1];
  else {
    // Rodrigues about k = ŷ × d, which lies in the floor plane.
    const kx = z / s;
    const kz = -x / s;
    const v = 1 - y;
    m = [y + kx * kx * v, -kz * s, kx * kz * v, kz * s, y, -kx * s, kx * kz * v, kx * s, y + kz * kz * v];
  }
  if (twist) {
    const c = Math.cos(twist);
    const t = Math.sin(twist);
    m = [m[0]! * c - m[2]! * t, m[1]!, m[0]! * t + m[2]! * c, m[3]! * c - m[5]! * t, m[4]!, m[3]! * t + m[5]! * c, m[6]! * c - m[8]! * t, m[7]!, m[6]! * t + m[8]! * c];
  }
  return euler(m);
}
const mid = (a: V3, b: V3): V3 => [(a[0] + b[0]) / 2, (a[1] + b[1]) / 2, (a[2] + b[2]) / 2];
const gap = (a: V3, b: V3): V3 => [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
/** A rounded bar from a to b, w across in local x and deep in local z. */
const slab = (a: V3, b: V3, w: number, deep: number, m: Mat, twist = 0): Part => ({
  s: "box",
  d: [w, Math.hypot(...gap(a, b)), deep],
  p: mid(a, b),
  r: aim(gap(a, b), twist),
  m,
});
/** A capsule whose ends are centred on a and b. */
const strut = (a: V3, b: V3, r: number, m: Mat): Part => ({ s: "cap", d: [r, Math.hypot(...gap(a, b)), r], p: mid(a, b), r: aim(gap(a, b)), m });
/** A cylinder from a, radius r0, to b, radius r1. */
const taper = (a: V3, b: V3, r0: number, r1: number, m: Mat): Part => ({ s: "cyl", d: [r1, Math.hypot(...gap(a, b)), r0], p: mid(a, b), r: aim(gap(a, b)), m });

/* ---------- Motion ---------- */

/**
 * The cubic between two table entries a and b, h apart, that leaves a at rate ra and arrives at b at rate rb, at the
 * fraction f of the way. Read this way a table of positions gives speeds that never jump.
 */
const hermite = (f: number, a: number, b: number, ra: number, rb: number, h: number) => {
  const f2 = f * f;
  const f3 = f2 * f;
  return (2 * f3 - 3 * f2 + 1) * a + (f3 - 2 * f2 + f) * h * ra + (3 * f2 - 2 * f3) * b + (f3 - f2) * h * rb;
};

/** Running integrals, from 0 to t, of rates that repeat every `period`: a table read in constant time. */
export function integral(period: number, rates: (t: number) => readonly number[], n = 720) {
  const dim = rates(0).length;
  const table = new Float64Array((n + 1) * dim);
  const slope = new Float64Array((n + 1) * dim);
  const h = period / n;
  let last = rates(0);
  slope.set(last, 0);
  for (let k = 1; k <= n; k++) {
    const now = rates(k * h);
    for (let j = 0; j < dim; j++) table[k * dim + j] = table[(k - 1) * dim + j]! + ((last[j]! + now[j]!) / 2) * h;
    slope.set(now, k * dim);
    last = now;
  }
  return (t: number, j: number) => {
    const c = Math.floor(t / period);
    const u = ((t - c * period) / period) * n;
    const k = Math.min(n - 1, Math.floor(u));
    const [a, b] = [k * dim + j, (k + 1) * dim + j];
    return c * table[n * dim + j]! + hermite(u - k, table[a]!, table[b]!, slope[a]!, slope[b]!, h);
  };
}

/** Turns a body-frame vector (forward, right) by a heading. */
const turnBy = (yaw: number, x: number, z: number): [number, number] => [
  x * Math.cos(yaw) + z * Math.sin(yaw),
  -x * Math.sin(yaw) + z * Math.cos(yaw),
];
/**
 * Where a body gets to on the floor when it drives a twist [forward, right, turn rate], measured in its own frame,
 * that repeats every `period`. Whole periods compose exactly, so the path stays right however long it runs.
 */
export function drive(period: number, twist: (t: number) => readonly number[], n = 1440): (t: number) => Pose {
  const xs = new Float64Array(n + 1);
  const zs = new Float64Array(n + 1);
  const hs = new Float64Array(n + 1);
  const h = period / n;
  for (let k = 0; k < n; k++) {
    const [a = 0, b = 0, w = 0] = twist((k + 0.5) * h);
    const [dx, dz] = turnBy(hs[k]! + (w * h) / 2, a * h, b * h);
    xs[k + 1] = xs[k]! + dx;
    zs[k + 1] = zs[k]! + dz;
    hs[k + 1] = hs[k]! + w * h;
  }
  // Velocities at the entries themselves, in the frame the period starts in, for reading between them.
  const [vx, vz, vh] = [0, 0, 0].map(() => new Float64Array(n + 1)) as [Float64Array, Float64Array, Float64Array];
  for (let k = 0; k <= n; k++) {
    const [a = 0, b = 0, w = 0] = twist(k * h);
    [vx[k], vz[k]] = turnBy(hs[k]!, a, b);
    vh[k] = w;
  }
  const D = { x: xs[n]!, z: zs[n]!, yaw: hs[n]! };
  return (t) => {
    const c = Math.floor(t / period);
    const u = ((t - c * period) / period) * n;
    const k = Math.min(n - 1, Math.floor(u));
    const f = u - k;
    const lx = hermite(f, xs[k]!, xs[k + 1]!, vx[k]!, vx[k + 1]!, h);
    const lz = hermite(f, zs[k]!, zs[k + 1]!, vz[k]!, vz[k + 1]!, h);
    const lh = hermite(f, hs[k]!, hs[k + 1]!, vh[k]!, vh[k + 1]!, h);
    // c whole periods: the sum of D's step turned by 0, θ, …, (c − 1)θ, a geometric series in e^{−iθ}.
    let px = c * D.x;
    let pz = c * D.z;
    if (Math.abs(D.yaw) > 1e-9) {
      const [qr, qi] = [1 - Math.cos(c * D.yaw), Math.sin(c * D.yaw)];
      const [dr, di] = [1 - Math.cos(D.yaw), Math.sin(D.yaw)];
      const den = dr * dr + di * di;
      const sr = (qr * dr + qi * di) / den;
      const si = (qi * dr - qr * di) / den;
      px = sr * D.x - si * D.z;
      pz = sr * D.z + si * D.x;
    }
    const [ox, oz] = turnBy(c * D.yaw, lx, lz);
    return { x: px + ox, z: pz + oz, yaw: c * D.yaw + lh };
  };
}
/** Velocity of a point fixed to a body that drives the twist [vx, vz, ω]. */
const pointVelocity = (tw: readonly number[], px: number, pz: number): [number, number] => [
  (tw[0] ?? 0) + (tw[2] ?? 0) * pz,
  (tw[1] ?? 0) - (tw[2] ?? 0) * px,
];
/** Heading of a direction on the floor, in the same sense as yaw. */
const heading = (x: number, z: number) => Math.atan2(-z, x);
/**
 * Spin rate of a wheel whose rollers touch the floor along u = (cos γ, sin γ) in its own frame (γ = 0: an ordinary
 * wheel or an omni wheel; ±45°: mecanum). Only motion along the roller axis has to come from the wheel turning.
 */
const rollRate = (v: readonly [number, number], dir: readonly [number, number], lean: number, r: number) =>
  -(v[0] * (dir[0] * Math.cos(lean) - dir[1] * Math.sin(lean)) + v[1] * (dir[1] * Math.cos(lean) + dir[0] * Math.sin(lean))) /
  (r * Math.cos(lean));

/**
 * The swivel angle of a caster that trails a distance d behind a pivot at (px, pz). Its wheel cannot slide sideways,
 * so θ' = −(vx sin θ + vz cos θ) / d − ω. Run to its periodic steady state, then tabled over one period.
 */
function trail(period: number, twist: (t: number) => readonly number[], px: number, pz: number, d: number, n = 2880) {
  const h = period / n;
  const rate = (t: number, th: number) => {
    const tw = twist(t);
    const [vx, vz] = pointVelocity(tw, px, pz);
    return -(vx * Math.sin(th) + vz * Math.cos(th)) / d - (tw[2] ?? 0);
  };
  const table = new Float64Array(n + 1);
  let th = 0;
  for (let lap = 0; lap < 4; lap++)
    for (let k = 0; k < n; k++) {
      const t = k * h;
      if (lap === 3) table[k] = th;
      const a = rate(t, th);
      const b = rate(t + h / 2, th + (a * h) / 2);
      const c = rate(t + h / 2, th + (b * h) / 2);
      const e = rate(t + h, th + c * h);
      th += ((a + 2 * b + 2 * c + e) * h) / 6;
    }
  table[n] = th;
  return (t: number) => {
    const u = (mod(t, period) / period) * n;
    const k = Math.min(n - 1, Math.floor(u));
    return table[k]! + (table[k + 1]! - table[k]!) * (u - k);
  };
}

/* ---------- Legs ---------- */

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
/**
 * Where a foot is in its cycle, from its hip. On the ground it moves back with the floor. In the air it follows a
 * cycloid over the floor, so that it leaves and lands at rest on the floor, not at rest on the body.
 */
export function foot(phase: number, g: Step, lead = 0): [x: number, up: number] {
  const p = phase - Math.floor(phase);
  if (p < g.duty) return [lead + g.stride * (0.5 - p / g.duty), 0];
  const s = (p - g.duty) / (1 - g.duty);
  return [lead + g.stride * (s - 0.5 - Math.sin(TAU * s) / (TAU * g.duty)), g.lift * Math.sin(Math.PI * s) ** 1.4];
}
/** The same gait marking time: feet lift and land where they were. */
const still = (g: Step): Step => ({ period: g.period * 1.1, duty: 0.6, stride: 0, lift: g.lift * 0.55 });

class Maker {
  links: Link[] = [];
  add(l: Link) {
    this.links.push(l);
    return this.links.length - 1;
  }
}

interface Leg {
  name: string;
  parent: number;
  at: V3;
  l1: number;
  l2: number;
  bend: 1 | -1;
  side: 1 | -1;
  thick?: number;
  /** Where the foot belongs at time t, from the hip: forward, and down (negative). */
  target: (t: number) => readonly [number, number];
  /** Hip roll that keeps the leg under the body when the body rolls. */
  roll?: (t: number) => number;
  /**
   * Where the foot belongs to the side of the hip, when the base shifts sideways but stays level. The hip then rolls
   * by exactly what puts it there, given that the thigh hangs from an arm across the hip.
   */
  lateral?: (t: number) => number;
  wheel?: number;
  /** How far a wheeled foot has rolled by time t. */
  rolled?: (t: number) => number;
}
/** A mammalian leg: hip roll, hip pitch, knee. Its foot goes where `target` says. */
function leg(mk: Maker, o: Leg) {
  const k = o.thick ?? 1;
  const end = o.at[0] >= 0 ? 1 : -1;
  let memo = NaN;
  let angles: readonly [number, number, number] = [0, 0, 0];
  const solve = (t: number) => {
    if (t !== memo) {
      memo = t;
      const [x, y] = o.target(t);
      if (o.lateral) {
        // Seen from the front, the foot at (y, side) is the point (−D, arm) of the leg's own plane rolled about the
        // hip: the leg's reach D in its plane follows from the distance, the roll from the angle between the two.
        const arm = 0.08 * o.side * k;
        const side = arm + o.lateral(t);
        const D = Math.sqrt(Math.max(0, y * y + side * side - arm * arm));
        // Both angles sit near ±π, below the hip, so their difference is wrapped to the turn it means.
        const roll = Math.atan2(side, y) - Math.atan2(arm, -D);
        angles = [...reach(o.l1, o.l2, x, -D, o.bend), Math.atan2(Math.sin(roll), Math.cos(roll))];
      } else angles = [...reach(o.l1, o.l2, x, y, o.bend), o.roll?.(t) ?? 0];
    }
    return angles;
  };
  const hip = mk.add({
    name: `${o.name}.hip`,
    parent: o.parent,
    at: o.at,
    joint: "x",
    turn: (t) => solve(t)[2],
    parts: [
      cyl(0.05 * k, 0.1 * k, [0, 0, 0], "dark", "x"),
      cyl(0.04 * k, 0.014, [end * 0.054 * k, 0, 0], "steel", "x"),
      ring(0.05 * k, 0.0045 * k, [end * 0.03 * k, 0, 0], "glow", "x"),
      box([0.066 * k, 0.058 * k, 0.05 * k], [0, -0.004, o.side * 0.044 * k], "dark"),
    ],
  });
  const thigh = mk.add({
    name: `${o.name}.thigh`,
    parent: hip,
    at: [0, 0, 0.08 * o.side * k],
    joint: "z",
    turn: (t) => solve(t)[0],
    parts: [
      cyl(0.06 * k, 0.066 * k, [0, 0, 0], "dark", "z"),
      cyl(0.046 * k, 0.012, [0, 0, o.side * 0.036 * k], "steel", "z"),
      ring(0.052 * k, 0.0045 * k, [0, 0, o.side * 0.037 * k], "glow", "z"),
      slab([0, -0.02 * k, 0], [0, -o.l1 * 0.9, 0], 0.07 * k, 0.052 * k, "shell"),
      // The rod that drives the knee from a motor at the hip.
      strut([0.014 * k, -0.04 * k, -o.side * 0.03 * k], [0.01 * k, -o.l1 * 0.93, -o.side * 0.03 * k], 0.008 * k, "dark"),
    ],
  });
  const shank = mk.add({
    name: `${o.name}.shank`,
    parent: thigh,
    at: [0, -o.l1, 0],
    joint: "z",
    turn: (t) => solve(t)[1],
    parts: [
      cyl(0.038 * k, 0.062 * k, [0, 0, 0], "steel", "z"),
      taper([0, 0, 0], [0, -o.l2 * (o.wheel ? 0.98 : 0.9), 0], 0.025 * k, 0.015 * k, "dark"),
      ...(o.wheel
        ? []
        : [ball(0.03 * k, [0, -o.l2, 0], "rubber"), ring(0.019 * k, 0.005 * k, [0, -o.l2 + 0.034 * k, 0], "steel")]),
    ],
  });
  if (o.wheel) {
    const r = o.wheel;
    // World angle of the wheel is hip + knee + its own: rolling without slipping fixes the sum.
    wheel(mk, `${o.name}.wheel`, shank, [0, -o.l2, 0.05 * o.side], r, 0.05, (t) => -(o.rolled?.(t) ?? 0) / r - solve(t)[0] - solve(t)[1]);
  }
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
  let memo = NaN;
  let angles: readonly [number, number, number] = [0, 0, 0];
  const solve = (t: number) => {
    if (t !== memo) {
      memo = t;
      const [x, up] = foot(t / o.gait.period + o.phase, o.gait);
      const fx = rest[0] + x;
      const fz = rest[1];
      const yaw = Math.atan2(fz, fx) - o.face;
      const r = Math.hypot(fx, fz) - o.coxa;
      const [h, kn] = reach(o.l1, o.l2, r, -o.hang + up, -1);
      angles = [yaw, h, kn];
    }
    return angles;
  };
  const coxa = mk.add({
    name: `${o.name}.coxa`,
    parent: o.parent,
    at: o.at,
    set: [0, -o.face, 0],
    joint: "y",
    turn: (t) => -solve(t)[0],
    parts: [
      cyl(0.05 * k, 0.08 * k, [0, 0, 0], "dark"),
      cyl(0.041 * k, 0.012, [0, 0.044 * k, 0], "steel"),
      ring(0.05 * k, 0.0045 * k, [0, 0.042 * k, 0], "glow"),
      slab([0.02 * k, 0, 0], [o.coxa, 0, 0], 0.05 * k, 0.056 * k, "shell"),
    ],
  });
  const femur = mk.add({
    name: `${o.name}.femur`,
    parent: coxa,
    at: [o.coxa, 0, 0],
    joint: "z",
    turn: (t) => solve(t)[1],
    parts: [
      cyl(0.045 * k, 0.074 * k, [0, 0, 0], "steel", "z"),
      ring(0.044 * k, 0.004 * k, [0, 0, 0.04 * k], "glow", "z"),
      ring(0.044 * k, 0.004 * k, [0, 0, -0.04 * k], "glow", "z"),
      slab([0, -0.015 * k, 0], [0, -o.l1 * 0.92, 0], 0.05 * k, 0.044 * k, "shell"),
    ],
  });
  mk.add({
    name: `${o.name}.tibia`,
    parent: femur,
    at: [0, -o.l1, 0],
    joint: "z",
    turn: (t) => solve(t)[2],
    parts: [
      cyl(0.036 * k, 0.062 * k, [0, 0, 0], "dark", "z"),
      taper([0, 0, 0], [0, -o.l2 * 0.92, 0], 0.024 * k, 0.012 * k, "dark"),
      ball(0.026 * k, [0, -o.l2, 0], "rubber"),
    ],
  });
}

/* ---------- Wheels ---------- */

interface WheelOptions {
  passive?: boolean;
  set?: V3;
  /** tire: an ordinary wheel. mecanum and omni: rollers round the rim. cleat: a rover's grousered rim. */
  kind?: "tire" | "mecanum" | "omni" | "cleat";
  /** Angle of a mecanum wheel's rollers to its rim. */
  lean?: number;
}
/** Barrel rollers round a rim; each axis leans `lean` from the rim's tangent towards the axle. */
function rollers(r: number, n: number, lean: number, z: number, offset: number, length: number, girth: number): Part[] {
  const out: Part[] = [];
  const rho = r - girth;
  for (let i = 0; i < n; i++) {
    const a = offset + (i / n) * TAU;
    const u: V3 = [-Math.sin(a) * Math.cos(lean), Math.cos(a) * Math.cos(lean), Math.sin(lean)];
    out.push({ s: "barrel", d: [girth, length, girth * 0.55], p: [Math.cos(a) * rho, Math.sin(a) * rho, z], r: aim(u), m: "rubber" });
  }
  return out;
}
function wheel(mk: Maker, name: string, parent: number, at: V3, r: number, w: number, turn: (t: number) => number, o: WheelOptions = {}) {
  const parts: Part[] = [];
  const kind = o.kind ?? "tire";
  if (kind === "mecanum") {
    parts.push(
      cyl(r * 0.52, w * 0.92, [0, 0, 0], "dark", "z"),
      cyl(r * 0.7, 0.008, [0, 0, w * 0.46], "steel", "z"),
      cyl(r * 0.7, 0.008, [0, 0, -w * 0.46], "steel", "z"),
      ...rollers(r, 9, o.lean ?? Math.PI / 4, 0, 0, (w * 0.86) / Math.sin(Math.abs(o.lean ?? Math.PI / 4)), r * 0.21),
    );
  } else if (kind === "omni") {
    const girth = r * 0.2;
    const length = ((TAU * (r - girth)) / 7) * 0.84;
    parts.push(
      cyl(r * 0.6, w * 0.9, [0, 0, 0], "dark", "z"),
      cyl(r * 0.74, 0.006, [0, 0, 0], "steel", "z"),
      ...rollers(r, 7, 0, -w * 0.25, 0, length, girth),
      ...rollers(r, 7, 0, w * 0.25, Math.PI / 7, length, girth),
    );
  } else if (kind === "cleat") {
    parts.push(cyl(r * 0.93, w, [0, 0, 0], "steel", "z"), cyl(r * 0.7, w * 1.04, [0, 0, 0], "dark", "z"));
    for (let i = 0; i < 14; i++) {
      const a = (i / 14) * TAU;
      parts.push(box([r * 0.16, r * 0.1, w * 1.02], [Math.cos(a) * r * 0.95, Math.sin(a) * r * 0.95, 0], "dark", [0, 0, a + Math.PI / 2 + 0.25]));
    }
    for (let i = 0; i < 6; i++) {
      const a = (i / 6) * TAU;
      parts.push(box([r * 0.5, r * 0.07, w * 1.08], [Math.cos(a) * r * 0.34, Math.sin(a) * r * 0.34, 0], "steel", [0, 0, a]));
    }
  } else {
    parts.push(
      tire(r, w, r * 0.64, [0, 0, 0], "rubber"),
      cyl(r * 0.66, w * 0.82, [0, 0, 0], "steel", "z"),
      cyl(r * 0.56, w * 0.9, [0, 0, 0], "dark", "z"),
    );
    for (let i = 0; i < 5; i++) {
      const a = (i / 5) * TAU;
      parts.push(box([r * 0.48, r * 0.1, w * 0.94], [Math.cos(a) * r * 0.29, Math.sin(a) * r * 0.29, 0], "steel", [0, 0, a]));
    }
  }
  parts.push(cyl(r * 0.17, w * 1.06, [0, 0, 0], "dark", "z"), cyl(r * 0.1, w * 1.12, [0, 0, 0], "glow", "z"));
  return mk.add({ name, parent, at, set: o.set, joint: "z", turn, passive: o.passive, parts });
}

/* ---------- Bodies ---------- */

/** How a wheeled body waiting in a queue edges forwards and back. */
const inch = (t: number) => 0.045 * Math.sin(0.9 * t);

const trunk = (parts: Part[]): Link => ({ name: "base", parent: -1, at: [0, 0, 0], joint: "free", parts });
const twin = (f: (s: 1 | -1) => Part[]) => [...f(-1), ...f(1)];

export interface QuadSpec {
  legs: 4 | 6 | 8;
  femur: number;
  tibia: number;
  length: number;
  width: number;
  thick: number;
  gait: Step;
  bend: 1 | -1;
}
/** A walker with two, three or four pairs of legs, trotting on diagonal pairs. */
export function walker(q: QuadSpec, idle = false): Body {
  const mk = new Maker();
  const pairs = q.legs / 2;
  const hang = (q.femur + q.tibia) * 0.82;
  const g = idle ? still(q.gait) : q.gait;
  const L = q.length;
  const W = q.width;
  const k = q.thick;
  const top = 0.02 + 0.06 * k;
  // Highest over the middle of a stance, twice a stride.
  const bob = (t: number) => g.lift * 0.12 * Math.cos(2 * TAU * (t / g.period - g.duty / 2));
  const roll = (t: number) => 0.012 * Math.sin((TAU * t) / g.period);
  mk.add(
    trunk([
      box([L * 0.9, 0.12 * k, W * 1.16], [0, 0.02, 0], "shell"),
      box([L * 0.78, 0.05, W * 0.92], [0, -0.04 * k, 0], "dark"),
      box([L * 0.64, 0.024, W * 0.78], [0, top + 0.008, 0], "dark"),
      box([L * 0.26, 0.008, W * 0.22], [-L * 0.08, top + 0.022, 0], "glow"),
      ...twin((s) => [box([L * 0.5, 0.012, 0.014], [0, top + 0.026, s * W * 0.3], "steel")]),
      // Both ends carry the same block; only the sensors tell front from back.
      ...twin((e) => [box([0.12, 0.1, 0.17], [e * (L / 2 + 0.04), 0.03, 0], "dark"), box([0.02, 0.064, 0.176], [e * (L / 2 + 0.098), 0.03, 0], "shell")]),
      box([0.012, 0.022, 0.11], [L / 2 + 0.112, 0.062, 0], "glow"),
      ball(0.019, [L / 2 + 0.104, 0.022, -0.046], "lens"),
      ball(0.019, [L / 2 + 0.104, 0.022, 0.046], "lens"),
    ]),
  );
  for (let i = 0; i < pairs; i++) {
    const x = L * (0.43 - (0.86 * i) / (pairs - 1));
    for (const side of [-1, 1] as const) {
      const z = side * W * 0.5;
      const phase = ((i + (side === 1 ? 1 : 0)) % 2) * 0.5;
      leg(mk, {
        name: `${pairs === 2 ? (i ? "hind" : "fore") : `l${i + 1}`}.${side < 0 ? "left" : "right"}`,
        parent: 0,
        at: [x, 0, z],
        l1: q.femur,
        l2: q.tibia,
        // The base bobs and rolls; the feet stay on the floor because the target is measured from the moving hip.
        target: (t) => {
          const [fx, up] = foot(t / g.period + phase, g);
          return [fx, -hang - bob(t) + z * Math.sin(roll(t)) + up];
        },
        roll: (t) => -roll(t),
        // Front and hind knees mirror each other, so the body has both of its symmetries.
        bend: (x < 0 ? q.bend : -q.bend) as 1 | -1,
        side,
        thick: k,
      });
    }
  }
  const speed = idle ? 0 : g.stride / (g.duty * g.period);
  return {
    links: mk.links,
    height: hang + 0.03 * k,
    speed,
    base: (t) => ({ y: bob(t), roll: roll(t) }),
    travel: idle ? undefined : (t) => ({ x: speed * t, z: 0, yaw: 0 }),
    cycle: g.period,
    size: 0.62 + L * 0.5,
  };
}

const TROT: Step = { period: 0.62, duty: 0.56, stride: 0.24, lift: 0.085 };

function radial(legs: number, o: { r: number; coxa: number; l1: number; l2: number; hang: number; out: number; gait: Step; tripod: boolean }, idle: boolean): Body {
  const mk = new Maker();
  const g = idle ? still(o.gait) : o.gait;
  mk.add(
    trunk([
      cyl(o.r, 0.09, [0, 0.01, 0], "shell"),
      cyl(o.r * 0.93, 0.03, [0, -0.048, 0], "dark"),
      cyl(o.r * 0.76, 0.036, [0, 0.07, 0], "dark"),
      ring(o.r * 0.92, 0.006, [0, 0.058, 0], "glow"),
      cyl(o.r * 0.36, 0.024, [0, 0.096, 0], "steel"),
      ball(o.r * 0.28, [0, 0.1, 0], "lens"),
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
      gait: g,
      thick: 0.85,
    });
  }
  const speed = idle ? 0 : g.stride / (g.duty * g.period);
  return { links: mk.links, height: o.hang + 0.026 * 0.85, speed, travel: idle ? undefined : (t) => ({ x: speed * t, z: 0, yaw: 0 }), cycle: g.period, size: o.r + o.out + 0.12 };
}

/** Cassie's layout: a pelvis, three joints a leg, knees that bend backwards and feet that end in points. */
function biped(idle: boolean): Body {
  const mk = new Maker();
  const g0: Step = { period: 0.82, duty: 0.6, stride: 0.34, lift: 0.1 };
  const g = idle ? still(g0) : g0;
  const hang = 0.66;
  // The pelvis rides highest over a planted foot and shifts towards it.
  const bob = (t: number) => 0.012 * Math.cos(2 * TAU * (t / g.period - 0.3));
  const sway = (t: number) => -0.02 * Math.cos(TAU * (t / g.period - 0.3));
  mk.add(
    trunk([
      box([0.22, 0.15, 0.24], [0, 0.1, 0], "shell"),
      box([0.16, 0.06, 0.3], [0, 0.005, 0], "dark"),
      box([0.08, 0.07, 0.17], [0.1, 0.205, 0], "dark"),
      ball(0.018, [0.14, 0.212, -0.04], "lens"),
      ball(0.018, [0.14, 0.212, 0.04], "lens"),
      box([0.008, 0.012, 0.11], [0.141, 0.182, 0], "glow"),
      box([0.12, 0.01, 0.16], [-0.03, 0.18, 0], "glow"),
      box([0.08, 0.1, 0.18], [-0.12, 0.11, 0], "dark"),
    ]),
  );
  for (const side of [-1, 1] as const) {
    const phase = side < 0 ? 0 : 0.5;
    leg(mk, {
      name: side < 0 ? "left" : "right",
      parent: 0,
      at: [0, 0, side * 0.09],
      l1: 0.36,
      l2: 0.4,
      target: (t) => {
        const [x, up] = foot(t / g.period + phase, g);
        return [x, -hang - bob(t) + up];
      },
      // The pelvis shifts over the planted foot; each foot stays where it is on the floor.
      lateral: (t) => -sway(t),
      bend: 1,
      side,
      thick: 1.05,
    });
  }
  const speed = idle ? 0 : g.stride / (g.duty * g.period);
  return {
    links: mk.links,
    height: hang + 0.03 * 1.05,
    speed,
    base: (t) => ({ y: bob(t), z: sway(t) }),
    travel: idle ? undefined : (t) => ({ x: speed * t, z: 0, yaw: 0 }),
    cycle: g.period,
    size: 0.86,
  };
}

/** The joint order and count of Unitree's G1: six a leg, three in the waist, seven an arm. */
function humanoid(idle: boolean): Body {
  const mk = new Maker();
  const g0: Step = { period: 1.0, duty: 0.6, stride: 0.42, lift: 0.08 };
  const g = idle ? still(g0) : g0;
  const H = 0.735;
  const phase = (t: number, side: number) => t / g.period + (side < 0 ? 0 : 0.5);
  const bob = (t: number) => 0.01 * Math.cos(2 * TAU * (t / g.period - 0.3));
  const sway = (t: number) => -0.016 * Math.cos(TAU * (t / g.period - 0.3));
  mk.add(
    trunk([
      box([0.16, 0.12, 0.24], [0, -0.01, 0], "shell"),
      box([0.12, 0.07, 0.3], [0, -0.055, 0], "dark"),
      ring(0.07, 0.005, [0, 0.05, 0], "glow"),
    ]),
  );
  const yaw = mk.add({
    name: "waist.yaw",
    parent: 0,
    at: [0, 0.07, 0],
    joint: "y",
    turn: (t) => (idle ? 0.03 : 0.1) * Math.sin(TAU * phase(t, -1)),
    parts: [cyl(0.068, 0.05, [0, 0, 0], "dark"), cyl(0.056, 0.012, [0, 0.03, 0], "steel")],
  });
  const roll = mk.add({ name: "waist.roll", parent: yaw, at: [0, 0.04, 0], joint: "x", turn: (t) => 0.4 * Math.asin(sway(t) / H), parts: [cyl(0.04, 0.12, [0, 0, 0], "steel", "x")] });
  const chest = mk.add({
    name: "waist.pitch",
    parent: roll,
    at: [0, 0.03, 0],
    joint: "z",
    turn: () => -0.04,
    parts: [
      box([0.15, 0.12, 0.22], [0, 0.07, 0], "dark"),
      box([0.2, 0.25, 0.3], [0, 0.235, 0], "shell"),
      box([0.024, 0.13, 0.18], [0.1, 0.24, 0], "dark"),
      box([0.006, 0.012, 0.12], [0.114, 0.285, 0], "glow"),
      box([0.09, 0.2, 0.24], [-0.125, 0.22, 0], "dark"),
      box([0.13, 0.06, 0.36], [0, 0.335, 0], "dark"),
    ],
  });
  mk.add({
    name: "head",
    parent: chest,
    at: [0, 0.38, 0],
    joint: "fixed",
    parts: [
      cyl(0.034, 0.07, [0, 0.02, 0], "dark"),
      cap(0.085, 0.05, [0, 0.13, 0], "shell"),
      box([0.06, 0.08, 0.15], [0.056, 0.13, 0], "dark"),
      ball(0.017, [0.084, 0.14, -0.034], "lens"),
      ball(0.017, [0.084, 0.14, 0.034], "lens"),
      box([0.008, 0.01, 0.1], [0.088, 0.112, 0], "glow"),
    ],
  });
  for (const side of [-1, 1] as const) {
    const n = side < 0 ? "left" : "right";
    // Arms swing against the legs: back as the foot on the same side lands in front.
    const swing = (t: number) => -(idle ? 0.07 : 0.32) * Math.cos(TAU * (phase(t, side) - 0.04));
    let p = mk.add({
      name: `${n}.shoulder.pitch`,
      parent: chest,
      at: [0, 0.33, side * 0.2],
      joint: "z",
      turn: swing,
      parts: [cyl(0.05, 0.07, [0, 0, 0], "dark", "z"), cyl(0.04, 0.012, [0, 0, side * 0.037], "steel", "z"), ring(0.045, 0.004, [0, 0, side * 0.039], "glow", "z")],
    });
    p = mk.add({ name: `${n}.shoulder.roll`, parent: p, at: [0, 0, side * 0.045], joint: "x", turn: () => -side * 0.05, parts: [cyl(0.042, 0.08, [0, -0.01, 0], "steel", "x")] });
    p = mk.add({ name: `${n}.shoulder.yaw`, parent: p, at: [0, -0.05, 0], joint: "y", turn: () => side * 0.06, parts: [cap(0.038, 0.13, [0, -0.1, 0], "shell"), cyl(0.04, 0.03, [0, -0.02, 0], "dark")] });
    p = mk.add({
      name: `${n}.elbow`,
      parent: p,
      at: [0, -0.22, 0],
      joint: "z",
      turn: (t) => 0.42 + 0.16 * (1 + swing(t) / 0.32) * (idle ? 0.3 : 1),
      parts: [cyl(0.036, 0.074, [0, 0, 0], "steel", "z"), taper([0, -0.01, 0], [0, -0.15, 0], 0.034, 0.028, "dark")],
    });
    p = mk.add({ name: `${n}.wrist.roll`, parent: p, at: [0, -0.17, 0], joint: "y", turn: () => 0, parts: [cyl(0.03, 0.03, [0, 0, 0], "steel"), ring(0.03, 0.003, [0, 0.016, 0], "glow")] });
    p = mk.add({ name: `${n}.wrist.pitch`, parent: p, at: [0, -0.03, 0], joint: "z", turn: () => 0.1, parts: [ball(0.025, [0, 0, 0], "steel")] });
    // A five-fingered hand at rest, palm to the body: four fingers in two segments, curled a little towards the
    // palm, index forwards, and a thumb set across in front of them.
    const inward = -side;
    const hand: Part[] = [box([0.07, 0.085, 0.032], [0, -0.06, 0], "shell"), box([0.064, 0.012, 0.03], [0, -0.1, 0], "dark")];
    for (const [x, l] of [
      [0.024, 0.046],
      [0.008, 0.05],
      [-0.008, 0.047],
      [-0.024, 0.038],
    ] as const) {
      const knuckle: V3 = [x, -0.105 - 0.55 * l, inward * 0.004];
      const tip: V3 = [x, knuckle[1] - 0.45 * l * Math.cos(0.6), inward * (0.004 + 0.45 * l * Math.sin(0.6))];
      hand.push(strut([x, -0.104, 0], knuckle, 0.0078, "dark"), strut(knuckle, tip, 0.007, "dark"));
    }
    hand.push(strut([0.03, -0.05, inward * 0.01], [0.044, -0.08, inward * 0.026], 0.009, "dark"), strut([0.044, -0.08, inward * 0.026], [0.04, -0.103, inward * 0.034], 0.008, "dark"));
    mk.add({ name: `${n}.wrist.yaw`, parent: p, at: [0, 0, 0], joint: "x", turn: () => 0, parts: hand });

    /*
     * The leg in three dimensions, in G1's order: hip pitch, roll and yaw, knee, ankle pitch and roll. The ankle goes
     * where the foot belongs, under a pelvis that bobs and shifts over the planted foot, and the sole stands flat on
     * the floor pointing straight ahead. Solved from the foot back: seen from the sole, the hip lies in the plane of
     * the knee once the ankle has rolled, and what is left of the rotation is the hip's own pitch, roll and yaw.
     */
    let memo = NaN;
    let legAngles = [0, 0, 0, 0, 0, 0];
    const solve = (t: number) => {
      if (t !== memo) {
        memo = t;
        const [X, up] = foot(phase(t, side), g, -0.02);
        const Y = -(H + bob(t)) + up;
        const Z = -sway(t);
        const ar = Math.atan2(Z, -Y);
        const Yr = Y * Math.cos(ar) - Z * Math.sin(ar);
        const span = Math.min(X * X + Yr * Yr, 0.8 * 0.8 * 0.999);
        const knee = -Math.acos(Math.max(-1, Math.min(1, (span - 0.32) / 0.32)));
        const ankle = Math.atan2(-0.4 - 0.4 * Math.cos(knee), -0.4 * Math.sin(knee)) - Math.atan2(Yr, X);
        // Hip = Rx(−ar)·Rz(−(ankle + knee)), taken apart as pitch about z, then roll about x, then yaw about y.
        const c = ankle + knee;
        const pitch = Math.atan2(-Math.sin(c), Math.cos(ar) * Math.cos(c));
        const roll = Math.asin(-Math.sin(ar) * Math.cos(c));
        const yaw = Math.atan2(-Math.sin(ar) * Math.sin(c), Math.cos(ar));
        legAngles = [pitch, roll, knee, ankle, ar, yaw];
      }
      return legAngles;
    };
    let q = mk.add({
      name: `${n}.hip.pitch`,
      parent: 0,
      at: [0, -0.06, side * 0.1],
      joint: "z",
      turn: (t) => solve(t)[0]!,
      parts: [cyl(0.06, 0.07, [0, 0, 0], "dark", "z"), ring(0.055, 0.004, [0, 0, side * 0.037], "glow", "z")],
    });
    q = mk.add({ name: `${n}.hip.roll`, parent: q, at: [0, 0, 0], joint: "x", turn: (t) => solve(t)[1]!, parts: [box([0.09, 0.07, 0.07], [0, -0.05, 0], "steel")] });
    q = mk.add({ name: `${n}.hip.yaw`, parent: q, at: [0, 0, 0], joint: "y", turn: (t) => solve(t)[5]!, parts: [cap(0.056, 0.22, [0, -0.2, 0], "shell"), box([0.03, 0.2, 0.07], [0.05, -0.2, 0], "dark")] });
    q = mk.add({
      name: `${n}.knee`,
      parent: q,
      at: [0, -0.4, 0],
      joint: "z",
      turn: (t) => solve(t)[2]!,
      parts: [cyl(0.052, 0.1, [0, 0, 0], "steel", "z"), taper([0, -0.02, 0], [0, -0.36, 0], 0.05, 0.034, "dark"), box([0.022, 0.24, 0.07], [0.04, -0.17, 0], "shell")],
    });
    q = mk.add({ name: `${n}.ankle.pitch`, parent: q, at: [0, -0.4, 0], joint: "z", turn: (t) => solve(t)[3]!, parts: [ball(0.034, [0, 0, 0], "steel")] });
    mk.add({
      name: `${n}.ankle.roll`,
      parent: q,
      at: [0, 0, 0],
      joint: "x",
      turn: (t) => solve(t)[4]!,
      parts: [box([0.22, 0.03, 0.1], [0.04, -0.055, 0], "rubber"), box([0.15, 0.035, 0.084], [0.02, -0.027, 0], "shell")],
    });
  }
  const speed = idle ? 0 : g.stride / (g.duty * g.period);
  return {
    links: mk.links,
    height: H + 0.13,
    speed,
    base: (t) => ({ y: bob(t), z: sway(t) }),
    travel: idle ? undefined : (t) => ({ x: speed * t, z: 0, yaw: 0 }),
    cycle: g.period,
    size: 1.02,
  };
}

/** Two driven wheels and a trailing caster, weaving down the room. */
function diffDrive(idle: boolean): Body {
  const mk = new Maker();
  const r = 0.11;
  const track = 0.2;
  const P = 9;
  const pivot: V3 = [-0.15, -0.03, 0];
  const twist = (t: number) => (idle ? [0, 0, 0] : [0.5, 0, 0.55 * Math.sin((TAU * t) / P)]);
  const swivel = trail(P, twist, pivot[0], pivot[2], 0.025);
  const spin = integral(P, (t) => {
    const tw = twist(t);
    const [vx, vz] = pointVelocity(tw, pivot[0], pivot[2]);
    const th = swivel(t);
    // The caster wheel rolls at the pivot's speed along the way it points.
    return [-pointVelocity(tw, 0, -track)[0] / r, -pointVelocity(tw, 0, track)[0] / r, -(vx * Math.cos(th) - vz * Math.sin(th)) / 0.03];
  });
  mk.add(
    trunk([
      cyl(0.165, 0.15, [0, 0.07, 0], "shell"),
      cyl(0.17, 0.03, [0, -0.005, 0], "dark"),
      cyl(0.152, 0.014, [0, 0.151, 0], "dark"),
      ring(0.158, 0.005, [0, 0.14, 0], "glow"),
      cyl(0.052, 0.05, [0.02, 0.183, 0], "dark"),
      cyl(0.047, 0.024, [0.02, 0.218, 0], "lens"),
      cyl(0.052, 0.012, [0.02, 0.236, 0], "steel"),
      box([0.03, 0.045, 0.1], [0.152, 0.08, 0], "dark"),
      ball(0.016, [0.166, 0.085, -0.026], "lens"),
      ball(0.016, [0.166, 0.085, 0.026], "lens"),
      ...twin((s) => [cyl(0.03, 0.06, [0, 0, s * 0.15], "steel", "z")]),
    ]),
  );
  wheel(mk, "left.wheel", 0, [0, 0, -track], r, 0.045, idle ? (t) => -inch(t) / r : (t) => spin(t, 0));
  wheel(mk, "right.wheel", 0, [0, 0, track], r, 0.045, idle ? (t) => -inch(t) / r : (t) => spin(t, 1));
  // A trailing caster swings round behind its pivot, the way a wheel that cannot slide sideways must.
  const yoke = mk.add({
    name: "caster.yoke",
    parent: 0,
    at: pivot,
    joint: "y",
    passive: true,
    turn: idle ? () => 0 : swivel,
    parts: [cyl(0.018, 0.03, [0, 0.01, 0], "steel"), ...twin((s) => [box([0.05, 0.05, 0.006], [-0.016, -0.025, s * 0.016], "dark")])],
  });
  mk.add({
    name: "caster.wheel",
    parent: yoke,
    at: [-0.025, -0.05, 0],
    joint: "z",
    passive: true,
    turn: idle ? (t) => -inch(t) / 0.03 : (t) => spin(t, 2),
    parts: [tire(0.03, 0.022, 0.018, [0, 0, 0], "rubber"), cyl(0.019, 0.024, [0, 0, 0], "steel", "z")],
  });
  return {
    links: mk.links,
    height: r,
    speed: idle ? 0 : 0.5,
    base: idle ? (t) => ({ x: inch(t) }) : undefined,
    travel: idle ? undefined : drive(P, twist),
    cycle: P,
    size: 0.46,
  };
}

/**
 * A car: rear-wheel drive, front wheels on kingpins joined by a tie rod. The left knuckle is steered; the linkage
 * decides the right one, so the inner wheel turns further than the outer, as near to Ackermann as a trapezoid gets.
 */
function ackermann(idle: boolean): Body {
  const mk = new Maker();
  const r = 0.1;
  const L = 0.5;
  const kz = 0.14;
  const wz = 0.19;
  const arm = 0.08;
  const P = 9;
  const v = idle ? 0 : 0.6;
  // Steering arms point at the middle of the rear axle, the classic Ackermann trapezoid.
  const beta = Math.atan2(kz, L);
  const armAt = (s: number): [number, number] => [-arm * Math.cos(beta), -s * arm * Math.sin(beta)];
  const end = (s: number, d: number): [number, number] => {
    const [ax, az] = turnBy(d, ...armAt(s));
    return [L / 2 + ax, s * kz + az];
  };
  const rod = Math.hypot(end(1, 0)[0] - end(-1, 0)[0], end(1, 0)[1] - end(-1, 0)[1]);
  /** The right knuckle's angle that closes the linkage for a given left one. */
  const close = (dl: number) => {
    const e = end(-1, dl);
    const p: [number, number] = [L / 2, kz];
    const dx = p[0] - e[0];
    const dz = p[1] - e[1];
    const d = Math.hypot(dx, dz);
    const a = (d * d + rod * rod - arm * arm) / (2 * d);
    const h = Math.sqrt(Math.max(0, rod * rod - a * a));
    const c1: [number, number] = [e[0] + (a * dx - h * dz) / d, e[1] + (a * dz + h * dx) / d];
    const c2: [number, number] = [e[0] + (a * dx + h * dz) / d, e[1] + (a * dz - h * dx) / d];
    const pick = c1[0] < c2[0] ? c1 : c2;
    return heading(pick[0] - p[0], pick[1] - p[1]) - heading(...armAt(1));
  };
  const steer = (t: number) => (idle ? 0.36 * Math.sin(0.8 * t) : 0.3 * Math.sin((TAU * t) / P));
  // The left knuckle takes the angle that puts its wheel on the turning circle of the commanded steer.
  const left = (t: number) => {
    const tan = Math.tan(steer(t));
    return Math.atan2(L * tan, L - kz * tan);
  };
  let memo = NaN;
  let right = 0;
  const closed = (t: number) => {
    if (t !== memo) {
      memo = t;
      right = close(left(t));
    }
    return right;
  };
  // The body turns about a point on the rear axle line: yaw rate v·tan δ / L.
  const twist = (t: number) => {
    const w = (v * Math.tan(steer(t))) / L;
    return [v, (-w * L) / 2, w];
  };
  const spin = integral(P, (t) => {
    const tw = twist(t);
    const front = (s: number, d: number) => {
      // The wheel's centre sits out along its axle from the kingpin, and goes round with the knuckle.
      const [ox, oz] = turnBy(d, 0, s * (wz - kz));
      const [hx, hz] = pointVelocity(tw, L / 2 + ox, s * kz + oz);
      return -(hx * Math.cos(d) - hz * Math.sin(d)) / r;
    };
    return [-pointVelocity(tw, -L / 2, -wz)[0] / r, -pointVelocity(tw, -L / 2, wz)[0] / r, front(-1, left(t)), front(1, close(left(t)))];
  });
  // Steering alone rolls a front wheel too: its contact patch swings along the wheel's own heading, |wz − kz| from the
  // kingpin, by as much as the knuckle turns.
  const left0 = left(0);
  const right0 = close(left0);
  const scrub = (s: number, d: number, d0: number) => (s * (wz - kz) * (d - d0)) / r;
  mk.add(
    trunk([
      // The body rides above the wheels so that the front pair can steer under it.
      box([0.36, 0.04, 0.18], [-0.03, 0.06, 0], "dark"),
      box([0.7, 0.09, 0.3], [0, 0.155, 0], "shell"),
      box([0.62, 0.02, 0.26], [0, 0.105, 0], "dark"),
      box([0.34, 0.06, 0.25], [-0.06, 0.23, 0], "dark"),
      box([0.3, 0.012, 0.23], [-0.06, 0.264, 0], "shell"),
      box([0.012, 0.034, 0.2], [0.112, 0.23, 0], "lens"),
      box([0.02, 0.024, 0.2], [0.352, 0.16, 0], "glow"),
      ...twin((s) => [ball(0.016, [0.348, 0.135, s * 0.09], "lens"), box([0.02, 0.03, 0.07], [-0.352, 0.165, s * 0.095], "amber")]),
      ...twin((s) => [box([0.12, 0.012, 0.04], [L / 2, 0.035, s * 0.07], "steel"), box([0.12, 0.012, 0.04], [-L / 2, 0.035, s * 0.07], "steel")]),
      cyl(0.022, 2 * wz - 0.08, [-L / 2, 0, 0], "steel", "z"),
      cyl(0.04, 0.09, [-L / 2, 0, 0], "dark", "z"),
    ]),
  );
  wheel(mk, "rear.left.wheel", 0, [-L / 2, 0, -wz], r, 0.07, (t) => spin(t, 0));
  wheel(mk, "rear.right.wheel", 0, [-L / 2, 0, wz], r, 0.07, (t) => spin(t, 1));
  const knuckle = (s: 1 | -1, turn: (t: number) => number, passive: boolean) =>
    mk.add({
      name: `front.${s < 0 ? "left" : "right"}.knuckle`,
      parent: 0,
      at: [L / 2, 0, s * kz],
      joint: "y",
      passive,
      turn,
      parts: [
        cyl(0.02, 0.1, [0, 0, 0], "steel"),
        slab([0, -0.02, 0], [armAt(s)[0], -0.02, armAt(s)[1]], 0.016, 0.014, "amber", Math.PI / 2),
        cyl(0.014, 0.05, [0, 0, s * 0.025], "steel", "z"),
      ],
    });
  const kl = knuckle(-1, left, false);
  wheel(mk, "front.left.wheel", kl, [0, 0, -(wz - kz)], r, 0.07, (t) => spin(t, 2) - scrub(-1, left(t), left0), { passive: true });
  // The tie rod hangs from the left arm and points at the right one.
  mk.add({
    name: "tie.rod",
    parent: kl,
    at: [armAt(-1)[0], -0.02, armAt(-1)[1]],
    joint: "y",
    passive: true,
    turn: (t) => {
      const a = end(-1, left(t));
      const b = end(1, closed(t));
      return Math.atan2(b[0] - a[0], b[1] - a[1]) - left(t);
    },
    parts: [cyl(0.008, rod, [0, 0, rod / 2], "steel", "z"), ball(0.012, [0, 0, 0], "dark"), ball(0.012, [0, 0, rod], "dark")],
  });
  const kr = knuckle(1, closed, true);
  wheel(mk, "front.right.wheel", kr, [0, 0, wz - kz], r, 0.07, (t) => spin(t, 3) - scrub(1, closed(t), right0), { passive: true });
  return { links: mk.links, height: r, speed: v, travel: idle ? undefined : drive(P, twist), cycle: P, size: 0.62 };
}

/** Four mecanum wheels: it drives a square without ever turning, sliding sideways on two of its sides. */
function mecanum(idle: boolean): Body {
  const mk = new Maker();
  const r = 0.1;
  const X = 0.25;
  const Z = 0.2;
  const P = 12;
  const V = 0.32;
  const wheels = [
    [X, -1],
    [X, 1],
    [-X, -1],
    [-X, 1],
  ] as const;
  // Rollers touching the floor lie tangent to a circle round the centre, which gives the base its grip in rotation.
  const lean = ([x, s]: readonly [number, number]) => ((x > 0) === s > 0 ? -Math.PI / 4 : Math.PI / 4);
  const course = (t: number) => {
    const tau = (4 * mod(t, P)) / P;
    const i = Math.floor(tau);
    return (Math.PI / 2) * (i + ease((tau - i - 0.55) / 0.45));
  };
  const twist = (t: number) => [V * Math.cos(course(t)), V * Math.sin(course(t)), 0];
  // Idle, it turns a little on the spot: each hub moves along ŷ × p.
  const turning = (t: number) => 0.25 * Math.sin(0.7 * t);
  const spin = integral(P, (t) => wheels.map((w) => rollRate(pointVelocity(twist(t), w[0], w[1] * Z), [1, 0], lean(w), r)));
  mk.add(
    trunk([
      box([0.62, 0.1, 0.3], [0, 0.085, 0], "shell"),
      box([0.56, 0.05, 0.28], [0, 0.015, 0], "dark"),
      box([0.48, 0.03, 0.24], [0, 0.148, 0], "dark"),
      box([0.36, 0.008, 0.16], [0, 0.167, 0], "steel"),
      box([0.014, 0.03, 0.22], [0.312, 0.09, 0], "glow"),
      ...twin((s) => [ball(0.016, [0.31, 0.055, s * 0.08], "lens")]),
      ...[X, -X].flatMap((x) => twin((s) => [cyl(0.028, 0.06, [x, 0, s * 0.15], "steel", "z")])),
      // Rubber bumpers fore and aft between the wheels, tail lights, an emergency stop, and rails on the deck.
      ...[1, -1].map((e) => box([0.024, 0.03, 0.27], [e * 0.316, 0.02, 0], "rubber")),
      ...twin((s) => [box([0.008, 0.02, 0.05], [-0.312, 0.1, s * 0.1], "amber"), box([0.34, 0.006, 0.016], [0, 0.174, s * 0.06], "steel")]),
      cyl(0.026, 0.008, [-0.205, 0.167, 0.09], "dark"),
      cyl(0.019, 0.016, [-0.205, 0.178, 0.09], "amber"),
    ]),
  );
  for (const [i, w] of wheels.entries()) {
    const name = `${w[0] > 0 ? "front" : "rear"}.${w[1] < 0 ? "left" : "right"}.wheel`;
    const g = lean(w);
    wheel(
      mk,
      name,
      0,
      [w[0], 0, w[1] * Z],
      r,
      0.085,
      idle ? (t) => (-turning(t) * (w[1] * Z * Math.cos(g) - w[0] * Math.sin(g))) / (r * Math.cos(g)) : (t) => spin(t, i),
      { kind: "mecanum", lean: g },
    );
  }
  return {
    links: mk.links,
    height: r,
    speed: idle ? 0 : V,
    base: idle ? (t) => ({ yaw: turning(t) }) : undefined,
    travel: idle ? undefined : drive(P, twist),
    cycle: P,
    size: 0.62,
  };
}

/** Three omni wheels at 120°, rollers tangent to their rims: it drives a circle facing one way throughout. */
function omni(idle: boolean): Body {
  const mk = new Maker();
  const R = 0.23;
  const r = 0.075;
  const P = 10;
  const V = 0.3;
  const angles = [0, 1, 2].map((i) => (i / 3) * TAU);
  // Each wheel rolls along its own x, which the mount turns to (sin a, −cos a) in the base's frame.
  const dir = (a: number): [number, number] => [Math.sin(a), -Math.cos(a)];
  const twist = (t: number) => [V * Math.cos((TAU * t) / P), V * Math.sin((TAU * t) / P), 0];
  const spin = integral(P, (t) => angles.map((a) => rollRate(pointVelocity(twist(t), Math.cos(a) * R, Math.sin(a) * R), dir(a), 0, r)));
  // Idle, it shuffles round a small circle.
  const shuffle = (t: number): [number, number] => [0.04 * (Math.cos(0.8 * t) - 1), 0.04 * Math.sin(0.8 * t)];
  mk.add(
    trunk([
      cyl(0.2, 0.09, [0, 0.075, 0], "shell"),
      cyl(0.19, 0.016, [0, 0.025, 0], "dark"),
      cyl(0.15, 0.03, [0, 0.135, 0], "dark"),
      ring(0.2, 0.006, [0, 0.12, 0], "glow"),
      cyl(0.06, 0.03, [0, 0.165, 0], "steel"),
      ball(0.048, [0, 0.18, 0], "lens"),
      ...angles.map((a) => box([0.09, 0.07, 0.08], [Math.cos(a) * 0.15, 0.02, Math.sin(a) * 0.15], "dark", [0, -a, 0])),
      // A ranging window in the rim between each pair of wheels.
      ...angles.map((a) => box([0.012, 0.03, 0.07], [Math.cos(a + Math.PI / 3) * 0.197, 0.075, Math.sin(a + Math.PI / 3) * 0.197], "dark", [0, -(a + Math.PI / 3), 0])),
    ]),
  );
  for (const [i, a] of angles.entries()) {
    wheel(
      mk,
      `wheel${i + 1}`,
      0,
      [Math.cos(a) * R, 0, Math.sin(a) * R],
      r,
      0.05,
      idle
        ? (t) => {
            const [dx, dz] = shuffle(t);
            return -(dx * dir(a)[0] + dz * dir(a)[1]) / r;
          }
        : (t) => spin(t, i),
      { kind: "omni", set: [0, -a + Math.PI / 2, 0] },
    );
  }
  return {
    links: mk.links,
    height: r,
    speed: 0,
    base: idle ? (t) => ({ x: shuffle(t)[0], z: shuffle(t)[1] }) : undefined,
    travel: idle ? undefined : drive(P, twist),
    cycle: P,
    size: 0.5,
  };
}

/**
 * Four modules that each steer and drive. Each module points along its hub's velocity: all four alike while it
 * crabs, then square to the centre when it spins on the spot, the pose where every wheel axis meets at one point.
 */
function swerve(idle: boolean): Body {
  const mk = new Maker();
  const r = 0.075;
  const P = 14;
  const mods = [
    [1, -1],
    [1, 1],
    [-1, -1],
    [-1, 1],
  ] as const;
  const spinning = (u: number) => gate(u, 7.5, 13.5, 1.2);
  // Turn a quarter in each period: the base looks the same after it, being square.
  const rate = Math.PI / 2 / 4.8;
  const twist = (t: number) => {
    const u = mod(t, P);
    const w = spinning(u);
    const a = 0.9 * Math.sin((TAU * u) / 7) * gate(u, 0, 7.5, 1);
    return [0.45 * (1 - w) * Math.cos(a), 0.45 * (1 - w) * Math.sin(a), rate * w];
  };
  const drives = integral(P, (t) => mods.map(([x, z]) => -Math.hypot(...pointVelocity(twist(t), x * 0.2, z * 0.2)) / r), 1440);
  mk.add(
    trunk([
      box([0.5, 0.08, 0.5], [0, 0.17, 0], "shell"),
      box([0.36, 0.05, 0.36], [0, 0.115, 0], "dark"),
      box([0.44, 0.04, 0.44], [0, 0.225, 0], "dark"),
      box([0.3, 0.008, 0.3], [0, 0.248, 0], "steel"),
      box([0.508, 0.012, 0.508], [0, 0.16, 0], "glow"),
    ]),
  );
  for (const [i, [x, z]] of mods.entries()) {
    const n = `${x > 0 ? "front" : "rear"}.${z < 0 ? "left" : "right"}`;
    const lock = heading(x, z);
    const m = mk.add({
      name: `${n}.steer`,
      parent: 0,
      at: [x * 0.2, 0.12, z * 0.2],
      joint: "y",
      turn: idle ? (t) => lock * (0.5 - 0.5 * Math.cos(0.5 * t)) : (t) => heading(...pointVelocity(twist(t), x * 0.2, z * 0.2)),
      parts: [
        cyl(0.06, 0.05, [0, 0, 0], "dark"),
        ring(0.06, 0.005, [0, -0.022, 0], "glow"),
        box([0.07, 0.02, 0.08], [0, -0.03, 0], "dark"),
        ...twin((s) => [box([0.05, 0.11, 0.01], [0, -0.085, s * 0.032], "steel")]),
      ],
    });
    wheel(mk, `${n}.drive`, m, [0, -0.12, 0], r, 0.045, idle ? () => 0 : (t) => drives(t, i));
  }
  return { links: mk.links, height: r, speed: idle ? 0 : 0.45, travel: idle ? undefined : drive(P, twist), cycle: P, size: 0.56 };
}

/** Two tracks round a sprocket and an idler, with flippers front and rear. It drives, then pivots on the spot. */
function tracked(idle: boolean): Body {
  const mk = new Maker();
  const P = 12;
  const loop = 0.1;
  // Nineteen sprocket teeth and forty-eight shoes on one pitch, so that the teeth mesh with the track all the way
  // round. That fixes the distance between the sprocket and the idler.
  const teeth = 19;
  const pitch = (TAU * loop) / teeth;
  const half = ((48 / teeth - 1) * Math.PI * loop) / 2;
  const pivoting = (u: number) => gate(u, 7.5, 11.5, 1);
  const rate = Math.PI / 2 / 3;
  const twist = (t: number) => {
    const w = idle ? 0 : pivoting(mod(t, P));
    return [idle ? 0 : 0.4 * (1 - w), 0, rate * w];
  };
  // Each belt runs at the speed of its side; skid steering is the difference.
  const run = integral(P, (t) => [pointVelocity(twist(t), 0, -0.2)[0], pointVelocity(twist(t), 0, 0.2)[0]]);
  mk.add(
    trunk([
      box([0.5, 0.12, 0.27], [0, 0.11, 0], "shell"),
      box([0.4, 0.05, 0.22], [0, 0.19, 0], "dark"),
      box([0.02, 0.04, 0.16], [0.255, 0.11, 0], "glow"),
      cyl(0.016, 0.12, [-0.08, 0.27, 0], "steel"),
      box([0.08, 0.06, 0.12], [-0.06, 0.34, 0], "dark"),
      ...twin((s) => [ball(0.014, [-0.018, 0.345, s * 0.03], "lens")]),
      box([0.004, 0.008, 0.08], [-0.018, 0.322, 0], "glow"),
      ...twin((s) => [
        box([0.48, 0.06, 0.03], [0, 0, s * 0.2], "dark"),
        // Road wheels ride the inside of the bottom run in pairs, either side of the shoes' guide horns.
        ...[-0.12, 0, 0.12].flatMap((x) => [
          cyl(0.04, 0.024, [x, -0.055, s * 0.2 - 0.034], "dark", "z"),
          cyl(0.04, 0.024, [x, -0.055, s * 0.2 + 0.034], "dark", "z"),
          cyl(0.012, 0.09, [x, -0.055, s * 0.2], "steel", "z"),
        ]),
      ]),
    ]),
  );
  for (const side of [-1, 1] as const) {
    const n = side < 0 ? "left" : "right";
    const s = (t: number) => (idle ? 0 : run(t, side < 0 ? 0 : 1));
    // Round the ends the shoes turn about the axle at the belt's speed over its radius, and so do the wheels.
    const wrap = (t: number) => -s(t) / loop;
    const sprocket: Part[] = [cyl(0.085, 0.03, [0, 0, -0.03], "dark", "z"), cyl(0.085, 0.03, [0, 0, 0.03], "dark", "z"), cyl(0.03, 0.11, [0, 0, 0], "steel", "z")];
    // Each tooth sits in the gap between two shoes, where the belt brings it.
    for (let i = 0; i < teeth; i++) {
      const a = Math.PI / 2 + ((i + 0.5) / teeth) * TAU;
      sprocket.push(box([0.014, 0.012, 0.09], [Math.cos(a) * 0.087, Math.sin(a) * 0.087, 0], "steel", [0, 0, a]));
    }
    sprocket.push(cyl(0.016, 0.114, [0, 0, 0], "glow", "z"));
    mk.add({ name: `${n}.sprocket`, parent: 0, at: [-half, 0, side * 0.2], joint: "z", turn: wrap, parts: sprocket });
    mk.add({
      name: `${n}.idler`,
      parent: 0,
      at: [half, 0, side * 0.2],
      joint: "z",
      passive: true,
      turn: wrap,
      parts: [
        cyl(0.094, 0.026, [0, 0, -0.036], "dark", "z"),
        cyl(0.094, 0.026, [0, 0, 0.036], "dark", "z"),
        cyl(0.06, 0.09, [0, 0, 0], "steel", "z"),
        ...[0, 1, 2, 3, 4].map((i) => box([0.07, 0.014, 0.094], [Math.cos((i / 5) * TAU) * 0.035, Math.sin((i / 5) * TAU) * 0.035, 0], "dark", [0, 0, (i / 5) * TAU])),
        cyl(0.014, 0.096, [0, 0, 0], "glow", "z"),
      ],
    });
    mk.add({
      name: `${n}.track`,
      parent: 0,
      at: [-half, 0, side * 0.2],
      joint: "fixed",
      passive: true,
      belt: { len: 2 * half, r: loop, pitch },
      turn: s,
      // A shoe: its plate, the grouser that bites the floor, and the guide horn that runs between the wheels.
      parts: [box([0.03, 0.01, 0.1], [0, 0, 0], "rubber"), box([0.01, 0.012, 0.1], [0, 0.011, 0], "rubber"), box([0.014, 0.01, 0.04], [0, -0.009, 0], "steel")],
    });
    for (const [f, x] of [
      ["front", half],
      ["rear", -half],
    ] as const) {
      const dir = x > 0 ? 1 : -1;
      // A flipper's belt: two pulleys and the straight runs that join them on their outside tangents.
      const [r1, r2, len] = [0.085, 0.045, 0.27];
      const tilt = Math.asin((r1 - r2) / len);
      const runs = twin((s2) => {
        const n2: [number, number] = [dir * Math.sin(tilt), s2 * Math.cos(tilt)];
        return [slab([r1 * n2[0], r1 * n2[1], 0], [dir * len + r2 * n2[0], r2 * n2[1], 0], 0.012, 0.052, "rubber")];
      });
      mk.add({
        name: `${n}.${f}.flipper`,
        parent: 0,
        at: [x, 0, side * 0.29],
        joint: "z",
        turn: (t) => dir * (0.35 + 0.32 * Math.sin(t * 0.8 + (x > 0 ? 0 : 1.6))),
        parts: [
          slab([0, 0, -side * 0.012], [dir * len, 0, -side * 0.012], 0.06, 0.016, "dark"),
          cyl(r1 - 0.006, 0.05, [0, 0, 0], "rubber", "z"),
          cyl(r2 - 0.006, 0.05, [dir * len, 0, 0], "rubber", "z"),
          ...runs,
          cyl(0.03, 0.056, [0, 0, 0], "steel", "z"),
          cyl(0.018, 0.06, [0, 0, 0], "glow", "z"),
        ],
      });
    }
  }
  return { links: mk.links, height: loop + 0.017, speed: idle ? 0 : 0.4, travel: idle ? undefined : drive(P, twist), cycle: P, size: 0.72 };
}

/**
 * A quadruped on driven wheels. It rolls, stops, then trots: two contact laws on one foot. A locked wheel turns
 * against its leg, so that whichever mode it is in, the wheel never slides on the floor.
 */
function wheelLeg(idle: boolean): Body {
  const mk = new Maker();
  const q = { femur: 0.24, tibia: 0.24, length: 0.56, width: 0.22, wheel: 0.085 };
  const P = 12;
  const g: Step = idle ? still(TROT) : { period: 0.6, duty: 0.58, stride: 0.2, lift: 0.08 };
  const vWalk = g.stride / (g.duty * g.period);
  const rolling = (t: number) => (idle ? 0 : 0.55 * gate(mod(t, P), 0, 5.4, 1));
  const walking = (t: number) => (idle ? 1 : gate(mod(t, P), 6, 11.6, 0.8));
  const sum = integral(P, (t) => [rolling(t), rolling(t) + walking(t) * vWalk]);
  // Where each foot is from its hip, the trot faded in and out.
  const step = (t: number, phase: number) => {
    const [fx, up] = foot(t / g.period + phase, g);
    return [walking(t) * fx, walking(t) * up] as const;
  };
  const phases = [0, 0.5, 0.5, 0];
  // A wheel on the floor rolls by exactly as far as its foot goes over the floor. Lifted clear, it holds still;
  // between one and four millimetres up it eases from one to the other, so a foot that barely leaves the floor
  // never skids.
  const rolls = integral(
    P,
    (t) =>
      phases.map((phase) => {
        const grip = 1 - ease((step(t, phase)[1] - 0.001) / 0.003);
        if (grip <= 0) return 0;
        const h = 1e-4;
        return grip * (rolling(t) + walking(t) * vWalk + (step(t + h, phase)[0] - step(t - h, phase)[0]) / (2 * h));
      }),
    2880,
  );
  const hang = (q.femur + q.tibia) * 0.82;
  const L = q.length;
  const W = q.width;
  const top = 0.08;
  const bob = (t: number) => walking(t) * g.lift * 0.12 * Math.cos(2 * TAU * (t / g.period - g.duty / 2));
  mk.add(
    trunk([
      box([L * 0.9, 0.12, W * 1.16], [0, 0.02, 0], "shell"),
      box([L * 0.78, 0.05, W * 0.92], [0, -0.04, 0], "dark"),
      box([L * 0.64, 0.024, W * 0.78], [0, top + 0.008, 0], "dark"),
      box([L * 0.26, 0.008, W * 0.22], [-L * 0.08, top + 0.022, 0], "glow"),
      ...twin((e) => [box([0.12, 0.1, 0.17], [e * (L / 2 + 0.04), 0.03, 0], "dark"), box([0.02, 0.064, 0.176], [e * (L / 2 + 0.098), 0.03, 0], "shell")]),
      box([0.012, 0.022, 0.11], [L / 2 + 0.112, 0.062, 0], "glow"),
      ball(0.019, [L / 2 + 0.104, 0.022, -0.046], "lens"),
      ball(0.019, [L / 2 + 0.104, 0.022, 0.046], "lens"),
    ]),
  );
  for (let i = 0; i < 2; i++) {
    const x = L * (0.43 - 0.86 * i);
    for (const side of [-1, 1] as const) {
      const phase = ((i + (side === 1 ? 1 : 0)) % 2) * 0.5;
      leg(mk, {
        name: `${i ? "hind" : "fore"}.${side < 0 ? "left" : "right"}`,
        parent: 0,
        at: [x, 0, side * W * 0.5],
        l1: q.femur,
        l2: q.tibia,
        target: (t) => {
          const [fx, up] = step(t, phase);
          return [fx, -hang - bob(t) + up];
        },
        bend: (x < 0 ? 1 : -1) as 1 | -1,
        side,
        wheel: q.wheel,
        rolled: (t) => rolls(t, i * 2 + (side < 0 ? 0 : 1)),
      });
    }
  }
  return {
    links: mk.links,
    height: hang + q.wheel,
    speed: idle ? 0 : (sum(P, 1) - sum(0, 1)) / P,
    base: (t) => ({ y: bob(t) }),
    travel: idle ? undefined : (t) => ({ x: sum(t, 1), z: 0, yaw: 0 }),
    cycle: idle ? g.period : P,
    size: 0.62 + L * 0.5,
  };
}

/** Two clusters of three wheels. On the flat each cluster stands on two wheels and they roll. */
function triwheel(idle: boolean): Body {
  const mk = new Maker();
  const R = 0.17;
  const r = 0.095;
  const v = idle ? 0 : 0.4;
  const stand = -TAU / 12;
  mk.add(
    trunk([
      // The cargo body on its chassis, a hatch set into the flat of its top, and a rubber bumper across the front.
      box([0.34, 0.4, 0.3], [0, 0.36, 0], "shell"),
      box([0.36, 0.05, 0.32], [0, 0.14, 0], "dark"),
      box([0.2, 0.008, 0.16], [0, 0.56, 0], "dark"),
      box([0.188, 0.012, 0.148], [0, 0.566, 0], "shell"),
      box([0.05, 0.004, 0.018], [-0.045, 0.5725, 0], "dark"),
      box([0.024, 0.2, 0.2], [0.172, 0.39, 0], "dark"),
      box([0.008, 0.03, 0.16], [0.186, 0.45, 0], "glow"),
      ...twin((s) => [ball(0.016, [0.184, 0.35, s * 0.05], "lens")]),
      box([0.03, 0.045, 0.3], [0.178, 0.205, 0], "rubber"),
      // A telescopic handle: sleeves down the back, steel tubes out of them, a rubber grip across.
      ...twin((s) => [
        cyl(0.022, 0.26, [-0.195, 0.48, s * 0.12], "dark"),
        cyl(0.016, 0.3, [-0.195, 0.75, s * 0.12], "steel"),
        box([0.04, 0.03, 0.04], [-0.195, 0.6, s * 0.12], "dark"),
        cyl(0.03, 0.06, [0, 0, s * 0.18], "steel", "z"),
        box([0.07, 0.17, 0.04], [0, 0.07, s * 0.165], "dark"),
        // Louvres on each flank over the cluster drive.
        ...[0, 1, 2].map((i) => box([0.16, 0.008, 0.004], [-0.02, 0.24 + i * 0.03, s * 0.151], "dark")),
      ]),
      cap(0.022, 0.24, [-0.195, 0.9, 0], "rubber", "z"),
      ...twin((s) => [cyl(0.026, 0.03, [-0.195, 0.9, s * 0.135], "dark", "z")]),
    ]),
  );
  for (const side of [-1, 1] as const) {
    const n = side < 0 ? "left" : "right";
    const c = mk.add({
      name: `${n}.cluster`,
      parent: 0,
      at: [0, 0, side * 0.22],
      joint: "z",
      turn: () => stand,
      parts: [
        cyl(0.05, 0.05, [0, 0, 0], "dark", "z"),
        ring(0.05, 0.005, [0, 0, side * 0.026], "glow", "z"),
        ...[0, 1, 2].map((i) => slab([0, 0, 0], [Math.cos((i / 3) * TAU) * R, Math.sin((i / 3) * TAU) * R, 0], 0.045, 0.03, "steel", Math.PI / 2)),
      ],
    });
    // The wheels are geared together: each turns by the distance travelled, less the cluster's own angle.
    for (let i = 0; i < 3; i++) {
      const a = (i / 3) * TAU;
      wheel(mk, `${n}.wheel${i + 1}`, c, [Math.cos(a) * R, Math.sin(a) * R, side * 0.04], r, 0.05, (t) => -(idle ? inch(t) : v * t) / r - stand, { passive: i > 0 });
    }
  }
  return {
    links: mk.links,
    height: r + R / 2,
    speed: v,
    base: idle ? (t) => ({ x: inch(t) }) : undefined,
    travel: idle ? undefined : (t) => ({ x: v * t, z: 0, yaw: 0 }),
    cycle: 2,
    size: 0.82,
  };
}

/** Six wheels on a passive rocker and bogie each side, joined across the deck by a differential bar. */
function rocker(idle: boolean): Body {
  const mk = new Maker();
  const r = 0.1;
  const v = idle ? 0 : 0.35;
  const spin = (t: number) => -(idle ? inch(t) : v * t) / r;
  mk.add(
    trunk([
      box([0.46, 0.14, 0.32], [0, 0.24, 0], "shell"),
      box([0.4, 0.05, 0.28], [0, 0.15, 0], "dark"),
      box([0.5, 0.02, 0.46], [-0.04, 0.325, 0], "dark"),
      box([0.44, 0.008, 0.4], [-0.04, 0.338, 0], "steel"),
      cyl(0.016, 0.26, [0.18, 0.46, 0], "steel"),
      box([0.09, 0.07, 0.15], [0.2, 0.61, 0], "shell"),
      ...twin((s) => [ball(0.016, [0.246, 0.615, s * 0.04], "lens")]),
      box([0.008, 0.014, 0.1], [0.247, 0.588, 0], "glow"),
      cyl(0.008, 0.1, [-0.2, 0.39, 0.13], "steel"),
      cyl(0.06, 0.012, [-0.2, 0.445, 0.13], "shell"),
      cyl(0.012, 0.02, [-0.2, 0.46, 0.13], "dark"),
      box([0.012, 0.03, 0.2], [0.232, 0.24, 0], "glow"),
    ]),
  );
  mk.add({
    name: "differential",
    parent: 0,
    at: [-0.05, 0.36, 0],
    joint: "y",
    passive: true,
    turn: () => 0,
    parts: [cyl(0.022, 0.03, [0, 0, 0], "dark"), box([0.03, 0.016, 0.44], [0, 0.012, 0], "steel")],
  });
  for (const side of [-1, 1] as const) {
    const n = side < 0 ? "left" : "right";
    const rk = mk.add({
      name: `${n}.rocker`,
      parent: 0,
      at: [0, 0.2, side * 0.22],
      joint: "z",
      passive: true,
      turn: () => 0,
      parts: [
        cyl(0.04, 0.05, [0, 0, 0], "steel", "z"),
        ring(0.04, 0.004, [0, 0, side * 0.026], "glow", "z"),
        slab([0, 0, 0], [0.32, -0.2, 0], 0.036, 0.036, "dark"),
        slab([0, 0, 0], [-0.2, -0.12, 0], 0.036, 0.036, "dark"),
        cyl(0.02, 0.07, [0.32, -0.2, side * 0.03], "steel", "z"),
      ],
    });
    wheel(mk, `${n}.front`, rk, [0.32, -0.2, side * 0.07], r, 0.08, spin, { kind: "cleat" });
    const bg = mk.add({
      name: `${n}.bogie`,
      parent: rk,
      at: [-0.2, -0.12, 0],
      joint: "z",
      passive: true,
      turn: () => 0,
      parts: [cyl(0.03, 0.05, [0, 0, 0], "steel", "z"), slab([0, 0, 0], [0.15, -0.08, 0], 0.03, 0.03, "dark"), slab([0, 0, 0], [-0.17, -0.08, 0], 0.03, 0.03, "dark")],
    });
    wheel(mk, `${n}.middle`, bg, [0.15, -0.08, side * 0.07], r, 0.08, spin, { kind: "cleat" });
    wheel(mk, `${n}.rear`, bg, [-0.17, -0.08, side * 0.07], r, 0.08, spin, { kind: "cleat" });
  }
  return {
    links: mk.links,
    height: r,
    speed: v,
    base: idle ? (t) => ({ x: inch(t) }) : undefined,
    travel: idle ? undefined : (t) => ({ x: v * t, z: 0, yaw: 0 }),
    cycle: 2,
    size: 0.76,
  };
}

/** A seven-joint arm with the KUKA iiwa's alternation of roll and pitch, on a two-wheel base with a ball caster. */
function manipulator(idle: boolean): Body {
  const mk = new Maker();
  const r = 0.1;
  const v = idle ? 0 : 0.25;
  mk.add(
    trunk([
      box([0.58, 0.16, 0.38], [0, 0.1, 0], "shell"),
      box([0.6, 0.03, 0.36], [0, 0.02, 0], "dark"),
      box([0.52, 0.02, 0.32], [0, 0.19, 0], "dark"),
      box([0.014, 0.04, 0.28], [0.296, 0.1, 0], "glow"),
      ...twin((s) => [ball(0.016, [0.292, 0.145, s * 0.1], "lens")]),
      cyl(0.078, 0.022, [0.14, 0.205, 0.1], "steel"),
      ring(0.078, 0.004, [0.14, 0.205, 0.1], "glow"),
      cyl(0.045, 0.03, [-0.23, -0.035, 0], "dark"),
    ]),
  );
  wheel(mk, "left.wheel", 0, [0, 0, -0.22], r, 0.05, (t) => (-v * t) / r);
  wheel(mk, "right.wheel", 0, [0, 0, 0.22], r, 0.05, (t) => (-v * t) / r);
  mk.add({ name: "caster", parent: 0, at: [-0.23, -0.06, 0], joint: "y", passive: true, turn: () => 0, parts: [ball(0.04, [0, 0, 0], "steel")] });
  const l = [0.14, 0.2, 0.16, 0.18, 0.13, 0.1, 0.07];
  const axes = ["y", "z", "y", "z", "y", "z", "y"] as const;
  const rest = [0.4, -0.5, 0.2, -1.1, 0.1, -0.9, 0];
  let p = 0;
  let at: V3 = [0.14, 0.22, 0.1];
  for (let i = 0; i < 7; i++) {
    const w = 0.062 - i * 0.005;
    const len = l[i]!;
    p = mk.add({
      name: `arm${i + 1}`,
      parent: p,
      at,
      joint: axes[i]!,
      turn: (t) => rest[i]! + 0.22 * Math.sin(t * 0.7 + i * 1.3),
      parts: [
        ...(axes[i] === "y"
          ? [cyl(w * 1.04, 0.045, [0, 0.012, 0], "dark"), ring(w * 1.04, 0.004, [0, 0.036, 0], "glow")]
          : [cyl(w * 0.98, w * 2.2, [0, 0, 0], "dark", "z"), ring(w * 0.8, 0.004, [0, 0, w * 1.1], "glow", "z"), ring(w * 0.8, 0.004, [0, 0, -w * 1.1], "glow", "z")]),
        cap(w * 0.78, Math.max(0.01, len - w * 1.2), [0, len / 2 + 0.02, 0], "shell"),
      ],
    });
    at = [0, len + 0.03, 0];
  }
  mk.add({
    name: "gripper",
    parent: p,
    at,
    joint: "fixed",
    parts: [
      cyl(0.03, 0.02, [0, 0, 0], "steel"),
      box([0.05, 0.03, 0.11], [0, 0.022, 0], "dark"),
      ...twin((s) => [box([0.026, 0.08, 0.014], [0, 0.07, s * 0.042], "steel"), box([0.022, 0.024, 0.006], [0, 0.098, s * 0.034], "rubber")]),
    ],
  });
  return { links: mk.links, height: r, speed: v, travel: idle ? undefined : (t) => ({ x: v * t, z: 0, yaw: 0 }), cycle: (TAU / 0.7) * 1, size: 0.9 };
}

/**
 * A chain of links on passive wheels, its nose steered along a serpentine. Wheels cannot slide sideways, so each link
 * trails the joint in front of it like a trailer behind a hitch, and the wave passes down the body.
 */
function snake(idle: boolean): Body {
  const mk = new Maker();
  const n = 9;
  const len = 0.19;
  const A = 0.55;
  const wave = 1.3;
  const v = idle ? 0 : 0.32;
  const r = 0.04;
  const track = 0.068;
  /** How far along its path the nose has gone. */
  const crawl = (t: number) => (idle ? 0.31 + 0.06 * Math.sin(0.7 * t) : v * t);
  // The nose's path: heading ψ(s) = A sin(2πs / λ), tabled over one wavelength.
  const psi = (s: number) => A * Math.sin((TAU * s) / wave);
  const N = 512;
  const px = new Float64Array(N + 1);
  const pz = new Float64Array(N + 1);
  const ds = wave / N;
  for (let k = 0; k < N; k++) {
    px[k + 1] = px[k]! + Math.cos(psi((k + 0.5) * ds)) * ds;
    pz[k + 1] = pz[k]! - Math.sin(psi((k + 0.5) * ds)) * ds;
  }
  const ahead = px[N]! / wave;
  let zMean = 0;
  for (let k = 0; k < N; k++) zMean += pz[k]! / N;
  const point = (s: number): [number, number] => {
    const c = Math.floor(s / wave);
    const u = (s - c * wave) / ds;
    const k = Math.min(N - 1, Math.floor(u));
    const f = u - k;
    return [c * px[N]! + px[k]! + (px[k + 1]! - px[k]!) * f, pz[k]! + (pz[k + 1]! - pz[k]!) * f];
  };
  /*
   * Link i's wheels sit d = len / 2 behind the joint at its front, h, and cannot slide sideways, so along the nose's
   * path its heading θ turns at θ′ = −(h′ · n) / d, where n points to its right; the joint behind it then moves at
   * h′ + len · n · θ′, and its wheels roll at h′ · u, u its heading. Nothing here depends on time, only on how far the
   * nose has gone, so once run to steady state every heading and roll is a table over one wavelength, exact whether the
   * nose goes forwards, backwards or back and forth.
   */
  const d = len / 2;
  const M = 1024;
  const dim = 2 * n;
  const rates = (s: number, y: Float64Array, out: Float64Array) => {
    let hx = Math.cos(psi(s));
    let hz = -Math.sin(psi(s));
    for (let i = 0; i < n; i++) {
      const c = Math.cos(y[i]!);
      const sn = Math.sin(y[i]!);
      const turn = -(hx * sn + hz * c) / d;
      out[i] = turn;
      out[n + i] = hx * c - hz * sn;
      hx += len * sn * turn;
      hz += len * c * turn;
    }
  };
  const table = new Float64Array((M + 1) * dim);
  {
    const h = wave / M;
    const y = new Float64Array(dim);
    const [k1, k2, k3, k4, y2] = [0, 0, 0, 0, 0].map(() => new Float64Array(dim)) as Float64Array[];
    const laps = 6;
    for (let lap = 0; lap < laps; lap++) {
      // Each link's roll is counted from the start of the lap that is kept.
      if (lap === laps - 1) y.fill(0, n);
      for (let k = 0; k < M; k++) {
        const s = k * h;
        if (lap === laps - 1) table.set(y, k * dim);
        rates(s, y, k1!);
        for (let j = 0; j < dim; j++) y2![j] = y[j]! + (h / 2) * k1![j]!;
        rates(s + h / 2, y2!, k2!);
        for (let j = 0; j < dim; j++) y2![j] = y[j]! + (h / 2) * k2![j]!;
        rates(s + h / 2, y2!, k3!);
        for (let j = 0; j < dim; j++) y2![j] = y[j]! + h * k3![j]!;
        rates(s + h, y2!, k4!);
        for (let j = 0; j < dim; j++) y[j] = y[j]! + (h / 6) * (k1![j]! + 2 * k2![j]! + 2 * k3![j]! + k4![j]!);
      }
    }
    table.set(y, M * dim);
  }
  /** Heading of link j (j < n), or how far link j − n has rolled, once the nose has gone s along its path. */
  const read = (s: number, j: number) => {
    const c = Math.floor(s / wave);
    const u = ((s - c * wave) / wave) * M;
    const k = Math.min(M - 1, Math.floor(u));
    const a = table[k * dim + j]!;
    const value = a + (table[(k + 1) * dim + j]! - a) * (u - k);
    return j < n ? value : value + c * table[M * dim + j]!;
  };
  let memo = NaN;
  let s = 0;
  let nose: [number, number] = [0, 0];
  const yaws = new Float64Array(n);
  const chain = (t: number) => {
    if (t === memo) return;
    memo = t;
    s = crawl(t);
    nose = point(s);
    for (let i = 0; i < n; i++) yaws[i] = read(s, i);
  };
  let parent = -1;
  for (let i = 0; i < n; i++) {
    const at: V3 = i ? [-len, 0, 0] : [0, 0, 0];
    const tone: Mat = i % 2 ? "dark" : "shell";
    parent = mk.add({
      name: i ? `link${i}` : "head",
      parent,
      at,
      joint: i ? "y" : "free",
      turn: (t) => {
        chain(t);
        return i ? yaws[i]! - yaws[i - 1]! : 0;
      },
      parts: [
        cap(0.046, len - 0.104, [-len / 2, 0.012, 0], i ? tone : "shell", "x"),
        ring(0.046, 0.0035, [-len / 2, 0.012, 0], "glow", "x"),
        cyl(0.03, 0.1, [0, 0.012, 0], "steel"),
        cyl(0.034, 0.012, [0, 0.058, 0], "dark"),
        ...(i
          ? []
          : [
              cap(0.046, 0.05, [0.03, 0.012, 0], "shell", "x"),
              box([0.03, 0.036, 0.074], [0.088, 0.018, 0], "dark"),
              box([0.006, 0.008, 0.05], [0.104, 0.002, 0], "glow"),
              ...twin((s) => [ball(0.011, [0.1, 0.026, s * 0.02], "lens")]),
            ]),
      ],
    });
    // The wheel on the outside of a turn goes further than the one inside: its offset times the link's heading.
    for (const side of [-1, 1] as const)
      mk.add({
        name: `${i ? `link${i}` : "head"}.wheel.${side < 0 ? "left" : "right"}`,
        parent,
        at: [-len / 2, 0, side * track],
        joint: "z",
        passive: true,
        turn: (t) => {
          chain(t);
          return -(read(s, n + i) + side * track * yaws[i]!) / r;
        },
        parts: [tire(r, 0.02, r * 0.62, [0, 0, 0], "rubber"), cyl(r * 0.64, 0.016, [0, 0, 0], "steel", "z"), cyl(r * 0.2, 0.024, [0, 0, 0], "dark", "z")],
      });
  }
  const lag = (n * len * ahead) / 2;
  return {
    links: mk.links,
    height: r,
    speed: v * ahead,
    base: (t) => {
      chain(t);
      return { x: nose[0] - ahead * s + lag, z: nose[1] - zMean, yaw: yaws[0]! };
    },
    travel: idle ? undefined : (t) => ({ x: ahead * v * t, z: 0, yaw: 0 }),
    cycle: idle ? TAU / 0.7 : wave / v,
    size: 0.94,
  };
}

/** A two-wheeled inverted pendulum. To speed up it must lean ahead of its wheels, and further than a / g. */
function balancer(idle: boolean): Body {
  const mk = new Maker();
  const r = 0.14;
  const [A, w] = idle ? [0.06, 1.5] : [0.36, 1.4];
  const x = (t: number) => A * Math.sin(w * t);
  /*
   * The lean that carries it, from angular momentum about the wheels' contact with the floor. The body (m, its centre
   * l above the axle, Ic about that centre) pivots at the axle, r up, and the motors that push the wheels push back on
   * it, so to accelerate at a it must lean further than a / g:
   *   (Ic + m l² + m r l) φ'' = m g l φ − a (m (r + l) + M r + J / r)
   * For a = −A ω² sin ωt the one motion that stays balanced is φ = a K / (m g l + ω² (Ic + m l² + m r l)), with K the
   * bracket: a forward lean in step with the push, near twice a / g for a 10 kg body on 1.2 kg hub-motor wheels.
   */
  const [m, l, Ic, M, J] = [10, 0.2, 0.25, 2.4, 0.03];
  const gain = (m * (r + l) + M * r + J / r) / (m * 9.81 * l + w * w * (Ic + m * l * l + m * r * l));
  const pitch = (t: number) => -gain * (-A * w * w * Math.sin(w * t));
  // A fender's arc over a wheel, clear of the tyre.
  const fender = (s: 1 | -1): Part[] =>
    Array.from({ length: 6 }, (_, i) => {
      const [a0, a1] = [0.42 + (i / 6) * 2.3, 0.42 + ((i + 1) / 6) * 2.3];
      return slab([0.172 * Math.cos(a0), 0.172 * Math.sin(a0), s * 0.17], [0.172 * Math.cos(a1), 0.172 * Math.sin(a1), s * 0.17], 0.012, 0.066, "dark");
    });
  mk.add(
    trunk([
      // The drive pod between the wheels: battery and both motors, low.
      box([0.2, 0.13, 0.25], [0, 0, 0], "dark"),
      box([0.22, 0.03, 0.262], [0, 0.075, 0], "shell"),
      box([0.006, 0.014, 0.16], [0.101, 0.024, 0], "glow"),
      // The torso, its face to the front, a grab handle behind.
      box([0.13, 0.3, 0.21], [0, 0.24, 0], "shell"),
      box([0.012, 0.13, 0.15], [0.066, 0.27, 0], "dark"),
      box([0.004, 0.008, 0.1], [0.073, 0.315, 0], "glow"),
      strut([-0.097, 0.35, -0.06], [-0.097, 0.35, 0.06], 0.009, "dark"),
      // The neck, the head with its visor and eyes, and a lidar on top.
      cyl(0.028, 0.06, [0, 0.42, 0], "steel"),
      box([0.15, 0.1, 0.24], [0.01, 0.5, 0], "shell"),
      box([0.012, 0.06, 0.2], [0.086, 0.5, 0], "dark"),
      box([0.004, 0.008, 0.14], [0.092, 0.465, 0], "glow"),
      cyl(0.04, 0.034, [0, 0.567, 0], "dark"),
      ring(0.04, 0.004, [0, 0.572, 0], "glow"),
      ...twin((s) => [
        cyl(0.055, 0.024, [0, 0, s * 0.135], "steel", "z"),
        ball(0.016, [0.09, 0.505, s * 0.045], "lens"),
        strut([-0.065, 0.35, s * 0.06], [-0.097, 0.35, s * 0.06], 0.009, "dark"),
        ...fender(s),
        slab([0, 0.185, s * 0.105], [0, 0.185, s * 0.15], 0.05, 0.012, "dark"),
      ]),
    ]),
  );
  // The wheel's angle in the world is −x / r; the joint measures it from the leaning body.
  for (const side of [-1, 1] as const)
    wheel(mk, `${side < 0 ? "left" : "right"}.wheel`, 0, [0, 0, side * 0.17], r, 0.05, (t) => -x(t) / r - pitch(t));
  return {
    links: mk.links,
    height: r,
    speed: 0,
    base: (t) => ({ pitch: pitch(t), x: idle ? x(t) : 0 }),
    travel: idle ? undefined : (t) => ({ x: x(t), z: 0, yaw: 0 }),
    cycle: TAU / w,
    size: 0.74,
  };
}

const WIDE: Step = { period: 1.0, duty: 0.62, stride: 0.16, lift: 0.07 };
/** Every archetype. Pass `true` for the body waiting on the spot. */
export const makers: Record<string, (idle?: boolean) => Body> = {
  "quad-mammal": (idle = false) => walker({ legs: 4, femur: 0.25, tibia: 0.25, length: 0.56, width: 0.2, thick: 1, gait: TROT, bend: 1 }, idle),
  "quad-sprawl": (idle = false) => {
    const mk = new Maker();
    const g = idle ? still(WIDE) : WIDE;
    mk.add(
      trunk([
        box([0.52, 0.09, 0.24], [0, 0.01, 0], "shell"),
        box([0.4, 0.04, 0.2], [0, -0.045, 0], "dark"),
        box([0.36, 0.04, 0.17], [0, 0.07, 0], "dark"),
        ...twin((e) => [box([0.1, 0.07, 0.13], [e * 0.3, 0.01, 0], "dark")]),
        box([0.2, 0.01, 0.08], [0, 0.095, 0], "glow"),
        box([0.008, 0.018, 0.09], [0.352, 0.025, 0], "glow"),
        ...twin((s) => [ball(0.014, [0.35, 0.0, s * 0.035], "lens")]),
      ]),
    );
    for (const x of [1, -1])
      for (const z of [-1, 1])
        sprawl(mk, { name: `${x > 0 ? "fore" : "hind"}.${z < 0 ? "left" : "right"}`, parent: 0, at: [x * 0.22, 0, z * 0.13], face: Math.atan2(z * 0.86, x * 0.5), coxa: 0.07, l1: 0.21, l2: 0.27, hang: 0.17, out: 0.36, phase: x * z > 0 ? 0 : 0.5, gait: g });
    const speed = idle ? 0 : g.stride / (g.duty * g.period);
    return { links: mk.links, height: 0.196, speed, travel: idle ? undefined : (t) => ({ x: speed * t, z: 0, yaw: 0 }), cycle: g.period, size: 0.68 };
  },
  "hexapod-radial": (idle = false) => radial(6, { r: 0.2, coxa: 0.07, l1: 0.16, l2: 0.28, hang: 0.2, out: 0.3, gait: { ...WIDE, duty: 0.55 }, tripod: true }, idle),
  octopod: (idle = false) => radial(8, { r: 0.22, coxa: 0.07, l1: 0.19, l2: 0.3, hang: 0.2, out: 0.34, gait: { ...WIDE, duty: 0.66, period: 1.2 }, tripod: false }, idle),
  "biped-point": (idle = false) => biped(idle),
  humanoid: (idle = false) => humanoid(idle),
  "diff-drive": (idle = false) => diffDrive(idle),
  ackermann: (idle = false) => ackermann(idle),
  mecanum: (idle = false) => mecanum(idle),
  "omni-3": (idle = false) => omni(idle),
  "swerve-4": (idle = false) => swerve(idle),
  "tracked-flipper": (idle = false) => tracked(idle),
  "wheel-leg": (idle = false) => wheelLeg(idle),
  "tri-wheel-stair": (idle = false) => triwheel(idle),
  "rocker-bogie": (idle = false) => rocker(idle),
  "mobile-manipulator": (idle = false) => manipulator(idle),
  "snake-crawler": (idle = false) => snake(idle),
  "balancer-2w": (idle = false) => balancer(idle),
};

/* ---------- Belts ---------- */

/** Where a point s along a belt loop is, and how far it has tilted: the top run moves forwards, the bottom back. */
export function along(b: Belt, s: number): [x: number, y: number, tilt: number] {
  const arc = Math.PI * b.r;
  let u = mod(s, 2 * b.len + 2 * arc);
  if (u < b.len) return [u, b.r, 0];
  u -= b.len;
  if (u < arc) {
    const a = Math.PI / 2 - u / b.r;
    return [b.len + b.r * Math.cos(a), b.r * Math.sin(a), a - Math.PI / 2];
  }
  u -= arc;
  if (u < b.len) return [b.len - u, -b.r, -Math.PI];
  u -= b.len;
  const a = -Math.PI / 2 - u / b.r;
  return [b.r * Math.cos(a), b.r * Math.sin(a), a - Math.PI / 2];
}
/** Where each shoe of a belt sits once the belt has run a distance s. The shoes close the loop exactly. */
export function shoes(b: Belt, s: number) {
  const total = 2 * b.len + 2 * Math.PI * b.r;
  const n = Math.max(1, Math.round(total / b.pitch));
  return Array.from({ length: n }, (_, k) => along(b, s + (k * total) / n));
}
