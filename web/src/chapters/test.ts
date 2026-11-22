import { constructions } from "../catalogue";
import { escape } from "../layout";
import { chapterClose, chapterOpen, copy, figure, icon, section, seg, toggle, type Chapter, type Refs } from "../parts";
import { organs, plate } from "./body";

export const test: Chapter = {
  id: "engine",
  number: "06",
  label: "The engine",
  short: "Engine",
  icon: "engine",
  colour: "#fb7185",
  title: "Algebra *all the way down*.",
  thesis:
    "Rust at the core. One set of kernels for CUDA, Metal and the CPU. A slower, stricter physics engine as referee. Underneath it all, forty pieces of mathematics that each have a job.",
  sections: [
    ["anatomy", "Twenty-three parts"],
    ["symmetry", "Teach one leg, get the rest"],
    ["constructions", "The math, piece by piece"],
    ["bench", "Check its arithmetic"],
    ["suite", "The Franken suite"],
    ["roadmap", "What ships first"],
  ],
};

/** A name people can say and one line on what it is good for. Titles and links come from the plan. */
const good: [name: string, text: string][] = [
  ["Joints as Lie groups", "One kernel covers every kind of joint, and the strange linkages come out right."],
  ["Typed frames", "Mixing up two reference frames is a compile error."],
  ["Inertia on a cone", "Optimizers can’t invent a body with impossible inertia."],
  ["Euler–Poincaré dynamics", "Momentum comes for free, and so do the laws it has to obey."],
  ["Steps that stay rigid", "Rotations stay rotations, step after step, with only rounding to clean up."],
  ["Contact as a cone program", "Contact has one answer, found by a solver that knows when it’s done."],
  ["Wheels and Lie brackets", "Tells you how well a wheeled base can maneuver before you simulate it."],
  ["Locomotion as holonomy", "Ranks snakes and swimmers by the shape of their wriggle, with no rollouts."],
  ["Two kinds of symmetry", "Finds the symmetries a robot really has once sensors and payload are counted."],
  ["Gaits as subgroups", "Writes out every gait a body can walk, with its timing."],
  ["Exact leg angles", "Solves a leg’s angles exactly, and says how many solutions there are."],
  ["Bodies that compose", "Edit one part and only that part is recomputed."],
  ["Control on groups", "Every new body gets an estimator and a controller from its own model."],
  ["Derived kernels", "The same arithmetic on three backends, with the paperwork attached."],
  ["Safe peeking", "Peek at the results as often as you like. The error rate still holds."],
  ["Gait stability", "Measures whether a gait recovers from a stumble, and how fast."],
  ["Missions as sets", "Returns families of designs, and the smallest change that would let one in."],
  ["Answers that survive edits", "Change a motor and keep every answer the change can’t touch."],
  ["The limits of information", "Says when more training can’t help, and which sensor would."],
  ["Blind corridors", "Shows the corridors where a robot goes blind."],
  ["Scaling laws", "Scales a verdict from a small robot to a large one."],
  ["Mirror-image pruning", "Skips the designs that are mirror images of each other."],
  ["Policies for families", "Scores most designs without training them."],
  ["Equivariant policies", "A policy that can’t tell which way you drew the axes."],
  ["Blended fidelity", "Referee-quality answers for a fraction of the referee’s time."],
  ["Trust in cheap tests", "Knows how far to trust a cheap test, and audits itself."],
  ["Rare failures", "Finds the one-in-100,000 failure without 100,000 tries."],
  ["Smallest failing world", "Hands you the smallest world that breaks the robot."],
  ["Causes by experiment", "Answers “what caused it?” by experiment."],
  ["One spec, three jobs", "One requirement scores the run, guides the attack and guards the robot."],
  ["Energy-first safety", "A safety budget in joules that survives every retraining."],
  ["Whole-region proofs", "One solve covers a whole range of stances and ratings."],
  ["Two-sided clearance", "Finds the doorway that’s too tight, from both sides."],
  ["Linkage design", "Designs linkages, and catches the ones that jam."],
  ["Structure and fatigue", "Sizes each part by the loads it met on the job."],
  ["Energy-keeping steps", "No energy creeping in over long runs, and it plays backwards."],
  ["Spin and slip friction", "Spin and slip share one friction cone."],
  ["Motors with heat", "Every joule accounted for, down to the hot motor."],
  ["What to measure next", "Buys a measurement only when it could change the decision."],
  ["Cheaper-part swaps", "Swaps in a cheaper part once it’s shown to be no worse."],
];
if (good.length !== constructions.length) throw Error(`${constructions.length} constructions and ${good.length} lines about them`);

