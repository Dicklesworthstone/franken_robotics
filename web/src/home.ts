import { answer, renderAnswer } from "./chapters/answer";
import { body, renderBody } from "./chapters/body";
import { evidence, renderEvidence } from "./chapters/evidence";
import { question, renderQuestion } from "./chapters/question";
import { renderTest, test } from "./chapters/test";
import { renderWorld, world } from "./chapters/world";
import { repo } from "./layout";
import { icon, range, seg, title, type Refs } from "./parts";

export const chapters = [question, body, world, answer, evidence, test];

export const homeNav = chapters
  .map(
    (c) =>
      `<a href="#${c.id}" data-spy="${c.id}" style="--chapter:${c.colour}"><span class="joint" aria-hidden="true"></span>${c.short}</a>`,
  )
  .join("");
/** The spec, as the plan sets it. Each opens the part of the page that shows it. */
const stats: [n: string, label: string, note: string, to: string][] = [
  ["18", "kinds of body", "Dogs, humanoids, rovers, tracks, snakes and wheel-leg hybrids.", "#bodies"],
  ["7", "kinds of building", "From apartments to factories and commercial kitchens.", "#rooms"],
  ["256", "robots per batch", "All different, stepped together on one GPU.", "#batch"],
  ["2 s", "design check", "Weak knees found before any simulation.", "#preflight"],
  ["10×", "the goal", "Ten times faster from a mission to a robot that passes.", "#roadmap"],
];

const hero = () => `
<section class="hero" aria-labelledby="hero-title" id="fig-lineup" data-fig="lineup">
<div class="lineup"><picture><source media="(max-width: 999px)" srcset="/media/lineup-tall.webp" width="1080" height="1180"><img src="/media/lineup.webp" width="1920" height="1100" alt="" fetchpriority="high"></picture><canvas aria-label="A crowd of 99 robots of eighteen kinds facing a doorway. The ones small enough for it are lit, the rest are dark."></canvas><p class="tip" data-tip hidden></p></div>
<div class="wrap hero-copy"><p class="pill"><span class="joint on" aria-hidden="true"></span>Robotics lab for AI agents<i></i>In design</p>
<h1 class="d1" id="hero-title"><span>Name the job.</span> <span>Get <em class="g">every robot</em></span> <span>that can do it.</span></h1>
<p class="lede">FrankenRobotics is a robotics lab that AI agents run. Give it a mission and a parts catalog. It designs the bodies, works out the controllers, tests them in thousands of worlds at once, and hands back every design that passes, with a build packet for the one you pick.</p>
<div class="cta"><a class="btn go" href="#lab">See what it does${icon("down")}</a><a class="btn" href="${repo}">${icon("star")}Star on GitHub</a></div></div>
<div class="wrap console-wrap"><div class="console"><div class="console-place"><p class="micro">Pick a place</p>${seg(
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
<p class="console-note">Drag a slider. Robots that still fit stay lit. FrankenRobotics checks size first, then torque, grip, sensing and battery.</p></div></section>
<div class="wrap"><ul class="stats">${stats
  .map(
    ([n, label, note, to]) =>
      `<li><a href="${to}"><b>${n}</b><strong>${label}</strong><span>${note}</span></a></li>`,
  )
  .join("")}</ul></div>`;

const roles: [name: string, pages: [string, string][], text: string][] = [
  ["You design robots", [["Skeleton", "14"], ["Habitat", "19"], ["Augur", "25"], ["Foundry", "31"]], "Bodies, buildings, missions and the workshop."],
  ["You build simulators", [["Kinesis", "16"], ["Traction", "18"], ["Spark", "21"], ["Speed", "36"]], "Dynamics, contact, GPU kernels and performance."],
  ["You build agents", [["Atelier", "28"], ["Tether", "32"], ["Arena", "24"], ["Safety", "37"]], "Commands, budgets and the sealed exam."],
  ["You’re here for the math", [["Constructions", "10"], ["Marrow", "13"], ["Lemma", "12"], ["Compendium", "appendix-d"]], "Lie groups, cones, certificates and proofs."],
];

const close = (ref: Refs, sections: number, words: string) => `
<section class="libcta" aria-labelledby="lib-title"><div class="lattice" aria-hidden="true"></div><div class="wrap"><div class="reveal"><p class="kick"><span class="joint on" aria-hidden="true"></span>In design, in the open</p><h2 class="d2" id="lib-title">${title("Follow *the build*.")}</h2><p class="lede">FrankenRobotics is being designed in public. The whole plan is here: ${sections} sections, about ${words} words and every equation, free to read and easy to search. Watch the repository to catch the first release.</p><div class="cta"><a class="btn go" href="${repo}">${icon("star")}Star on GitHub</a><a class="btn" href="/plan/">Read the plan${icon("right")}</a><a class="btn" href="/search/" data-palette>${icon("search")}Search it</a></div></div><div class="paths">${roles
  .map(
    ([name, pages, text]) =>
      `<div class="path reveal"><h3>${name}</h3><p>${text}</p><p class="steps">${pages.map(([l, s]) => `<a class="tag" href="${ref(s)}">${l}</a>`).join("")}</p></div>`,
  )
  .join("")}</div></div></section>`;

export function home(o: { ref: Refs; session: string; sections: number; words: string }) {
  return (
    hero() +
    renderQuestion(o.ref, o.session) +
    renderBody(o.ref) +
    renderWorld(o.ref) +
    renderAnswer(o.ref) +
    renderEvidence(o.ref) +
    renderTest(o.ref) +
    close(o.ref, o.sections, o.words)
  );
}
