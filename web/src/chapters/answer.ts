import { escape } from "../layout";
import { chapterClose, chapterOpen, figure, icon, range, section, seg, type Chapter, type Refs } from "../parts";

export const answer: Chapter = {
  id: "answer",
  number: "04",
  label: "The answer",
  short: "Answer",
  icon: "answer",
  colour: "#c6f25a",
  title: "Answer the whole *family*.",
  thesis:
    "A mission names the bodies, the worlds, what the controller may observe and when. The answer is three sets: designs with a construction that works, designs excluded with a witness, and the ones still undecided, with the cheapest experiment that would split them.",
  sections: [
    ["runners", "14,400 runners"],
    ["family", "Three sets, refined"],
    ["stance", "A certificate you can check"],
    ["docking", "When the packet arrives"],
    ["session", "An agent works the query"],
  ],
};

export interface Step {
  command: string;
  result: string;
}

/** The second session of §32.16, split into commands and results at build time. */
export function sessionSteps(block: string): Step[] {
  const steps: Step[] = [];
  for (const line of block.split("\n")) {
    const hash = line.search(/(?:^|\s)#\s/);
    const code = (hash < 0 ? line : line.slice(0, hash)).trim();
    const note = hash < 0 ? "" : line.slice(hash).replace(/^\s*#\s*/, "").trim();
    if (code.startsWith("$ ")) steps.push({ command: code.slice(2), result: note });
    else if (steps.length) {
      const last = steps.at(-1)!;
      if (code) last.command = last.command.replace(/\s*\\$/, "") + " " + code;
      if (note) last.result += (last.result ? " " : "") + note;
    }
  }
  return steps.map((s) => ({ command: s.command.replace(/\s*\\\s*/g, " ").replace(/\s+/g, " "), result: s.result }));
}

/** One line on what each step is for. Matched by verb, so a changed session stops the build. */
const GIST: [verb: RegExp, title: string, text: string][] = [
  [/^frob session open/, "Open a session", "The token carries scopes and a compute budget. It has no scope that reads sealed worlds."],
  [/^frob mission new/, "State the mission", "A template becomes a query file that names the designs, the worlds and the clock."],
  [/^frob mission check/, "Check the question", "The order of the quantifiers is part of the query. Here: for each design some policy, for all worlds."],
  [/^frob mission synthesize .*--estimate/, "Ask what it would cost", "An estimate comes back with an interval before any budget is spent."],
  [/^frob mission synthesize/, "Start the work", "The call returns a job instead of holding the connection open."],
  [/^frob job wait/, "Read the three sets", "412 cells: 61 in, 179 out with witnesses, 172 unresolved."],
  [/^frob mission show .*inner/, "Cheapest design that works", "Every inner cell has a construction behind it."],
  [/^frob mission show .*unresolved/, "Cheapest design not yet settled", "It costs less, so it is worth an experiment, and the answer names one."],
  [/^frob mission refute/, "Ask why one is out", "The refusal holds for every controller and states the smallest change that would lift it."],
  [/^frob design new/, "Build the candidate", "A design is made from the centre of the unresolved cell."],
  [/^frob nemesis search carrier-b .*nem-1/, "Attack it", "Three worlds inside the declared family break it."],
  [/^frob nemesis shrink/, "Shrink the failure", "Two edits from nominal are enough to make it fail."],
  [/^frob autopsy/, "Find the cause", "Change one thing in forked replays, and compare with a placebo."],
  [/^frob design edit/, "Repair it", "A larger wheel, recorded as a typed edit."],
  [/^frob result revalidate/, "Keep what still holds", "Fourteen earlier results survive the edit and five are run again."],
  [/^frob nemesis search carrier-b .*nem-2/, "Attack it again", "Finding nothing in 4,096 worlds is reported as exactly that."],
  [/^frob eval qualify .*--dry/, "See the price of qualifying", "Each submission spends error budget, so a retry costs power."],
  [/^frob eval qualify/, "Qualify once", "200 sealed worlds. The agent learns the verdict and no per-world data."],
  [/^frob prove bundle/, "Bundle the proof", "Everything the auditor needs goes into one file."],
  [/^frob-verify/, "Audit it offline", "A separate program that shares no code with the prover checks the bundle."],
];

export function renderAnswer(ref: Refs, session: string) {
  const steps = sessionSteps(session);
  const gists = steps.map((s) => {
    const g = GIST.find(([verb]) => verb.test(s.command));
    if (!g) throw Error("No gist for session step: " + s.command);
    return { ...s, title: g[1], text: g[2] };
  });
  return (
    chapterOpen(answer) +
    section({
      ref,
      id: "runners",
      index: "04.1",
      eyebrow: "A family, drawn",
      title: "14,400 *runners*.",
      kicker:
        "One spring leg each, and every pixel on the left is a different robot. All are released together at 5 m/s. Most fall within a few steps, and the ones still running trace a crescent. Move the window to watch its robots on the right.",
      body: `<figure class="fig film reveal" id="fig-film" data-fig="film"><div class="film-bar"><span class="basis toy">A toy, for intuition</span><dl class="hud"><div><dt>Time</dt><dd data-time>0.0 s</dd></div><div><dt>Still running</dt><dd data-standing>14,400</dd></div><div><dt>Window</dt><dd data-view>68°, 34 kN/m</dd></div></dl><div class="film-tools"><button type="button" data-again aria-label="Release the runners again">${icon("redo")}</button><button type="button" data-pause aria-label="Pause" aria-pressed="false"><span class="on">${icon("pause")}</span><span class="off">${icon("play")}</span></button></div></div><div class="film-grid"><div class="film-map"><span class="ax y num">Spring stiffness, 5 to 55 kN/m</span><div class="film-sq"><canvas data-map aria-label="A map of 14,400 one-legged runners, each with its own leg stiffness and landing angle. Those still running form a crescent."></canvas><p class="probe num" data-probe hidden></p></div><span class="ax x num"><span>56°</span><span>Landing angle</span><span>80°</span></span></div><div class="film-close"><canvas data-close aria-label="Close-up of the runners inside the window, each on its own spring."></canvas></div></div><div class="film-key"><span><i class="k-in"></i>Still running, soft spring to stiff</span><span><i class="k-out"></i>Fallen, later to sooner</span></div><figcaption class="block"><span><small>Figure</small>04.1</span><span><small>Basis</small>Runs in your browser; not FrankenRobotics code</span><span class="grow"><small>Note</small>The textbook spring-mass runner. The crescent was published by Seyfarth, Geyer, Günther and Blickhan in 2002.</span></figcaption></figure>`,
    }) +
    section({
      ref,
      id: "family",
      index: "04.2",
      eyebrow: "Augur",
      title: "In, out and *unresolved*.",
      kicker:
        "The four-footed carrier below has two lengths and one rating to choose. Boxes that certainly work are green, boxes that certainly fail are red, and the amber ones straddle the edge. Refining shrinks the amber and never moves a settled box. [§25](§25)",
      body: figure({
        name: "region",
        no: "04.2",
        label: "Every footprint at once",
        title: "Refine the *unsettled* boxes.",
        basis: "computed",
        source: [["§25.13", ref("25.13")]],
        note: "A box is settled by checking its corners, because the condition is linear in 1/a and 1/b. Axes are half-length and half-width of the footprint.",
        ref,
        className: "feature",
        body: `<div class="region"><div class="region-view"><canvas aria-label="Map of footprints by half-length and half-width: boxes that work, boxes that fail and boxes still open"></canvas><p class="crosshair num" data-tip hidden></p></div><div class="region-side"><div class="tri"><p class="in"><small>Inner, with a witness</small><b data-in>0%</b></p><p class="open"><small>Unresolved</small><b data-open>100%</b></p><p class="out"><small>Excluded, with a witness</small><b data-out>0%</b></p></div><div class="controls stack">${range("c", "Rating of each foot", 110, 240, 1, 160, "160 N")}${range("w", "Payload", 250, 500, 5, 400, "400 N")}</div><div class="btnrow"><button type="button" class="btn go sm" data-refine>Refine</button><button type="button" class="btn sm" data-auto>${icon("play")}Refine to the end</button><button type="button" class="btn sm" data-reset>Start over</button></div><div class="cellinfo" data-info aria-live="polite"><p class="micro">Point at a box</p><p>Its certificate, or the cheapest next test, appears here.</p></div></div></div>`,
      }),
    }) +
    section({
      ref,
      id: "stance",
      index: "04.3",
      eyebrow: "Known answer",
      title: "A certificate you can check *by hand*.",
      kicker:
        "How narrow can this robot be and still balance a 400 N payload anywhere in a 300 by 150 mm rectangle? The exact answer is 3/14 m. Drag the width and the payload, and the arithmetic is redone in exact fractions. [§25.13](§25.13)",
      body: figure({
        name: "stance",
        no: "04.3",
        label: "Four feet, at most 160 N each, a = 1/3 m",
        title: "Find the *narrowest* body.",
        basis: "computed",
        source: [["§25.13", ref("25.13")]],
        note: "Fixed pose, coplanar contacts, normal forces only. The payload's position is known before the forces are chosen.",
        ref,
        body: `<div class="stance"><div class="stance-view" data-view></div><div class="stance-side">${seg(
          "share",
          "How load is shared",
          [
            ["independent", "Independent foot forces"],
            ["springs", "Four equal springs"],
          ],
          "independent",
        )}<div class="controls stack">${range("b", "Half-width b", 170, 300, 1, 250, "250 mm")}</div><div class="cert"><p class="micro" data-head></p><p class="verdict" data-verdict></p><pre data-proof></pre></div><div class="presets"><button type="button" class="tag" data-b="200">b = 1/5 m</button><button type="button" class="tag" data-b="214.2857142857">b = 3/14 m</button><button type="button" class="tag" data-b="250">b = 1/4 m</button></div></div></div>`,
      }),
    }) +
    section({
      ref,
      id: "docking",
      index: "04.4",
      eyebrow: "C-19 and NS-22",
      title: "When the packet *arrives*.",
      kicker:
        "A cart must stop at 1 after three pushes, and nobody knows whether its motor has gain 1 or 2. One noisy velocity reading can tell. No amount of training helps if it arrives too late, and a stronger motor buys tolerance to a worse sensor. [C-19](§C-19)",
      body: figure({
        name: "docking",
        no: "04.4",
        label: "The docking fixture",
        title: "Sensor, motor and delay in *one* inequality.",
        basis: "computed",
        source: [
          ["C-19", ref("C-19")],
          ["NS-22", ref("7")],
        ],
        note: "A policy exists exactly when ε < (U + ½)/4 and the packet arrives before the second command. No device timing follows from this scalar model.",
        ref,
        body: `<div class="docking"><div class="docking-side"><div class="controls stack">${range("U", "Motor limit U", 0.5, 1, 0.01, 0.75, "0.75")}${range("eps", "Sensor error ε", 0, 0.45, 0.005, 0.1, "0.100")}${range("p", "First push p", 0.02, 0.75, 0.005, 0.25, "0.250")}</div>${seg(
          "when",
          "Packet arrives",
          [
            ["early", "Before push 2"],
            ["late", "After push 3"],
          ],
          "early",
        )}<div class="presets"><button type="button" class="tag" data-set="0.75,0.1,0.25">p = 1/4</button><button type="button" class="tag" data-set="0.75,0.3,0.625">p = 5/8</button><button type="button" class="tag" data-set="0.75,0.3125,0.625">The exact edge</button></div><div class="cert"><p class="verdict" data-verdict></p><p data-why></p></div></div><div class="docking-view"><div data-carts></div><div data-chart></div></div></div>`,
      }),
    }) +
    section({
      ref,
      id: "session",
      index: "04.5",
      eyebrow: "Tether",
      title: "An agent works the *query*.",
      kicker:
        "The plan's second worked session, word for word. Twenty commands take an agent from a mission to a bundle that a separate program checks with no network. Every number in it is illustrative. [§32.16](§32.16)",
      body: figure({
        name: "session",
        no: "04.5",
        label: "carrier-wet-strip",
        title: "From mission to *audited bundle*.",
        basis: "illustrative",
        source: [["§32.16", ref("32.16")]],
        note: "Commands and results are copied from the plan when this page is built. The captions are this page's.",
        ref,
        className: "feature",
        body: `<div class="session"><ol class="session-steps" role="list">${gists
          .map(
            (s, i) =>
              `<li><button type="button" data-i="${i}" aria-pressed="${i === 0}"><span class="num">${String(i + 1).padStart(2, "0")}</span>${escape(s.title)}</button></li>`,
          )
          .join("")}</ol><div class="session-main"><div class="term"><p class="term-bar"><i></i><i></i><i></i><span>agent-17 · s:2f6a…</span></p><div class="term-body" data-term tabindex="0" aria-label="Terminal transcript">${gists
          .map(
            (s, i) =>
              `<div class="tline" data-step="${i}"><p class="cmd"><span>$</span> ${escape(s.command)}</p>${s.result ? `<p class="res">${escape(s.result)}</p>` : ""}</div>`,
          )
          .join("")}</div></div><div class="session-note"><p class="micro" data-n>Step 1 of ${gists.length}</p><h5 data-t>${escape(gists[0]!.title)}</h5><p data-d>${escape(gists[0]!.text)}</p><div class="btnrow"><button type="button" class="btn sm" data-prev>${icon("left")}Back</button><button type="button" class="btn go sm" data-next>Next${icon("right")}</button><button type="button" class="btn sm" data-play>${icon("play")}Play</button></div></div></div><template data-gists>${escape(JSON.stringify(gists.map((g) => [g.title, g.text])))}</template></div>`,
      }),
    }) +
    chapterClose
  );
}
