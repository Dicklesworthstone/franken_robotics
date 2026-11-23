import {
  AmbientLight,
  BoxGeometry,
  BufferGeometry,
  CanvasTexture,
  Color,
  CylinderGeometry,
  DirectionalLight,
  DynamicDrawUsage,
  Float32BufferAttribute,
  Fog,
  InstancedMesh,
  LineBasicMaterial,
  LineLoop,
  Mesh,
  MeshBasicMaterial,
  MeshStandardMaterial,
  PerspectiveCamera,
  Plane,
  PlaneGeometry,
  Raycaster,
  RepeatWrapping,
  SphereGeometry,
  Vector2,
  Vector3,
} from "three";
import { RoundedBoxGeometry } from "three/examples/jsm/geometries/RoundedBoxGeometry.js";
import { $, clamp, fit, fmt, loop, rng } from "./lib";
import { foot, reach } from "./bodies";
import { lit, renderer } from "./three-kit";

const SIDE = 16;
const COUNT = SIDE * SIDE;
const GAP = 1.9;
/** Everyone walks at the same speed, so the field keeps its shape while the floor goes by. */
const SPEED = 0.62;

interface Row {
  legs: number;
  femur: number;
  tibia: number;
  length: number;
  width: number;
  thick: number;
  period: number;
  duty: number;
  lift: number;
  hang: number;
  bend: number;
  wave: boolean;
}

function table(seed: number): Row[] {
  const u = rng(seed);
  return Array.from({ length: COUNT }, () => {
    const legs = [4, 6, 8][Math.floor(u() * 3)]!;
    const femur = 0.16 + u() * 0.22;
    const tibia = femur * (0.75 + u() * 0.65);
    const period = 0.48 + u() * 0.5;
    const duty = legs === 4 ? 0.52 + u() * 0.12 : 0.56 + u() * 0.16;
    return {
      legs,
      femur,
      tibia,
      length: 0.4 + u() * 0.28 + (legs - 4) * 0.11,
      width: 0.2 + u() * 0.2,
      thick: 0.75 + u() * 0.6,
      period,
      duty,
      lift: 0.05 + u() * 0.08,
      hang: (femur + tibia) * (0.68 + u() * 0.2),
      bend: u() < 0.5 ? 1 : -1,
      wave: legs > 4 && u() < 0.4,
    };
  });
}

