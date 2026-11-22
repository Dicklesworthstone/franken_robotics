import { chapterClose, chapterOpen, copy, figure, icon, section, seg, toggle, type Chapter, type Refs } from "../parts";
import { archetypes } from "../figures/archetypes";

export const body: Chapter = {
  id: "body",
  number: "02",
  label: "The body",
  short: "Body",
  icon: "body",
  colour: "#4ade80",
  title: "A body is *data*.",
  thesis:
    "A robot is a composition of modules over typed ports, and it lowers to tables. Different robots can then share one batch, a new design recompiles nothing, and an agent can check a body in seconds before it simulates one step.",
  sections: [
    ["bodies", "Eighteen bodies"],
    ["anatomy", "The anatomy"],
    ["preflight", "Eight stages of preflight"],
    ["symmetry", "What the sensors break"],
    ["batch", "256 robots, one dispatch"],
  ],
};

/** The subsystems of §0.5, in the order the plate lists them. */
export const organs: [key: string, name: string, section: string, text: string, side: "left" | "right"][] = [
  ["cortex", "Cortex", "23", "The environment API, learning, and one policy for a whole class of bodies.", "left"],
  ["sensorium", "Sensorium", "20", "Sensor models, and a clock that records when each reading was taken, sent and used.", "left"],
  ["reflex", "Reflex", "22", "State estimation, a controller synthesized from the model, gaits, and shields on commands.", "left"],
  ["spark", "Spark", "21", "The GPU backends. Kernel arithmetic is derived once and emitted for a CPU reference, CUDA and Metal.", "left"],
  ["sinew", "Sinew", "15", "Actuators, transmissions, batteries and winding temperatures.", "left"],
  ["skeleton", "Skeleton", "14", "The design language: modules with port contracts, archetypes, preflight, the model compiler.", "left"],
  ["marrow", "Marrow", "13", "Lie groups, finite groups, cones and order theory, under everything else.", "left"],
  ["kinesis", "Kinesis", "16", "Dynamics and contact solves on three tiers, with the CPU referee as the permanent reference.", "left"],
  ["traction", "Traction", "18", "Collision, friction, wheels, tracks and soft ground.", "left"],
  ["habitat", "Habitat", "19", "Terrain and interiors, generated or scanned, and whether a body fits through them.", "left"],
  ["airlock", "Airlock", "33", "The only code that names FrankenSim's volatile types.", "left"],
  ["phylum", "Phylum", "17", "Families of different bodies in one batch.", "right"],
  ["augur", "Augur", "25", "Mission queries: which designs can do the job, which cannot, and what to test next.", "right"],
  ["atelier", "Atelier", "28", "The studio loop: analysis, comparison, lineage, and which verdicts an edit withdraws.", "right"],
  ["nemesis", "Nemesis", "26", "The search for worlds that break a design, and bounds on rare failures.", "right"],
  ["autopsy", "Autopsy", "27", "Failures explained by intervening on forked replays.", "right"],
  ["chronicle", "Chronicle", "29", "Telemetry, replay, the energy ledger and the record of evidence.", "right"],
  ["lemma", "Lemma", "12", "Proofs, certificates and an offline auditor that shares no code with the prover.", "right"],
  ["gauntlet", "Gauntlet", "38", "The test program, and the harness for the ten-fold test.", "right"],
  ["foundry", "Foundry", "31", "Structure, tolerance, fatigue and build packets.", "right"],
  ["ground", "Ground", "30", "Calibration against imported measurements of real robots.", "right"],
  ["tether", "Tether", "32", "The command line, the daemon, the MCP server and the viewer.", "right"],
  ["arena", "Arena", "24", "Tasks, requirements, metrics, and a sealed lane for qualification.", "right"],
];

