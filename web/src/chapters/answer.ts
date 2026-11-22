import { chapterClose, chapterOpen, figure, icon, range, section, seg, type Chapter, type Refs } from "../parts";

export const answer: Chapter = {
  id: "answer",
  number: "04",
  label: "The answer",
  short: "Answers",
  icon: "answer",
  colour: "#c6f25a",
  title: "One question, *every answer*.",
  thesis:
    "Ask which robots can carry a tray across the kitchen and you get the whole family: the designs that work, the ones that can’t and why, and the handful worth testing.",
  sections: [
    ["runners", "14,400 runners"],
    ["family", "Every design at once"],
    ["stance", "How narrow can it get?"],
    ["docking", "Better sensor, or stronger motor?"],
  ],
};

export const renderAnswer = (ref: Refs) =>
  chapterOpen(answer) +
  section({
    ref,
    id: "runners",
    eyebrow: "A family, live",
    title: "14,400 runners, *released together*.",
    kicker:
      "Each pixel is a different one-legged robot, let go at 5 m/s. Most fall within a few steps. The ones still running trace the bright crescent, and that crescent is the answer.",
    body: `<figure class="fig film reveal" id="fig-film" data-fig="film"><div class="film-bar"><p class="todo">Move the window to watch its robots run</p><dl class="hud"><div><dt>Time</dt><dd data-time>0.0 s</dd></div><div><dt>Still running</dt><dd data-standing>14,400</dd></div><div><dt>Window</dt><dd data-view>68°, 34 kN/m</dd></div></dl><div class="film-tools"><button type="button" data-again aria-label="Release the runners again">${icon("redo")}</button><button type="button" data-pause aria-label="Pause" aria-pressed="false"><span class="on">${icon("pause")}</span><span class="off">${icon("play")}</span></button></div></div><div class="film-grid"><div class="film-map"><span class="ax y num">Leg stiffness, 5 to 55 kN/m</span><div class="film-sq"><canvas data-map aria-label="A map of 14,400 one-legged runners, each with its own leg stiffness and landing angle. Those still running form a crescent."></canvas><p class="probe num" data-probe hidden></p></div><span class="ax x num"><span>56°</span><span>Landing angle</span><span>80°</span></span></div><div class="film-close"><canvas data-close aria-label="Close-up of the runners inside the window, each on its own spring."></canvas></div></div><div class="film-key"><span><i class="k-in"></i>Still running, soft leg to stiff</span><span><i class="k-out"></i>Fallen, late to early</span></div><figcaption class="fig-foot">The spring-legged runner of Seyfarth, Geyer, Günther and Blickhan (2002), computed in your browser.</figcaption></figure>`,
  }) +
  section({
    ref,
    id: "family",
    eyebrow: "The family map",
    title: "See every design *at once*.",
    kicker:
      "Each point on this map is a four-legged carrier with a different stance. Green boxes work, red ones can’t, and amber ones straddle the edge. Refine them and the amber shrinks to a line.",
    more: ["How missions are answered", "25"],
    body: figure({
      name: "region",
      todo: "Press refine, then point at any box to see why",
      kind: "live",
      ref,
      className: "feature",
      body: `<div class="region"><div class="region-view"><canvas aria-label="Map of stances by half-length and half-width: boxes that work, boxes that can't and boxes still open"></canvas><p class="crosshair num" data-tip hidden></p></div><div class="region-side"><div class="tri"><p class="in"><small>Works</small><b data-in>0%</b></p><p class="open"><small>Too close to call</small><b data-open>100%</b></p><p class="out"><small>Can’t work</small><b data-out>0%</b></p></div><div class="controls stack">${range("c", "Limit on each foot", 110, 240, 1, 160, "160 N")}${range("w", "Payload", 250, 500, 5, 400, "400 N")}</div><div class="btnrow"><button type="button" class="btn go sm" data-refine>Refine</button><button type="button" class="btn sm" data-auto>${icon("play")}Refine to the end</button><button type="button" class="btn sm" data-reset>Start over</button></div><div class="cellinfo" data-info aria-live="polite"><p class="micro">Point at a box</p><p>The reason it works, or doesn’t, shows up here.</p></div></div></div>`,
    }),
  }) +
  section({
    ref,
    id: "stance",
    eyebrow: "An exact limit",
    title: "How narrow can it *get*?",
    kicker:
      "This robot has to balance a 40 kg load anywhere in the marked rectangle. Squeeze it until a foot gives out. The limit is 3/14 of a meter from centerline to foot, and you can check the fraction by hand.",
    more: ["The worked answer", "25.13"],
    body: figure({
      name: "stance",
      todo: "Squeeze the robot and drag the load",
      kind: "live",
      ref,
      body: `<div class="stance"><div class="stance-view" data-view></div><div class="stance-side">${seg(
        "share",
        "How the load is shared",
        [
          ["independent", "Active legs"],
          ["springs", "Plain springs"],
        ],
        "independent",
      )}<div class="controls stack">${range("b", "Centerline to foot", 170, 300, 1, 250, "250 mm")}</div><div class="cert"><p class="micro" data-head></p><p class="verdict" data-verdict></p><pre data-proof></pre></div><div class="presets"><button type="button" class="tag" data-b="200">1/5 m</button><button type="button" class="tag" data-b="214.2857142857">3/14 m</button><button type="button" class="tag" data-b="250">1/4 m</button></div></div></div>`,
    }),
  }) +
  section({
    ref,
    id: "docking",
    eyebrow: "Trade-offs",
    title: "Better sensor, or *stronger motor*?",
    kicker:
      "A cart has three pushes to stop on the mark, and it doesn’t know how strong its own motor is. One sensor reading can tell it, if it arrives in time. Trade one part against another and watch the answer flip.",
    more: ["Sensing, timing and control", "C-19"],
    body: figure({
      name: "docking",
      todo: "Make the sensor worse, then make the motor stronger",
      kind: "live",
      ref,
      body: `<div class="docking"><div class="docking-side"><div class="controls stack">${range("U", "Motor strength", 0.5, 1, 0.01, 0.75, "0.75")}${range("eps", "Sensor error", 0, 0.45, 0.005, 0.1, "0.100")}${range("p", "First push", 0.02, 0.75, 0.005, 0.25, "0.250")}</div>${seg(
        "when",
        "Reading arrives",
        [
          ["early", "Reading on time"],
          ["late", "Reading too late"],
        ],
        "early",
      )}<div class="presets"><button type="button" class="tag" data-set="0.75,0.1,0.25">Gentle push</button><button type="button" class="tag" data-set="0.75,0.3,0.625">Firm push, poor sensor</button><button type="button" class="tag" data-set="0.75,0.3125,0.625">The exact limit</button></div><div class="cert"><p class="verdict" data-verdict></p><p data-why></p></div></div><div class="docking-view"><div data-carts></div><div data-chart></div></div></div>`,
    }),
  }) +
  chapterClose;
