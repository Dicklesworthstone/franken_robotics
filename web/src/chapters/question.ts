import { escape } from "../layout";
import { chapterClose, chapterOpen, copy, figure, icon, section, type Chapter, type Refs } from "../parts";
import { art, type Art } from "./art";
import { renderSession } from "./session";

export const question: Chapter = {
  id: "lab",
  number: "01",
  label: "The lab",
  short: "Lab",
  icon: "question",
  colour: "#fbbf24",
  title: "From a mission to *a machine*.",
  thesis:
    "Most robot projects start with a body and ask what it can do. FrankenRobotics starts with the job and works back to every body that could do it.",
  sections: [
    ["what", "What it does"],
    ["session", "Watch an agent run it"],
    ["loop", "Where the time goes"],
    ["stories", "What changes for you"],
  ],
};

const products: [art: Art, verb: string, name: string, text: string, parts: [string, string][]][] = [
  [
    "worlds",
    "Simulate",
    "Thousands of worlds at once",
    "Legged, wheeled, tracked and hybrid robots run side by side on the GPU, across generated terrain and through whole generated buildings.",
    [
      ["Arena", "24"],
      ["Cortex", "23"],
    ],
  ],
  [
    "studio",
    "Design",
    "A studio agents can drive",
    "One command line and one MCP server to create a robot, check it, run it, compare versions and see what an edit changed.",
    [
      ["Atelier", "28"],
      ["Tether", "32"],
    ],
  ],
  [
    "family",
    "Ask",
    "The whole family in one query",
    "Give it a mission and a parts catalog. Get every design that works, the reason each of the others fails, and the cheapest test for the close calls.",
    [["Augur", "25"]],
  ],
  [
    "crash",
    "Break",
    "An opponent for every robot",
    "Nemesis hunts for the world that beats your design and strips it to the details that matter. Autopsy replays the fall until the cause is clear.",
    [
      ["Nemesis", "26"],
      ["Autopsy", "27"],
    ],
  ],
  [
    "workshop",
    "Build",
    "A path to the workshop",
    "Calibrate against real measurements, then export a build packet with parts, solid geometry, tolerances and fatigue margins.",
    [
      ["Ground", "30"],
      ["Foundry", "31"],
    ],
  ],
  [
    "algebra",
    "Prove",
    "Math you can check",
    "One Lie-group model produces the GPU kernels, the tests that police them, and certificates that a separate program re-checks offline.",
    [
      ["Marrow", "13"],
      ["Lemma", "12"],
      ["Spark", "21"],
    ],
  ],
];

/** Scenarios of §7 in plain words. The numbers are the scenarios' own. */
const stories: [title: string, scenario: string, today: string, after: string][] = [
  [
    "Catch weak knees before training",
    "NS-01",
    "An agent edits the robot’s XML, starts training, and learns up to an hour later that the knees can’t carry it.",
    "`frob design check` answers in under two seconds. The knees are 31% short on torque, and three repairs are ready to apply.",
  ],
  [
    "Go back to one world out of 4,096",
    "NS-02",
    "World 1,337 fills with NaNs. To look at it, you re-run the whole batch on the same GPU.",
    "`frob sim replay --world 1337` rebuilds that one world on the CPU from its address, stops at the first bad step and shows the contact that started it.",
  ],
  [
    "Get the family, skip the guessing",
    "NS-13",
    "An agent scores candidates one at a time and reports the best. Nobody knows about the bodies it never tried.",
    "One query returns every stance that carries the load and proves the rest can’t. The limit is exact: 3/14 of a meter from centerline to foot.",
  ],
  [
    "Run 256 different robots together",
    "NS-14",
    "A varied population often means a rebuild for each design, with compiles that can take minutes.",
    "One batch steps 256 bodies with three different leg counts, 32 worlds each, on a single RTX 4090. Nothing is recompiled.",
  ],
  [
    "Decide 300 designs in an hour",
    "NS-06",
    "The usual loop tries one candidate per turn.",
    "An agent runs thirty edit-and-test cycles in an hour on one workstation, ten variants at a time. `frob design bisect` finds the edit that broke stair climbing.",
  ],
  [
    "Shrink a failure to three details",
    "NS-15",
    "Random testing says 0.4% of runs fail, and the agent reads logs to guess why.",
    "Nemesis finds a failing world and strips it down: a door sill 9 mm higher, one slippery tile, a depth frame 40 ms late. Take away any one and the robot makes it.",
  ],
  [
    "Name the cause of a fall",
    "NS-16",
    "An agent sorts the anomalies by time and guesses.",
    "Autopsy rewinds to just before the fall and changes one thing at a time. Lifting the knee torque limit saves 94% of the replays; better grip saves 8%. It was the knee.",
  ],
  [
    "Stop lucky designs getting through",
    "NS-17",
    "Submit enough candidates to the same test and one passes by chance.",
    "Every submission meets fresh sealed worlds, and one error budget covers the whole campaign, even after 500 retries.",
  ],
  [
    "Fix the room for $12",
    "NS-18",
    "Every repair an agent considers is a change to the robot.",
    "The clearance map finds the culprit, a 19 mm threshold. A $12 rubber ramp fixes it with the robot untouched. A bigger drive wheel costs $140. You see both prices.",
  ],
  [
    "Walk a new body straight away",
    "NS-19",
    "Every new design waits for its own training run: about seven minutes for a quadruped, up to an hour for a humanoid.",
    "Lengthen the femur 8% and widen the stance, and the policy trained for that family of bodies walks the new one, at 78% of a specialist’s score.",
  ],
  [
    "Test in your own kitchen",
    "NS-12",
    "You test in a home that somebody else modeled.",
    "A walkthrough video of your kitchen becomes a private test scene. Candidate robots work in it, and the scan never leaves your machine.",
  ],
  [
    "Hand the winner to a workshop",
    "NS-20",
    "The loop ends with a score.",
    "Foundry turns the design into a parts list, solid geometry, a tolerance stack that closes and fatigue margins from the simulated loads, with one link 18% lighter at the same margin.",
  ],
];

