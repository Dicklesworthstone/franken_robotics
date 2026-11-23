import { expect, test } from "bun:test";
import { Matrix4, Vector3 } from "three";
import { archetypes } from "./figures/archetypes";
import { makers, type Solid } from "./figures/bodies";
import { envelope, frames, ownPlace, placed } from "./figures/envelope";
import { shapeOf } from "./figures/three-kit";

test("every archetype is drawn with the actuated coordinates its row in the plan states", () => {
  expect(Object.keys(makers)).toEqual(archetypes.map((a) => a.id));
  for (const a of archetypes) {
    const { links } = makers[a.id]!();
    const active = links.filter((l) => l.joint !== "free" && l.joint !== "fixed" && !l.passive);
    expect({ id: a.id, dof: active.length }).toEqual({ id: a.id, dof: a.dof });
  }
});

test("links come after their parents and never produce a number that is not finite", () => {
  for (const make of Object.values(makers)) {
    const { links } = make();
    for (const [i, l] of links.entries()) {
      expect(l.parent).toBeLessThan(i);
      for (const t of [0, 0.13, 0.5, 0.77, 4.2]) expect(Number.isFinite(l.turn?.(t) ?? 0)).toBe(true);
    }
  }
});

test("what is drawn only close up lies inside the size the door test measures", () => {
  for (const a of archetypes) {
    const body = makers[a.id]!();
    const plain = envelope(body);
    const close = envelope(body, true);
    // Close up may stop short of the crowd's stand-in for a part, as a lens stops short of its glass ball or a
    // gripper's fingers close inside the still ones that stand open far off, but never reaches past it, nor falls
    // more than a few centimetres short.
    for (const k of ["width", "height", "circle"] as const) {
      const over = close[k] - plain[k];
      expect({ id: a.id, k, over: over <= 1e-9 && over > -0.05 }).toEqual({ id: a.id, k, over: true });
    }
  }
});

test("every moving part is somewhere finite at every moment", () => {
  const out = new Matrix4();
  for (const make of Object.values(makers))
    for (const l of make().links)
      for (const p of l.parts.filter((p) => p.spin || p.span))
        for (const t of [0, 0.13, 0.5, 0.77, 4.2, 11.9]) {
          const len = placed(p, ownPlace(p), t, out);
          expect(Number.isFinite(len) && len > 0 && out.elements.every(Number.isFinite)).toBe(true);
        }
});

test("gears in mesh meet tooth to gap at every moment, which also holds them to the ratio of their teeth", () => {
  const part = new Matrix4();
  const w = new Matrix4();
  // Every train, and how many pairs mesh in it: a pair that drifted apart would drop out of the count.
  const trains: Record<string, number> = { "swerve-4": 4, "tri-wheel-stair": 12, "snake-crawler": 8 };
  const pairs: Record<string, number[]> = {};
  for (const a of archetypes) {
    const body = makers[a.id]!();
    const { pose } = frames(body);
    const gears = body.links.flatMap((l, i) => l.parts.filter((p) => p.s === "gear").map((p) => ({ p, i, own: ownPlace(p) })));
    for (const t of [0, 0.37, 1.1, 2.9, 6.3]) {
      let count = 0;
      const world = pose(t, new Matrix4());
      const at = gears.map(({ p, i, own }) => {
        placed(p, own, t, part);
        w.multiplyMatrices(world[i]!, part);
        return {
          p,
          c: new Vector3().setFromMatrixPosition(w),
          axis: new Vector3().setFromMatrixColumn(w, 1).normalize(),
          x0: new Vector3().setFromMatrixColumn(w, 0).normalize(),
        };
      });
      for (let i = 0; i < at.length; i++)
        for (let j = i + 1; j < at.length; j++) {
          const [g1, g2] = [at[i]!, at[j]!];
          const sgn = g1.axis.dot(g2.axis);
          if (Math.abs(sgn) < 0.9999) continue;
          const d = g2.c.clone().sub(g1.c);
          const axial = d.dot(g1.axis);
          if (Math.abs(axial) > (g1.p.d[1] + g2.p.d[1]) / 2) continue;
          const across = d.addScaledVector(g1.axis, -axial);
          if (Math.abs(across.length() - g1.p.d[0] - g2.p.d[0]) > 5e-4) continue;
          const u = across.normalize();
          const phase = (g: typeof g1, dir: Vector3) =>
            (g.p.d[2] * Math.atan2(g.x0.clone().cross(dir).dot(g.axis), g.x0.dot(dir))) / (2 * Math.PI);
          const miss = (((phase(g1, u) + Math.sign(sgn) * phase(g2, u.clone().negate()) - 0.5) % 1) + 1) % 1;
          expect({ body: a.id, t, miss: Math.min(miss, 1 - miss) < 0.02 }).toEqual({ body: a.id, t, miss: true });
          count++;
        }
      if (count) (pairs[a.id] ??= []).push(count);
    }
  }
  expect(pairs).toEqual(Object.fromEntries(Object.entries(trains).map(([id, n]) => [id, [n, n, n, n, n]])));
});

test("every solid has finite geometry at every level of detail", () => {
  const sizes: Record<Solid, [number, number, number]> = {
    box: [0.1, 0.05, 0.02],
    cyl: [0.03, 0.1, 0.03],
    ball: [0.02, 0.02, 0.02],
    cap: [0.02, 0.08, 0.02],
    tire: [0.1, 0.05, 0.064],
    ring: [0.05, 0.004, 0.05],
    barrel: [0.02, 0.06, 0.01],
    gear: [0.04, 0.01, 36],
    hex: [0.006, 0.004, 0.006],
    coil: [0.02, 0.08, 7],
    arc: [0.05, 0.004, 2],
    fender: [0.135, 0.085, 1.44],
    pulley: [0.03, 0.02, 0.004],
    bellows: [0.02, 0.06, 6],
    fins: [0.03, 0.06, 18],
    bolts: [0.03, 0.003, 8],
  };
  for (const [s, d] of Object.entries(sizes))
    for (const q of [0, 1, 2] as const) {
      const pos = shapeOf(s as Solid, d, q).getAttribute("position");
      expect({ s, q, ok: pos.count > 0 && Array.from(pos.array).every(Number.isFinite) }).toEqual({ s, q, ok: true });
    }
});
