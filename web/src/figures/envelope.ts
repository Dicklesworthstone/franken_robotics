import { Euler, Matrix4, Quaternion, Vector3 } from "three";
import { shoes, type Body, type Link, type Part } from "./bodies";

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

/** The box and circle that hold everything drawn, over the whole of the body's motion. */
export function envelope(body: Body): Envelope {
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
  const own = (p: Part) =>
    new Matrix4().compose(
      new Vector3(...p.p),
      new Quaternion().setFromEuler(new Euler(...(p.r ?? [0, 0, 0]))),
      ONE,
    );
  const parts = body.links.map((l) => l.parts.map((p) => [p, own(p)] as const));
  const belts = body.links.map((l) => (l.belt ? treads(l) : undefined));
  const reach = (p: Part) => {
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
      const h = p.d[1] / 2;
      e.half = Math.max(e.half, Math.abs(centre.z) + h * Math.abs(axis.z) + r);
      e.top = Math.max(e.top, centre.y + h * Math.abs(axis.y) + r);
      for (const end of [-h, h]) e.radius = Math.max(e.radius, Math.hypot(centre.x + axis.x * end, centre.z + axis.z * end) + r);
    } else if (p.s !== "box") {
      // Round solids: a cylinder reaches h·|a·u| + r·√(1 − (a·u)²) along any direction u.
      axis.setFromMatrixColumn(m, 1);
      centre.setFromMatrixPosition(m);
      const [r, h] =
        p.s === "ring" ? [p.d[0] + p.d[1], p.d[1]] : p.s === "tire" ? [p.d[0], p.d[1] / 2] : [Math.max(p.d[0], p.d[2]), p.d[1] / 2];
      e.half = Math.max(e.half, Math.abs(centre.z) + h * Math.abs(axis.z) + r * Math.sqrt(Math.max(0, 1 - axis.z ** 2)));
      e.top = Math.max(e.top, centre.y + h * Math.abs(axis.y) + r * Math.sqrt(Math.max(0, 1 - axis.y ** 2)));
      for (const end of [-h, h])
        for (let a = 0; a < 24; a++) {
          v.set(r * Math.cos((a * Math.PI) / 12), end, r * Math.sin((a * Math.PI) / 12)).applyMatrix4(m);
          e.radius = Math.max(e.radius, Math.hypot(v.x, v.z));
        }
    } else
      for (let c = 0; c < 8; c++) {
        v.set((c & 1 ? 0.5 : -0.5) * p.d[0], (c & 2 ? 0.5 : -0.5) * p.d[1], (c & 4 ? 0.5 : -0.5) * p.d[2]).applyMatrix4(m);
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
            m.multiplyMatrices(shoe, local);
            reach(p);
          }
        }
      else
        for (const [p, local] of list) {
          m.multiplyMatrices(world[i]!, local);
          reach(p);
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
