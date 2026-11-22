import {
  AmbientLight,
  BoxGeometry,
  Color,
  DirectionalLight,
  Group,
  HemisphereLight,
  Mesh,
  MeshStandardMaterial,
  OrthographicCamera,
  Vector3,
} from "three";
import { $, $$, clamp, fit, loop, mix, reduced, seg } from "./lib";
import { generate, type Finish, type Interior } from "./interiors";
import { lit, renderer } from "./three-kit";

export default function mount(root: HTMLElement) {
  const canvas = $<HTMLCanvasElement>(root, "canvas");
  const gl = renderer(canvas);
  if (!gl) return;
  const scene = lit(gl, 0.22);
  const camera = new OrthographicCamera(-5, 5, 5, -5, 0.1, 80);
  scene.add(new AmbientLight("#cfe8f7", 0.08), new HemisphereLight("#bfe0f5", "#0b1a22", 0.32));
  const key = new DirectionalLight("#ffe8c4", 1.75);
  key.position.set(5, 11, 4);
  key.castShadow = true;
  key.shadow.mapSize.set(2048, 2048);
  const sc = key.shadow.camera;
  sc.left = sc.bottom = -9;
  sc.right = sc.top = 9;
  sc.far = 34;
  key.shadow.bias = -0.0006;
  key.shadow.normalBias = 0.025;
  scene.add(key);

  const m = (color: string, roughness = 0.75, metalness = 0, opacity = 1) =>
    new MeshStandardMaterial({ color, roughness, metalness, transparent: opacity < 1, opacity });
  const mats: Record<Finish, MeshStandardMaterial> = {
    floor: m("#3a474d", 0.95),
    tile: m("#49585f", 0.85),
    wall: m("#8ba0aa", 0.95),
    wood: m("#8a6238", 0.8),
    fabric: m("#3f6b80", 0.95),
    steel: m("#93a5ae", 0.42, 0.55),
    shelf: m("#b9c6cb", 0.8),
    stock: m("#c98f4e", 0.85),
    stock2: m("#3f7fa6", 0.8),
    stock3: m("#d7a63a", 0.8),
    dark: m("#232c30", 0.7, 0.2),
    glass: m("#9adcff", 0.08, 0.1, 0.28),
    green: m("#5f9a62", 0.9),
    belt: m("#30393d", 0.6),
    mat: m("#1f3a46", 0.98),
    hazard: m("#fbbf24", 0.5, 0, 0.82),
  };
  mats.hazard.emissive = new Color("#fbbf24");
  mats.hazard.emissiveIntensity = 0.9;

  const unit = new BoxGeometry(1, 1, 1);
  const pins = $(root, "[data-pins]");
  const list = $(root, "[data-hazards]");
  let room: Interior;
  let group: Group | undefined;
  let seed = 41;
  let place = "kitchen";
  let born = 0;
  let rise: { mesh: Mesh; y: number; h: number; delay: number }[] = [];
  let spans: HTMLElement[] = [];

  const light = (i: number) => {
    spans.forEach((s, j) => s.classList.toggle("on", i === j));
    $$(list, "li").forEach((s, j) => s.classList.toggle("on", i === j));
  };
  const build = () => {
    if (group) scene.remove(group);
    group = new Group();
    room = generate(place, seed);
    rise = [];
    for (const b of room.blocks) {
      const mesh = new Mesh(unit, mats[b.f]);
      mesh.scale.set(b.w, b.h, b.d);
      mesh.position.set(b.x, (b.y ?? 0) + b.h / 2, b.z);
      mesh.rotation.y = b.turn ?? 0;
      mesh.castShadow = b.f !== "glass" && b.f !== "hazard" && b.h > 0.02;
      mesh.receiveShadow = b.f !== "hazard";
      group.add(mesh);
      if (b.h > 0.03 && b.f !== "wall" && (b.y ?? 0) >= 0)
        rise.push({ mesh, y: b.y ?? 0, h: b.h, delay: ((b.x + room.w / 2) / room.w + (b.z + room.d / 2) / room.d) * 0.4 + (b.y ?? 0) * 0.12 });
    }
    scene.add(group);
    born = performance.now();
    $(root, "[data-seedno]").textContent = `seed ${seed}`;
    $(root, "[data-expose]").textContent = room.expose;
    pins.innerHTML = room.hazards.map((_, i) => `<span>${i + 1}</span>`).join("");
    spans = $$(pins, "span");
    list.innerHTML = room.hazards
      .map(
        (h, i) =>
          `<li><i>${i + 1}</i><div><b>${h.name}</b> <span class="num">${h.size}</span><small>${h.effect}</small></div></li>`,
      )
      .join("");
    $$(list, "li").forEach((li, i) => {
      li.addEventListener("pointerenter", () => light(i));
      li.addEventListener("pointerleave", () => light(-1));
    });
    size();
    view.poke();
  };
  const v = new Vector3();
  let W = 1;
  let H = 1;
  let zoom = 8;
  const size = () => {
    // The footprint seen at 45 degrees, with room for the walls above it.
    const across = (room.w + room.d) * 0.74;
    const tall = (room.w + room.d) * 0.46 + 1.4;
    zoom = Math.max(across / (W / H), tall) * 0.53;
  };
  let turn = 0;
  let shownZoom = 0;
  const draw = (dt: number, now: number) => {
    if (view.running) turn += dt;
    shownZoom = shownZoom ? mix(shownZoom, zoom, 1 - Math.exp(-dt * 5)) : zoom;
    camera.top = shownZoom;
    camera.bottom = -shownZoom;
    camera.right = (shownZoom * W) / H;
    camera.left = -camera.right;
    camera.updateProjectionMatrix();
    const a = Math.PI / 4 + Math.sin(turn * 0.22) * 0.16;
    camera.position.set(Math.sin(a) * 20, 15.5, Math.cos(a) * 20);
    camera.lookAt(0, 0.2, 0);
    const t = reduced ? 9 : (now - born) / 1000;
    for (const r of rise) {
      const k = clamp((t - r.delay) / 0.45, 0, 1);
      const e = 1 - (1 - k) ** 3;
      r.mesh.scale.y = Math.max(0.0001, r.h * e);
      r.mesh.position.y = r.y + (r.h * e) / 2;
    }
    mats.hazard.emissiveIntensity = reduced ? 0.8 : 0.7 + 0.35 * Math.sin(now / 420);
    gl.render(scene, camera);
    for (const [i, h] of room.hazards.entries()) {
      v.set(h.x, h.y ?? 0.2, h.z).project(camera);
      spans[i]!.style.transform = `translate(${(((v.x + 1) / 2) * W).toFixed(1)}px, ${(((1 - v.y) / 2) * H).toFixed(1)}px)`;
    }
  };
  const view = loop(canvas, draw);
  seg(root, "place", (p) => {
    place = p;
    build();
  });
  $(root, "[data-seed]").addEventListener("click", () => {
    seed = (seed * 1103515245 + 12345) % 997;
    build();
  });
  room = generate(place, seed);
  fit(
    canvas,
    (w, h, ratio) => {
      W = w;
      H = h;
      gl.setPixelRatio(ratio);
      gl.setSize(w, h, false);
      size();
      view.poke();
    },
    1.75,
  );
  build();
}
