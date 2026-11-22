import {
  AdditiveBlending,
  AmbientLight,
  BoxGeometry,
  type BufferGeometry,
  CanvasTexture,
  Color,
  CylinderGeometry,
  DirectionalLight,
  DynamicDrawUsage,
  Float32BufferAttribute,
  Fog,
  Group,
  InstancedMesh,
  Matrix4,
  Mesh,
  MeshBasicMaterial,
  MeshStandardMaterial,
  PerspectiveCamera,
  Plane,
  PlaneGeometry,
  Raycaster,
  RingGeometry,
  SphereGeometry,
  Vector2,
  Vector3,
} from "three";
import { RoundedBoxGeometry } from "three/examples/jsm/geometries/RoundedBoxGeometry.js";
import { mergeGeometries } from "three/examples/jsm/utils/BufferGeometryUtils.js";
import { archetypes } from "./archetypes";
import { makers, type Mat, type Part } from "./bodies";
import { envelope, frames, verdict, type Envelope, type Place, type Verdict } from "./envelope";
import { $, clamp, fit, fmt, loop, mix, range, reduced, rng, seg, smooth } from "./lib";
import { lit, renderer } from "./three-kit";

/** How the crowd stands: rows run away from the camera, columns across it. */
const ROWS = 11;
const COLS = 9;
const GAP_X = 1.72;
const GAP_Z = 1.62;

export const PLACES: Record<string, Place> = {
  hall: { width: 1.2, height: 2, circle: 2 },
  door: { width: 0.6, height: 1.7, circle: 1.2 },
  shelf: { width: 1.1, height: 0.42, circle: 1.5 },
  landing: { width: 0.9, height: 1.4, circle: 0.86 },
};
const HUE: Record<Verdict, string> = { fits: "#4ade80", wide: "#fbbf24", tall: "#6cc8f7", long: "#fb7185" };

/** The front rows are cast, so that the bodies a reader meets first are plainly different machines. */
const CAST = [
  ["quad-mammal", 1.02], ["humanoid", 0.86], ["hexapod-radial", 0.96], ["wheel-leg", 1.02], ["tracked-flipper", 0.9], ["rocker-bogie", 0.92], ["mobile-manipulator", 0.86], ["balancer-2w", 1.02], ["quad-sprawl", 0.8],
  ["biped-point", 1], ["swerve-4", 1.05], ["quad-mammal", 0.8], ["octopod", 0.92], ["tri-wheel-stair", 0.96], ["ackermann", 1.1], ["humanoid", 0.7], ["mecanum", 1.05], ["diff-drive", 1.1],
] as const;

export interface Member {
  id: string;
  scale: number;
  x: number;
  z: number;
  beat: number;
}
/** The crowd is the same on every visit. */
export function crowd(rows = ROWS, cols = COLS): Member[] {
  const u = rng(1818);
  const ids = archetypes.map((a) => a.id);
  const list: Member[] = [];
  for (let r = 0; r < rows; r++)
    for (let c = 0; c < cols; c++) {
      const cast = CAST[r * cols + c];
      let id: string = cast?.[0] ?? "";
      if (!id) {
        const near = [list[list.length - 1]?.id, list[list.length - cols]?.id];
        do id = ids[Math.floor(u() * ids.length)]!;
        while (near.includes(id) || (id === "snake-crawler" && u() < 0.7));
      }
      const scale = cast?.[1] ?? (id === "snake-crawler" ? 0.55 + u() * 0.2 : 0.62 + u() * 0.62);
      list.push({
        id,
        scale,
        x: -r * GAP_X + (u() - 0.5) * 0.3,
        z: (c - (cols - 1) / 2 + (r % 2 ? 0.5 : 0)) * GAP_Z + (u() - 0.5) * 0.3,
        beat: u() * 5,
      });
    }
  return list;
}

