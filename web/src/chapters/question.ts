import { stories } from "../catalogue";
import { chapterClose, chapterOpen, copy, icon, figure, range, section, seg, type Chapter, type Refs, spell } from "../parts";

export const question: Chapter = {
  id: "question",
  number: "01",
  label: "The question",
  short: "Question",
  icon: "question",
  colour: "#fbbf24",
  title: "Where the *day* goes.",
  thesis:
    "A design agent edits a robot, trains a controller, watches it fail and edits again. Published figures put most of that day in training, so the plan starts by timing it.",
  sections: [
    ["what", "Six products, one model"],
    ["stories", "Twenty-four stories"],
    ["loop", "What one edit costs"],
    ["ceilings", "Four shortcuts and their ceilings"],
    ["withdrawn", "Claims we withdrew"],
  ],
};

const products: [title: string, parts: [string, string][], text: string][] = [
  [
    "A proving ground",
    [
      ["Arena", "24"],
      ["Cortex", "23"],
    ],
    "Thousands of worlds at once on a GPU, with legged, wheeled, tracked and hybrid robots in generated terrain and interiors. Results are qualified in a lane that the search cannot read.",
  ],
  [
    "A design studio for agents",
    [
      ["Atelier", "28"],
      ["Tether", "32"],
    ],
    "A command line and an MCP server. An agent makes a robot, checks it without simulating it, runs it, compares two versions, and asks which earlier verdicts survive an edit.",
  ],
  [
    "A mission-query engine",
    [["Augur", "25"]],
    "Give it a mission, a parts catalogue, and what the robot can observe and when. It returns the designs that can do the job, the ones that cannot, the undecided rest, and the cheapest experiment that would shrink the gap.",
  ],
  [
    "An adversary and an autopsy",
    [
      ["Nemesis", "26"],
      ["Autopsy", "27"],
    ],
    "A search for the worlds that break a design, each shrunk to the fewest edits that still break it. The cause of a failure is found by changing one thing in a forked replay.",
  ],
  [
    "A path to reality and the workshop",
    [
      ["Ground", "30"],
      ["Foundry", "31"],
    ],
    "Calibration against imported measurements, with predictions frozen before the comparison, and build packets that list parts, tolerances and fatigue margins.",
  ],
  [
    "A mathematical instrument",
    [
      ["Marrow", "13"],
      ["Lemma", "12"],
      ["Spark", "21"],
    ],
    "One Lie-algebraic model generates the GPU kernels, supplies the test oracles and prunes designs before any simulation runs.",
  ],
];

const withdrawn: [claim: string, why: string][] = [
  [
    "Deterministic GPU replay",
    "Newton 1.8.0 and Isaac Lab 3.0.0 both document opt-in bitwise run-to-run determinism.",
  ],
  [
    "Different robots in one GPU batch",
    "ManiSkill 3.0.1 merges articulations with different structures, and two Newton solvers document heterogeneous worlds.",
  ],
  [
    "Baseline control with no training needed",
    "Sampling-based predictive control runs a real quadruped at 50 Hz on a desktop CPU. Each published controller still uses cost weights chosen by hand.",
  ],
  [
    "An agent interface to a simulator",
    "Isaac Sim ships an MCP server for documentation, and a community server exposes 42 tools that drive the simulator.",
  ],
  [
    "Agents learn of a mistake “six hours later”",
    "Published training takes about seven minutes for a flat-ground quadruped and about an hour for a default rough-terrain humanoid.",
  ],
  [
    "Ten-fold from a faster simulation step",
    "MuJoCo Warp already reports about 2.55 million humanoid world-steps per second. The step has no order of magnitude left to give.",
  ],
];

