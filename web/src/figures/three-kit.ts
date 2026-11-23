import {
  BoxGeometry,
  CapsuleGeometry,
  Color,
  Curve,
  CylinderGeometry,
  ExtrudeGeometry,
  LatheGeometry,
  Path,
  Shape,
  TubeGeometry,
  Vector3,
  MeshPhysicalMaterial,
  MeshStandardMaterial,
  PMREMGenerator,
  PCFSoftShadowMap,
  SRGBColorSpace,
  ACESFilmicToneMapping,
  Scene,
  SphereGeometry,
  TorusGeometry,
  Vector2,
  WebGLRenderer,
  type BufferGeometry,
} from "three";
import { RoomEnvironment } from "three/examples/jsm/environments/RoomEnvironment.js";
import { RoundedBoxGeometry } from "three/examples/jsm/geometries/RoundedBoxGeometry.js";
import { mergeGeometries } from "three/examples/jsm/utils/BufferGeometryUtils.js";
import { type Mat, rounding, SHEET, type Solid } from "./bodies";

/** A renderer on the figure's own canvas, or a sentence in its place where WebGL is missing. */
export function renderer(canvas: HTMLCanvasElement) {
  try {
    const gl = new WebGLRenderer({ canvas, antialias: true, alpha: true, powerPreference: "high-performance" });
    gl.outputColorSpace = SRGBColorSpace;
    gl.toneMapping = ACESFilmicToneMapping;
    gl.shadowMap.enabled = true;
    gl.shadowMap.type = PCFSoftShadowMap;
    return gl;
  } catch {
    const p = document.createElement("p");
    p.className = "noscript";
    p.textContent = "This figure is drawn with WebGL, which this browser does not offer.";
    canvas.replaceWith(p);
    return undefined;
  }
}

export function lit(gl: WebGLRenderer, intensity = 0.55) {
  const scene = new Scene();
  const pmrem = new PMREMGenerator(gl);
  scene.environment = pmrem.fromScene(new RoomEnvironment(), 0.04).texture;
  scene.environmentIntensity = intensity;
  pmrem.dispose();
  return scene;
}

export function materials(accent = "#4ade80"): Record<Mat, MeshStandardMaterial> {
  const m = (color: string, roughness: number, metalness: number) =>
    new MeshStandardMaterial({ color, roughness, metalness });
  const glow = m(accent, 0.4, 0);
  glow.emissive = new Color(accent);
  glow.emissiveIntensity = 1.5;
  // Painted shells carry a clear lacquer; sensor windows are dark glass with a little light inside.
  const shell = new MeshPhysicalMaterial({ color: "#e4ece7", roughness: 0.42, metalness: 0.04, clearcoat: 0.7, clearcoatRoughness: 0.22 });
  const lens = new MeshPhysicalMaterial({
    color: "#04100b",
    roughness: 0.06,
    metalness: 0.3,
    clearcoat: 1,
    clearcoatRoughness: 0.03,
    emissive: new Color(accent),
    emissiveIntensity: 0.16,
  });
  return {
    shell,
    dark: m("#1b2621", 0.5, 0.45),
    steel: m("#9aa9a1", 0.26, 0.92),
    rubber: m("#0b0f0d", 0.86, 0),
    glow,
    amber: m("#fbbf24", 0.4, 0.2),
    lens,
  };
}

/** A tire's cross-section, rim to rim round the tread, as points of (radius, height). */
function tireProfile(R: number, w: number, rim: number, fine: boolean) {
  const h = w / 2;
  const wall = R - rim;
  // One side, from the rim to the middle of the tread; the other side mirrors it.
  const half: [number, number][] = [
    [rim, -h * 0.86],
    [rim + wall * 0.3, -h],
    [R - wall * 0.24, -h * 0.97],
    [R - wall * 0.08, -h * 0.84],
    [R - wall * 0.01, -h * 0.64],
    [R, -h * 0.46],
  ];
  if (fine) {
    // A groove round the tread on each side of the middle.
    const g = Math.min(0.005, wall * 0.08);
    half.push([R, -h * 0.34], [R - g, -h * 0.3], [R - g, -h * 0.2], [R, -h * 0.16]);
  }
  half.push([R, 0]);
  const other = half.slice(0, -1).reverse().map(([x, y]) => [x, -y] as [number, number]);
  return [...half, ...other].map(([x, y]) => new Vector2(x, y));
}
/** A barrel roller: fat in the middle, rounded off to its ends. */
function barrelProfile(mid: number, len: number, end: number, n: number) {
  const pts = [new Vector2(0, -len / 2)];
  for (let i = 0; i <= n; i++) {
    const y = -len / 2 + (i / n) * len;
    const u = (2 * y) / len;
    pts.push(new Vector2(end + (mid - end) * (1 - u * u), y));
  }
  pts.push(new Vector2(0, len / 2));
  return pts;
}