const PAINT: Record<Mat, string> = {
  shell: "#e9f0ec",
  dark: "#202c26",
  steel: "#9dada4",
  rubber: "#0d1210",
  glow: "#ffffff",
  amber: "#fbbf24",
};
function shape(p: Part, fine: boolean): BufferGeometry {
  const [a, b, c] = p.d;
  let geo: BufferGeometry;
  if (p.s === "ball") geo = new SphereGeometry(a, fine ? 16 : 10, fine ? 10 : 7);
  else if (p.s === "cyl") geo = new CylinderGeometry(a, c, b, fine ? 22 : 14);
  else {
    const r = Math.min(a, b, c) * 0.26;
    geo = r > 0.005 ? new RoundedBoxGeometry(a, b, c, fine ? 2 : 1, r) : new BoxGeometry(a, b, c);
  }
  const flat = geo.index ? geo.toNonIndexed() : geo;
  flat.deleteAttribute("uv");
  const own = new Group();
  own.position.set(...p.p);
  if (p.r) own.rotation.set(...p.r);
  own.updateMatrix();
  flat.applyMatrix4(own.matrix);
  const tint = new Color(PAINT[p.m]);
  const n = flat.getAttribute("position").count;
  flat.setAttribute(
    "color",
    new Float32BufferAttribute(Array.from({ length: n }, () => [tint.r, tint.g, tint.b]).flat(), 3),
  );
  return flat;
}

function glowDisc() {
  const c = document.createElement("canvas");
  c.width = c.height = 128;
  const g = c.getContext("2d")!;
  const grad = g.createRadialGradient(64, 64, 2, 64, 64, 64);
  grad.addColorStop(0, "rgba(255,255,255,.95)");
  grad.addColorStop(0.35, "rgba(255,255,255,.34)");
  grad.addColorStop(1, "rgba(255,255,255,0)");
  g.fillStyle = grad;
  g.fillRect(0, 0, 128, 128);
  return new CanvasTexture(c);
}

