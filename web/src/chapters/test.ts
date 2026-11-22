import { chapterClose, chapterOpen, copy, figure, section, seg, type Chapter, type Refs } from "../parts";

export const test: Chapter = {
  id: "test",
  number: "06",
  label: "The test",
  short: "Test",
  icon: "test",
  colour: "#fb7185",
  title: "How this could *fail*.",
  thesis:
    "Ten-fold is a target: time to an independently accepted answer, against the strongest rival that can be composed from existing tools. The plan states what it expects, what every outcome would be called, and which parts are dropped.",
  sections: [
    ["ranges", "What to expect"],
    ["outcomes", "The report, already worded"],
    ["not", "What will not deliver it"],
    ["milestones", "Nine milestones"],
    ["status", "Where it stands"],
  ],
};

/** Expected ratio by mission class, §2.6. Drawn on a log axis from 0.5 to 100. */
const ranges: [name: string, lo: number, hi: number, shown: string, must: string][] = [
  ["Quadruped briefs", 3, 10, "3 to 10", "Many designs per turn, under about 1% of them trained, and cheap rungs that rank designs as training would."],
  ["Humanoid briefs", 5, 20, "5 to 20", "A baseline or family policy stands in for training when designs are ranked. Otherwise near parity."],
  ["Wheeled, tracked and hybrid, indoors", 3, 10, "3 to 10, low confidence", "Clearance and static analysis remove candidates before any training."],
  ["Family queries with amendments, warm", 1, 3, "1 to 3", "Deriving the abstraction is cheaper than an expert doing it by hand. Cold may be below 1."],
  ["Finding and repairing failures", 2, 20, "2 to 20 in episodes", "A good distress score and a witness that the failure was avoidable. Up to 100 is a target, and about 3 in repair turns."],
  ["Facilities and several robots", 0, 0, "not estimated", "A ratio is claimed only for what was run."],
];
const X = (v: number) => ((Math.log10(v) - Math.log10(0.5)) / (Math.log10(100) - Math.log10(0.5))) * 100;

const notDeliver: [what: string, number: string, why: string][] = [
  ["A faster step", "≤ 7.7×", "Throughput is a parity requirement. An infinitely fast step leaves the rest of training in place."],
  ["Compiling a kernel per design", "≈ 1.2×", "Specializing the recursions is capped near here. An optimization for survivors at most."],
  ["Half precision for contact", "refuted", "Not built."],
  ["Sampling control instead of training", "4,000×", "About this many times the cost of a policy rollout per simulated second. Kept as a fallback."],
  ["One universal policy", "unaffordable", "Out of reach at an affordable pretraining budget."],
  ["Static checks alone", "≤ 1.19×", "Their value is feedback in seconds. They prune few trainings."],
  ["Reusing rollouts after an edit", "≤ 10%", "Exact reuse is at most about this share of compute."],
  ["A faster solver for family queries", "≈ 1.46×", "A pilot measured 1.345× warm, under a ceiling set by work the solver does not touch."],
];

const milestones: [id: string, name: string, weeks: number, at: number, path: boolean, exit: string, stories: string][] = [
  ["M0", "Spine and airlock", 2, 0, true, "Foundations admitted, one kernel on three backends, and the rival's loop timed by phase", ""],
  ["M1", "Referee studio, first mission slice, CPU only", 5, 2, true, "The docking scenario, answered and audited before any GPU work", "NS-01 NS-07 NS-13 NS-22"],
  ["M2", "Batch, backends, families", 6, 7, true, "Kill decisions for kernels, contact, Metal and mixed batches on record", "NS-02 NS-10 NS-14"],
  ["M3", "Places, tasks, adversaries, qualification", 6, 13, true, "The partial ten-fold gate recorded, whatever the ratio", "NS-03 NS-04 NS-09 NS-15 NS-16 NS-17"],
  ["M4", "Amortized control, co-design, queries at scale", 8, 19, true, "Rank agreement measured and the family-policy bets decided", "NS-05 NS-06 NS-08 NS-18 NS-19 NS-23"],
  ["M5", "Proofs, reality, manufacture", 6, 21, false, "The Reality lane has a record, or a recorded no-claim", "NS-11 NS-12 NS-20 NS-21"],
  ["M7", "The ten-fold tournament", 6, 27, true, "The measured ratio, published with its phase ledger", "NS-24"],
];

