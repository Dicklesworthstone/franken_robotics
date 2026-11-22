import { chapterClose, chapterOpen, figure, icon, range, section, type Chapter, type Refs } from "../parts";

export const evidence: Chapter = {
  id: "crash",
  number: "05",
  label: "The crash lab",
  short: "Crash lab",
  icon: "crash",
  colour: "#b7a2fb",
  title: "Break it *here first*.",
  thesis:
    "FrankenRobotics hunts for the world that beats your robot, boils it down to the details that matter, and replays the failure until the cause is clear.",
  sections: [
    ["shrink", "The smallest world that breaks it"],
    ["autopsy", "Find out why it fell"],
    ["retries", "Luck doesn’t pass"],
  ],
};

export const renderEvidence = (ref: Refs) =>
  chapterOpen(evidence) +
  section({
    ref,
    id: "shrink",
    eyebrow: "Nemesis",
    title: "The smallest world that *breaks it*.",
    kicker:
      "This robot fails in a world with five things wrong. The search takes them away, test by test, until it finds the pair that’s enough on its own.",
    more: ["How Nemesis attacks", "26"],
    body: figure({
      name: "shrink",
      todo: "Press run and watch it narrow down",
      kind: "live",
      ref,
      body: `<div class="shrink"><div class="chips" data-chips></div><div class="shrink-log" data-log></div><div class="shrink-foot"><div class="btnrow"><button type="button" class="btn go sm" data-step>Next test</button><button type="button" class="btn sm" data-run>${icon("play")}Run</button><button type="button" class="btn sm" data-reset>${icon("redo")}Start over</button></div><p data-say aria-live="polite"></p></div></div>`,
    }),
  }) +
  section({
    ref,
    id: "autopsy",
    eyebrow: "Autopsy",
    title: "Find out *why it fell*.",
    kicker:
      "Go back to just before the fall, change one thing and replay it 200 times. Then make a change that shouldn’t matter, and subtract whatever that saves.",
    more: ["How Autopsy finds a cause", "27"],
    body: figure({
      name: "autopsy",
      todo: "Change how many replays each fix saves",
      kind: "example",
      ref,
      body: `<div class="autopsy"><div class="dots3" data-dots></div><div class="autopsy-side"><div class="controls stack">${range("torque", "Saved by stronger knees", 0, 200, 1, 164, "164")}${range("friction", "Saved by better grip", 0, 200, 1, 38, "38")}${range("placebo", "Saved by a harmless change", 0, 100, 1, 12, "12")}${range("flip", "Sensitivity to tiny nudges", 0, 0.3, 0.01, 0.04, "0.04")}</div><div class="verdicts" data-out aria-live="polite"></div></div></div>`,
    }),
  }) +
  section({
    ref,
    id: "retries",
    eyebrow: "The sealed exam",
    title: "Luck doesn’t *pass*.",
    kicker:
      "Let an agent resubmit worthless designs 500 times and an ordinary 5% test will pass one sooner or later. FrankenRobotics spreads a single error budget over every retry.",
    more: ["How qualification works", "24"],
    body: figure({
      name: "retries",
      todo: "Submit 500 worthless designs, 400 times over",
      kind: "live",
      ref,
      body: `<div class="retries"><div class="retries-grids"><div><p class="micro asis">Ordinary 5% test</p><canvas data-naive width="400" height="400" aria-label="Campaigns under the ordinary test"></canvas><p class="big out" data-naive-n>0</p><small>campaigns fooled</small></div><div><p class="micro asis">FrankenRobotics</p><canvas data-spend width="400" height="400" aria-label="Campaigns under the shared error budget"></canvas><p class="big in" data-spend-n>0</p><small>campaigns fooled</small></div></div><div class="retries-plot"><div data-plot></div><p class="num" data-count>Submission 0 of 500</p><div class="btnrow"><button type="button" class="btn go sm" data-run>${icon("play")}Submit 500 times</button><button type="button" class="btn sm" data-seed>${icon("redo")}New seed</button></div></div></div>`,
    }),
  }) +
  chapterClose;
