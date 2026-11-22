import {
  AmbientLight,
  CanvasTexture,
  DirectionalLight,
  Group,
  Mesh,
  MeshBasicMaterial,
  PerspectiveCamera,
  PlaneGeometry,
  Quaternion,
  RepeatWrapping,
  ShadowMaterial,
  Euler,
  Vector3,
} from "three";
import { $, $$, clamp, fit, loop, mix, reduced, smooth } from "./lib";
import { envelope, type Envelope } from "./envelope";
import { archetypes } from "./archetypes";
import { makers, type Body, type Link } from "./bodies";
import { lit, materials, renderer, solid } from "./three-kit";

const AXIS = { x: new Vector3(1, 0, 0), y: new Vector3(0, 1, 0), z: new Vector3(0, 0, 1) };

function gridTexture() {
  const c = document.createElement("canvas");
  c.width = c.height = 256;
  const g = c.getContext("2d")!;
  g.strokeStyle = "rgba(134,239,172,.55)";
  g.lineWidth = 2;
  g.strokeRect(0, 0, 256, 256);
  g.strokeStyle = "rgba(134,239,172,.16)";
  g.lineWidth = 1;
  for (const v of [64, 128, 192]) {
    g.beginPath();
    g.moveTo(v, 0);
    g.lineTo(v, 256);
    g.moveTo(0, v);
    g.lineTo(256, v);
    g.stroke();
  }
  const t = new CanvasTexture(c);
  t.wrapS = t.wrapT = RepeatWrapping;
  t.repeat.set(28, 28);
  t.anisotropy = 8;
  return t;
}
function fadeTexture() {
  const c = document.createElement("canvas");
  c.width = c.height = 256;
  const g = c.getContext("2d")!;
  const grad = g.createRadialGradient(128, 128, 8, 128, 128, 128);
  grad.addColorStop(0, "#fff");
  grad.addColorStop(0.45, "#666");
  grad.addColorStop(1, "#000");
  g.fillStyle = grad;
  g.fillRect(0, 0, 256, 256);
  return new CanvasTexture(c);
}

