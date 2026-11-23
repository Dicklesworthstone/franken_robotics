/** The arithmetic behind the figures, against answers that the plan states or that a second method gives. */
import { expect, test } from "bun:test";
import { fisher, label } from "./figures/autopsy";
import { majority, shell } from "./figures/bench";
import { makers } from "./figures/bodies";
import { A, clearance, complex, longest } from "./figures/corner";
import { pushes, ride, verdict as docks } from "./figures/docking";
import { envelope, verdict as fits } from "./figures/envelope";
import { generate, interiors } from "./figures/interiors";
import { q, rng } from "./figures/lib";
import { climb } from "./figures/lip";
import { crowd, PLACES } from "./figures/lineup";
import { outcome } from "./figures/outcomes";
import { allocation } from "./figures/region";
import { simulate } from "./figures/retries";
import { fails, run } from "./figures/shrink";
import { argue, balance, EDGE, springs } from "./figures/stance";
import { stabilizer } from "./figures/symmetry";

const M = [q(60), q(30)] as const;

test("the payload fixture of §25.13 has its exact edge at 3/14 m", () => {
  expect(balance(q(1, 4), ...M).f.map(String)).toEqual(["155", "135", "105", "5"]);
  expect(balance(EDGE, ...M).f.map(String)).toEqual(["160", "130", "110", "0"]);
  expect(argue(EDGE, false).covers).toBe(true);
  expect(argue(q(3, 14).sub(q(1, 1_000_000)), false).covers).toBe(false);
  expect(balance(q(1, 5), ...M).lo.toString()).toBe("165");
  expect(springs(q(1, 4), ...M).map(String)).toEqual(["175", "115", "85", "25"]);
  expect(argue(q(1, 4), true).covers).toBe(false);
});

test("a corner that balances means every payload position balances, with forces that sum and tip correctly", () => {
  for (const b of [EDGE, q(1, 4), q(3, 10)])
    for (let i = -4; i <= 4; i++)
      for (let j = -4; j <= 4; j++) {
        const px = q(60 * i, 4);
        const py = q(30 * j, 4);
        const { ok, f } = balance(b, px, py);
        expect(ok).toBe(true);
        const [pp, pm, mp, mm] = f as [(typeof f)[0], (typeof f)[0], (typeof f)[0], (typeof f)[0]];
        expect(pp.add(pm).add(mp).add(mm).toString()).toBe("400");
        expect(pp.add(pm).sub(mp).sub(mm).div(q(3)).sub(px).n).toBe(0n);
        expect(pp.add(mp).sub(pm).sub(mm).mul(b).sub(py).n).toBe(0n);
        for (const v of f) expect(v.value >= 0 && v.value <= 160).toBe(true);
      }
});

test("the map of footprints agrees with the exact argument", () => {
  for (let mm = 120; mm <= 400; mm++)
    expect(allocation(1 / 3, mm / 1000, 400, 160).ok).toBe(argue(q(mm, 1000), false).covers);
});

test("the docking fixture: exact docking, and the thresholds 1/8 and 5/16", () => {
  for (const g of [1, 2])
    for (const p of [0.25, 0.4, 0.625]) {
      const end = ride(g, pushes(g, p)).at(-1)!;
      expect(end.x).toBeCloseTo(1, 12);
      expect(end.v).toBeCloseTo(0, 12);
    }
  expect(docks(0.75, 0.1249, 0.25, false)).toBe("ok");
  expect(docks(0.75, 0.1251, 0.25, false)).toBe("alias");
  expect(docks(0.75, 0.3124, 0.625, false)).toBe("ok");
  expect(docks(0.75, 0.3126, 0.625, false)).toBe("alias");
  expect(docks(0.75, 0, 0.25, true)).toBe("late");
  expect(docks(0.75, 0.1, 0.7, false)).toBe("authority");
  // Inside the permitted range no push exceeds the motor, and just outside one does.
  const peak = (p: number) => Math.max(...[1, 2].flatMap((g) => pushes(g, p)).map(Math.abs));
  expect(peak(0.25)).toBeLessThanOrEqual(0.75 + 1e-12);
  expect(peak(0.625)).toBeLessThanOrEqual(0.75 + 1e-12);
  expect(peak(0.626)).toBeGreaterThan(0.75);
  expect(peak(0.249)).toBeGreaterThan(0.75);
});

