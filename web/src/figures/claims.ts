import { $, $$, seg } from "./lib";

/** Weakest first, as §8.2 orders them. */
const CLASSES = ["Estimated", "Tested", "Certified", "Proved"];
const CLASS: Record<string, string> = {
  Certified: "Certified",
  Tested: "Tested",
  Assumed: "Estimated",
  "Bench-measured": "Tested",
  "Field-calibrated": "Tested",
  "50 loose runs": "Estimated",
  "Anytime-valid": "Tested",
};
const NEXT: Record<string, string> = {
  friction: "Measure the friction coefficient. It is one field of the interface card, and everything built on that card carries it.",
  study: "Run the comparison as a paired, anytime-valid study on common world addresses.",
  kernel: "Certify the kernel's group-membership identity instead of testing it.",
};
const LABEL: Record<string, string> = {
  kernel: "kernel identity",
  friction: "friction field of the interface card",
  study: "paired study",
};

export default function mount(root: HTMLElement) {
  const draw = () => {
    const inputs = (["kernel", "friction", "study"] as const).map((key) => ({ key, cls: CLASS[value[key].value]! }));
    const rank = Math.min(...inputs.map((i) => CLASSES.indexOf(i.cls)));
    // When several inputs tie, the assumed card field is the one worth naming first.
    const weak = (["friction", "study", "kernel"] as const).find((k) => CLASSES.indexOf(CLASS[value[k].value]!) === rank)!;
    for (const el of $$(root, "[data-input]")) el.classList.toggle("weak", el.dataset.input === weak);
    $(root, "[data-ladder]").innerHTML = CLASSES.map((c, i) => `<span${i === rank ? ' class="on"' : ""}>${c}</span>`).join("");
    const gpu = value.tier.value === "GPU";
    const none = value.real.value === "None";
    $(root, "[data-report]").textContent = [
      "kind      Design comparison",
      `class     ${CLASSES[rank]}`,
      `weakest   ${LABEL[weak]}`,
      `tier      ${gpu ? "GPU, so provisional until promoted" : "Referee"}`,
      `reality   ${none ? "no-claim" : "the calibrated family the band names"}`,
      "execution_authorized      false",
      "hardware_safety_signoff   false",
      "sim_to_real_certified     false",
    ].join("\n");
    $(root, "[data-next]").textContent =
      rank === 0
        ? NEXT[weak]!
        : gpu
          ? "Promote the finalists to the Referee. The class is already as high as a statistical study can make it."
          : none
            ? "Freeze the predictions, then measure a real robot. Nothing in simulation can change the reality scope."
            : "Nothing in this list raises it further. Proved and Certified come only from a checker.";
  };
  const value = {
    kernel: seg(root, "kernel", draw),
    friction: seg(root, "friction", draw),
    study: seg(root, "study", draw),
    tier: seg(root, "tier", draw),
    real: seg(root, "real", draw),
  };
  draw();
}
