import { $, $$, reduced, seen } from "./lib";

const ORDER = ["a", "c", "d", "e", "b"];
const NAMES: Record<string, string> = {
  a: "Greased floor",
  b: "Stiffer door",
  c: "Heavier tray",
  d: "Wet sole",
  e: "Mat edge",
};
export const fails = (s: string[]) =>
  (s.includes("a") && s.includes("b")) || (s.includes("c") && s.includes("d") && s.includes("e"));

export interface Test {
  set: string[];
  failed: boolean;
  why: string;
  /** The list the search continues with when this test fails. */
  keep?: string[];
}

/** Delta debugging as §26.7 states it, then every single edit and every pair. */
export function run(order = ORDER): Test[] {
  const log: Test[] = [];
  const seen = new Set<string>();
  const key = (s: string[]) => [...s].sort().join("");
  const test = (set: string[], why: string) => {
    const failed = fails(set);
    if (!seen.has(key(set))) {
      seen.add(key(set));
      log.push({ set, failed, why });
    }
    return failed;
  };
  let list = order;
  let k = 2;
  const parts = () =>
    Array.from({ length: k }, (_, i) =>
      list.slice(Math.floor((i * list.length) / k), Math.floor(((i + 1) * list.length) / k)),
    );
  search: while (list.length > 1) {
    const ps = parts();
    for (const p of ps)
      if (test(p, k === 2 ? "half" : "part")) {
        list = p;
        k = 2;
        log.at(-1)!.keep = list;
        continue search;
      }
    if (k > 2)
      for (const p of ps) {
        const rest = list.filter((e) => !p.includes(e));
        if (test(rest, "complement")) {
          list = rest;
          k = Math.max(2, k - 1);
          log.at(-1)!.keep = list;
          continue search;
        }
      }
    if (k >= list.length) break;
    k = Math.min(list.length, 2 * k);
  }
  for (const e of order) test([e], "single");
  for (let i = 0; i < order.length; i++)
    for (let j = i + 1; j < order.length; j++) {
      const pair = [order[i]!, order[j]!];
      if (test(pair, "pair") && pair.length < list.length) {
        list = pair;
        log.at(-1)!.keep = pair;
      }
    }
  return log;
}

export default function mount(root: HTMLElement) {
  const log = run();
  const chips = $(root, "[data-chips]");
  const box = $(root, "[data-log]");
  const say = $(root, "[data-say]");
  const step = $<HTMLButtonElement>(root, "[data-step]");
  const play = $<HTMLButtonElement>(root, "[data-run]");
  const label = play.innerHTML;
  const firstPair = log.findIndex((t, i) => t.why === "pair" || (t.why === "single" && i > 4));
  chips.innerHTML = ORDER.map((e) => `<div class="chip" data-e="${e}"><b>${e}</b><small>${NAMES[e]}</small></div>`).join("");
  const set = (s: string[]) => `{${s.join(", ")}}`;
  let n = 0;
  let timer = 0;
  const paint = () => {
    const now = log[n - 1];
    const kept = log.slice(0, n).findLast((t) => t.keep)?.keep ?? ORDER;
    const done = n === log.length;
    for (const c of $$(chips, ".chip")) {
      const e = c.dataset.e!;
      c.className = "chip";
      if (done) c.classList.add(kept.includes(e) ? "kept" : "gone");
      else if (now?.set.includes(e)) c.classList.add("in-test");
      else if (n && !kept.includes(e) && (now?.why ?? "") !== "pair") c.classList.add("gone");
    }
    box.innerHTML = log
      .slice(0, n)
      .map(
        (t, i) =>
          `<p${t.failed ? ' class="fails"' : ""}><span>${i + 1}</span><span>${t.why} ${set(t.set)}</span><em>${t.failed ? "fails" : "passes"}</em></p>`,
      )
      .join("");
    box.scrollTop = box.scrollHeight;
    say.textContent = done
      ? `${set(kept)} replaces {c, d, e} after ${n} different tests. It is labelled minimum among pairs.`
      : !now
        ? "All five edits together make the robot fail. Which of them are needed?"
        : now.keep && now.why !== "pair"
          ? `Still failing without the rest, so the search continues inside ${set(now.keep)}.`
          : now.why === "pair" || (firstPair >= 0 && n > firstPair)
            ? "Delta debugging has stopped at {c, d, e}: take any one away and the failure goes. Now the pairs."
            : now.failed
              ? "This one fails."
              : "Passes, so these edits alone are not the cause.";
    step.disabled = done;
  };
  const stop = () => {
    clearInterval(timer);
    timer = 0;
    play.innerHTML = label;
  };
  const next = () => {
    if (n < log.length) n++;
    if (n === log.length) stop();
    paint();
  };
  step.addEventListener("click", () => {
    stop();
    next();
  });
  const go = () => {
    if (n === log.length) n = 0;
    play.textContent = "Stop";
    timer = window.setInterval(next, reduced ? 60 : 420);
  };
  play.addEventListener("click", () => (timer ? stop() : go()));
  if (!reduced) seen(root, () => void (!n && !timer && go()));
  $(root, "[data-reset]").addEventListener("click", () => {
    stop();
    n = 0;
    paint();
  });
  paint();
}
