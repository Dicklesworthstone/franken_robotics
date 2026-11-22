import { answer, renderAnswer } from "./chapters/answer";
import { body, renderBody } from "./chapters/body";
import { evidence, renderEvidence } from "./chapters/evidence";
import { question, renderQuestion } from "./chapters/question";
import { renderTest, test } from "./chapters/test";
import { renderWorld, world } from "./chapters/world";
import { icon, range, seg, title, type Refs } from "./parts";

export const chapters = [question, body, world, answer, evidence, test];

export const homeNav = chapters
  .map(
    (c) =>
      `<a href="#${c.id}" data-spy="${c.id}" style="--chapter:${c.colour}"><span class="joint" aria-hidden="true"></span><span class="num">${c.number}</span>${c.short}</a>`,
  )
  .join("");
export const homeDock = chapters
  .map(
    (c) =>
      `<li><a href="#${c.id}" data-spy="${c.id}" style="--chapter:${c.colour}">${icon(c.icon)}${c.short}</a></li>`,
  )
  .join("");

const stats: [n: string, label: string, note: string, section: string][] = [
  ["18", "archetype bodies", "Legged, wheeled, tracked and hybrid, each with a known answer.", "14.10"],
  ["40", "constructions", "Each owns a type, a verb, a test that fails and a payoff or kill date.", "10"],
  ["24", "North-Star scenarios", "Stories with a before, an after and a milestone.", "7"],
  ["10×", "target", "The wording for missing it is already written.", "39.10"],
  ["0", "results so far", "No FrankenRobotics physics step has been measured.", "0.2"],
];

const hero = (ref: Refs) => `
<section class="hero" aria-labelledby="hero-title" id="fig-lineup" data-fig="lineup">
<div class="lineup"><picture><source media="(max-width: 999px)" srcset="/media/lineup-tall.webp" width="1080" height="1180"><img src="/media/lineup.webp" width="1920" height="1100" alt="" fetchpriority="high"></picture><canvas aria-label="A crowd of 99 robots of eighteen kinds facing a doorway. The ones small enough for it are lit, the rest are dark."></canvas><p class="tip" data-tip hidden></p></div>
<div class="wrap hero-copy"><p class="pill"><span class="joint on" aria-hidden="true"></span>Robot design<i class="opt"></i><span class="opt">Physics</span><i></i>Planning stage</p>
<h1 class="d1" id="hero-title">${title("Every body that can *do the job*.")}</h1>
<p class="lede">FrankenRobotics is a plan for a robotics lab that AI agents can use. State a mission and it should return the whole family: the robots that can do it, the robots that cannot and why, and the cheapest experiment that would settle the rest.</p>
<div class="cta"><a class="btn go" href="#family">See a family answer${icon("down")}</a><a class="btn" href="/plan/">Explore the plan${icon("right")}</a></div>
<p class="honest">Nothing has been built yet. The figures on this page run in your browser, and each one names its source in the plan.</p></div>
<div class="wrap console-wrap"><div class="console"><div class="console-place"><p class="micro">Choose a place</p>${seg(
  "place",
  "Place",
  [
    ["door", "Narrow door"],
    ["shelf", "Low shelf"],
    ["landing", "Tight landing"],
    ["hall", "Open hall"],
  ],
  "door",
)}</div><div class="console-dials">${range("width", "Door width", 0.3, 1.2, 0.01, 0.6, "0.60 m")}${range("height", "Headroom", 0.2, 2, 0.01, 1.7, "1.70 m")}${range("circle", "Clear floor", 0.4, 2, 0.01, 1.2, "1.20 m")}</div><div class="console-out"><p class="tally"><b data-n="fits">0</b> of <span data-total>99</span> fit</p><div class="tallybar" aria-hidden="true"><i data-bar="fits"></i><i data-bar="wide"></i><i data-bar="tall"></i><i data-bar="long"></i></div><p class="tallykeys"><span class="wide"><b data-n="wide">0</b> too wide</span><span class="tall"><b data-n="tall">0</b> too tall</span><span class="long"><b data-n="long">0</b> too long</span></p></div><button type="button" class="console-pause" data-pause aria-label="Pause" aria-pressed="false"><span class="on">${icon("pause")}</span><span class="off">${icon("play")}</span></button></div>
<p class="console-note"><span class="basis toy">A toy, for intuition</span><span>Eighteen <a class="txt" href="#bodies">archetypes of §14.10</a> at several sizes, sorted by size alone. The plan also asks about torque, grip, sensing and time.</span></p></div></section>
<div class="wrap"><ul class="stats">${stats
  .map(
    ([n, label, note, s]) =>
      `<li><a href="${ref(s)}"><b>${n}</b><strong>${label}</strong><span>${note}</span></a></li>`,
  )
  .join("")}</ul></div>`;

const roles: [name: string, pages: [string, string][], text: string][] = [
  ["You design robots", [["§14", "14"], ["§19", "19"], ["§25", "25"], ["§31", "31"]], "Bodies, places, mission queries and what reaches the workshop."],
  ["You build simulators", [["§16", "16"], ["§18", "18"], ["§21", "21"], ["§36", "36"]], "Dynamics tiers, contact, GPU kernels and the performance program."],
  ["You build agents", [["§28", "28"], ["§32", "32"], ["§24", "24"], ["§37", "37"]], "Verbs, budgets, refusals, and a qualification lane an agent cannot read."],
  ["You doubt it", [["§2", "2"], ["§39", "39"], ["§41", "41"], ["§1.3", "1.3"]], "Ceilings, outcome rules, the pre-mortem and the withdrawn claims."],
];

const library = (ref: Refs, version: string, sections: number, words: string) => `
<section class="libcta" aria-labelledby="lib-title"><div class="lattice" aria-hidden="true"></div><div class="wrap"><div class="reveal"><p class="kick"><span class="joint on" aria-hidden="true"></span>The whole plan</p><h2 class="d2" id="lib-title">${title("Read the *rest*.")}</h2><p class="lede">This page is a tour. The plan itself is ${sections} sections and appendices and about ${words} words: every equation, gate, risk and open question, at version ${version}. It is searchable, cross-linked and free to download.</p><div class="cta"><a class="btn go" href="/plan/">Open the library${icon("right")}</a><a class="btn" href="/search/" data-palette>${icon("search")}Search it</a><a class="btn" href="/plan.md" download>Download Markdown</a></div></div><div class="paths">${roles
  .map(
    ([name, pages, text]) =>
      `<div class="path reveal"><h2>${name}</h2><p>${text}</p><p class="steps">${pages.map(([l, s]) => `<a class="tag" href="${ref(s)}">${l}</a>`).join("")}</p></div>`,
  )
  .join("")}</div></div></section>`;

export function home(o: {
  ref: Refs;
  version: string;
  session: string;
  sections: number;
  words: string;
}) {
  return (
    hero(o.ref) +
    renderQuestion(o.ref) +
    renderBody(o.ref) +
    renderWorld(o.ref) +
    renderAnswer(o.ref, o.session) +
    renderEvidence(o.ref) +
    renderTest(o.ref, o.version) +
    library(o.ref, o.version, o.sections, o.words)
  );
}