/* ---------- Machine elements ---------- */

const inv = (a: number) => Math.tan(a) - a;
const PRESSURE = (20 * Math.PI) / 180;
/**
 * A spur gear's outline: involute flanks off a base circle at a 20° pressure angle, a full tooth of addendum m and
 * dedendum 1.25 m, tooth 0 centred on +x. A small gear keeps a plain bore; a large one has a hub, a web and six
 * lightening holes in it.
 */
function gearShape(R: number, z: number, fine: boolean) {
  const m = (2 * R) / z;
  const ra = R + m;
  const rf = R - 1.25 * m;
  const rb = R * Math.cos(PRESSURE);
  // Half the angle a tooth takes at radius r: π/2z at the pitch circle, less the involute's own turn.
  const half = (r: number) => Math.max(0.12 * (Math.PI / z), Math.PI / (2 * z) + inv(PRESSURE) - inv(Math.acos(Math.min(1, rb / Math.max(r, rb)))));
  const steps = fine ? 3 : 2;
  const pts: [number, number][] = [];
  const at = (r: number, a: number) => pts.push([r * Math.cos(a), r * Math.sin(a)]);
  const r0 = Math.max(rf, rb);
  for (let k = 0; k < z; k++) {
    const c = (k / z) * Math.PI * 2;
    at(rf, c - Math.PI / z);
    at(rf, c - half(r0) - 0.04 / z);
    for (let i = 0; i <= steps; i++) {
      const r = r0 + ((ra - r0) * i) / steps;
      at(r, c - half(r));
    }
    for (let i = steps; i >= 0; i--) {
      const r = r0 + ((ra - r0) * i) / steps;
      at(r, c + half(r));
    }
    at(rf, c + half(r0) + 0.04 / z);
  }
  const shape = new Shape(pts.map(([x, y]) => new Vector2(x, y)));
  const curve = fine ? 16 : 10;
  if (z >= 30) {
    const hub = R * 0.3;
    const rim = rf - 1.6 * m;
    const centre = (hub + rim) / 2;
    const hole = Math.min(((rim - hub) / 2) * 0.78, centre * Math.sin(Math.PI / 6) * 0.72);
    for (let i = 0; i < 6; i++) {
      const a = ((i + 0.5) / 6) * Math.PI * 2;
      shape.holes.push(new Path().absarc(centre * Math.cos(a), centre * Math.sin(a), hole, 0, Math.PI * 2, true));
    }
    shape.holes.push(new Path().absarc(0, 0, R * 0.12, 0, Math.PI * 2, true));
  } else shape.holes.push(new Path().absarc(0, 0, Math.min(rf * 0.34, R * 0.3), 0, Math.PI * 2, true));
  return { shape, m, curve };
}
/** A plate cut to a shape and given thickness w along local y, its edges chamfered. Centred on y = 0. */
function plate(shape: Shape, w: number, chamfer: number, curve: number) {
  const bevel = Math.min(chamfer, w * 0.2);
  const geo = new ExtrudeGeometry(shape, {
    depth: Math.max(1e-5, w - 2 * bevel),
    bevelEnabled: bevel > 0,
    bevelThickness: bevel,
    bevelSize: bevel,
    bevelOffset: -bevel,
    bevelSegments: 1,
    curveSegments: curve,
  });
  // The shape's plane becomes the xz plane with its angles turning about +y, as every round solid's do.
  return geo.translate(0, 0, -(w - 2 * bevel) / 2).rotateX(-Math.PI / 2);
}
/** A socket head cap screw's head: round, chamfered, with a hexagonal socket. Axis y, top at +h/2. */
function screwHead(r: number, fine: boolean) {
  const h = r * 1.1;
  const outer = new Shape().absarc(0, 0, r, 0, Math.PI * 2, false);
  const hex = new Path();
  const s = r * 0.5;
  for (let i = 0; i <= 6; i++) {
    const a = (i / 6) * Math.PI * 2;
    if (i) hex.lineTo(s * Math.cos(a), s * Math.sin(a));
    else hex.moveTo(s * Math.cos(a), s * Math.sin(a));
  }
  outer.holes.push(hex);
  const head = plate(outer, h, r * 0.14, fine ? 14 : 8);
  // The socket's floor, half way down.
  const floor = new CylinderGeometry(s, s, h * 0.5, 6, 1, false, Math.PI / 2).translate(0, -h * 0.25, 0);
  return merged([head, floor]);
}
/** Geometries joined into one, all made non-indexed so that any mix can be joined. */
export function merged(list: BufferGeometry[]) {
  const flat = list.map((g) => {
    const one = g.index ? g.toNonIndexed() : g;
    for (const name of Object.keys(one.attributes)) if (name !== "position" && name !== "normal" && name !== "uv") one.deleteAttribute(name);
    return one;
  });
  return mergeGeometries(flat)!;
}
/** A helix of `turns` round radius R over length len, with a closed (dead) coil at each end as a spring has. */
class Helix extends Curve<Vector3> {
  constructor(
    private R: number,
    private len: number,
    private turns: number,
  ) {
    super();
  }
  override getPoint(u: number, out = new Vector3()) {
    const a = u * this.turns * Math.PI * 2;
    // Pitch is zero over the first and last 0.6 turn, so the ends sit flat.
    const dead = 0.6 / this.turns;
    const live = Math.min(1, Math.max(0, (u - dead) / (1 - 2 * dead)));
    const w = this.R * 0.13;
    const y = -this.len / 2 + w + (this.len - 2 * w) * live;
    return out.set(this.R * Math.cos(a), y, -this.R * Math.sin(a));
  }
}
/** A spring's wire is 0.13 of its coil radius: a spring index of about 7.7. */
export const WIRE = 0.13;

