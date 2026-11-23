import { Euler, Matrix4, Quaternion, Vector3 } from "three";
import { girth, SHEET, shoes, type Body, type Link, type Part } from "./bodies";

const AXIS = { x: new Vector3(1, 0, 0), y: new Vector3(0, 1, 0), z: new Vector3(0, 0, 1) };
const ONE = new Vector3(1, 1, 1);
const Z = new Vector3(0, 0, 1);

/** A body's links with the fixed parts of their transforms worked out once. */
export function frames(body: Body) {
  const links = body.links.map((l) => ({
    parent: l.parent,
    at: new Vector3(...l.at),
    set: l.set ? new Quaternion().setFromEuler(new Euler(...l.set)) : new Quaternion(),
    axis: l.joint === "free" || l.joint === "fixed" ? undefined : AXIS[l.joint],
    turn: l.turn,
  }));
  const world = links.map(() => new Matrix4());
  const local = new Matrix4();
  const at = new Vector3();
  const turn = new Quaternion();
  const lean = new Euler();
  /** World transforms of every link at time t, with the base standing at `root`. */
  const pose = (t: number, root: Matrix4) => {
    const off = body.base?.(t) ?? {};
    for (const [i, l] of links.entries()) {
      if (l.parent < 0) {
        at.set(off.x ?? 0, body.height + (off.y ?? 0), off.z ?? 0);
        turn.setFromEuler(lean.set(off.roll ?? 0, off.yaw ?? 0, off.pitch ?? 0));
        world[i]!.multiplyMatrices(root, local.compose(at, turn, ONE));
        continue;
      }
      turn.copy(l.set);
      if (l.axis && l.turn) turn.multiply(spin.setFromAxisAngle(l.axis, l.turn(t)));
      world[i]!.multiplyMatrices(world[l.parent]!, local.compose(l.at, turn, ONE));
    }
    return world;
  };
  return { pose };
}
const spin = new Quaternion();

/** The shoes of a belt, in its link's frame: how many there are, and where each one is at time t. */
export function treads(link: Link) {
  const belt = link.belt!;
  const count = shoes(belt, 0).length;
  const q = new Quaternion();
  const p = new Vector3();
  let memo = NaN;
  let list = shoes(belt, 0);
  return {
    count,
    place(t: number, k: number, out: Matrix4) {
      if (t !== memo) {
        memo = t;
        list = shoes(belt, link.turn?.(t) ?? 0);
      }
      const [x, y, tilt] = list[k]!;
      return out.compose(p.set(x, y, 0), q.setFromAxisAngle(Z, tilt), ONE);
    },
  };
}

export interface Envelope {
  /** Metres: across the body, floor to top, and the floor circle that holds it whichever way it faces. */
  width: number;
  height: number;
  circle: number;
}

const Y = new Vector3(0, 1, 0);
const pa = new Vector3();
const pb = new Vector3();
const pd = new Vector3();
const pq = new Quaternion();
const turnAbout = new Matrix4();
const shift = new Matrix4();
/**
 * Where a part is in its link at time t: its own place `own`, turned about its axle when it spins, or set between its
 * two points when it spans them. Returns its length along local y, which a span stretches.
 */
export function placed(p: Part, own: Matrix4, t: number, out: Matrix4): number {
  if (p.span) {
    const [a, b] = p.span(t);
    pa.set(...a);
    pb.set(...b);
    pd.subVectors(pb, pa);
    const len = pd.length();
    pq.setFromUnitVectors(Y, len > 1e-9 ? pd.divideScalar(len) : Y);
    out.compose(pa.add(pb).multiplyScalar(0.5), pq, ONE);
    return len;
  }
  if (p.spin) {
    const { at, axis, angle } = p.spin;
    turnAbout.makeRotationAxis(pd.set(...axis).normalize(), angle(t));
    // About the axle through `at`: move it to the origin, turn, move it back.
    out.makeTranslation(-at[0], -at[1], -at[2]).multiply(own).premultiply(turnAbout).premultiply(shift.makeTranslation(at[0], at[1], at[2]));
    return p.d[1];
  }
  out.copy(own);
  return p.d[1];
}
/** A part's own place in its link, from its position and turn. */
export const ownPlace = (p: Part) => new Matrix4().compose(new Vector3(...p.p), new Quaternion().setFromEuler(new Euler(...(p.r ?? [0, 0, 0]))), ONE);

/**
 * The box and circle that hold everything drawn, over the whole of the body's motion. Parts drawn only close up are
 * left out unless `close` is set: a test holds them inside the rest, so that the door test need not pay for them.
 */
