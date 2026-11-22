import { escape } from "../layout";
import { figure, icon, section, type Refs } from "../parts";

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
  return steps.map((s) => ({
    command: s.command.replace(/\s*\\\s*/g, " ").replace(/\s+/g, " "),
    // The terminal shows what happened; the fine print of a result is in the plan.
    result: s.result.replace(/\s+/g, " ").replace(/; (not a certificate|all three authorization flags false)$/, ""),
  }));
}

/** One line on what each step is for. Matched by verb, so a changed session stops the build. */
const GIST: [verb: RegExp, title: string, text: string][] = [
  [/^frob session open/, "Open a session", "The agent gets a budget of GPU and CPU seconds, and a token that can’t open the sealed test worlds."],
  [/^frob mission new/, "State the mission", "A template becomes a query: which designs, which worlds, how long."],
  [/^frob mission check/, "Check the question", "Two body layouts, 412 regions of design space, and a controller for each that has to work in every world."],
  [/^frob mission synthesize .*--estimate/, "Get a quote", "640 GPU-seconds, before spending one of them."],
  [/^frob mission synthesize/, "Start the search", "It runs as a job, so the agent can get on with something else."],
  [/^frob job wait/, "Read the answer", "61 regions work. 179 can’t, each with its reason. 172 are too close to call."],
  [/^frob mission show .*inner/, "Find the cheapest sure thing", "It costs 1,480 in parts."],
  [/^frob mission show .*unresolved/, "Spot a bargain", "A design at 1,120 hasn’t been settled yet, and the answer says which test would settle it."],
  [/^frob mission refute/, "Ask why one is out", "Loaded, it can’t pass the door, whatever the controller. Narrowing the track by 14 mm would change that."],
  [/^frob design new/, "Build the bargain", "A concrete robot from the middle of the cheaper region."],
  [/^frob nemesis search carrier-b .*nem-1/, "Attack it", "The adversary finds three worlds that beat it."],
  [/^frob nemesis shrink/, "Shrink the failure", "Two details are enough: a counter lip 4 mm higher, with a wet strip 40 mm before it."],
  [/^frob autopsy/, "Find the cause", "Keep the strip dry and 60 of 64 replays succeed. A harmless change saves 2. The cause is grip."],
  [/^frob design edit/, "Fix it", "The wheel grows from 55 to 65 mm."],
  [/^frob result revalidate/, "Keep what still holds", "14 of 19 earlier results survive the edit. The other five need 41 GPU-seconds."],
  [/^frob nemesis search carrier-b .*nem-2/, "Attack it again", "4,096 worlds, and none of them beats it."],
  [/^frob eval qualify .*--dry/, "Read the exam rules", "200 sealed worlds, and it may fail at most three."],
  [/^frob eval qualify/, "Take the exam", "It fails one world in 200. Qualified."],
  [/^frob prove bundle/, "Pack the evidence", "Everything an auditor needs goes into one file."],
  [/^frob-verify/, "Audit it offline", "A separate program re-checks the file with no network and none of the same code."],
];

export function renderSession(ref: Refs, session: string) {
  const gists = sessionSteps(session).map((s) => {
    const g = GIST.find(([verb]) => verb.test(s.command));
    if (!g) throw Error("No gist for session step: " + s.command);
    return { ...s, title: g[1], text: g[2] };
  });
  return section({
    ref,
    id: "session",
    eyebrow: "Watch an agent run it",
    title: "Twenty commands to a *qualified robot*.",
    kicker:
      "The mission: carry a load across a wet strip of kitchen floor. The agent finds a cheap design, breaks it, fixes the wheel and puts it through the exam.",
    more: ["How agents drive it", "32"],
    body: figure({
      name: "session",
      todo: "Step through it, or press play",
      kind: "example",
      ref,
      className: "feature",
      body: `<div class="session"><ol class="session-steps" role="list">${gists
        .map(
          (s, i) =>
            `<li><button type="button" data-i="${i}" aria-pressed="${i === 0}"><span class="num">${String(i + 1).padStart(2, "0")}</span>${escape(s.title)}</button></li>`,
        )
        .join("")}</ol><div class="session-main"><div class="term"><p class="term-bar"><i></i><i></i><i></i><span>agent-17</span></p><div class="term-body" data-term tabindex="0" aria-label="Terminal transcript">${gists
        .map(
          (s, i) =>
            `<div class="tline" data-step="${i}"><p class="cmd"><span>$</span> ${escape(s.command)}</p>${s.result ? `<p class="res">${escape(s.result)}</p>` : ""}</div>`,
        )
        .join("")}</div></div><div class="session-note"><p class="micro" data-n>Step 1 of ${gists.length}</p><h4 data-t>${escape(gists[0]!.title)}</h4><p data-d>${escape(gists[0]!.text)}</p><div class="btnrow"><button type="button" class="btn sm" data-prev>${icon("left")}Back</button><button type="button" class="btn go sm" data-next>Next${icon("right")}</button><button type="button" class="btn sm" data-play>${icon("play")}Play</button></div></div></div><template data-gists>${escape(JSON.stringify(gists.map((g) => [g.title, g.text])))}</template></div>`,
    }),
  });
}