/**
 * Geometry for one solid at a level of detail: 2 for a body seen close, 1 and 0 for a crowd. Round solids lie along
 * local y. Boxes get a bevel that does not stretch with their proportions.
 */
export function shapeOf(s: Solid, d: readonly [number, number, number], q: 0 | 1 | 2 = 2): BufferGeometry {
  const [a, b, c] = d;
  const round = (r: number, lo: number, hi: number) => Math.round(Math.min(hi, Math.max(lo, r)));
  const fine = q === 2;
  switch (s) {
    case "ball":
      return new SphereGeometry(a, [9, 14, 40][q]!, [6, 9, 26][q]!);
    case "cyl": {
      const r = Math.max(a, c);
      if (fine) return turned(a, b, c, round(r * 1000, 24, 96));
      return new CylinderGeometry(a, c, b, round(r * 260, q ? 10 : 7, q ? 22 : 14));
    }
    case "cap":
      return new CapsuleGeometry(a, b, [2, 3, 10][q]!, fine ? round(a * 700, 18, 64) : round(a * 260, q ? 10 : 7, q ? 18 : 12));
    case "tire":
      return fine ? tread(a, b, c) : new LatheGeometry(tireProfile(a, b, c, false), q ? 18 : 12);
    case "ring":
      return new TorusGeometry(a, b, [3, 4, 12][q]!, fine ? round(a * 1600, 48, 128) : round(a * 220, q ? 12 : 8, q ? 24 : 16)).rotateX(Math.PI / 2);
    case "barrel":
      return new LatheGeometry(barrelProfile(a, b, c, [2, 3, 8][q]!), [5, 7, 18][q]!);
    case "gear": {
      if (!fine) return new CylinderGeometry(a, a, b, Math.min(24, Math.round(c)));
      const { shape, m, curve } = gearShape(a, Math.round(c), true);
      return plate(shape, b, m * 0.16, curve);
    }
    case "hex":
      return flat(new CylinderGeometry(a, a, b, 6));
    case "coil":
      return fine ? new TubeGeometry(new Helix(a, b, c), Math.round(c * 28), a * WIRE, 8) : new CylinderGeometry(a, a, b, 10, 1, true);
    case "fender": {
      // A sector of a ring, extruded across its width with its edges eased.
      const [lo, hi] = [a * (1 - SHEET / 2), a * (1 + SHEET / 2)];
      const shape = new Shape().moveTo(hi, 0);
      shape.absarc(0, 0, hi, 0, c, false);
      shape.lineTo(lo * Math.cos(c), lo * Math.sin(c));
      shape.absarc(0, 0, lo, c, 0, true);
      shape.closePath();
      return plate(shape, b, a * SHEET * 0.3, fine ? round(a * 200 * c, 16, 64) : 8);
    }
    case "web": {
      // A disc with c round windows on a circle between its hub and its rim, cut right through it. Every curve gets
      // the same number of segments, so the rim is drawn as four quarters to make it four times as smooth as a window.
      const shape = new Shape();
      for (let k = 0; k < 4; k++) shape.absarc(0, 0, a, (k * Math.PI) / 2, ((k + 1) * Math.PI) / 2, false);
      for (let i = 0; i < Math.round(c); i++) {
        const t = (i / Math.round(c)) * Math.PI * 2;
        shape.holes.push(new Path().absarc(Math.cos(t) * a * 0.62, Math.sin(t) * a * 0.62, a * 0.15, 0, Math.PI * 2, true));
      }
      return plate(shape, b, b * 0.15, fine ? 18 : q ? 6 : 4);
    }
    case "arc":
      return new TorusGeometry(a, b, fine ? 10 : 4, fine ? round(a * 600 * (c / Math.PI), 8, 64) : 6, c).rotateX(-Math.PI / 2);
    case "pulley": {
      // Flanges at full radius either side of a rounded groove `c` deep.
      const h = b / 2;
      const pts: [number, number][] = [
        [a * 0.3, -h],
        [a - 0.002, -h],
        [a, -h + 0.002],
        [a, -h * 0.72],
        [a - c, -h * 0.4],
        [a - c * 1.06, 0],
        [a - c, h * 0.4],
        [a, h * 0.72],
        [a, h - 0.002],
        [a - 0.002, h],
        [a * 0.3, h],
        [a * 0.3, -h],
      ];
      return new LatheGeometry(pts.map(([x, y]) => new Vector2(x, y)), fine ? round(a * 700, 20, 72) : 10);
    }
    case "bellows": {
      // A rubber boot: c folds between the full radius and 0.8 of it.
      const n = Math.max(1, Math.round(c));
      const pts: Vector2[] = [new Vector2(a * 0.8, -b / 2)];
      for (let i = 0; i < n; i++) {
        const y = -b / 2 + ((i + 0.5) / n) * b;
        pts.push(new Vector2(a, y - b / (6 * n)), new Vector2(a, y + b / (6 * n)), new Vector2(a * 0.8, -b / 2 + ((i + 1) / n) * b));
      }
      return new LatheGeometry(pts, fine ? round(a * 700, 16, 48) : 8);
    }
    case "fins": {
      // A motor can: a core at 0.8 of the radius and c cooling fins standing out from it to the full radius.
      const n = Math.max(3, Math.round(c));
      const core = new CylinderGeometry(a * 0.8, a * 0.8, b, fine ? round(a * 700, 20, 64) : 10);
      if (!fine) return core;
      const fin = (i: number) =>
        new BoxGeometry(a * 0.24, b * 0.94, Math.min(a * 0.09, ((Math.PI * 2 * a * 0.8) / n) * 0.4))
          .translate(a * 0.88, 0, 0)
          .rotateY(((i + 0.5) / n) * Math.PI * 2);
      return merged([core, ...Array.from({ length: n }, (_, i) => fin(i))]);
    }
    case "bolts": {
      // c socket-head screws on a circle of radius a, heads of radius b facing +y, centred on y = 0.
      const n = Math.max(1, Math.round(c));
      if (!fine) return new CylinderGeometry(a + b, a + b, b * 1.1, 12);
      const one = screwHead(b, true);
      return merged(Array.from({ length: n }, (_, i) => one.clone().translate(a * Math.cos((i / n) * Math.PI * 2), 0, -a * Math.sin((i / n) * Math.PI * 2))));
    }
    default: {
      // Machined edges are broken, not rounded: past a few centimetres the bevel grows slowly with the block.
      const r = rounding([a, b, c]);
      return r > 0 ? new RoundedBoxGeometry(a, b, c, [1, 2, 4][q]!, r) : new BoxGeometry(a, b, c);
    }
  }
}
/**
 * A cylinder as a machinist leaves it: both rims chamfered, the bevel narrow beside flat faces so that the faces stay
 * flat in the light and only the edges catch it. Radius top at +y, bottom at −y, as CylinderGeometry has them.
 */