export function envelope(body: Body, close = false): Envelope {
  const { pose } = frames(body);
  const root = new Matrix4();
  const m = new Matrix4();
  const shoe = new Matrix4();
  const axis = new Vector3();
  const centre = new Vector3();
  const v = new Vector3();
  const e = { half: 0, top: 0, radius: 0 };
  const point = (x: number, y: number, z: number) => {
    e.half = Math.max(e.half, Math.abs(z));
    e.top = Math.max(e.top, y);
    e.radius = Math.max(e.radius, Math.hypot(x, z));
  };
  const parts = body.links.map((l) => l.parts.filter((p) => (close ? !p.coarse : !p.fine)).map((p) => [p, ownPlace(p)] as const));
  const belts = body.links.map((l) => (l.belt ? treads(l) : undefined));
  const part = new Matrix4();
  const reach = (p: Part, len: number) => {
    if (p.s === "ball") {
      centre.setFromMatrixPosition(m);
      const r = p.d[0];
      e.half = Math.max(e.half, Math.abs(centre.z) + r);
      e.top = Math.max(e.top, centre.y + r);
      e.radius = Math.max(e.radius, Math.hypot(centre.x, centre.z) + r);
    } else if (p.s === "cap") {
      // A capsule is a segment grown by its radius.
      axis.setFromMatrixColumn(m, 1);
      centre.setFromMatrixPosition(m);
      const r = p.d[0];
      const h = len / 2;
      e.half = Math.max(e.half, Math.abs(centre.z) + h * Math.abs(axis.z) + r);
      e.top = Math.max(e.top, centre.y + h * Math.abs(axis.y) + r);
      for (const end of [-h, h]) e.radius = Math.max(e.radius, Math.hypot(centre.x + axis.x * end, centre.z + axis.z * end) + r);
    } else if (p.s === "arc" || p.s === "fender") {
      // A bent tube or a fender reaches only as far round as it sweeps: the corners of its section, all the way round,
      // pushed out a little so that the polygon through them holds the curve between.
      const [R, b, sweep] = p.d;
      const [lo, hi, half] = p.s === "arc" ? [R - b, R + b, b] : [R * (1 - SHEET / 2), R * (1 + SHEET / 2), b / 2];
      const out = 1 / Math.cos(sweep / 64);
      for (let a = 0; a <= 32; a++) {
        const angle = (a / 32) * sweep;
        for (const r of [lo, hi * out])
          for (const h of [-half, half]) {
            v.set(r * Math.cos(angle), h, -r * Math.sin(angle)).applyMatrix4(m);
            point(v.x, v.y, v.z);
          }
      }
    } else if (p.s !== "box") {
      // Round solids: a cylinder reaches h·|a·u| + r·√(1 − (a·u)²) along any direction u.
      axis.setFromMatrixColumn(m, 1);
      centre.setFromMatrixPosition(m);
      const [r, half] = girth(p);
      // A span stretches the solid along its axis; a ring has no length to stretch.
      const h = p.s === "ring" ? half : half * (len / p.d[1]);
      e.half = Math.max(e.half, Math.abs(centre.z) + h * Math.abs(axis.z) + r * Math.sqrt(Math.max(0, 1 - axis.z ** 2)));
      e.top = Math.max(e.top, centre.y + h * Math.abs(axis.y) + r * Math.sqrt(Math.max(0, 1 - axis.y ** 2)));
      for (const end of [-h, h])
        for (let a = 0; a < 24; a++) {
          v.set(r * Math.cos((a * Math.PI) / 12), end, r * Math.sin((a * Math.PI) / 12)).applyMatrix4(m);
          e.radius = Math.max(e.radius, Math.hypot(v.x, v.z));
        }
    } else
      for (let c = 0; c < 8; c++) {
        v.set((c & 1 ? 0.5 : -0.5) * p.d[0], (c & 2 ? 0.5 : -0.5) * len, (c & 4 ? 0.5 : -0.5) * p.d[2]).applyMatrix4(m);
        point(v.x, v.y, v.z);
      }
  };
  const span = Math.max(2, body.cycle ?? 2);
  const n = 48;
  for (let k = 0; k < n; k++) {
    const t = (k / n) * span;
    const world = pose(t, root);
    for (const [i, list] of parts.entries()) {
      const belt = belts[i];
      if (belt)
        for (let s = 0; s < belt.count; s++) {
          belt.place(t, s, shoe).premultiply(world[i]!);
          for (const [p, local] of list) {
            const len = placed(p, local, t, part);
            m.multiplyMatrices(shoe, part);
            reach(p, len);
          }
        }
      else
        for (const [p, local] of list) {
          const len = placed(p, local, t, part);
          m.multiplyMatrices(world[i]!, part);
          reach(p, len);
        }
    }
  }
  return { width: 2 * e.half, height: e.top, circle: 2 * e.radius };
}

export interface Place {
  width: number;
  height: number;
  circle: number;
}
export type Verdict = "fits" | "wide" | "tall" | "long";
/** The first dimension that stops a body of this size, or that it fits. */
export function verdict(e: Envelope, scale: number, place: Place): Verdict {
  if (e.width * scale > place.width) return "wide";
  if (e.height * scale > place.height) return "tall";
  if (e.circle * scale > place.circle) return "long";
  return "fits";
}
