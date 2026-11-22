import {
  BoxGeometry,
  Color,
  CylinderGeometry,
  MeshStandardMaterial,
  PMREMGenerator,
  PCFSoftShadowMap,
  SRGBColorSpace,
  ACESFilmicToneMapping,
  Scene,
  SphereGeometry,
  WebGLRenderer,
  type BufferGeometry,
} from "three";
import { RoomEnvironment } from "three/examples/jsm/environments/RoomEnvironment.js";
import { RoundedBoxGeometry } from "three/examples/jsm/geometries/RoundedBoxGeometry.js";
import type { Mat } from "./bodies";

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
  return {
    shell: m("#e4ece7", 0.36, 0.08),
    dark: m("#1b2621", 0.5, 0.45),
    steel: m("#9aa9a1", 0.28, 0.9),
    rubber: m("#0b0f0d", 0.86, 0),
    glow,
    amber: m("#fbbf24", 0.4, 0.2),
  };
}

const cache = new Map<string, BufferGeometry>();
/** Geometry for one solid. Boxes get a bevel that does not stretch with their proportions. */
export function solid(s: "box" | "cyl" | "ball", d: readonly [number, number, number]) {
  const key = s + d.map((v) => v.toFixed(4)).join();
  let geo = cache.get(key);
  if (!geo) {
    if (s === "ball") geo = new SphereGeometry(d[0], 24, 16);
    else if (s === "cyl") geo = new CylinderGeometry(d[0], d[2], d[1], 36);
    else {
      const r = Math.min(...d) * 0.28;
      geo = r > 0.004 ? new RoundedBoxGeometry(d[0], d[1], d[2], 3, r) : new BoxGeometry(...d);
    }
    cache.set(key, geo);
  }
  return geo;
}
