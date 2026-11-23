import {
  BoxGeometry,
  CapsuleGeometry,
  Color,
  CylinderGeometry,
  LatheGeometry,
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
import type { Mat, Solid } from "./bodies";

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

/**
 * Geometry for one solid at a level of detail: 2 for a body seen close, 1 and 0 for a crowd. Round solids lie along
 * local y. Boxes get a bevel that does not stretch with their proportions.
 */
export function shapeOf(s: Solid, d: readonly [number, number, number], q: 0 | 1 | 2 = 2): BufferGeometry {
  const [a, b, c] = d;
  const round = (r: number, lo: number, hi: number) => Math.round(Math.min(hi, Math.max(lo, r)));
  switch (s) {
    case "ball":
      return new SphereGeometry(a, [9, 14, 28][q]!, [6, 9, 18][q]!);
    case "cyl": {
      const r = Math.max(a, c);
      return new CylinderGeometry(a, c, b, q === 2 ? round(r * 520, 20, 48) : round(r * 260, q ? 10 : 7, q ? 22 : 14));
    }
    case "cap":
      return new CapsuleGeometry(a, b, [2, 3, 7][q]!, q === 2 ? round(a * 520, 18, 40) : round(a * 260, q ? 10 : 7, q ? 18 : 12));
    case "tire":
      return new LatheGeometry(tireProfile(a, b, c, q === 2), q === 2 ? round(a * 420, 32, 64) : q ? 18 : 12);
    case "ring":
      return new TorusGeometry(a, b, [3, 4, 8][q]!, q === 2 ? round(a * 600, 24, 72) : round(a * 220, q ? 12 : 8, q ? 24 : 16)).rotateX(Math.PI / 2);
    case "barrel":
      return new LatheGeometry(barrelProfile(a, b, c, [2, 3, 6][q]!), [5, 7, 12][q]!);
    default: {
      const r = Math.min(a, b, c) * 0.28;
      return r > 0.004 ? new RoundedBoxGeometry(a, b, c, [1, 2, 3][q]!, r) : new BoxGeometry(a, b, c);
    }
  }
}

const cache = new Map<string, BufferGeometry>();
/** Geometry for one solid seen close, made once. */
export function solid(s: Solid, d: readonly [number, number, number]) {
  const key = s + d.map((v) => v.toFixed(4)).join();
  let geo = cache.get(key);
  if (!geo) cache.set(key, (geo = shapeOf(s, d, 2)));
  return geo;
}
