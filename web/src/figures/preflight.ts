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
      ["", "all eight checks passed"],
      ["dim", "next: frob control synth quad-a"],
    ],
  },
  knee: {
    stage: 6,
    head: "frob design check quad-a --json",
    lines: [
      ["bad", "exit 2 · check 6, sizing"],
      ["", "the knees are too weak to climb a step"],
      ["", "they need 31% more torque than these motors give"],
      ["warn", "repair: swap-actuator --joints '*.knee' --to qdd-l"],
    ],
    fix: "swap-actuator '*.knee' → qdd-l",
  },
  roller: {
    stage: 4,
    head: "frob design check mecanum-b --json",
    lines: [
      ["bad", "exit 2 · check 4, motion"],
      ["", "this base can't slide sideways"],
      ["", "all four wheels carry their rollers the same way,"],
      ["", "so it controls 2 directions out of 3"],
      ["warn", "repair: flip the rollers on one diagonal"],
    ],
    fix: "flip the rollers on one diagonal",
  },
  door: {
    stage: 7,
    head: "frob design check carrier-c --scene kitchen-17 --json",
    lines: [
      ["bad", "exit 2 · check 7, place"],
      ["", "no route through kitchen 17"],
      ["", "the tight spot is door D, on every possible path"],
      ["dim", "fixes for the room are listed beside fixes for the robot"],
      ["warn", "repair: narrow the track until it passes"],
    ],
    fix: "narrow the track",
  },
  beacon: {
    stage: 8,
    head: "frob design check scout-2 --task pick-side --json",
    lines: [
      ["bad", "exit 2 · check 8, information"],
      ["", "from its two beacons, left and right look identical"],
      ["", "both sides read √2, and they call for opposite moves"],
      ["", "no amount of training can fix that"],
      ["warn", "repair: add a beacon at (0, 2), which reads 1 or 3"],
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
          fix.textContent = c.fix ? "Apply the repair" : "Nothing to repair";
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