function turned(top: number, len: number, bottom: number, segments: number) {
  const h = len / 2;
  const cut = Math.min(0.0025, Math.min(top, bottom) * 0.12, len * 0.2);
  if (cut < 0.0002) return new CylinderGeometry(top, bottom, len, segments);
  const e = cut * 0.4;
  // From the middle of the bottom face round the rim, up the side and over the top to its middle.
  const at = (u: number) => bottom + (top - bottom) * u;
  const pts: [number, number][] = [
    [0, -h],
    [bottom - cut - e, -h],
    [bottom - cut, -h],
    [at(cut / len), -h + cut],
    [at((cut + e) / len), -h + cut + e],
    [at(1 - (cut + e) / len), h - cut - e],
    [at(1 - cut / len), h - cut],
    [top - cut, h],
    [top - cut - e, h],
    [0, h],
  ];
  return new LatheGeometry(pts.map(([x, y]) => new Vector2(x, y)), segments);
}
/** Flat faces for a prism, so that a hexagon reads as one and not as a coarse cylinder. */
function flat(geo: BufferGeometry) {
  const one = geo.toNonIndexed();
  one.computeVertexNormals();
  return one;
}
/**
 * A tyre close up: the casing turned on a lathe a tread's depth under the full radius, and blocks standing on it in
 * two staggered rows either side of a groove round the middle. Nothing reaches past the outer radius R: each block is
 * set in by as much as its flat face's corners would stand out past the curve.
 */