export default function mount(root: HTMLElement) {
  const canvas = $<HTMLCanvasElement>(root, "canvas");
  const small = innerWidth < 760;
  const gl = renderer(canvas);
  if (!gl) return;
  gl.shadowMap.enabled = !small;
  const scene = lit(gl, 0.2);
  scene.fog = new Fog("#030705", 10, 28);
  const camera = new PerspectiveCamera(27, 1, 0.1, 60);
  scene.add(new AmbientLight("#bfe8d0", 0.16));
  const key = new DirectionalLight("#fff1dc", 2.5);
  key.position.set(5, 8, 7);
  key.target.position.set(-3, 0, 0);
  key.castShadow = !small;
  key.shadow.mapSize.set(2048, 2048);
  key.shadow.radius = 4;
  key.shadow.bias = -0.0006;
  key.shadow.normalBias = 0.02;
  const sc = key.shadow.camera;
  sc.left = sc.bottom = -11;
  sc.right = sc.top = 11;
  sc.near = 1;
  sc.far = 30;
  scene.add(key, key.target);
  const rim = new DirectionalLight("#5eead4", 2.9);
  rim.position.set(-9, 4.5, -5);
  scene.add(rim);

  const floor = new Mesh(
    new PlaneGeometry(90, 90),
    new MeshStandardMaterial({ color: "#060c09", roughness: 1, metalness: 0, envMapIntensity: 0.1 }),
  );
  floor.rotation.x = -Math.PI / 2;
  floor.receiveShadow = true;
  scene.add(floor);

  // Every link shape is drawn once, instanced over every body that has it.
  const members = small ? crowd(8, 7) : crowd();
  const kinds = new Map(archetypes.map((a) => [a.id, makers[a.id]!()]));
  const sizes = new Map<string, Envelope>([...kinds].map(([id, b]) => [id, envelope(b)]));
  const bodies = members.map((m) => {
    const body = kinds.get(m.id)!;
    return {
      ...m,
      body,
      size: sizes.get(m.id)!,
      frame: frames(body),
      clock: m.beat,
      root: new Matrix4().compose(
        new Vector3(m.x, 0, m.z),
        new Group().quaternion.setFromAxisAngle(new Vector3(0, 1, 0), (m.beat - 2.5) * 0.05),
        new Vector3(m.scale, m.scale, m.scale),
      ),
      says: "fits" as Verdict,
      next: "fits" as Verdict,
      flip: 0,
      lit: 1,
      drawn: -1,
      hue: new Color(HUE.fits),
      slots: [] as [mesh: InstancedMesh, slot: number, link: number, lamp: boolean][],
    };
  });
  const groups = new Map<string, { parts: Part[]; lamp: boolean; users: [number, number][] }>();
  for (const [b, one] of bodies.entries())
    for (const [l, link] of one.body.links.entries())
      for (const lamp of [false, true]) {
        const parts = link.parts.filter((p) => (p.m === "glow") === lamp);
        if (!parts.length) continue;
        const sign = JSON.stringify([lamp, parts]);
        let group = groups.get(sign);
        if (!group) groups.set(sign, (group = { parts, lamp, users: [] }));
        group.users.push([b, l]);
      }
  const plain = new MeshStandardMaterial({ vertexColors: true, roughness: 0.42, metalness: 0.12 });
  const bright = new MeshBasicMaterial({ toneMapped: false });
  const meshes: InstancedMesh[] = [];
  for (const { parts, lamp, users } of groups.values()) {
    const geo = mergeGeometries(parts.map((p) => shape(p, !small)))!;
    const mesh = new InstancedMesh(geo, lamp ? bright : plain, users.length);
    mesh.instanceMatrix.setUsage(DynamicDrawUsage);
    mesh.frustumCulled = false;
    mesh.castShadow = !small && !lamp;
    mesh.receiveShadow = !small && !lamp;
    scene.add(mesh);
    meshes.push(mesh);
    for (const [slot, [b, l]] of users.entries()) bodies[b]!.slots.push([mesh, slot, l, lamp]);
  }

  // A pool of light under each body that fits.
  const pools = new InstancedMesh(
    new PlaneGeometry(1, 1).rotateX(-Math.PI / 2),
    new MeshBasicMaterial({ map: glowDisc(), blending: AdditiveBlending, transparent: true, depthWrite: false, fog: true }),
    bodies.length,
  );
  pools.frustumCulled = false;
  const poolAt = new Matrix4();
  for (const [i, b] of bodies.entries()) {
    const d = b.size.circle * b.scale * 1.75;
    pools.setMatrixAt(i, poolAt.makeScale(d, 1, d).setPosition(b.x, 0.012 + (i % 7) * 0.0006, b.z));
    pools.setColorAt(i, new Color(0, 0, 0));
  }
  scene.add(pools);

  // The line that crosses the floor when the place changes.
  const sweep = new Mesh(
    new PlaneGeometry(1.5, 60).rotateX(-Math.PI / 2),
    new MeshBasicMaterial({ map: glowDisc(), color: "#4ade80", blending: AdditiveBlending, transparent: true, depthWrite: false, opacity: 0 }),
  );
  sweep.position.y = 0.02;
  scene.add(sweep);

  // The doorway itself, at its real size in front of the crowd, with the clear floor drawn before it.
  const wire = new MeshBasicMaterial({ color: "#7df0a8", toneMapped: false, transparent: true, opacity: 0.95 });
  const post = () => {
    const m = new Mesh(new BoxGeometry(0.02, 1, 0.02), wire);
    gate.add(m);
    return m;
  };
  const gate = new Group();
  const left = post();
  const right = post();
  const lintel = new Mesh(new BoxGeometry(0.02, 0.02, 1), wire);
  const ring = new Mesh(
    new RingGeometry(0.488, 0.5, 72).rotateX(-Math.PI / 2),
    new MeshBasicMaterial({ color: "#7df0a8", toneMapped: false, transparent: true, opacity: 0.6, depthWrite: false }),
  );
  ring.position.set(0.95, 0.016, 0);
  gate.add(lintel, ring);
  gate.position.set(small ? 1.3 : 1.25, 0, small ? 0.2 : 0.95);
  scene.add(gate);

  const place: Place = { ...PLACES.door! };
  const drawn: Place = { ...place };
  let now = performance.now();
  let began = -1e9;
  const FRONT = 1.2;
  const BACK = -(small ? 8 : ROWS) * GAP_X;
  const PACE = 10.5;

  const counts = { fits: $(root, "[data-n=fits]"), wide: $(root, "[data-n=wide]"), tall: $(root, "[data-n=tall]"), long: $(root, "[data-n=long]") };
  const bars = { fits: $(root, "[data-bar=fits]"), wide: $(root, "[data-bar=wide]"), tall: $(root, "[data-bar=tall]"), long: $(root, "[data-bar=long]") };
  $(root, "[data-total]").textContent = String(bodies.length);
  const judge = (wave: boolean) => {
    const n: Record<Verdict, number> = { fits: 0, wide: 0, tall: 0, long: 0 };
    began = wave && !reduced ? now : began;
    for (const b of bodies) {
      b.next = verdict(b.size, b.scale, place);
      n[b.next]++;
      b.flip = wave && !reduced ? now + ((FRONT - b.x) / PACE) * 1000 : now;
    }
    for (const k of Object.keys(n) as Verdict[]) {
      counts[k].textContent = String(n[k]);
      bars[k].style.flexGrow = String(n[k]);
    }
    view.poke();
  };

  const ink = new Color();
  const DARK = new Color("#0b1410");
  const lamp = new Color();
  const off = new Color();
  const paint = (i: number) => {
    const b = bodies[i]!;
    const k = smooth(b.lit);
    ink.set("#ffffff").lerp(off.set(DARK).lerp(lamp.copy(b.hue), 0.022), 1 - k);
    lamp.copy(b.hue).multiplyScalar(mix(0.5, 1.3, k));
    for (const [mesh, slot, , isLamp] of b.slots) {
      mesh.setColorAt(slot, isLamp ? lamp : ink);
      mesh.instanceColor!.needsUpdate = true;
    }
    pools.setColorAt(i, off.set(HUE.fits).multiplyScalar(0.62 * k));
    pools.instanceColor!.needsUpdate = true;
    b.drawn = b.lit;
  };

  let drift = 0;
  const aim = new Vector3();
  const look = { x: 0, y: 0, tx: 0, ty: 0 };
  let W = 1;
  let H = 1;
  const draw = (dt: number, t: number) => {
    now = t;
    const moving = view.running;
    for (const k of ["width", "height", "circle"] as const)
      drawn[k] = reduced ? place[k] : mix(drawn[k], place[k], 1 - Math.exp(-dt * 7));
    left.scale.y = right.scale.y = drawn.height;
    left.position.set(0, drawn.height / 2, -drawn.width / 2);
    right.position.set(0, drawn.height / 2, drawn.width / 2);
    lintel.scale.z = drawn.width + 0.02;
    lintel.position.y = drawn.height;
    ring.scale.setScalar(drawn.circle);

    const age = (t - began) / 1000;
    const at = FRONT - age * PACE;
    (sweep.material as MeshBasicMaterial).opacity = age >= 0 && at > BACK ? 0.5 * smooth((at - BACK) / 6) : 0;
    sweep.position.x = at;

    for (const [i, b] of bodies.entries()) {
      if (b.says !== b.next && t >= b.flip) {
        b.says = b.next;
        b.drawn = -1;
      }
      const want = b.says === "fits" ? 1 : 0;
      b.lit = reduced ? want : b.lit + clamp(want - b.lit, -dt * 3.4, dt * 3.4);
      b.hue.lerp(off.set(HUE[b.says]), reduced ? 1 : 1 - Math.exp(-dt * 9));
      if (moving) b.clock += dt * smooth(b.lit);
      const world = b.frame.pose(b.clock, b.root);
      for (const [mesh, slot, link] of b.slots) world[link]!.toArray(mesh.instanceMatrix.array, slot * 16);
      if (b.drawn !== b.lit || (b.lit > 0 && b.lit < 1) || b.says !== "fits") paint(i);
    }
    for (const mesh of meshes) mesh.instanceMatrix.needsUpdate = true;

    if (moving) drift += dt;
    look.x = mix(look.x, look.tx, 1 - Math.exp(-dt * 3));
    look.y = mix(look.y, look.ty, 1 - Math.exp(-dt * 3));
    const tall = W / H < 1.05;
    const a = (tall ? 0.46 : 0.5) + Math.sin(drift * 0.11) * 0.07 + look.x * 0.07;
    const far = tall ? 10.8 : 9.7;
    camera.position.set(Math.cos(a) * far, 2.75 + Math.sin(drift * 0.17) * 0.16 - look.y * 0.3, Math.sin(a) * far);
    camera.lookAt(aim.set(-1.6, tall ? 0.15 : 0.2, tall ? 0.1 : -0.15));
    gl.render(scene, camera);
    if (!root.dataset.live) root.dataset.live = "";
  };
  const view = loop(canvas, draw);
  fit(
    canvas,
    (w, h, ratio) => {
      W = w;
      H = h;
      gl.setPixelRatio(ratio);
      gl.setSize(w, h, false);
      camera.aspect = w / h;
      camera.fov = w / h < 1.05 ? 34 : 27;
      // On a wide screen the crowd stands to the right of the headline.
      if (w > 1000) camera.setViewOffset(w, h, -w * 0.2, h * 0.07, w, h);
      else camera.clearViewOffset();
      camera.updateProjectionMatrix();
      view.poke();
    },
    small ? 1.6 : 1.75,
  );

  // Controls. The tour of places stops at the first touch.
  const metres = (v: number) => `${fmt(v, 2)} m`;
  let touring = !reduced;
  const stop = () => {
    touring = false;
    where.set("");
  };
  const change = (k: keyof Place) => (v: number) => {
    stop();
    place[k] = v;
    judge(false);
  };
  const sliders = {
    width: range(root, "width", metres, change("width")),
    height: range(root, "height", metres, change("height")),
    circle: range(root, "circle", metres, change("circle")),
  };
  const go = (id: string, wave = true) => {
    Object.assign(place, PLACES[id]);
    for (const k of ["width", "height", "circle"] as const) sliders[k].set(place[k]);
    judge(wave);
  };
  const where = seg(root, "place", (id) => {
    touring = false;
    go(id);
  });
  const order = ["door", "shelf", "landing", "hall"];
  let step = 0;
  setInterval(() => {
    if (!touring || !view.running || document.hidden || scrollY > innerHeight * 0.7) return;
    step = (step + 1) % order.length;
    where.set(order[step]!);
    go(order[step]!);
  }, 6200);

  const pause = $<HTMLButtonElement>(root, "[data-pause]");
  pause.setAttribute("aria-pressed", String(!view.running));
  pause.addEventListener("click", () => {
    view.running = !view.running;
    touring &&= view.running;
    pause.setAttribute("aria-pressed", String(!view.running));
    pause.setAttribute("aria-label", view.running ? "Pause" : "Play");
  });

  // What a body is, and what stops it.
  const tip = $(root, "[data-tip]");
  const halo = new Mesh(
    new RingGeometry(0.47, 0.5, 64).rotateX(-Math.PI / 2),
    new MeshBasicMaterial({ color: "#ffffff", transparent: true, opacity: 0.85, depthWrite: false, toneMapped: false }),
  );
  halo.visible = false;
  halo.position.y = 0.03;
  scene.add(halo);
  const ray = new Raycaster();
  const ground = new Plane(new Vector3(0, 1, 0), -0.28);
  const hit = new Vector3();
  const reason: Record<Verdict, (b: (typeof bodies)[number]) => string> = {
    fits: () => "fits",
    wide: (b) => `too wide: ${metres(b.size.width * b.scale)} in a ${metres(place.width)} door`,
    tall: (b) => `too tall: ${metres(b.size.height * b.scale)} under ${metres(place.height)}`,
    long: (b) => `too long: needs ${metres(b.size.circle * b.scale)} of clear floor`,
  };
  const over = (e: PointerEvent) => {
    const box = canvas.getBoundingClientRect();
    const x = e.clientX - box.left;
    const y = e.clientY - box.top;
    if (e.pointerType !== "touch") {
      look.tx = (x / box.width) * 2 - 1;
      look.ty = (y / box.height) * 2 - 1;
    }
    ray.setFromCamera(new Vector2((x / box.width) * 2 - 1, 1 - (y / box.height) * 2), camera);
    const at = ray.ray.intersectPlane(ground, hit);
    let best: (typeof bodies)[number] | undefined;
    let near = 0.62;
    if (at && camera.position.distanceTo(hit) < 19)
      for (const b of bodies) {
        const d = Math.hypot(b.x - hit.x, b.z - hit.z);
        if (d < near) {
          near = d;
          best = b;
        }
      }
    halo.visible = !!best;
    tip.hidden = !best;
    if (best) {
      halo.position.set(best.x, 0.03, best.z);
      halo.scale.setScalar(best.size.circle * best.scale);
      (halo.material as MeshBasicMaterial).color.set(HUE[best.next]);
      tip.innerHTML = `<b>${best.id}</b><span class="${best.next}">${reason[best.next](best)}</span>`;
      tip.style.left = `${clamp(x + 18, 8, box.width - tip.offsetWidth - 8)}px`;
      tip.style.top = `${clamp(y - tip.offsetHeight - 14, 8, box.height - tip.offsetHeight - 8)}px`;
    }
    view.poke();
  };
  canvas.addEventListener("pointermove", over);
  canvas.addEventListener("pointerdown", over);
  canvas.addEventListener("pointerleave", () => {
    halo.visible = false;
    tip.hidden = true;
    look.tx = look.ty = 0;
    view.poke();
  });

  go("door", false);
  for (const [i, b] of bodies.entries()) {
    b.says = b.next;
    b.lit = b.says === "fits" ? 1 : 0;
    b.hue.set(HUE[b.says]);
    paint(i);
  }
}
