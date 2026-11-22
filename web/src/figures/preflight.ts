import { $, $$, reduced, seg } from "./lib";

interface Case {
  /** The stage that refuses, or 0. */
  stage: number;
  head: string;
  lines: [cls: string, text: string][];
  fix?: string;
}
const cases: Record<string, Case> = {
  ok: {
    stage: 0,
    head: "frob design check quad-a --json",
    lines: [
      ["ok", "exit 0"],
      ["", "all eight stages passed"],
      ["dim", "evidence class and model identity attached"],
      ["dim", "next_actions: frob control synth quad-a"],
    ],
  },
  knee: {
    stage: 6,
    head: "frob design check quad-a --json",
    lines: [
      ["bad", "exit 2 · FROB-REF-131"],
      ["", "stage 6, statics and sizing"],
      ["", "operating point outside the envelope: *.knee"],
      ["", "needs 31% more torque than the card allows"],
      ["dim", "at the stepping-up point"],
      ["warn", "repair 1: swap-actuator --joints '*.knee' --to qdd-l"],
    ],
    fix: "swap-actuator '*.knee' → qdd-l",
  },
  roller: {
    stage: 4,
    head: "frob design check mecanum-b --json",
    lines: [
      ["bad", "exit 2 · FROB-REF-118"],
      ["", "stage 4, motion"],
      ["", "growth vector (2, 3); this class of base must have (3)"],
      ["", "twist to wheel rates has rank 2, not 3"],
      ["dim", "curvature passes here, so curvature is the wrong test"],
      ["warn", "repair 1: flip the roller axes on one diagonal"],
    ],
    fix: "flip roller axes on one diagonal",
  },
  door: {
    stage: 7,
    head: "frob design check carrier-c --scene kitchen-17 --json",
    lines: [
      ["bad", "exit 2 · FROB-REF-119"],
      ["", "stage 7, place"],
      ["", "no admissible route; bottleneck: door D"],
      ["", "an impossibility cut holds for every path"],
      ["dim", "retrofits from the catalogue are listed beside design edits"],
      ["warn", "repair 1: narrow the track until the clearance complex passes"],
    ],
    fix: "narrow the track",
  },
  beacon: {
    stage: 8,
    head: "frob design check scout-2 --task pick-side --json",
    lines: [
      ["bad", "exit 2 · FROB-REF-120"],
      ["", "stage 8, information: aliased-all-probes"],
      ["", "worlds (0, 1) and (0, −1) read √2 and √2"],
      ["", "and need opposite actions"],
      ["dim", "the local observability test passes at both points"],
      ["warn", "repair 1: add a beacon at (0, 2), which reads 1 or 3"],
    ],
    fix: "add a beacon at (0, 2)",
  },
};

export default function mount(root: HTMLElement) {
  const stages = $$(root, "[data-stage]");
  const out = $(root, "[data-out]");
  const fix = $<HTMLButtonElement>(root, "[data-fix]");
  const check = $(stages[0]!, ".state").innerHTML;
  const cross = check.replace(/ d="[^"]+"/, ' d="M6 6l12 12M18 6 6 18"');
  let timer = 0;
  let repaired = "";
  const print = (c: Case) => {
    out.innerHTML =
      `<span class="dim">$ ${c.head}</span>\n` +
      c.lines.map(([cls, t]) => (cls ? `<span class="${cls}">${t}</span>` : t)).join("\n");
  };
  const run = (key: string) => {
    clearTimeout(timer);
    const c = cases[key]!;
    for (const s of stages) s.className = "";
    out.innerHTML = `<span class="dim">$ ${c.head}</span>`;
    if (repaired) out.innerHTML += `\n<span class="ok">applied: ${repaired}</span>`;
    fix.disabled = true;
    let i = 0;
    const next = () => {
      if (i > 0) {
        const failed = c.stage === i;
        const s = stages[i - 1]!;
        s.className = failed ? "fail" : "pass";
        $(s, ".state").innerHTML = failed ? cross : check;
        if (failed || i === 8) {
          for (const rest of stages.slice(i)) rest.className = "unrun";
          print(c);
          if (repaired) out.innerHTML += `\n<span class="ok">applied: ${repaired}</span>`;
          repaired = "";
          fix.disabled = !c.fix;
          fix.textContent = c.fix ? "Apply the first repair" : "Nothing to repair";
          return;
        }
      }
      stages[i++]!.className = "run";
      if (reduced) next();
      else timer = window.setTimeout(next, 260);
    };
    next();
  };
  const state = seg(root, "case", (v) => {
    repaired = "";
    run(v);
  });
  fix.addEventListener("click", () => {
    const c = cases[state.value]!;
    if (!c.fix) return;
    repaired = c.fix;
    state.set("ok");
    run("ok");
  });
  // Runs once when the figure scrolls into view.
  new IntersectionObserver(
    (entries, io) => {
      if (!entries[0]!.isIntersecting) return;
      io.disconnect();
      run(state.value);
    },
    { threshold: 0.3 },
  ).observe(root);
}