/** A standing machine with every subsystem drawn as a part of it. Groups carry data-part. */
const plate = () => {
  const g = (part: string, inner: string) => `<g data-part="${part}">${inner}</g>`;
  const side = (f: (s: 1 | -1) => string) => f(1) + f(-1);
  const X = (s: number, x: number) => 260 + s * x;
  const limb = (x1: number, y1: number, x2: number, y2: number, w: number) => {
    const dx = x2 - x1,
      dy = y2 - y1,
      n = Math.hypot(dx, dy),
      ox = (-dy / n) * w,
      oy = (dx / n) * w;
    const f = (v: number) => v.toFixed(1);
    return `<path class="shell" d="M${f(x1 + ox)} ${f(y1 + oy)}L${f(x2 + ox * 0.8)} ${f(y2 + oy * 0.8)}A${f(w * 0.8)} ${f(w * 0.8)} 0 0 1 ${f(x2 - ox * 0.8)} ${f(y2 - oy * 0.8)}L${f(x1 - ox)} ${f(y1 - oy)}A${w} ${w} 0 0 1 ${f(x1 + ox)} ${f(y1 + oy)}Z"/>`;
  };
  const line = (x1: number, y1: number, x2: number, y2: number, c = "") =>
    `<path class="${c}" d="M${x1} ${y1}L${x2} ${y2}"/>`;
  const joints: [number, number, number][] = [
    [104, 172, 22],
    [126, 282, 16],
    [34, 402, 20],
    [42, 520, 18],
    [46, 634, 12],
  ];
  const skin = side(
    (s) =>
      limb(X(s, 104), 172, X(s, 126), 282, 19) +
      limb(X(s, 126), 282, X(s, 138), 378, 15) +
      limb(X(s, 34), 402, X(s, 42), 520, 24) +
      limb(X(s, 42), 520, X(s, 46), 634, 18),
  );
  return `<svg viewBox="0 0 520 740" role="img" aria-label="A standing humanoid machine drawn as an anatomical plate, with each subsystem as one of its parts">
<defs><radialGradient id="pl-g"><stop offset="0" stop-color="#4ade80" stop-opacity=".16"/><stop offset="1" stop-color="#4ade80" stop-opacity="0"/></radialGradient></defs>
<ellipse cx="260" cy="350" rx="250" ry="340" fill="url(#pl-g)"/>
${g("arena", `<ellipse class="ring" cx="260" cy="676" rx="206" ry="32"/><ellipse class="ring thin" cx="260" cy="676" rx="168" ry="25"/>`)}
${g("habitat", `<path class="thin" d="M24 704h110v-16h44M496 704H386v-28h-36"/><path class="thin" d="M24 704h472"/>`)}
${g("phylum", `<path class="ghost" d="M58 660l14-92-10-96 22-120 40 0M86 470h34M462 660l-8-120 12-64-20-96-44-6M430 500l-30 160"/>`)}
<g class="base">${skin}
<path class="shell" d="M176 146q84-22 168 0l14 40-38 136q-60 20-120 0l-38-136z"/><path class="shell" d="M208 336q52 16 104 0l10 44q-62 34-124 0z"/><rect class="shell" x="246" y="118" width="28" height="22" rx="6"/>
<path class="shell" d="M212 62q0-30 48-30t48 30v34q0 28-48 28t-48-28z"/></g>
${g("cortex", `<path class="fillable" d="M222 62q0-21 38-21t38 21q-18 9-38 9t-38-9z"/><path class="thin" d="M240 45q-6 10 2 22M260 42v28M280 45q6 10-2 22"/>`)}
${g("sensorium", `<rect class="fillable" x="222" y="78" width="76" height="22" rx="11"/><circle cx="244" cy="89" r="5"/><circle cx="276" cy="89" r="5"/><path class="thin" d="M206 89h-34M314 89h34M196 80q-8 9 0 18M324 80q8 9 0 18"/>`)}
${g("augur", `<path class="thin" d="M226 24a48 48 0 0 1 68 0M212 10a68 68 0 0 1 96 0"/><circle class="fillable" cx="260" cy="28" r="4"/>`)}
${g("reflex", `<path class="wire" d="M260 136v196"/>${[152, 174, 196, 252, 274, 296, 318].map((y) => `<path class="thin" d="M252 ${y}h16"/>`).join("")}`)}
${g("spark", `<path class="fillable" d="M260 198l21 12v25l-21 12-21-12v-25z"/><path class="wire" d="M264 205l-11 19h14l-11 17"/>`)}
${g("chronicle", `<path class="thin" d="M300 178h34M298 190h38M297 202h36M296 214h34M294 226h33"/>`)}
${g("airlock", `<rect class="fillable" x="246" y="288" width="28" height="28" rx="8"/><circle cx="260" cy="302" r="8"/><path class="thin" d="M260 294v16M252 302h16"/>`)}
${g("lemma", `<circle class="fillable" cx="204" cy="196" r="17"/><path class="wire" d="M196 196l6 7 11-14"/>`)}
${g("skeleton", side((s) => line(X(s, 104), 172, X(s, 126), 282, "bone") + line(X(s, 126), 282, X(s, 138), 378, "bone") + line(X(s, 34), 402, X(s, 42), 520, "bone") + line(X(s, 42), 520, X(s, 46), 634, "bone")) + `<path class="bone" d="M158 164q102-18 204 0M218 356q42 12 84 0"/>`)}
${g("marrow", side((s) => [420, 436, 452, 468, 484, 500].map((y) => `<circle cx="${X(s, 35 + (y - 402) * 0.068)}" cy="${y}" r="3.2"/>`).join("")))}
${g("sinew", side((s) => joints.map(([x, y, r]) => `<circle class="fillable" cx="${X(s, x)}" cy="${y}" r="${r}"/><circle class="thin" cx="${X(s, x)}" cy="${y}" r="${r * 0.55}"/>`).join("")))}
${g("kinesis", side((s) => `<path class="thin" d="M${X(s, 72)} 494a34 34 0 0 ${s === 1 ? 1 : 0} 0 52"/><path class="thin" d="M${X(s, 84)} 486a46 46 0 0 ${s === 1 ? 1 : 0} 0 68"/>`))}
${g("gauntlet", side((s) => `<path class="fillable" d="M${X(s, 126)} 384l${s * 26} -4 ${s * 6} 26 ${s * -8} 30 ${s * -20} 2 ${s * -8} -26z"/><path class="thin" d="M${X(s, 134)} 410v22M${X(s, 141)} 410v22M${X(s, 148)} 408v20"/>`).replaceAll("z", "Z"))}
${g("foundry", `<path class="thin" d="M214 342l8 5M306 342l-8 5M190 172l7 6M330 172l-7 6"/><circle cx="216" cy="344" r="3.5"/><circle cx="304" cy="344" r="3.5"/><circle cx="190" cy="160" r="3.5"/><circle cx="330" cy="160" r="3.5"/><circle cx="222" cy="372" r="3.5"/><circle cx="298" cy="372" r="3.5"/>`)}
${g("traction", side((s) => `<path class="fillable" d="M${X(s, 30)} 648h${s * 44}l${s * 10} 18h${s * -62}z"/><path class="thin" d="M${X(s, 46)} 672l${s * -16} 30M${X(s, 46)} 672l${s * 16} 30"/>`))}
${g("ground", `<path class="thin" d="M378 690v-34M372 656h12M378 690l-5 8h10z"/><path class="thin" d="M388 662h16M388 670h10M388 678h16"/>`)}
${g("nemesis", `<path class="wire hot" d="M84 560l26 10-14 12 30 12-18 8 26 14"/>`)}
${g("autopsy", `<path class="thin dash" d="M150 300h220M150 306h220"/><path class="thin" d="M150 292v22M370 292v22"/>`)}
${g("atelier", `<path class="thin" d="M36 140v512M28 140h16M28 652h16M36 396h-10"/><path class="thin" d="M36 140l-4 10h8zM36 652l-4-10h8z"/>`)}
${g("tether", `<path class="wire" d="M346 250q70 10 74 70t60 70q30 12 30 40"/><rect class="fillable" x="338" y="242" width="12" height="16" rx="3"/>`)}
</svg>`;
};