export default function mount(root: HTMLElement) {
  const canvas = $<HTMLCanvasElement>(root, "canvas");
  const gl = renderer(canvas);
  if (!gl) return;
  const scene = lit(gl, 0.42);
  scene.fog = new Fog("#050d09", 13, 33);
  const camera = new PerspectiveCamera(32, 1, 0.1, 80);
  scene.add(new AmbientLight("#bfe8d0", 0.35));
  const key = new DirectionalLight("#fff3dc", 2.1);
  key.position.set(7, 14, 5);
  key.castShadow = true;
  key.shadow.mapSize.set(2048, 2048);
  const sc = key.shadow.camera;
  sc.left = sc.bottom = -18;
  sc.right = sc.top = 18;
  sc.far = 50;
  key.shadow.bias = -0.0008;
  key.shadow.normalBias = 0.03;
  scene.add(key);
  const rim = new DirectionalLight("#2dd4bf", 1.2);
  rim.position.set(-8, 4, -9);
  scene.add(rim);

  const c = document.createElement("canvas");
  c.width = c.height = 128;
  const g = c.getContext("2d")!;
  g.fillStyle = "#0a1a13";
  g.fillRect(0, 0, 128, 128);
  g.strokeStyle = "rgba(134,239,172,.16)";
  g.lineWidth = 2;
  g.strokeRect(0, 0, 128, 128);
  const grid = new CanvasTexture(c);
  grid.wrapS = grid.wrapT = RepeatWrapping;
  grid.repeat.set(90 / GAP, 90 / GAP);
  grid.anisotropy = 8;
  const floor = new Mesh(new PlaneGeometry(90, 90), new MeshStandardMaterial({ map: grid, color: "#7f9a8c", roughness: 1, metalness: 0, envMapIntensity: 0.25 }));
  floor.rotation.x = -Math.PI / 2;
  floor.receiveShadow = true;
  scene.add(floor);

  const max = COUNT * 8;
  const std = (roughness: number, metalness: number) => new MeshStandardMaterial({ roughness, metalness });
  const make = (geo: BufferGeometry, mat: MeshStandardMaterial | MeshBasicMaterial, n: number, shadow = true) => {
    const m = new InstancedMesh(geo, mat, n);
    m.instanceMatrix.setUsage(DynamicDrawUsage);
    m.frustumCulled = false;
    m.castShadow = shadow;
    m.receiveShadow = shadow;
    scene.add(m);
    return m;
  };
  const trunks = make(new RoundedBoxGeometry(1, 1, 1, 2, 0.16), std(0.4, 0.1), COUNT * 2);
  // Limbs taper from the joint above to the joint below.
  const limbs = make(new CylinderGeometry(0.5, 0.34, 1, 12), std(0.45, 0.2), max * 2);
  const heads = make(new RoundedBoxGeometry(1, 1, 1, 2, 0.2), std(0.5, 0.35), COUNT);
  const hubs = make(new CylinderGeometry(1, 1, 1, 18).rotateX(Math.PI / 2), std(0.35, 0.6), max * 2);
  const feet = make(new SphereGeometry(1, 12, 8), std(0.9, 0), max, false);
  const lamps = make(new BoxGeometry(1, 1, 1), new MeshBasicMaterial(), COUNT, false);
  lamps.castShadow = false;

  let rows = table(14);
  const hue = { 4: new Color("#4ade80"), 6: new Color("#6cc8f7"), 8: new Color("#fbbf24") } as Record<number, Color>;
  const paint = () => {
    const shell = new Color();
    const dark = new Color("#22302a");
    const steel = new Color("#8fa097");
    let leg = 0;
    for (const [i, r] of rows.entries()) {
      shell.set("#dfe8e2").lerp(hue[r.legs]!, 0.28);
      trunks.setColorAt(i * 2, shell);
      trunks.setColorAt(i * 2 + 1, dark);
      heads.setColorAt(i, dark);
      lamps.setColorAt(i, hue[r.legs]!);
      for (let l = 0; l < r.legs; l++, leg++) {
        limbs.setColorAt(leg * 2, shell);
        limbs.setColorAt(leg * 2 + 1, dark);
        hubs.setColorAt(leg * 2, steel);
        hubs.setColorAt(leg * 2 + 1, steel);
        feet.setColorAt(leg, dark);
      }
    }
    limbs.count = leg * 2;
    hubs.count = leg * 2;
    feet.count = leg;
    for (const m of [trunks, heads, limbs, hubs, feet, lamps]) m.instanceColor!.needsUpdate = true;
    $(root, "[data-links]").textContent = fmt(COUNT + leg * 3);
  };

  // Every part turns about the z axis only, so its matrix is written straight into the buffer.
  const put = (m: InstancedMesh, i: number, x: number, y: number, z: number, a: number, sx: number, sy: number, sz: number) => {
    const e = m.instanceMatrix.array as Float32Array;
    const o = i * 16;
    const cos = Math.cos(a);
    const sin = Math.sin(a);
    e[o] = cos * sx;
    e[o + 1] = sin * sx;
    e[o + 2] = 0;
    e[o + 3] = 0;
    e[o + 4] = -sin * sy;
    e[o + 5] = cos * sy;
    e[o + 6] = 0;
    e[o + 7] = 0;
    e[o + 8] = 0;
    e[o + 9] = 0;
    e[o + 10] = sz;
    e[o + 11] = 0;
    e[o + 12] = x;
    e[o + 13] = y;
    e[o + 14] = z;
    e[o + 15] = 1;
  };
  const gait = { period: 1, duty: 0.5, stride: 0, lift: 0 };
  const pose = (t: number) => {
    let leg = 0;
    for (let i = 0; i < COUNT; i++) {
      const r = rows[i]!;
      const bx = (Math.floor(i / SIDE) - (SIDE - 1) / 2) * GAP;
      const bz = ((i % SIDE) - (SIDE - 1) / 2) * GAP;
      // Highest over the middle of each stance, in step with this robot's own feet.
      const bob = r.lift * 0.1 * Math.cos(4 * Math.PI * (t / r.period + i * 0.137 - r.duty / 2));
      const by = r.hang + 0.03 + bob;
      put(trunks, i * 2, bx, by + 0.02, bz, 0, r.length, 0.13 * r.thick, r.width);
      put(trunks, i * 2 + 1, bx, by + 0.1 * r.thick, bz, 0, r.length * 0.74, 0.05, r.width * 0.72);
      put(heads, i, bx + r.length / 2 + 0.04, by + 0.03, bz, 0, 0.1, 0.1 * r.thick, r.width * 0.66);
      put(lamps, i, bx + r.length / 2 + 0.092, by + 0.045, bz, 0, 0.012, 0.022, r.width * 0.5);
      gait.period = r.period;
      gait.duty = r.duty;
      gait.stride = SPEED * r.duty * r.period;
      gait.lift = r.lift;
      const pairs = r.legs / 2;
      for (let p = 0; p < pairs; p++) {
        const hx = bx + r.length * (0.43 - (0.86 * p) / (pairs - 1));
        for (let s = -1; s <= 1; s += 2, leg++) {
          const phase = r.wave ? p / pairs + (s > 0 ? 0.5 : 0) : ((p + (s > 0 ? 1 : 0)) % 2) * 0.5;
          const [fx, up] = foot(t / r.period + phase + i * 0.137, gait);
          const bend = (pairs === 2 && p ? -r.bend : r.bend) as 1 | -1;
          const [h, k] = reach(r.femur, r.tibia, fx, -r.hang + up - bob, bend);
          const hz = bz + s * (r.width / 2 + 0.05 * r.thick);
          const a1 = h;
          const kx = hx + Math.sin(a1) * r.femur;
          const ky = by - Math.cos(a1) * r.femur;
          const a2 = h + k;
          const tx = kx + Math.sin(a2) * r.tibia;
          const ty = ky - Math.cos(a2) * r.tibia;
          const w = 0.05 * r.thick;
          put(hubs, leg * 2, hx, by, hz, 0, w * 1.05, w * 1.05, w * 1.5);
          put(hubs, leg * 2 + 1, kx, ky, hz, 0, w * 0.72, w * 0.72, w * 1.2);
          put(limbs, leg * 2, (hx + kx) / 2, (by + ky) / 2, hz, a1, w * 1.1, r.femur, w * 0.85);
          put(limbs, leg * 2 + 1, (kx + tx) / 2, (ky + ty) / 2, hz, a2, w * 0.62, r.tibia, w * 0.6);
          put(feet, leg, tx, ty, hz, 0, w * 0.55, w * 0.55, w * 0.55);
        }
      }
    }
    for (const m of [trunks, heads, limbs, hubs, feet, lamps]) m.instanceMatrix.needsUpdate = true;
  };

  const ring = new LineLoop(
    new BufferGeometry().setAttribute(
      "position",
      new Float32BufferAttribute(
        Array.from({ length: 64 }, (_, i) => [Math.cos((i / 32) * Math.PI) * 0.8, 0.02, Math.sin((i / 32) * Math.PI) * 0.8]).flat(),
        3,
      ),
    ),
    new LineBasicMaterial({ color: "#c6f25a" }),
  );
  ring.visible = false;
  scene.add(ring);

  let clock = 0;
  let drift = 0.2;
  const draw = (dt: number) => {
    if (view.running) {
      clock += dt;
      drift += dt * 0.045;
    }
    pose(clock);
    grid.offset.x = (grid.offset.x + (view.running ? (dt * SPEED) / GAP : 0)) % 1;
    const a = -0.72 + Math.sin(drift) * 0.3;
    const far = camera.aspect < 1 ? 25 : 19;
    camera.position.set(Math.sin(a) * far, 9.5 + Math.sin(drift * 0.7) * 1.2, Math.cos(a) * far);
    camera.lookAt(0, 0.35, 0);
    gl.render(scene, camera);
  };
  const view = loop(canvas, draw);
  fit(
    canvas,
    (w, h, ratio) => {
      gl.setPixelRatio(ratio);
      gl.setSize(w, h, false);
      camera.aspect = w / h;
      camera.updateProjectionMatrix();
      view.poke();
    },
    1.75,
  );

  const tip = $(root, "[data-tip]");
  const ray = new Raycaster();
  const ground = new Plane(new Vector3(0, 1, 0), -0.2);
  const hit = new Vector3();
  const over = (e: PointerEvent) => {
    const box = canvas.getBoundingClientRect();
    const x = e.clientX - box.left;
    const y = e.clientY - box.top;
    ray.setFromCamera(new Vector2((x / box.width) * 2 - 1, 1 - (y / box.height) * 2), camera);
    const at = ray.ray.intersectPlane(ground, hit);
    const r = at ? Math.round(hit.x / GAP + (SIDE - 1) / 2) : -1;
    const col = at ? Math.round(hit.z / GAP + (SIDE - 1) / 2) : -1;
    const inside = r >= 0 && r < SIDE && col >= 0 && col < SIDE;
    ring.visible = inside;
    tip.hidden = !inside;
    if (!inside) return view.poke();
    const i = r * SIDE + col;
    const b = rows[i]!;
    ring.position.set((r - (SIDE - 1) / 2) * GAP, 0, (col - (SIDE - 1) / 2) * GAP);
    tip.textContent = `row ${i}\nlegs   ${b.legs}\nfemur  ${b.femur.toFixed(3)} m\ntibia  ${b.tibia.toFixed(3)} m\nperiod ${b.period.toFixed(2)} s`;
    tip.style.left = `${clamp(x + 16, 8, box.width - tip.offsetWidth - 8)}px`;
    tip.style.top = `${clamp(y - tip.offsetHeight - 12, 8, box.height - tip.offsetHeight - 8)}px`;
    view.poke();
  };
  canvas.addEventListener("pointermove", over);
  canvas.addEventListener("pointerdown", over);
  canvas.addEventListener("pointerleave", () => {
    ring.visible = false;
    tip.hidden = true;
    view.poke();
  });
  let seed = 14;
  $(root, "[data-seed]").addEventListener("click", () => {
    rows = table(++seed);
    paint();
    view.poke();
  });
  paint();
}