export const renderTest = (ref: Refs, version: string) =>
  chapterOpen(test) +
  section({
    ref,
    id: "ranges",
    index: "06.1",
    eyebrow: "Expected ranges",
    title: "What to *expect*.",
    kicker:
      "Against a rival that trains a policy for every design, these are the ranges the ceilings allow. Against the strongest composed rival on fixed-topology CUDA missions, the expected ratio is near one. [§2.6](§2.6)",
    body: figure({
      name: "ranges",
      no: "06.1",
      label: "Hypothesis, to be replaced by measurement",
      title: "One class reaches past *ten*.",
      basis: "plan",
      source: [["§2.6", ref("2.6")]],
      note: "Ratios of time to a decision. They are not forecasts with probabilities.",
      ref,
      body: `<div class="rangechart"><div class="rc-axis" aria-hidden="true">${[1, 3, 10, 30, 100].map((v) => `<span style="left:${X(v).toFixed(2)}%"${v === 10 ? ' class="ten"' : ""}>${v}×</span>`).join("")}</div>${ranges
        .map(
          ([name, lo, hi, shown, must]) =>
            `<div class="rc-row"><div class="rc-name"><b>${name}</b><span>${shown}</span></div><div class="rc-track">${[1, 3, 10, 30, 100].map((v) => `<i class="tick${v === 10 ? " ten" : ""}" style="left:${X(v).toFixed(2)}%"></i>`).join("")}${hi ? `<i class="span" style="left:${X(lo).toFixed(2)}%;width:${(X(hi) - X(lo)).toFixed(2)}%"></i>` : ""}</div><p>${copy(must)}</p></div>`,
        )
        .join("")}</div>`,
    }),
  }) +
  section({
    ref,
    id: "outcomes",
    index: "06.2",
    eyebrow: "Outcome rules",
    title: "The report, *already worded*.",
    kicker:
      "Thresholds are fixed when the first suite hash is committed and never changed after a sealed run. Drag the measured ratio to read the sentence the report must print. [§39.10](§39.10)",
    body: figure({
      name: "outcomes",
      no: "06.2",
      label: "One stratum of the tournament",
      title: "Choose a *result*.",
      basis: "plan",
      source: [["§39.10", ref("39.10")]],
      note: "Sentences are the plan's. The guard, the futility stop and the tie tests are simplified here to the ratio and two switches.",
      ref,
      body: `<div class="outcomes"><div class="outcome-track" data-track><i class="needle" data-needle></i><span class="z z0">rival faster</span><span class="z z1">withdrawn</span><span class="z z2">measured</span><span class="z z3">ten-fold</span></div><label class="range" data-range="r"><span><b>Measured ratio</b><output>4.2</output></span><input type="range" min="-30" max="140" step="1" value="62" style="--p:54%"></label><div class="outcome-opts">${seg(
        "tie",
        "Against a short program on the rival's engine",
        [
          ["no", "The short program loses"],
          ["yes", "The short program ties"],
        ],
        "no",
      )}${seg(
        "guard",
        "Lower confidence bound",
        [
          ["no", "Guard not met"],
          ["yes", "Guard met"],
        ],
        "yes",
      )}</div><div class="sentence" aria-live="polite"><p class="micro" data-label></p><blockquote data-text></blockquote><p data-does></p></div></div>`,
    }),
  }) +
  section({
    ref,
    id: "not",
    index: "06.3",
    eyebrow: "Closed doors",
    title: "What will *not* deliver it.",
    kicker:
      "No work package is funded on a hope the evidence has already closed. No single mechanism delivers ten-fold, and no conjunction has been shown to. [§2.5](§2.5)",
    layout: "split",
    body: `<ul class="ruled doors">${notDeliver
      .map(
        ([what, n, why]) =>
          `<li class="reveal"><span class="num">${n}</span><div><h4>${what}</h4><p>${copy(why)}</p></div></li>`,
      )
      .join("")}</ul>`,
  }) +
  section({
    ref,
    id: "milestones",
    index: "06.4",
    eyebrow: "Delivery",
    title: "Nine *milestones*.",
    kicker:
      "The first studio runs on a CPU, so it ships even if every GPU and proof foundation fails its admission. The critical path adds up to 33 weeks; M6 and M8 are per-epic and sit outside it. [§43](§43)",
    body: figure({
      name: "milestones",
      no: "06.3",
      label: "Planning estimates, elapsed weeks for a swarm",
      title: "Thirty-three weeks on the *critical path*.",
      basis: "plan",
      source: [["§43.1", ref("43.1")]],
      note: "Estimates exist so that a schedule gate can ask a question. No gate cites them.",
      ref,
      body: `<ol class="gantt">${milestones
        .map(
          ([id, name, weeks, at, path, exit, stories]) =>
            `<li class="${path ? "on" : "off"}"><div class="g-name"><span class="num">${id}</span><b>${name}</b><small>${exit}</small></div><div class="g-track"><i style="--at:${((at / 33) * 100).toFixed(2)}%;--w:${((weeks / 33) * 100).toFixed(2)}%"><em>${weeks} wk</em></i></div><p class="tags">${stories
              .split(" ")
              .filter(Boolean)
              .map((s) => `<a class="tag" href="${ref(s)}">${s}</a>`)
              .join("")}</p></li>`,
        )
        .join("")}</ol>`,
    }),
  }) +
  section({
    ref,
    id: "status",
    index: "06.5",
    eyebrow: "Honest status",
    title: "Where it *stands*.",
    kicker: `The plan is at version ${version}. No FrankenRobotics code has been released and no FrankenRobotics physics step has been measured.`,
    layout: "split",
    body: `<div class="status"><article class="reveal"><p class="micro">Done</p><h4>The plan, and the checks behind it</h4><ul><li>A plan with 71 published sections and appendices, searchable here.</li><li>An audit of the strongest incumbents at pinned versions, with sixteen claims withdrawn.</li><li>Paper-scale pilots of the mission query, recorded with their exact counts.</li><li>A known-answer suite and a catalogue of counterexamples.</li></ul></article><article class="reveal"><p class="micro">Required before anything is called implemented</p><h4>Eight conditions, all of them</h4><ol class="eight"><li>Code in the crate the plan names</li><li>A contract with ten sections</li><li>Known answers against a pinned oracle</li><li>A claim product on every output</li><li>A CPU reference path</li><li>Reachable through the command line</li><li>Cancellation tested</li><li>A recorded pass with measured values</li></ol><p><a class="txt" href="${ref("0.2")}">Read the rule in §0.2</a></p></article><article class="reveal"><p class="micro">If the speed claim is withdrawn</p><h4>What stays</h4><p>The plan keeps the claims that never depended on speed. Any world re-runs alone on the CPU from its address. Comparisons keep error control through an agent's retries. Family answers are checked by an auditor that shares no code with their producer. Failures arrive as small replayable worlds with a cause found by intervention. Each is still to be built and tested.</p><p><a class="txt" href="${ref("39.10")}">Read it in §39.10</a></p></article></div>`,
  }) +
  chapterClose;
