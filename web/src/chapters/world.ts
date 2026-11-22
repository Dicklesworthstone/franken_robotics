import { chapterClose, chapterOpen, figure, icon, range, section, seg, type Chapter, type Refs, toggle } from "../parts";

export const world: Chapter = {
  id: "world",
  number: "03",
  label: "The world",
  short: "World",
  icon: "world",
  colour: "#6cc8f7",
  title: "The room is a *variable*.",
  thesis:
    "A robot fails at a threshold, a wet mat or a corner that is four centimetres too tight. FrankenRobotics would generate the places, name each hazard with its dimensions, and treat a twelve-dollar ramp as an answer on the same footing as a new gearbox.",
  sections: [
    ["rooms", "Seven interiors"],
    ["corner", "Fits, cannot fit, not yet known"],
    ["lip", "Two wheels at one threshold"],
  ],
};

export const renderWorld = (ref: Refs) =>
  chapterOpen(world) +
  section({
    ref,
    id: "rooms",
    index: "03.1",
    eyebrow: "Habitat",
    title: "Seven *interiors*.",
    kicker:
      "Each kind of place is a grammar with a seed: rooms, zones, fixtures, clutter, and a list of hazards with dimension ranges. A bounded search found published generators for homes, kitchens, shops and warehouses, and none for factories or commercial kitchens. [§19.7](§19.7)",
    body: figure({
      name: "rooms",
      no: "03.1",
      label: "The seven archetypes of §19.7",
      title: "Pick a place. *Reseed* it.",
      basis: "toy",
      source: [
        ["§19.7", ref("19.7")],
        ["§19.8", ref("19.8")],
      ],
      note: "A small generator written for this page. Hazard names and ranges are from the plan's lists, where every dimension is still marked `assumed`.",
      ref,
      body: `${seg(
        "place",
        "Interior",
        [
          ["home", "Home"],
          ["apartment", "Apartment"],
          ["office", "Office"],
          ["warehouse", "Warehouse"],
          ["plant", "Factory"],
          ["retail", "Shop"],
          ["kitchen", "Commercial kitchen"],
        ],
        "kitchen",
      )}<div class="rooms"><div class="rooms-view"><canvas aria-label="An isometric model of the generated interior with its hazards marked"></canvas><div class="pins" data-pins></div><button type="button" class="btn sm" data-seed>${icon("redo")}Another seed</button><p class="seed num" data-seedno></p></div><div class="rooms-side"><p class="micro">Hazards placed</p><ul data-hazards></ul><p class="expose"><small>What to measure when it fails</small><span data-expose></span></p></div></div>`,
    }),
  }) +
  section({
    ref,
    id: "corner",
    index: "03.2",
    eyebrow: "Clearance complex",
    title: "Fits, cannot fit, *not yet known*.",
    kicker:
      "Two quick tests are wrong here in opposite directions: the disc inside the body says every length fits, and the disc around it says a body that fits cannot. The clearance complex answers with a path, with a proof that there is none, or with the cells it has to refine. [§19.11](§19.11)",
    body: figure({
      name: "corner",
      no: "03.2",
      label: "A right-angle turn between corridors 0.90 m wide",
      title: "How long may the *body* be?",
      basis: "computed",
      source: [["§19.11", ref("19.11")]],
      note: "Every cell is classified by the bound in §19.11 with field and registration error set to zero. The last readout is the closed form, 2√2·a − 2b.",
      ref,
      body: `<div class="corner"><div class="corner-view" data-plan></div><div class="corner-map"><canvas width="960" height="960" aria-label="Slice of the space of positions at the chosen heading: free cells, blocked cells and cells still open"></canvas><ul class="legend"><li class="in">Free throughout the cell</li><li class="open">Undecided at this size</li><li class="out">Blocked throughout the cell</li></ul></div><div class="corner-side"><div class="controls stack">${range("L", "Body length", 0.8, 1.6, 0.01, 1.2, "1.20 m")}${range("b", "Body width", 0.4, 0.75, 0.01, 0.6, "0.60 m")}${range("psi", "Map drawn when turned by", 0, 90, 1, 45, "45°")}</div>${toggle("follow", "Map follows the body", true)}${seg(
        "res",
        "Cell size",
        [
          ["0", "10 cm"],
          ["1", "5 cm"],
          ["2", "2.5 cm"],
          ["3", "1.25 cm"],
        ],
        "2",
      )}<div class="verdicts"><p data-v="inner"><small>Disc inside the body</small><b></b></p><p data-v="outer"><small>Disc around the body</small><b></b></p><p data-v="cell" class="main"><small>Clearance complex</small><b></b></p><p data-v="truth"><small>Closed form</small><b></b></p></div></div></div>`,
    }),
  }) +
  section({
    ref,
    id: "lip",
    index: "03.3",
    eyebrow: "Known answer",
    title: "Two wheels at one *threshold*.",
    kicker:
      "A larger wheel needs less grip to climb a 12 mm lip and more torque to do it. So at one setting only the 75 mm wheel gets over, at another only the 60 mm wheel does, and a rule that says bigger is better fails the test. [§14.14](§14.14)",
    body: figure({
      name: "lip",
      no: "03.3",
      label: "Quasi-static climb, 25 N on the wheel, 12 mm lip",
      title: "Which wheel gets *over*?",
      basis: "computed",
      source: [
        ["§38.17", ref("38.17")],
        ["§14.14", ref("14.14")],
      ],
      note: "Lever arm a = √(2rh − h²), lift arm r − h, torque W·a, friction at least a ÷ (r − h).",
      ref,
      body: `<div class="lip"><div class="lip-view" data-view></div><div class="lip-side"><div class="controls stack">${range("mu", "Friction at the edge", 0.5, 1, 0.01, 0.7, "0.70")}${range("cap", "Torque limit", 0.8, 1.3, 0.01, 1.2, "1.20 N·m")}${range("h", "Lip height", 4, 24, 1, 12, "12 mm")}</div><div class="presets"><button type="button" class="tag" data-set="0.7,1.2">Only the 75 mm wheel</button><button type="button" class="tag" data-set="0.8,0.95">Only the 60 mm wheel</button></div><table class="data"><thead><tr><th>Wheel</th><th>Lever</th><th>Torque</th><th>Friction needed</th><th></th></tr></thead><tbody data-rows></tbody></table></div></div>
<aside class="repair"><div><p class="micro">NS-18 · prices illustrative</p><h5>The repair may live in the room.</h5><p>A wheeled carrier cannot get through kitchen doorway D. The cut names a 19 mm threshold. Two repairs enter one budget as competing, testable decisions.</p></div><dl><div><dt>1:12 rubber threshold ramp</dt><dd>USD 12</dd><span>Policy stays frozen</span></div><div><dt>Larger drive wheel</dt><dd>USD 140</dd><span>Changes the gearbox</span></div></dl></aside>`,
    }),
  }) +
  chapterClose;