const suite: [name: string, repo: string, role: string, text: string][] = [
  ["FrankenSim", "frankensim", "Referee physics", "Featherstone dynamics, contact with certified collision checks, and a 29-joint humanoid model."],
  ["asupersync", "asupersync", "Concurrency", "Every job can be canceled cleanly, replayed and tested under a simulated clock."],
  ["FrankenSQLite", "frankensqlite", "The ledger", "Every run, design and verdict in one database you can query."],
  ["FrankenTorch", "frankentorch", "Learning", "The gateway to Apple GPUs, and a second opinion on every gradient."],
  ["FrankenSymPy", "frankensympy", "Derivation", "Works out the kernels symbolically and checks their certificates."],
  ["FrankenLean", "franken_lean", "Proof", "Re-checks the theorems, independently."],
  ["FrankenTUI", "frankentui", "Dashboard", "The lab in your terminal."],
  ["fastmcp_rust", "fastmcp_rust", "Agent access", "The MCP server that agents talk to."],
];

const roadmap: [name: string, text: string][] = [
  ["The design", "The full plan, public and searchable."],
  ["Foundations", "One kernel running on the CPU, CUDA and Metal."],
  ["The CPU studio", "Design checks, the referee and the first mission answers, with no GPU needed."],
  ["GPU batches", "256 different robots at once, on CUDA and on Apple silicon."],
  ["Worlds and adversaries", "Generated buildings, Nemesis, Autopsy and the sealed exam."],
  ["Families at scale", "Shared policies, co-design and big mission queries."],
  ["The workshop", "Calibration against real robots, certificates and build packets."],
  ["The ten-times test", "Sealed missions, head to head with the best stack anyone can assemble."],
];

const organ = (ref: Refs, side: "left" | "right") =>
  organs
    .filter((o) => o[4] === side)
    .map(
      ([key, name, s, text]) =>
        `<li data-organ="${key}"><a href="${ref(s)}"><b>${name}</b></a><p>${copy(text)}</p></li>`,
    )
    .join("");

