import { constructions } from "../catalogue";
import { escape } from "../layout";
import { chapterClose, chapterOpen, copy, figure, icon, range, section, seg, spell, type Chapter, type Refs } from "../parts";

export const evidence: Chapter = {
  id: "evidence",
  number: "05",
  label: "The evidence",
  short: "Evidence",
  icon: "evidence",
  colour: "#b7a2fb",
  title: "Claims that stay in their *class*.",
  thesis:
    "An agent that optimizes will find any path by which a lucky run can be reported as a proof. So what a result claims, how strongly, and which input limits it is a type that no one can set by hand.",
  sections: [
    ["claims", "Seven claims, four strengths"],
    ["shrink", "The smallest failing world"],
    ["autopsy", "The cause of a failure"],
    ["retries", "Five hundred retries"],
    ["bench", "Check it by hand"],
    ["constructions", "Forty constructions"],
  ],
};

const claims: [name: string, meaning: string, not: string][] = [
  ["Proof", "A stated proposition holds in the declared model.", "Says nothing about floating-point results or the world."],
  ["Known answer", "The code recovers an analytic truth.", "Says nothing away from the tested cases."],
  ["Parity", "It reproduces a reference tool or a published number.", "Two programs that share a solver share its errors."],
  ["Fidelity", "A fast tier agrees with a slower, more trusted one.", "The trusted tier may be wrong."],
  ["Design comparison", "One design beats another, or meets a mission, on a stated family of worlds.", "Says nothing outside that family."],
  ["Reality", "The simulation agrees with physical measurement.", "Everywhere uncalibrated, the report says no-claim."],
  ["Performance", "A speed on named hardware.", "Says nothing about another workload or machine."],
];