export const renderQuestion = (ref: Refs, session: string) =>
  chapterOpen(question) +
  section({
    ref,
    id: "what",
    eyebrow: "What it does",
    title: "Six tools that share *one model*.",
    kicker:
      "Simulation, design, search, testing, calibration and proof all read the same description of the robot, so whatever one tool learns, the others can use.",
    body: `<ul class="bento">${products
      .map(
        ([pic, verb, name, text, parts]) =>
          `<li class="reveal ${pic}"><div class="bento-art" aria-hidden="true">${art[pic]}</div><div class="bento-copy"><p class="micro">${verb}</p><h4>${escape(name)}</h4><p>${copy(text)}</p><p class="tags">${parts
            .map(([p, s]) => `<a class="tag" href="${ref(s)}">${p}</a>`)
            .join("")}</p></div></li>`,
      )
      .join("")}</ul>`,
  }) +
  renderSession(ref, session) +
  section({
    ref,
    id: "loop",
    eyebrow: "Where the time goes",
    title: "Training eats *the clock*.",
    kicker:
      "In a typical design loop, 85 to 90% of the time goes into training a controller. FrankenRobotics rules out the bodies that can’t work first, then trains the few that might.",
    body: figure({
      name: "loop",
      todo: "One pass of the loop: edit, compile, train, evaluate",
      note: "Times are composed from published training runs.",
      ref,
      body: `<div class="bars">
<div class="bar-row"><div class="bar-label"><b>Robot dog</b><span>about 6 minutes</span></div><div class="track"><i class="train" style="--w:85%"><span>Training, about 85%</span></i><i class="rest" style="--w:15%"><span>The rest</span></i></div></div>
<div class="bar-row"><div class="bar-label"><b>Humanoid</b><span>about 31 minutes</span></div><div class="track"><i class="train" style="--w:90%"><span>Training, over 90%</span></i><i class="rest" style="--w:10%"></i></div></div></div>
<div class="facts4"><p><b class="num">80 to 1,600</b>candidates trained in one published design campaign</p><p><b class="num">7.5 to 30</b>GPU-minutes to train each of them</p><p><b class="num">15.8%</b>turn out to be duds after the full training run</p><p class="ours"><b class="num">2 seconds</b>is the budget for the first six FrankenRobotics checks</p></div>`,
    }),
  }) +
  section({
    ref,
    id: "stories",
    eyebrow: "What changes for you",
    title: "Twelve jobs it’s *built for*.",
    kicker: "Everyday problems in robot design, how they go today, and how they go with FrankenRobotics.",
    body: `<div class="fig storyfig reveal" id="fig-stories" data-fig="stories"><div class="story-rail"><ol class="story-list">${stories
      .map(
        ([name, id], i) =>
          `<li><a href="${ref(id)}" data-story="${i}"${i ? "" : ' aria-current="true"'}><span class="num">${String(i + 1).padStart(2, "0")}</span><b>${escape(name)}</b></a></li>`,
      )
      .join("")}</ol></div><div class="story-stage">${stories
      .map(
        ([name, id, today, after], i) =>
          `<article class="story${i ? "" : " on"}" data-panel="${i}" data-n="${String(i + 1).padStart(2, "0")}" aria-labelledby="st-${i}"><h4 id="st-${i}">${escape(name)}</h4><div class="ba"><div class="before"><p class="micro">Today</p><p>${copy(today)}</p></div><div class="after"><p class="micro">With FrankenRobotics</p><p>${copy(after)}</p></div></div><a class="more" href="${ref(id)}">Read the full scenario${icon("right")}</a></article>`,
      )
      .join("")}<div class="story-nav"><button type="button" class="btn sm" data-prev>${icon("left")}Previous</button><span class="num" data-at>1 of ${stories.length}</span><button type="button" class="btn sm" data-next>Next${icon("right")}</button></div></div></div>`,
  }) +
  chapterClose;
