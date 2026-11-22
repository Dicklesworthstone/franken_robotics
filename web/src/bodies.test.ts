import { expect, test } from "bun:test";
import { archetypes } from "./figures/archetypes";
import { makers } from "./figures/bodies";

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