export default function mount(root: HTMLElement) {
  const canvas = $<HTMLCanvasElement>(root, ".stage-view canvas");
  const gl = renderer(canvas);
  if (!gl) return;
  const scene = lit(gl);
  const camera = new PerspectiveCamera(30, 1, 0.05, 60);
  const mats = materials();
  scene.add(new AmbientLight("#bfe8d0", 0.25));
  const key = new DirectionalLight("#fff6e6", 2.6);
  key.castShadow = true;
  key.shadow.mapSize.set(2048, 2048);
  key.shadow.radius = 5;
  key.shadow.bias = -0.0004;
  key.shadow.normalBias = 0.012;
  scene.add(key, key.target);
  const rim = new DirectionalLight("#4ade80", 1.7);
  rim.position.set(-3, 1.6, -2.6);
  scene.add(rim);
  const grid = gridTexture();
  const floor = new Mesh(
    new PlaneGeometry(14, 14),
    new MeshBasicMaterial({ map: grid, alphaMap: fadeTexture(), transparent: true, depthWrite: false, opacity: 0.5 }),
  );
  floor.rotation.x = -Math.PI / 2;
  const catcher = new Mesh(new PlaneGeometry(14, 14), new ShadowMaterial({ opacity: 0.6 }));
  catcher.rotation.x = -Math.PI / 2;
  catcher.position.y = 0.001;
  catcher.receiveShadow = true;
  scene.add(floor, catcher);

  interface Posed {
    link: Link;
    node: Group;
    set: Quaternion;
    burst: Vector3;
  }
  let body: Body;
  let posed: Posed[] = [];
  let rootNode: Group | undefined;
  let index = 0;
  let born = 0;
  let clock = 0;
  let size = 1;
  let hull: Envelope = { width: 1, height: 1, circle: 1 };
  let span = 1;
  let tall = 1;
  const swing = new Quaternion();

  const build = (i: number) => {
    index = i;
    if (rootNode) scene.remove(rootNode);
    body = makers[archetypes[i]!.id]!();
    hull = envelope(body);
    posed = [];
    for (const link of body.links) {
      const node = new Group();
      const set = new Quaternion();
      if (link.set) set.setFromEuler(new Euler(...link.set));
      for (const part of link.parts) {
        const mesh = new Mesh(solid(part.s, part.d), mats[part.m]);
        mesh.position.set(...part.p);
        if (part.r) mesh.rotation.set(...part.r);
        mesh.castShadow = true;
        mesh.receiveShadow = part.m !== "glow";
        node.add(mesh);
      }
      const [x, y, z] = link.at;
      const burst = new Vector3(x, y + 0.02, z).normalize().multiplyScalar(link.parent < 0 ? 0 : 0.16);
      posed.push({ link, node, set, burst });
      if (link.parent < 0) rootNode = node;
      else posed[link.parent]!.node.add(node);
    }
    scene.add(rootNode!);
    born = performance.now();
    const a = archetypes[i]!;
    $(root, "[data-no]").textContent = `Body ${i + 1} of ${archetypes.length}`;
    $(root, "[data-name]").textContent = a.name;
    $(root, "[data-layout]").textContent = a.layout;
    $(root, "[data-tests]").textContent = a.tests;
    $(root, "[data-dof]").textContent = String(a.dof);
    $(root, "[data-sym]").textContent = a.symmetry;
    $(root, "[data-links]").textContent = String(body.links.length);
    const num = (v: number) => (Math.abs(v) < 5e-4 ? "0" : v.toFixed(3).replace("-", "−"));
    $(root, "[data-rows]").innerHTML = body.links
      .map((l) => {
        const active = l.joint !== "free" && l.joint !== "fixed" && !l.passive;
        const name = l.joint === "free" || l.joint === "fixed" ? l.joint : `turn ${l.joint}${l.passive ? ", passive" : ""}`;
        return `<tr><td>${l.name}</td><td>${l.parent < 0 ? "world" : body.links[l.parent]!.name}</td><td${active ? ' class="act"' : ""}>${name}</td><td>${l.at.map(num).join(" ")}</td></tr>`;
      })
      .join("");
    $(root, ".datawrap").scrollTop = 0;
  };

  const pose = (t: number, open: number) => {
    const off = body.base?.(t) ?? {};
    for (const [i, { link, node, set, burst }] of posed.entries()) {
      node.position.set(...link.at);
      if (link.joint === "free") {
        node.position.set(off.x ?? 0, body.height + (off.y ?? 0) + (1 - open) * 0.25, off.z ?? 0);
        node.rotation.set(off.roll ?? 0, off.yaw ?? 0, off.pitch ?? 0);
        continue;
      }
      const k = smooth(clamp(open * 1.9 - (i / posed.length) * 0.9, 0, 1));
      if (k < 1) node.position.addScaledVector(burst, 1 - k);
      node.scale.setScalar(Math.max(0.001, k));
      node.quaternion.copy(set);
      if (link.joint !== "fixed" && link.turn) {
        swing.setFromAxisAngle(AXIS[link.joint], link.turn(t));
        node.quaternion.multiply(swing);
      }
    }
  };

  let yaw = -0.62;
  let pitch = 0.3;
  let vyaw = 0;
  let held = false;
  const target = new Vector3();
  const draw = (dt: number, now: number) => {
    if (view.running) clock += dt;
    const open = reduced ? 1 : clamp((now - born) / 900, 0, 1);
    pose(clock, open);
    grid.offset.x = (grid.offset.x + (view.running ? dt * body.speed * 2 : 0)) % 1;
    size = mix(size, body.size, 1 - Math.exp(-dt * 6));
    if (!held) {
      vyaw = mix(vyaw, view.running ? 0.12 : 0, 1 - Math.exp(-dt * 1.2));
      yaw += vyaw * dt;
    }
    // The camera backs off until the whole body fits, whichever way the frame is shaped.
    const k = 1 - Math.exp(-dt * 6);
    span = mix(span, hull.circle, k);
    tall = mix(tall, hull.height, k);
    const half = Math.tan((camera.fov * Math.PI) / 360);
    const upright = (tall * Math.cos(pitch) + span * Math.sin(pitch)) / 2 / half;
    const across = span / 2 / (half * camera.aspect);
    const dist = Math.max(upright, across) * 1.16 + span * 0.3;
    target.set(0, tall * 0.48, 0);
    camera.position.set(
      Math.cos(yaw) * Math.cos(pitch) * dist,
      target.y + Math.sin(pitch) * dist,
      -Math.sin(yaw) * Math.cos(pitch) * dist,
    );
    camera.lookAt(target);
    key.position.set(1.4 * size, 3.4 * size, 1.9 * size);
    key.target.position.copy(target);
    const c = key.shadow.camera;
    c.left = c.bottom = -size * 1.5;
    c.right = c.top = size * 1.5;
    c.near = 0.1;
    c.far = size * 8;
    c.updateProjectionMatrix();
    gl.render(scene, camera);
  };
  const view = loop(canvas, draw);
  fit(canvas, (w, h, ratio) => {
    gl.setPixelRatio(ratio);
    gl.setSize(w, h, false);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
    view.poke();
  });

  let px = 0;
  let py = 0;
  canvas.addEventListener("pointerdown", (e) => {
    held = true;
    px = e.clientX;
    py = e.clientY;
    vyaw = 0;
    canvas.setPointerCapture(e.pointerId);
  });
  canvas.addEventListener("pointermove", (e) => {
    if (!held) return;
    const dx = e.clientX - px;
    yaw -= dx * 0.008;
    if (e.pointerType !== "touch") pitch = clamp(pitch + (e.clientY - py) * 0.006, 0.02, 1.35);
    vyaw = -dx * 0.25;
    px = e.clientX;
    py = e.clientY;
    view.poke();
  });
  for (const type of ["pointerup", "pointercancel"] as const)
    canvas.addEventListener(type, () => {
      held = false;
    });
  canvas.addEventListener("keydown", (e) => {
    const turn = { ArrowLeft: [0.16, 0], ArrowRight: [-0.16, 0], ArrowUp: [0, 0.1], ArrowDown: [0, -0.1] }[e.key];
    if (!turn) return;
    e.preventDefault();
    yaw += turn[0]!;
    pitch = clamp(pitch + turn[1]!, 0.02, 1.35);
    view.poke();
  });

  const buttons = $$<HTMLButtonElement>(root, ".stage-list button");
  const pick = (i: number, focus = false) => {
    for (const [j, b] of buttons.entries()) {
      b.setAttribute("aria-checked", String(i === j));
      b.tabIndex = i === j ? 0 : -1;
    }
    const list = buttons[i]!.parentElement!;
    if (list.scrollWidth > list.clientWidth)
      list.scrollTo({ left: buttons[i]!.offsetLeft - list.clientWidth / 2 + 80, behavior: reduced ? "auto" : "smooth" });
    if (focus) buttons[i]!.focus({ preventScroll: true });
    build(i);
    view.poke();
  };
  for (const [i, b] of buttons.entries()) {
    b.addEventListener("click", () => pick(i));
    b.addEventListener("keydown", (e) => {
      const step = { ArrowDown: 1, ArrowRight: 1, ArrowUp: -1, ArrowLeft: -1 }[e.key];
      if (!step) return;
      e.preventDefault();
      pick((index + step + buttons.length) % buttons.length, true);
    });
  }
  build(0);
  size = body!.size;
}