function tread(R: number, w: number, rim: number) {
  const depth = Math.min(0.006, (R - rim) * 0.18);
  const casing = new LatheGeometry(tireProfile(R - depth, w, rim, false), Math.round(Math.min(96, Math.max(40, R * 600))));
  const n = Math.max(16, Math.round((Math.PI * 2 * R) / 0.022));
  // Each block is sunk well into the casing, so that none stands proud of it where the shoulders fall away.
  const sunk = depth + 0.006;
  const block = (u: number, z0: number, z1: number) => {
    const len = ((Math.PI * 2 * R) / n) * 0.62;
    const sag = R - Math.sqrt(R * R - (len / 2) ** 2);
    return new RoundedBoxGeometry(sunk, len, (z1 - z0) * 0.98, 1, Math.min(0.0016, depth * 0.3))
      .translate(R - sag - sunk / 2, 0, (z0 + z1) / 2)
      .rotateZ(u);
  };
  const h = w / 2;
  const blocks: BufferGeometry[] = [];
  for (let i = 0; i < n; i++) {
    const u = (i / n) * Math.PI * 2;
    blocks.push(block(u, -h * 0.84, -h * 0.12), block(u + Math.PI / n, h * 0.12, h * 0.84));
  }
  // The casing's axis is y; the blocks were laid out round z, so they are turned onto y to match.
  const ring = merged(blocks).rotateX(Math.PI / 2);
  return merged([casing, ring]);
}

const cache = new Map<string, BufferGeometry>();
/** Geometry for one solid seen close, made once. */
export function solid(s: Solid, d: readonly [number, number, number]) {
  const key = s + d.map((v) => v.toFixed(4)).join();
  let geo = cache.get(key);
  if (!geo) cache.set(key, (geo = shapeOf(s, d, 2)));
  return geo;
}