export const renderEvidence = (ref: Refs) =>
  chapterOpen(evidence) +
  section({
    ref,
    id: "claims",
    index: "05.1",
    eyebrow: "The claim product",
    title: "Seven claims, four *strengths*.",
    kicker:
      "Every result makes one kind of claim, and its strength is the weakest of its inputs. Change an input and see what the report is allowed to say. [§8](§8)",
    body:
      `<ol class="claims reveal">${claims
        .map(
          ([name, meaning, not], i) =>
            `<li><span class="num">${i + 1}</span><h4>${name}</h4><p>${copy(meaning)}</p><small>${copy(not)}</small></li>`,
        )
        .join("")}</ol>` +
      figure({
        name: "claims",
        no: "05.1",
        label: "The worked case of §8.2",
        title: "Find the *weakest* input.",
        basis: "computed",
        source: [["§8.2", ref("8.2")]],
        note: "The combination rule is a minimum. Tier takes the lowest, and the three authorization flags have no constructor that yields true.",
        ref,
        body: `<div class="claimfig"><div class="inputs">${(
          [
            ["kernel", "Kernel identity", ["Certified", "Tested"], "Certified", "Group membership of the step"],
            ["friction", "Friction coefficient", ["Assumed", "Bench-measured", "Field-calibrated"], "Assumed", "A field of the interface card"],
            ["study", "Paired study", ["50 loose runs", "Anytime-valid"], "Anytime-valid", "On common world addresses"],
            ["tier", "Physics tier", ["GPU", "Referee"], "GPU", "Which solver produced it"],
            ["real", "Real-robot data", ["None", "Band issued"], "None", "Predictions frozen first"],
          ] as const
        )
          .map(
            ([key, name, options, on, note]) =>
              `<div class="input" data-input="${key}"><div><b>${name}</b><small>${note}</small></div>${seg(key, name, options.map((o) => [o, o] as [string, string]), on)}</div>`,
          )
          .join("")}</div><div class="ladder" data-ladder></div><div class="report" aria-live="polite"><p class="micro">The report</p><pre data-report></pre><p class="nextaction" data-next></p></div></div>`,
      }),
  }) +
  section({
    ref,
    id: "shrink",
    index: "05.2",
    eyebrow: "Nemesis",
    title: "The smallest *failing* world.",
    kicker:
      "A failing world is a list of edits to a passing one. Delta debugging removes edits until every one that is left is needed, which can still miss a smaller failure. So for short lists the plan also tries every pair. [§26.7](§26.7)",
    body: figure({
      name: "shrink",
      no: "05.2",
      label: "Fails if {a, b} or {c, d, e}",
      title: "Run the *shrink*.",
      basis: "computed",
      source: [["§26.7", ref("26.7")]],
      note: "The counterexample of §26.7, with its list order a, c, d, e, b. The edits' names here are made up.",
      ref,
      body: `<div class="shrink"><div class="chips" data-chips></div><div class="shrink-log" data-log></div><div class="shrink-foot"><div class="btnrow"><button type="button" class="btn go sm" data-step>Next test</button><button type="button" class="btn sm" data-run>${icon("play")}Run</button><button type="button" class="btn sm" data-reset>${icon("redo")}Start over</button></div><p data-say aria-live="polite"></p></div></div>`,
    }),
  }) +
  section({
    ref,
    id: "autopsy",
    index: "05.3",
    eyebrow: "Autopsy",
    title: "The cause of a *failure*.",
    kicker:
      "Fork the replay just before it fails, change one thing, and run it again. Then do the same with a change that should not matter. What is left after subtracting the placebo is the contrast. [§27.6](§27.6)",
    body: figure({
      name: "autopsy",
      no: "05.3",
      label: "200 failing worlds, re-run from their forks",
      title: "Subtract the *placebo*.",
      basis: "illustrative",
      source: [["§27.6", ref("27.6")]],
      note: "Rescue counts 164, 38 and 12 are the plan's illustrative numbers. Raise the placebo above 0.2 and the rule names nobody.",
      ref,
      body: `<div class="autopsy"><div class="dots3" data-dots></div><div class="autopsy-side"><div class="controls stack">${range("torque", "Rescued: torque limit off", 0, 200, 1, 164, "164")}${range("friction", "Rescued: friction at nominal", 0, 200, 1, 38, "38")}${range("placebo", "Rescued: placebo", 0, 100, 1, 12, "12")}${range("flip", "Flip rate at 10⁻¹²", 0, 0.3, 0.01, 0.04, "0.04")}</div><div class="verdicts" data-out aria-live="polite"></div></div></div>`,
    }),
  }) +
  section({
    ref,
    id: "retries",
    index: "05.4",
    eyebrow: "NS-17",
    title: "Five hundred *retries*.",
    kicker:
      "An agent submits design after design to the same gate. None of them is any good. Testing each at 5% lets one through within the first few dozen. Spending the error budget as α ÷ i(i + 1) keeps the whole campaign under 5%. [NS-17](§7)",
    body: figure({
      name: "retries",
      no: "05.4",
      label: "Campaigns with no real improvement",
      title: "Let the agent keep *trying*.",
      basis: "toy",
      source: [
        ["§24", ref("24")],
        ["NS-17", ref("7")],
      ],
      note: "Each submission draws a uniform p-value. 400 campaigns per rule, a seeded generator in this page.",
      ref,
      body: `<div class="retries"><div class="retries-grids"><div><p class="micro asis">Every test at 5%</p><canvas data-naive width="400" height="400" aria-label="Campaigns under the naive rule"></canvas><p class="big out" data-naive-n>0</p><small>campaigns promoted something worthless</small></div><div><p class="micro asis">Test i at 5% ÷ i(i + 1)</p><canvas data-spend width="400" height="400" aria-label="Campaigns under the spending rule"></canvas><p class="big in" data-spend-n>0</p><small>campaigns promoted something worthless</small></div></div><div class="retries-plot"><div data-plot></div><p class="num" data-count>Submission 0 of 500</p><div class="btnrow"><button type="button" class="btn go sm" data-run>${icon("play")}Submit 500 times</button><button type="button" class="btn sm" data-seed>${icon("redo")}New seed</button></div></div></div>`,
    }),
  }) +
  section({
    ref,
    id: "bench",
    index: "05.5",
    eyebrow: "Known answers",
    title: "Check it *by hand*.",
    kicker:
      "Six of the plan's fixtures, small enough to do on paper. Each is set beside the plausible mistake it exists to catch. [Appendix M](§appendix-m)",
    body: figure({
      name: "bench",
      no: "05.5",
      label: "The fixture and the plausible mistake",
      title: "Six answers known in *advance*.",
      basis: "computed",
      source: [
        ["Appendix M", ref("appendix-m")],
        ["C-04", ref("C-04")],
      ],
      ref,
      body: `${seg(
        "bench",
        "Fixture",
        [
          ["arm", "Two-link arm"],
          ["shell", "A box, solid or shell"],
          ["mid", "Between two good designs"],
          ["stage", "Two stages at 95%"],
          ["cayley", "One Cayley step"],
          ["reads", "Reading it again"],
        ],
        "arm",
      )}<div class="bench" data-bench></div>`,
    }),
  }) +
  section({
    ref,
    id: "constructions",
    index: "05.6",
    eyebrow: "The mathematics",
    title: `${spell(constructions.length, true)} *constructions*.`,
    kicker:
      "Each is a piece of mathematics with a job. It says what it replaces, what it pays for, and the test that would retire it. Frontier and moonshot items run a pilot first and have a fallback named in advance. [§10](§10)",
    body: `<div class="fig elements reveal" id="fig-constructions" data-fig="constructions"><div class="seg" role="group" aria-label="Tier" data-seg="tier">${[
      ["", "All"],
      ["S", "Solid"],
      ["F", "Frontier"],
      ["M", "Moonshot"],
    ]
      .map(([v, t]) => `<button type="button" data-v="${v}" aria-pressed="${!v}">${t}</button>`)
      .join("")}</div><div class="element-grid"><ol>${constructions
      .map(
        (c, i) =>
          `<li data-tiers="${c.tiers.join("")}"><a href="${c.href}" data-i="${i}" class="el t-${c.tiers[0]}"><span class="num">${c.id.slice(2)}</span><b>${escape(c.title)}</b><i class="tiers">${c.tiers.map((k) => `<em class="${k}">${k}</em>`).join("")}</i></a></li>`,
      )
      .join("")}</ol><aside class="element-card" data-card aria-live="polite"><p class="micro">Pick a construction</p><p class="hint">What it replaces, what it pays and its test appear here, in the plan's words.</p></aside></div></div>`,
  }) +
  chapterClose;