export const renderQuestion = (ref: Refs) =>
  chapterOpen(question) +
  section({
    ref,
    id: "what",
    index: "01.1",
    eyebrow: "What it is",
    title: "Six products, one *model*.",
    kicker:
      "They are planned to share one robot description, one ledger of evidence and one registry of commands, so that a fact learned in one is usable in the next and is withdrawn when an edit makes it false. [§1.5](§1.5)",
    layout: "split",
    body: `<ol class="ruled products">${products
      .map(
        ([name, parts, text], i) =>
          `<li class="reveal"><span class="num">${i + 1}</span><div><h4>${name}</h4><p>${copy(text)}</p></div><p class="tags">${parts
            .map(([p, s]) => `<a class="tag" href="${ref(s)}">${p}</a>`)
            .join("")}</p></li>`,
      )
      .join("")}</ol>`,
  }) +
  section({
    ref,
    id: "stories",
    index: "01.2",
    eyebrow: "North-Star scenarios",
    title: "What should *change*.",
    kicker: `The plan is held to ${spell(stories.length)} short stories. Each is tagged with the first milestone that can show it, and the wording below is the plan's own. Numbers in them are targets or illustrations unless the story says it is a known answer. [§7](§7)`,
    body: `<div class="fig storyfig reveal" id="fig-stories" data-fig="stories"><div class="story-rail"><div class="seg story-filter" role="group" aria-label="Milestone" data-seg="milestone"><button type="button" data-v="" aria-pressed="true">All</button>${[...new Set(stories.map((s) => s.milestone))]
      .sort()
      .map((m) => `<button type="button" data-v="${m}" aria-pressed="false">${m}</button>`)
      .join("")}</div><ol class="story-list">${stories
      .map(
        (s, i) =>
          `<li data-m="${s.milestone}"><a href="${s.href}" data-story="${i}"${i ? "" : ' aria-current="true"'}><span class="num">${s.id.slice(3)}</span><b>${s.title}</b><small>${s.milestone}</small></a></li>`,
      )
      .join("")}</ol></div><div class="story-stage">${stories
      .map(
        (s, i) =>
          `<article class="story${i ? "" : " on"}" data-panel="${i}" data-n="${s.id.slice(3)}" aria-labelledby="st-${i}"><p class="kick small"><span class="num">${s.id}</span><i></i>First shown at ${s.milestone}${s.later ? ` ${s.later}` : ""}</p><h4 id="st-${i}">${s.title}</h4><div class="ba${s.before ? "" : " solo"}">${s.before ? `<div class="before"><p class="micro">Before</p><p>${s.before}</p></div>` : ""}<div class="after"><p class="micro">${s.before ? "After" : "The story"}</p><p>${s.after}</p></div></div></article>`,
      )
      .join("")}<div class="story-nav"><button type="button" class="btn sm" data-prev>${icon("left")}Previous</button><span class="num" data-at>1 of ${stories.length}</span><button type="button" class="btn sm" data-next>Next${icon("right")}</button></div></div></div>`,
  }) +
  section({
    ref,
    id: "loop",
    index: "01.3",
    eyebrow: "The loop today",
    title: "What one *edit* costs.",
    kicker:
      "No one has published a phase ledger of a whole design loop on today's software, so the plan composes one from separate papers and puts measuring it first. [§2.2](§2.2)",
    body: figure({
      name: "loop",
      no: "01.3",
      label: "One iteration: edit, compile, train, evaluate",
      title: "Training is most of the *clock*.",
      basis: "published",
      source: [["§2.2", ref("2.2")]],
      note: "A composite joins figures from different stacks. It is not a timing of any one loop.",
      ref,
      body: `<div class="bars">
<div class="bar-row"><div class="bar-label"><b>Quadruped</b><span>about 5.9 minutes</span></div><div class="track"><i class="train" style="--w:85%"><span>Training, about 85%</span></i><i class="rest" style="--w:15%"><span>The rest</span></i></div></div>
<div class="bar-row"><div class="bar-label"><b>Humanoid</b><span>about 31 minutes</span></div><div class="track"><i class="train" style="--w:90%"><span>Training, over 90%</span></i><i class="rest" style="--w:10%"></i></div></div></div>
<div class="facts4"><p><b class="num">80 to 1,600</b>candidates trained per published campaign</p><p><b class="num">7.5 to 30</b>GPU-minutes per trained candidate</p><p><b class="num">15.8%</b>of candidates turn out degenerate after a full training run, with a strong language model proposing them</p><p><b class="num">None found</b>No published ten-fold gain in time to a qualified legged-robot design</p></div>`,
    }),
  }) +
  section({
    ref,
    id: "ceilings",
    index: "01.4",
    eyebrow: "The ceilings",
    title: "Four shortcuts and where they *stop*.",
    kicker:
      "Before any speed test is run, the plan computes the most it could show. Move the slider in each tab and watch the curve level off. [§2.3](§2.3)",
    body: figure({
      name: "ceilings",
      no: "01.4",
      label: "Speedup of the whole loop",
      title: "The dashed line is *ten*.",
      basis: "published",
      source: [["§2.3", ref("2.3")]],
      note: "Shares and times are from published runs; the curves are arithmetic on them.",
      ref,
      body: `${seg(
        "tab",
        "Shortcut",
        [
          ["engine", "A faster engine"],
          ["turn", "One design per turn"],
          ["cost", "Train fewer"],
          ["miss", "Miss no deadline"],
        ],
        "engine",
      )}
<p class="lead-in" data-text>The environment step is 87% of training for a rough-terrain humanoid, 85% for a dexterous hand and 46% for a cart-pole. However fast the step becomes, the other share of training stays.</p>
<div class="plot" data-plot></div>
<div class="controls">${range("x", "Engine speed-up", 0, 100, 1, 34, "× 10")}</div><div class="reads" data-reads></div>`,
    }),
  }) +
  section({
    ref,
    id: "withdrawn",
    index: "01.5",
    eyebrow: "The audit",
    title: "Claims we *withdrew*.",
    kicker:
      "The strongest incumbents were read at pinned versions. Sixteen statements from an earlier edition, or a careless reading of it, did not survive. Six of them are here; [all sixteen are in §1.3](§1.3).",
    layout: "split",
    body: `<ul class="ruled struck">${withdrawn
      .map(
        ([claim, why]) =>
          `<li class="reveal"><h4><s>${copy(claim)}</s></h4><p>${copy(why)}</p></li>`,
      )
      .join("")}</ul>
<aside class="callout reveal"><p class="micro">What is left to claim</p><blockquote>One design identity and one evidence ledger across static analysis, simulation tiers, statistics and solid mechanics.</blockquote><p>That is N13, word for word, and the plan calls it a hypothesis “whose only test is the end-to-end benchmark against a composed incumbent baseline”. <a class="txt" href="${ref("1.6")}">§1.6</a></p></aside>`,
  }) +
  chapterClose;
