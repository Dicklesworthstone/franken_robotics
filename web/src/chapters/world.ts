import { chapterClose, chapterOpen, figure, icon, range, section, seg, type Chapter, type Refs, toggle } from "../parts";

export const world: Chapter = {
  id: "world",
  number: "03",
  label: "The world",
  short: "Worlds",
  icon: "world",
  colour: "#6cc8f7",
  title: "Real rooms, *real trouble*.",
  thesis:
    "Robots get stuck on thresholds, slide on wet mats and jam in corners four centimeters too tight. FrankenRobotics generates whole buildings full of those hazards and knows the size of every one.",
  sections: [
    ["rooms", "Seven kinds of building"],
    ["corner", "Will it make the turn?"],
    ["lip", "Bigger wheel, or bigger motor?"],
  ],
};

export const renderWorld = (ref: Refs) =>
  chapterOpen(world) +
  section({
    ref,
    id: "rooms",
    eyebrow: "Generated buildings",
    title: "Seven kinds of building, *a new plan every seed*.",
    kicker:
      "Homes, apartments, offices, warehouses, factories, shops and commercial kitchens, each with its own rooms, clutter and traps.",
    more: ["How places are generated", "19"],
    body: figure({
      name: "rooms",
      todo: "Pick a place, then try another seed",
      kind: "live",
      ref,
      body: `${seg(
        "place",
        "Interior",
        [
          ["kitchen", "Commercial kitchen"],
          ["home", "Home"],
          ["apartment", "Apartment"],
          ["office", "Office"],
          ["warehouse", "Warehouse"],
          ["plant", "Factory"],
          ["retail", "Shop"],
        ],
        "kitchen",
      )}<div class="rooms"><div class="rooms-view"><canvas aria-label="An isometric model of the generated interior with its hazards marked"></canvas><div class="pins" data-pins></div><button type="button" class="btn sm" data-seed>${icon("redo")}Another seed</button><p class="seed num" data-seedno></p></div><div class="rooms-side"><p class="micro">Hazards in this one</p><ul data-hazards></ul><p class="expose"><small>What it tests</small><span data-expose></span></p></div></div>`,
    }),
  }) +
  section({
    ref,
    id: "corner",
    eyebrow: "Clearance",
    title: "Will it make *the turn*?",
    kicker:
      "Quick checks get this wrong in both directions. The clearance map looks at every position and heading, then hands back a path, or proof that none exists.",
    more: ["How clearance is decided", "19.11"],
    body: figure({
      name: "corner",
      todo: "Stretch the robot until it jams",
      kind: "live",
      note: "The exact answer for this corner is 2√2·a − 2b, for a corridor a wide and a body b wide.",
      ref,
      body: `<div class="corner"><div class="corner-view" data-plan></div><div class="corner-map"><canvas width="960" height="960" aria-label="Map of where the robot's center can be at the chosen heading: clear, blocked, and too close to call"></canvas><ul class="legend"><li class="in">Clear</li><li class="open">Too close to call</li><li class="out">Blocked</li></ul></div><div class="corner-side"><div class="controls stack">${range("L", "Body length", 0.8, 1.6, 0.01, 1.2, "1.20 m")}${range("b", "Body width", 0.4, 0.75, 0.01, 0.6, "0.60 m")}${range("psi", "Heading shown on the map", 0, 90, 1, 45, "45°")}</div>${toggle("follow", "Map follows the robot", true)}${seg(
        "res",
        "Map detail",
        [
          ["0", "10 cm"],
          ["1", "5 cm"],
          ["2", "2.5 cm"],
          ["3", "1.25 cm"],
        ],
        "2",
      )}<div class="verdicts"><p data-v="inner"><small>Quick check, optimistic</small><b></b></p><p data-v="outer"><small>Quick check, cautious</small><b></b></p><p data-v="cell" class="main"><small>Clearance map</small><b></b></p><p data-v="truth"><small>Exact answer</small><b></b></p></div></div></div>`,
    }),
  }) +
  section({
    ref,
    id: "lip",
    eyebrow: "Thresholds",
    title: "Bigger wheel, or *bigger motor*?",
    kicker:
      "A bigger wheel needs less grip to climb a threshold and more torque to do it. At one setting only the large wheel gets over, and at another only the small one does.",
    more: ["Wheels, tracks and traction", "18"],
    body: figure({
      name: "lip",
      todo: "Change the grip and the torque",
      kind: "live",
      ref,
      body: `<div class="lip"><div class="lip-view" data-view></div><div class="lip-side"><div class="controls stack">${range("mu", "Grip at the edge", 0.5, 1, 0.01, 0.7, "0.70")}${range("cap", "Torque limit", 0.8, 1.3, 0.01, 1.2, "1.20 N·m")}${range("h", "Threshold height", 4, 24, 1, 12, "12 mm")}</div><div class="presets"><button type="button" class="tag" data-set="0.7,1.2">Only the 75 mm wheel</button><button type="button" class="tag" data-set="0.8,0.95">Only the 60 mm wheel</button></div><table class="data"><thead><tr><th>Wheel</th><th>Lever</th><th>Torque</th><th>Grip needed</th><th></th></tr></thead><tbody data-rows></tbody></table></div></div>
<aside class="repair"><div><p class="micro">Or change the room</p><h4>Sometimes the cheapest robot part is a ramp.</h4><p>A cart can’t get through the kitchen door because of a 19 mm threshold. FrankenRobotics prices the fix for the room beside the fix for the robot.</p></div><dl><div><dt>Rubber threshold ramp</dt><dd>$12</dd><span>Robot untouched</span></div><div><dt>Larger drive wheel</dt><dd>$140</dd><span>New gearbox ratio</span></div></dl></aside>`,
    }),
  }) +
  chapterClose;
