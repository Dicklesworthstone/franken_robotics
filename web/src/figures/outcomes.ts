import { $, $$, range, seg, marks } from "./lib";

export function outcome(r: number, guard: boolean, tie: boolean) {
  if (r < 1)
    return {
      zone: 0,
      label: "The rival is faster",
      text: "The measured ratio.",
      does: "As withdrawn. Where licences allow, the rival's method is adopted behind the verbs.",
    };
  if (r < 3)
    return {
      zone: 1,
      label: "withdrawn",
      text: "“Measured R̂ with [L, U] on ⟨stratum, regime⟩. The speed claim for this class is withdrawn.”",
      does: "KILL-TENFOLD-SPEED. The class leaves the speed claim in the plan, the README and every report.",
    };
  if (tie)
    return {
      zone: r >= 10 && guard ? 3 : 2,
      label: "Tie with a short program",
      text: "“A short program over ⟨engine⟩ reached the same accepted answers within a factor of 1.5.”",
      does: "KILL-TENFOLD-TIE. Ship the layer over that engine for that stratum, and keep engine work only for the gaps that are internal to an engine.",
    };
  if (r < 10 || !guard)
    return {
      zone: 2,
      label: r < 10 ? "measured" : "measured, because the guard was not met",
      text: "“⟨R̂⟩ times shorter, and at least ⟨L⟩ with 95% confidence, on ⟨stratum, regime⟩.”",
      does: "The word “ten-fold” is not used for the class anywhere. The ledger names the binding ceiling.",
    };
  return {
    zone: 3,
    label: "ten-fold",
    text: "“Ten-fold on ⟨stratum, regime⟩: measured R̂ with [L, U]”",
    does: "BET-13 is met for that stratum and for no other.",
  };
}

export default function mount(root: HTMLElement) {
  const needle = $(root, "[data-needle]");
  const zones = $$(root, ".z");
  const ratio = (v: number) => 10 ** (v / 100);
  const draw = () => {
    const v = r.value;
    const o = outcome(ratio(v), guard.value === "yes", tie.value === "yes");
    needle.style.left = `${((v + 30) / 170) * 100}%`;
    zones.forEach((z, i) => z.classList.toggle("on", i === o.zone));
    $(root, "[data-label]").textContent = o.label;
    $(root, "[data-text]").innerHTML = marks(o.text);
    $(root, "[data-does]").textContent = o.does;
  };
  const r = range(root, "r", (v) => `${ratio(v).toFixed(ratio(v) < 10 ? 1 : 0)}×`, draw);
  const tie = seg(root, "tie", draw);
  const guard = seg(root, "guard", draw);
  draw();
}