test("the lip fixture: 36 mm and 0.9 N·m against a larger wheel that needs less grip", () => {
  const small = climb(60, 12);
  const big = climb(75, 12);
  expect(small.a).toBeCloseTo(36, 12);
  expect(small.torque).toBeCloseTo(0.9, 12);
  expect(small.mu).toBeCloseTo(0.75, 12);
  expect(big.torque).toBeCloseTo(1.0173, 4);
  expect(big.mu).toBeCloseTo(0.6459, 4);
});

test("clearance in the corridor matches a direct search over points", () => {
  const u = rng(5);
  for (let i = 0; i < 400; i++) {
    const [x, y, th] = [u() * 2, u() * 2, u() * Math.PI];
    const [c, s] = [Math.cos(th), Math.sin(th)];
    const [L, b] = [0.3 + u(), 0.2 + u() * 0.5];
    let hit = false;
    for (let j = 0; j <= 60 && !hit; j++)
      for (let k = 0; k <= 60; k++) {
        const lx = (j / 60 - 0.5) * L;
        const ly = (k / 60 - 0.5) * b;
        const px = x + c * lx - s * ly;
        const py = y + s * lx + c * ly;
        if (px < 0 || py < 0 || (px > A && py > A)) {
          hit = true;
          break;
        }
      }
    const v = clearance(x, y, c, s, L, b);
    if (hit) expect(v).toBeLessThan(1e-9);
    // The grid of points can miss a corner by its own pitch.
    else expect(v).toBeGreaterThan(-Math.hypot(L, b) / 60);
  }
});

test("the clearance complex never contradicts the closed form, and settles both ways", () => {
  const said = new Set<string>();
  for (const b of [0.4, 0.6])
    for (const L of [0.8, 1.0, longest(b) - 0.02, longest(b) + 0.02, longest(b) + 0.4, 2.2])
      for (const level of [0, 1]) {
        if (L <= b) continue;
        const { verdict } = complex(L, b, level);
        said.add(verdict);
        if (L < longest(b)) expect(verdict).not.toBe("blocked");
        else expect(verdict).not.toBe("fits");
      }
  expect([...said].sort()).toEqual(["blocked", "fits", "open"]);
});

test("the shrink stops at three edits, and the pass over pairs finds the two", () => {
  const log = run();
  expect(log.every((t) => t.failed === fails(t.set))).toBe(true);
  const keeps = log.filter((t) => t.keep).map((t) => [...t.keep!].sort().join(""));
  expect(keeps).toContain("cde");
  expect(keeps.at(-1)).toBe("ab");
  expect(new Set(log.map((t) => [...t.set].sort().join(""))).size).toBe(log.length);
});

test("autopsy labels follow the thresholds of §27.6", () => {
  expect(label(164, 12, 0.04)).toMatchObject({ tag: "but-for", contrast: 0.76 });
  expect(label(38, 12, 0.04)).toMatchObject({ tag: "contributing", contrast: 0.13 });
  expect(label(164, 12, 0.2).tag).not.toBe("but-for");
  expect(label(190, 41, 0.04).tag).toBe("chaotic: unattributable");
  expect(label(14, 12, 0).tag).toBe("not named");
  // One-sided Fisher by hand for a table small enough to check in the tail: equal counts give more than one half.
  expect(fisher(12, 12)).toBeGreaterThan(0.5);
  expect(fisher(200, 0)).toBeLessThan(1e-50);
  expect(fisher(0, 0)).toBeCloseTo(1, 9);
});