const stages: [name: string, text: string, target: string][] = [
  ["Form", "Units, ports, module digests, fills and locks", "100 ms"],
  ["Mass", "Mass roll-up, inertia positivity by exact LDLᵀ", "50 ms"],
  ["Mechanism", "Joint classes, loop closure, mobility, self-collision", "400 ms"],
  ["Motion", "Growth vector of the wheels, curvature of shape-changers", "200 ms"],
  ["Structure", "Both symmetry groups, port power balance", "150 ms"],
  ["Statics and sizing", "Tip-over, torque, speed, heat and the bus", "600 ms"],
  ["Place", "Reach, sensor coverage, clearance through the scene", "2 s"],
  ["Information", "Two worlds that look the same and need opposite actions", "10 s"],
];

export const renderBody = (ref: Refs) =>
  chapterOpen(body) +
  section({
    ref,
    id: "bodies",
    index: "02.1",
    eyebrow: "Archetypes",
    title: "Eighteen *bodies*.",
    kicker:
      "Each archetype exists to exercise named mechanisms, and each has a known answer that fails if the mechanism is wrong. Pick one, drag to turn it, and read the table it is drawn from. [§14.10](§14.10)",
    body: figure({
      name: "stage",
      no: "02.1",
      label: "The archetypes of §14.10",
      title: "Drawn from *its table*.",
      basis: "drawing",
      source: [["§14.10", ref("14.10")]],
      note: "Proportions and motions are this page's own. The layout, actuated coordinates, symmetry group and purpose are the plan's.",
      ref,
      body: `<div class="stage"><div class="stage-list" role="radiogroup" aria-label="Archetype">${archetypes
        .map(
          (a, i) =>
            `<button type="button" role="radio" aria-checked="${i === 0}" tabindex="${i === 0 ? 0 : -1}" data-i="${i}"><span class="num">${String(i + 1).padStart(2, "0")}</span><b>${a.id}</b><small>${a.dof}</small></button>`,
        )
        .join("")}</div>
<div class="stage-view"><canvas tabindex="0" aria-label="A turning model of the selected robot. Drag, or use the arrow keys, to turn it."></canvas><p class="stage-hint">Drag to turn</p><dl class="stage-hud"><div><dt>Actuated</dt><dd data-dof>${archetypes[0]!.dof}</dd></div><div><dt>Symmetry as built</dt><dd data-sym>${archetypes[0]!.symmetry}</dd></div><div><dt>Links</dt><dd data-links></dd></div></dl></div>
<div class="stage-info" aria-live="polite"><p class="micro" data-no>Archetype 01</p><h5 data-name>${archetypes[0]!.id}</h5><p data-layout>${copy(archetypes[0]!.layout)}</p><p class="tests"><small>What it tests</small><span data-tests>${copy(archetypes[0]!.tests)}</span></p><div class="datawrap"><table class="data"><thead><tr><th>link</th><th>parent</th><th>joint</th><th>offset, m</th></tr></thead><tbody data-rows></tbody></table></div></div></div>`,
    }),
  }) +
  section({
    ref,
    id: "anatomy",
    index: "02.2",
    eyebrow: "Subsystems",
    title: "Assembled from *parts*.",
    kicker:
      "“The subsystems are named for the parts, for what the creature does, and for what is done to it.” Point at a name, or at the machine. [§0.5](§0.5)",
    body: figure({
      name: "anatomy",
      no: "02.2",
      label: "Twenty-three subsystems",
      title: "The *anatomy*.",
      basis: "plan",
      source: [["§0.5", ref("0.5")]],
      className: "plain",
      body: `<div class="anatomy"><ul class="organs left">${organs
        .filter((o) => o[4] === "left")
        .map(
          ([key, name, s, text]) =>
            `<li data-organ="${key}"><a href="${ref(s)}"><b>${name}</b><span>§${s}</span></a><p>${copy(text)}</p></li>`,
        )
        .join("")}</ul><div class="plate">${plate()}</div><div class="organ-now" data-now></div><ul class="organs right">${organs
        .filter((o) => o[4] === "right")
        .map(
          ([key, name, s, text]) =>
            `<li data-organ="${key}"><a href="${ref(s)}"><b>${name}</b><span>§${s}</span></a><p>${copy(text)}</p></li>`,
        )
        .join("")}</ul></div>`,
    }),
  }) +
  section({
    ref,
    id: "preflight",
    index: "02.3",
    eyebrow: "design check",
    title: "Eight stages before the *first step*.",
    kicker:
      "Static checks stop at the first refusal and answer with a repair the agent can apply as an edit. Stages 1 to 6 have a target of two seconds together. [§14.11](§14.11)",
    body: figure({
      name: "preflight",
      no: "02.3",
      label: "frob design check",
      title: "Hand it a *faulty* robot.",
      basis: "illustrative",
      source: [
        ["§14.11", ref("14.11")],
        ["§7", ref("7")],
      ],
      note: "Stage names, refusal codes and time targets are the plan's. The parallel-roller and beacon cases are known answers; the sizing case is the story of NS-01.",
      ref,
      body: `<div class="preflight"><div>${seg(
        "case",
        "Robot to check",
        [
          ["ok", "No fault"],
          ["knee", "Undersized knee"],
          ["roller", "Parallel rollers"],
          ["door", "Too wide for door D"],
          ["beacon", "Two beacons"],
        ],
        "knee",
      )}<ol class="stages">${stages
        .map(
          ([name, text, t], i) =>
            `<li data-stage="${i + 1}"><span class="num">${i + 1}</span><div><b>${name}</b><small>${text}</small></div><em>${t}</em><span class="state">${icon("check")}</span></li>`,
        )
        .join("")}</ol></div><div class="term" aria-live="polite"><p class="term-bar"><i></i><i></i><i></i><span>frob design check</span></p><pre data-out></pre><button type="button" class="btn sm" data-fix>Apply the first repair</button></div></div>`,
    }),
  }) +
  section({
    ref,
    id: "symmetry",
    index: "02.4",
    eyebrow: "C-09",
    title: "What the sensors *break*.",
    kicker:
      "A body has one symmetry group as a mechanism and a smaller one once its sensors, payload and task are counted. Tests may use the first. Anything that ties weights or folds states together may use only the second. [C-09](§C-09)",
    body: figure({
      name: "symmetry",
      no: "02.4",
      label: "A quadruped from above",
      title: "Mount something and *recount*.",
      basis: "computed",
      source: [["C-09", ref("C-09")]],
      note: "The page tries the four planar isometries of the rectangle against every labelled feature.",
      ref,
      body: `<div class="symm"><div class="symm-view" data-view></div><div class="symm-side"><div class="toggles">${toggle("lidar", "Forward lidar", true)}${toggle("payload", "Payload on one side")}${toggle("goal", "Commanded forward")}${toggle("knees", "All knees bend back")}${toggle("gear", "One gearbox swapped")}</div><div class="groups"><div><small>As a mechanism</small><b data-mech>C₂ × C₂</b><span data-mech-n></span></div><div><small>With what it senses and carries</small><b data-obs>C₂</b><span data-obs-n></span></div></div><p class="say" data-say></p></div></div>`,
    }),
  }) +
  section({
    ref,
    id: "batch",
    index: "02.5",
    eyebrow: "Phylum",
    title: "256 robots, *one dispatch*.",
    kicker:
      "NS-14 asks for 256 different designs with three leg counts in a single batch, 32 worlds each, with no kernel compiled. Incumbents already mix topologies, so the plan may claim only two measured numbers against them. [§17](§17)",
    body: figure({
      name: "batch",
      no: "02.5",
      label: "One table of 256 bodies",
      title: "Every one is a *different* robot.",
      basis: "drawing",
      source: [
        ["§17", ref("17")],
        ["NS-14", ref("7")],
      ],
      note: "Leg count, femur, tibia, stance and gait speed are drawn at random per body. The motion is kinematic.",
      ref,
      className: "bleed",
      body: `<div class="batch"><canvas aria-label="A field of 256 walking robots with four, six or eight legs and different proportions"></canvas><div class="batch-hud"><dl><div><dt>Bodies</dt><dd>256</dd></div><div><dt>Links posed per frame</dt><dd data-links>0</dd></div><div><dt>Kernels compiled</dt><dd>0</dd></div></dl><button type="button" class="btn sm" data-seed>${icon("redo")}Another 256</button></div><p class="batch-tip" data-tip hidden></p></div>`,
    }),
  }) +
  chapterClose;