export const renderTest = (ref: Refs) =>
  chapterOpen(test) +
  section({
    ref,
    id: "anatomy",
    eyebrow: "Twenty-three parts",
    title: "Assembled, *like the monster*.",
    kicker: "Every subsystem is named for what it does to the robot. Point at a part to meet it.",
    body: figure({
      name: "anatomy",
      todo: "Point at a name, or at the machine",
      ref,
      className: "plain",
      body: `<div class="anatomy"><ul class="organs left">${organ(ref, "left")}</ul><div class="plate">${plate()}</div><div class="organ-now" data-now></div><ul class="organs right">${organ(ref, "right")}</ul></div>`,
    }),
  }) +
  section({
    ref,
    id: "symmetry",
    eyebrow: "Symmetry",
    title: "Teach one leg, *get the rest*.",
    kicker:
      "A robot dog’s legs mirror each other, so a policy can share what it learns. Bolt on a lidar or an off-center load and some of that symmetry goes. FrankenRobotics works out what’s left.",
    more: ["Two kinds of symmetry", "C-09"],
    body: figure({
      name: "symmetry",
      todo: "Mount something and watch the symmetry change",
      kind: "live",
      ref,
      body: `<div class="symm"><div class="symm-view" data-view></div><div class="symm-side"><div class="toggles">${toggle("lidar", "Forward lidar", true)}${toggle("payload", "Payload on one side")}${toggle("goal", "Told to go forward")}${toggle("knees", "All knees bend back")}${toggle("gear", "One gearbox swapped")}</div><div class="groups"><div><small>The mechanism</small><b data-mech>C₂ × C₂</b><span data-mech-n></span></div><div><small>With sensors and load</small><b data-obs>C₂</b><span data-obs-n></span></div></div><p class="say" data-say></p></div></div>`,
    }),
  }) +
  section({
    ref,
    id: "constructions",
    eyebrow: "Forty constructions",
    title: "The math, *piece by piece*.",
    kicker: "Each one replaces a guess or a hack with something you can calculate. Pick one to see what it buys.",
    more: ["All forty, in full", "10"],
    body: `<div class="fig elements reveal" id="fig-constructions" data-fig="constructions"><div class="seg" role="group" aria-label="Kind" data-seg="tier">${[
      ["", "All"],
      ["S", "Core"],
      ["F", "Frontier"],
      ["M", "Moonshot"],
    ]
      .map(([v, t]) => `<button type="button" data-v="${v}" aria-pressed="${!v}">${t}</button>`)
      .join("")}</div><div class="element-grid"><ol>${constructions
      .map(
        (c, i) =>
          `<li data-tiers="${c.tiers.join("")}" data-good="${escape(good[i]![1])}" data-title="${escape(c.title)}"><a href="${c.href}" data-i="${i}" class="el t-${c.tiers[0]}"><span class="num">${c.id.slice(2)}</span><b>${good[i]![0]}</b></a></li>`,
      )
      .join("")}</ol><aside class="element-card" data-card aria-live="polite"><p class="micro">Pick one</p><p class="hint">What it’s good for shows up here.</p></aside></div></div>`,
  }) +
  section({
    ref,
    id: "bench",
    eyebrow: "Worked by hand",
    title: "Check its *arithmetic*.",
    kicker: "Six problems small enough for paper. Each shows the right answer beside the mistake it catches.",
    more: ["The whole answer book", "appendix-m"],
    body: figure({
      name: "bench",
      todo: "Pick a problem",
      kind: "live",
      ref,
      body: `${seg(
        "bench",
        "Problem",
        [
          ["arm", "Two-link arm"],
          ["shell", "Solid or hollow"],
          ["mid", "Between two good designs"],
          ["stage", "Two stages at 95%"],
          ["cayley", "One big turn"],
          ["reads", "Reading it again"],
        ],
        "arm",
      )}<div class="bench" data-bench></div>`,
    }),
  }) +
  section({
    ref,
    id: "suite",
    eyebrow: "The Franken suite",
    title: "Stitched from *parts that work*.",
    kicker: "FrankenRobotics stands on a family of Rust projects by the same author. These eight do the heaviest lifting.",
    body: `<ul class="suite">${suite
      .map(
        ([name, slug, role, text]) =>
          `<li class="reveal"><a href="https://github.com/Dicklesworthstone/${slug}"><p class="micro">${role}</p><h4>${name}${icon("out")}</h4><p>${text}</p></a></li>`,
      )
      .join("")}</ul>`,
  }) +
  section({
    ref,
    id: "roadmap",
    eyebrow: "Roadmap",
    title: "What ships *first*.",
    kicker: "The design is public today. The build follows in this order, so there’s something to use long before the end.",
    more: ["Every milestone", "43"],
    body: `<ol class="road">${roadmap
      .map(
        ([name, text], i) =>
          `<li class="reveal${i ? "" : " now"}"><span class="road-dot" aria-hidden="true"></span><div><p class="micro">${i ? `Step ${i}` : "Today"}</p><h4>${name}</h4><p>${text}</p></div></li>`,
      )
      .join("")}</ol>`,
  }) +
  chapterClose;
