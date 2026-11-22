import { Euler, Matrix4, Quaternion, Vector3 } from "three";
import type { Body, Part } from "./bodies";

const AXIS = { x: new Vector3(1, 0, 0), y: new Vector3(0, 1, 0), z: new Vector3(0, 0, 1) };
const ONE = new Vector3(1, 1, 1);

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

export interface Envelope {
  /** Metres: across the body, floor to top, and the floor circle that holds it whichever way it faces. */
  width: number;
  height: number;
  circle: number;
}

/** The box and circle that hold everything drawn, over two seconds of motion. */
export function envelope(body: Body): Envelope {
  const { pose } = frames(body);
  const root = new Matrix4();
  const m = new Matrix4();
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
  for (let k = 0; k < 24; k++) {
    const world = pose(k / 12, root);
    for (const [i, list] of parts.entries())
      for (const [p, local] of list) {
        m.multiplyMatrices(world[i]!, local);
        if (p.s === "ball") {
          centre.setFromMatrixPosition(m);
          const r = p.d[0];
          const far = Math.hypot(centre.x, centre.z);
          e.half = Math.max(e.half, Math.abs(centre.z) + r);
          e.top = Math.max(e.top, centre.y + r);
          e.radius = Math.max(e.radius, far + r);
        } else if (p.s === "cyl") {
          // A cylinder reaches h·|a·u| + r·√(1 − (a·u)²) along any direction u.
          axis.setFromMatrixColumn(m, 1);
          centre.setFromMatrixPosition(m);
          const r = Math.max(p.d[0], p.d[2]);
          const h = p.d[1] / 2;
          e.half = Math.max(e.half, Math.abs(centre.z) + h * Math.abs(axis.z) + r * Math.sqrt(Math.max(0, 1 - axis.z ** 2)));
          e.top = Math.max(e.top, centre.y + h * Math.abs(axis.y) + r * Math.sqrt(Math.max(0, 1 - axis.y ** 2)));
          for (const end of [-h, h])
            for (let a = 0; a < 12; a++) {
              v.set(r * Math.cos((a * Math.PI) / 6), end, r * Math.sin((a * Math.PI) / 6)).applyMatrix4(m);
              e.radius = Math.max(e.radius, Math.hypot(v.x, v.z));
            }
        } else
          for (let c = 0; c < 8; c++) {
            v.set(
              (c & 1 ? 0.5 : -0.5) * p.d[0],
              (c & 2 ? 0.5 : -0.5) * p.d[1],
              (c & 4 ? 0.5 : -0.5) * p.d[2],
            ).applyMatrix4(m);
            point(v.x, v.y, v.z);
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
