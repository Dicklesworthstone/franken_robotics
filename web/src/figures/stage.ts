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
  InstancedMesh,
  Matrix3,
  Matrix4,
  Raycaster,
  Vector2,
  Vector3,
  type BufferGeometry,
} from "three";
import { $, $$, clamp, fit, loop, mix, reduced, smooth } from "./lib";
import { envelope, ownPlace, placed, treads, type Envelope } from "./envelope";
import { archetypes } from "./archetypes";
import { makers, type Body, type Link, type Mat, type Part } from "./bodies";
import { lit, materials, merged, renderer, solid } from "./three-kit";

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
  // The floor is drawn where the body has driven to, so its uv transform is set by hand every frame.
  t.matrixAutoUpdate = false;
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
    /** A belt's shoes, one instanced mesh for each of their parts. */
    belt?: { meshes: InstancedMesh[]; locals: Matrix4[]; tread: ReturnType<typeof treads> };
    /** Parts that move within their link: gears on their shafts, rods between their pins. */
    moving: { part: Part; mesh: Mesh; own: Matrix4 }[];
  }
  /** Merged geometry made for the body on show, freed when another is chosen. */
  let made: BufferGeometry[] = [];
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
  const shoe = new Matrix4();
  const cell = new Matrix4();
  const uv = new Matrix3();
  /** Moves the grid under the body to where it has driven: tiles are half a metre, the plane 14 m across. */
  const ground = (t: number) => {
    const at = body.travel?.(t) ?? { x: 0, z: 0, yaw: 0 };
    const c = Math.cos(at.yaw);
    const s = Math.sin(at.yaw);
    const k = 14 / 0.5;
    const ox = (at.x - 7 * c + 7 * s) / 0.5;
    const oz = (at.z + 7 * s + 7 * c) / 0.5;
    const wrap = (v: number) => v - Math.floor(v);
    uv.set(k * c, -k * s, wrap(ox), -k * s, -k * c, wrap(oz), 0, 0, 1);
    grid.matrix.copy(uv);
  };

  const build = (i: number) => {
    index = i;
    if (rootNode) scene.remove(rootNode);
    for (const geo of made) geo.dispose();
    made = [];
    body = makers[archetypes[i]!.id]!();
    hull = envelope(body);
    posed = [];
    zoomGoal = 1;
    canvas.style.touchAction = "";
    panGoal.set(0, 0, 0);
    for (const link of body.links) {
      const node = new Group();
      const set = new Quaternion();
      if (link.set) set.setFromEuler(new Euler(...link.set));
      let belt: Posed["belt"];
      const moving: Posed["moving"] = [];
      if (link.belt) {
        const tread = treads(link);
        belt = { tread, meshes: [], locals: [] };
        for (const part of link.parts) {
          const mesh = new InstancedMesh(solid(part.s, part.d), mats[part.m], tread.count);
          mesh.castShadow = mesh.receiveShadow = true;
          mesh.frustumCulled = false;
          belt.meshes.push(mesh);
          const turn = new Quaternion().setFromEuler(new Euler(...(part.r ?? [0, 0, 0])));
          belt.locals.push(new Matrix4().compose(new Vector3(...part.p), turn, new Vector3(1, 1, 1)));
          node.add(mesh);
        }
      } else {
        // Parts that stand still in their link are joined into one mesh for each finish: a few draw calls a link
        // however much detail it carries. Parts that move keep meshes of their own.
        // Close-up detail casts no shadow: it would double its cost for shadows too small to see.
        const still = new Map<string, BufferGeometry[]>();
        for (const part of link.parts) {
          if (part.coarse) continue;
          if (part.spin || part.span) {
            const mesh = new Mesh(solid(part.s, part.d), mats[part.m]);
            mesh.matrixAutoUpdate = false;
            mesh.castShadow = !part.fine;
            mesh.receiveShadow = part.m !== "glow";
            node.add(mesh);
            moving.push({ part, mesh, own: ownPlace(part) });
            continue;
          }
          const key = `${part.m}${part.fine ? " fine" : ""}`;
          const list = still.get(key) ?? [];
          list.push(solid(part.s, part.d).clone().applyMatrix4(ownPlace(part)));
          still.set(key, list);
        }
        for (const [key, list] of still) {
          const geo = merged(list);
          for (const g of list) g.dispose();
          made.push(geo);
          const [m, fine] = key.split(" ") as [Mat, string?];
          const mesh = new Mesh(geo, mats[m]);
          mesh.castShadow = !fine;
          mesh.receiveShadow = m !== "glow";
          node.add(mesh);
        }
      }
      const [x, y, z] = link.at;
      const burst = new Vector3(x, y + 0.02, z).normalize().multiplyScalar(link.parent < 0 ? 0 : 0.16);
      posed.push({ link, node, set, burst, belt, moving });
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
        const name = l.belt ? "belt, passive" : l.joint === "free" || l.joint === "fixed" ? l.joint : `turn ${l.joint}${l.passive ? ", passive" : ""}`;
        return `<tr><td>${l.name}</td><td>${l.parent < 0 ? "world" : body.links[l.parent]!.name}</td><td${active ? ' class="act"' : ""}>${name}</td><td>${l.at.map(num).join(" ")}</td></tr>`;
      })
      .join("");
    $(root, ".datawrap").scrollTop = 0;
  };

  const stretch = new Vector3(1, 1, 1);
  const pose = (t: number, open: number) => {
    const off = body.base?.(t) ?? {};
    for (const [i, { link, node, set, burst, belt, moving }] of posed.entries()) {
      for (const { part, mesh, own } of moving) {
        const len = placed(part, own, t, mesh.matrix);
        if (part.span) mesh.matrix.scale(stretch.set(1, len / part.d[1], 1));
        mesh.matrixWorldNeedsUpdate = true;
      }
      if (belt)
        for (let k = 0; k < belt.tread.count; k++) {
          belt.tread.place(t, k, shoe);
          for (const [j, mesh] of belt.meshes.entries()) mesh.setMatrixAt(k, cell.multiplyMatrices(shoe, belt.locals[j]!));
        }
      if (belt) for (const mesh of belt.meshes) mesh.instanceMatrix.needsUpdate = true;
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
  /*
   * Zoom divides the distance the camera stands back; pan moves the point it turns about. Both ease towards where
   * the hand puts them. Zooming in on a point keeps that point where it is on the screen.
   */
  const MAX = 12;
  let zoom = 1;
  let zoomGoal = 1;
  const pan = new Vector3();
  const panGoal = new Vector3();
  const centre = new Vector3();
  let dist = 1;
  const clampPan = () => {
    const r = Math.max(span, tall) * 0.6;
    panGoal.x = clamp(panGoal.x, -r, r);
    panGoal.z = clamp(panGoal.z, -r, r);
    panGoal.y = clamp(panGoal.y, -tall * 0.48, tall * 0.6);
  };
  /** Zoom by `k` towards a point of the world, or towards the middle of the view. */
  const zoomBy = (k: number, at?: Vector3) => {
    const next = clamp(zoomGoal * k, 1, MAX);
    if (next === 1) panGoal.set(0, 0, 0);
    else if (at) {
      // The point under the hand stays put: the centre moves towards it by what the zoom takes away.
      const goal = centre.clone().add(panGoal);
      goal.sub(at).multiplyScalar(zoomGoal / next).add(at);
      panGoal.copy(goal.sub(centre));
    }
    zoomGoal = next;
    // Zoomed in, a touch on the model pans and pinches it; at the start it scrolls the page as before.
    canvas.style.touchAction = next > 1 ? "none" : "";
    clampPan();
    view.poke();
  };
  const ray = new Raycaster();
  const ndc = new Vector2();
  /** The point of the body under a place on the canvas, or the point at the same depth as the centre of view. */
  const under = (x: number, y: number) => {
    const box = canvas.getBoundingClientRect();
    ndc.set(((x - box.left) / box.width) * 2 - 1, -((y - box.top) / box.height) * 2 + 1);
    ray.setFromCamera(ndc, camera);
    const hit = rootNode ? ray.intersectObject(rootNode, true)[0] : undefined;
    if (hit) return hit.point.clone();
    return ray.ray.at(camera.position.distanceTo(centre.clone().add(pan)), new Vector3());
  };
  /** Moves the centre of view by a drag of dx, dy pixels, so that the body follows the hand. */
  const panBy = (dx: number, dy: number) => {
    const per = (2 * dist * Math.tan((camera.fov * Math.PI) / 360)) / canvas.clientHeight;
    const right = new Vector3().setFromMatrixColumn(camera.matrixWorld, 0);
    const up = new Vector3().setFromMatrixColumn(camera.matrixWorld, 1);
    panGoal.addScaledVector(right, -dx * per).addScaledVector(up, dy * per);
    clampPan();
    view.poke();
  };
  const draw = (dt: number, now: number) => {
    if (view.running) clock += dt;
    const open = reduced ? 1 : clamp((now - born) / 900, 0, 1);
    pose(clock, open);
    ground(clock);
    size = mix(size, body.size, 1 - Math.exp(-dt * 6));
    if (!held) {
      // Close up the turning slows, so that what is being looked at stays in view.
      vyaw = mix(vyaw, view.running ? 0.12 / zoom : 0, 1 - Math.exp(-dt * 1.2));
      yaw += vyaw * dt;
    }
    // The camera backs off until the whole body fits, whichever way the frame is shaped.
    const k = 1 - Math.exp(-dt * 6);
    span = mix(span, hull.circle, k);
    tall = mix(tall, hull.height, k);
    const ease = reduced ? 1 : 1 - Math.exp(-dt * 10);
    zoom = mix(zoom, zoomGoal, ease);
    pan.lerp(panGoal, ease);
    const half = Math.tan((camera.fov * Math.PI) / 360);
    const upright = (tall * Math.cos(pitch) + span * Math.sin(pitch)) / 2 / half;
    const across = span / 2 / (half * camera.aspect);
    dist = (Math.max(upright, across) * 1.16 + span * 0.3) / zoom;
    centre.set(0, tall * 0.48, 0);
    target.copy(centre).add(pan);
    camera.position.set(
      target.x + Math.cos(yaw) * Math.cos(pitch) * dist,
      target.y + Math.sin(pitch) * dist,
      target.z - Math.sin(yaw) * Math.cos(pitch) * dist,
    );
    camera.lookAt(target);
    const near = clamp(dist * 0.02, 0.004, 0.05);
    if (near !== camera.near) {
      camera.near = near;
      camera.updateProjectionMatrix();
    }
    // The light keeps its direction; close up its shadow covers only what is in view, and so is sharper.
    key.position.set(target.x + 1.4 * size, target.y + 3.4 * size, target.z + 1.9 * size);
    key.target.position.copy(target);
    const c = key.shadow.camera;
    const reach = Math.min(size * 1.5, Math.max(0.25, dist * half * Math.max(1, camera.aspect) * 2));
    c.left = c.bottom = -reach;
    c.right = c.top = reach;
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

  // One finger or the mouse turns the body; two fingers pinch to zoom and move it; a right or shift drag moves it.
  const down = new Map<number, { x: number; y: number }>();
  let px = 0;
  let py = 0;
  let panning = false;
  let pinch: { d: number; zoom: number; mx: number; my: number; at: Vector3 } | undefined;
  const pair = () => {
    const [a, b] = [...down.values()];
    return { d: Math.hypot(a!.x - b!.x, a!.y - b!.y), mx: (a!.x + b!.x) / 2, my: (a!.y + b!.y) / 2 };
  };
  canvas.addEventListener("pointerdown", (e) => {
    down.set(e.pointerId, { x: e.clientX, y: e.clientY });
    held = true;
    px = e.clientX;
    py = e.clientY;
    vyaw = 0;
    panning = e.button === 2 || e.button === 1 || e.shiftKey;
    canvas.setPointerCapture(e.pointerId);
    if (down.size === 2) {
      const p = pair();
      pinch = { d: Math.max(1, p.d), zoom: zoomGoal, mx: p.mx, my: p.my, at: under(p.mx, p.my) };
    }
  });
  canvas.addEventListener("pointermove", (e) => {
    if (!down.has(e.pointerId)) return;
    down.set(e.pointerId, { x: e.clientX, y: e.clientY });
    if (pinch && down.size >= 2) {
      const p = pair();
      zoomBy((pinch.zoom * p.d) / pinch.d / zoomGoal, pinch.at);
      panBy(p.mx - pinch.mx, p.my - pinch.my);
      pinch.mx = p.mx;
      pinch.my = p.my;
      return;
    }
    const dx = e.clientX - px;
    const dy = e.clientY - py;
    px = e.clientX;
    py = e.clientY;
    if (panning) {
      panBy(dx, dy);
      return;
    }
    yaw -= dx * 0.008;
    if (e.pointerType !== "touch") pitch = clamp(pitch + dy * 0.006, 0.02, 1.35);
    vyaw = -dx * 0.25;
    view.poke();
  });
  for (const type of ["pointerup", "pointercancel"] as const)
    canvas.addEventListener(type, (e) => {
      down.delete(e.pointerId);
      if (down.size < 2) pinch = undefined;
      if (down.size === 1) {
        // The finger left behind carries on turning from where it is.
        const [rest] = [...down.values()];
        px = rest!.x;
        py = rest!.y;
      }
      if (!down.size) held = false;
    });
  canvas.addEventListener("contextmenu", (e) => e.preventDefault());
  // A pinch on a trackpad arrives as a wheel with ctrl held; Safari sends gestures instead. A plain wheel scrolls
  // the page, as it would anywhere else.
  canvas.addEventListener(
    "wheel",
    (e) => {
      if (!e.ctrlKey && !e.metaKey) return;
      e.preventDefault();
      const step = e.deltaMode === 1 ? e.deltaY * 16 : e.deltaY;
      zoomBy(Math.exp(-clamp(step, -60, 60) * 0.012), under(e.clientX, e.clientY));
    },
    { passive: false },
  );
  let gesture = 1;
  canvas.addEventListener("gesturestart", (e) => {
    e.preventDefault();
    gesture = 1;
  });
  canvas.addEventListener("gesturechange", (e) => {
    e.preventDefault();
    const g = e as unknown as { scale: number; clientX: number; clientY: number };
    if (down.size >= 2) return;
    zoomBy(g.scale / gesture, under(g.clientX, g.clientY));
    gesture = g.scale;
  });
  // A double click goes in on the point under it, two and a half times; at the closest it goes back out.
  canvas.addEventListener("dblclick", (e) => {
    if (zoomGoal >= MAX * 0.99) zoomBy(1 / MAX);
    else zoomBy(2.5, under(e.clientX, e.clientY));
  });
  canvas.addEventListener("keydown", (e) => {
    // With Ctrl or Cmd held these keys zoom the page, and that is left to the browser.
    const zoomKey = e.ctrlKey || e.metaKey || e.altKey ? undefined : { "+": 1.5, "=": 1.5, "-": 1 / 1.5, _: 1 / 1.5, "0": 0 }[e.key];
    if (zoomKey !== undefined) {
      e.preventDefault();
      if (zoomKey) zoomBy(zoomKey);
      else zoomBy(1 / MAX);
      return;
    }
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