test("outcomes change at 1, 3 and 10, and ten-fold needs its guard", () => {
  expect([0.99, 1, 2.99, 3, 9.99, 10].map((r) => outcome(r, true, false).zone)).toEqual([0, 1, 1, 2, 2, 3]);
  expect(outcome(40, false, false).zone).toBe(2);
  expect(outcome(5, true, true).label).toContain("Tie");
});

test("the rectangle's symmetries are counted from what breaks them", () => {
  const legs = [1, -1].flatMap((x) => [1, -1].map((y) => ({ kind: "leg", x, y, dx: -x })));
  expect(stabilizer(legs)).toHaveLength(4);
  expect(stabilizer([...legs, { kind: "lidar", x: 1.1, y: 0, dx: 1 }])).toEqual(["identity", "left to right"]);
  expect(stabilizer([...legs, { kind: "load", x: 0, y: 0.4 }])).toEqual(["identity", "front to back"]);
  expect(stabilizer(legs.map((l) => ({ ...l, dx: -1 })))).toEqual(["identity", "left to right"]);
  expect(stabilizer([...legs, { kind: "load", x: 0.3, y: 0.4 }])).toEqual(["identity"]);
});

test("spending the error budget holds a worthless campaign to about five per cent", () => {
  let naive = 0;
  let spend = 0;
  let n = 0;
  for (let seed = 1; seed <= 12; seed++) {
    const r = simulate(seed);
    n += r.naive.length;
    naive += r.naive.filter(Boolean).length;
    spend += r.spend.filter(Boolean).length;
  }
  expect(naive / n).toBeGreaterThan(0.99);
  // Four standard errors above the bound.
  expect(spend / n).toBeLessThan(0.05 + 4 * Math.sqrt((0.05 * 0.95) / n));
  expect(spend / n).toBeGreaterThan(0.03);
});

test("majority reads and the hollow box match hand arithmetic", () => {
  expect(majority(1)).toEqual([3, 4]);
  expect(majority(3)).toEqual([54, 64]);
  expect(majority(5)[0] / majority(5)[1]).toBeCloseTo(918 / 1024, 12);
  // 0.024 − 0.395 · 0.295 · 0.195 = 0.001277625 m³ of metal; the plan rounds its mass to 3.45 kg.
  expect(shell(0.0025)).toBeCloseTo(0.001277625 * 2700, 9);
  expect(shell(0.1)).toBeCloseTo(64.8, 9);
});

test("the crowd is the same for everyone, and every body is sorted by its own measured size", () => {
  const all = crowd();
  expect(all).toHaveLength(99);
  expect(crowd()).toEqual(all);
  const size = Object.fromEntries(Object.entries(makers).map(([id, make]) => [id, envelope(make())]));
  for (const e of Object.values(size)) for (const v of Object.values(e)) expect(v).toBeGreaterThan(0.05);
  const count = (place: string) => all.filter((m) => fits(size[m.id]!, m.scale, PLACES[place]!) === "fits").length;
  expect(count("door")).toBe(71);
  expect(count("hall")).toBeGreaterThan(count("door"));
  expect(count("shelf")).toBeLessThan(count("door"));
  // A smaller copy of a body that fits also fits.
  for (const m of all)
    if (fits(size[m.id]!, m.scale, PLACES.door!) === "fits")
      expect(fits(size[m.id]!, m.scale * 0.9, PLACES.door!)).toBe("fits");
});

test("every place generates finite geometry and numbered hazards for many seeds", () => {
  for (const place of Object.keys(interiors))
    for (let seed = 1; seed <= 60; seed++) {
      const room = generate(place, seed);
      expect(JSON.stringify(room)).toBe(JSON.stringify(generate(place, seed)));
      expect(room.hazards.length).toBeGreaterThan(2);
      const numbers = JSON.stringify(room).match(/null/g);
      expect(numbers).toBeNull();
    }
});
