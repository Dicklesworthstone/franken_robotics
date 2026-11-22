const tracks = [
  {
    name: "Design a body",
    icon: "M5 19h14 M8 18l4-9 5 3 3-7 M10 8l2-2 3 2 M7 18h4",
    chapters: "§§ 13–15, 28",
    text: "Describe a leg, a wheel or a transmission in the same mechanical language. Trace a change in geometry through loads, torque and power.",
    link: "14",
    action: "Explore robot morphology",
  },
  {
    name: "Build a world",
    icon: "M3 18l6-10 4 5 3-5 5 10z M7 18l6-5",
    chapters: "§§ 16–21",
    text: "Take a robot from a warehouse aisle to a kitchen doorway. Floor friction, contact geometry and sensor timing can each change the outcome.",
    link: "19",
    action: "Explore the environments",
  },
  {
    name: "Test a mission",
    icon: "M3 19h18 M5 16l4-5 4 2 6-8 M16 5h3v3",
    chapters: "§§ 24–29",
    text: "Ask which bodies can carry the tray. Map the feasible designs, find a world that breaks a candidate, and replay the failure with one change.",
    link: "25",
    action: "Explore mission queries",
  },
];
const studio = [
  {
    stage: "Synthesize",
    question: "Which bodies could carry the tray?",
    command: "frob mission show q:5b1e… --set unresolved --order cost --limit 3 --json",
    answer:
      "The cheapest unresolved design costs 1,120 in the plan’s example. The studio proposes a test of its controller before spending more on a prototype.",
  },
  {
    stage: "Refute",
    question: "Could another controller fit it through the door?",
    command: "frob mission refute q:5b1e… --cell c:0119 --json",
    answer:
      "The loaded body is too wide for the doorway. The example identifies a 14 mm reduction in track width as the smallest change that would remove this geometric obstruction.",
  },
  {
    stage: "Diagnose",
    question: "Which change in the kitchen caused the failure?",
    command: "frob autopsy intervene r:5c11… --key aut-1 --json",
    answer:
      "Two edits reproduce the failure: a higher counter lip and a wet strip before it. Restoring dry friction removes the failure in 60 of 64 example replays. A placebo changes two.",
  },
  {
    stage: "Revise",
    question: "Which results survive a bigger wheel?",
    command: "frob result revalidate carrier-b --since d:a3f1… --json",
    answer:
      "After increasing the wheel radius from 55 to 65 mm, the example retains 14 of 19 results and identifies five tests to run again.",
  },
];
export function home() {
  return `<section class="hero wrap">
<div class="hero-copy"><p class="eyebrow"><i></i> Robotics research <span>/</span> Plan v1.0</p>
<h1>A proving ground<br>for <em>robots.</em></h1><p class="intro">How do you know which robot to build? FrankenRobotics proposes a physics lab where an AI agent can design a body, test it in a world, and explain why it works.</p>
<div class="actions"><a class="primary" href="/plan/">Explore the plan <b>↗</b></a><a class="secondary" href="#field-guide">Field guide <b>↓</b></a></div></div>
<figure class="hero-plate"><img src="/media/quadruped.webp" alt="Concept illustration of a four-legged research robot on a laboratory floor" fetchpriority="high" width="1400" height="933"><span class="corner tl"></span><span class="corner tr"></span><span class="corner bl"></span><span class="corner br"></span><figcaption><span class="cross">⌖</span> Quadruped morphology <small>Concept plate</small></figcaption></figure>
</section>
<div class="stats"><dl class="wrap">${[
    ["24", "Mission scenarios"],
    ["7", "Kinds of claim"],
    ["3", "Dynamics tiers"],
    ["10×", "Research target"],
  ]
    .map(([v, l]) => `<div><dt>${l}</dt><dd>${v}</dd></div>`)
    .join("")}</dl></div>
<section class="wrap guide" id="field-guide"><p class="eyebrow"><i></i> Field guide</p><h2>Inside the <em>proving ground.</em></h2>
<div class="cards">${tracks.map((t) => `<a class="card" href="/plan/${t.link}/"><div class="card-top"><svg viewBox="0 0 24 24" aria-hidden="true"><path d="${t.icon}"/></svg><span>${t.chapters}</span></div><h3>${t.name}</h3><p>${t.text}</p><strong>${t.action} <b>→</b></strong></a>`).join("")}</div></section>
<section class="feature"><div class="wrap split"><div><p class="eyebrow"><i></i> Worked example</p><h2>How wide should<br>a robot <em>stand?</em></h2><p class="lede">Four feet support a 400 N load. Its centre of mass can shift 150 mm fore or aft and 75 mm sideways. Each foot can supply up to 160 N. How narrow can the stance be?</p><a class="inline" href="/plan/25/#s-25.13">Read the derivation ↗</a></div>
<div class="instrument"><div class="instrument-head"><strong>Four-foot stance</strong><span>Static load model</span></div>
<div class="choices"><span class="micro">Half-width</span>${[
    ["0", "200 mm"],
    ["1", "3/14 m"],
    ["2", "250 mm"],
  ]
    .map(([i, v]) => `<button data-stance="${i}" aria-pressed="${i === "1"}">${v}</button>`)
    .join("")}</div>
<div class="measure"><span class="micro">Diagonal force difference · <span id="force">160 N</span></span><span class="micro">Limit · 160 N</span></div>
<div class="gauge"><div class="fill"></div><i></i></div>
<div class="answer" aria-live="polite"><strong id="stance-title">Exactly at the limit</strong><p id="stance-detail">One feasible allocation is 160, 130, 110 and 0 N at the positive load corner. Symmetry covers the other corners.</p></div>
<small>The foot forces can adjust as the load moves. Half-length is fixed at ⅓ m.</small></div></div></section>
<section class="wrap guide studio"><p class="eyebrow"><i></i> Agent design studio</p><h2>One delivery, <em>four design decisions.</em></h2><p class="lede">Follow a carrier through a kitchen with a wet floor and a raised counter lip. These commands and outcomes come from the plan’s illustrative CLI session.</p><div class="studio-steps">${studio.map((s, i) => `<details name="studio-step" ${i === 0 ? "open" : ""}><summary><span>${s.stage}</span>${s.question}</summary><div><pre><code>${s.command}</code></pre><p>${s.answer}</p></div></details>`).join("")}</div><a class="inline" href="/plan/32/#s-32.16">Read the full agent session ↗</a></section>
<section class="wrap guide"><p class="eyebrow"><i></i> Research agenda</p><h2>What would count as <em>evidence?</em></h2><div class="rows">
<a href="/plan/9/"><span>Mathematics</span><div><h3>Give every frame and force a type.</h3><p>Lie groups describe poses. Their tangent spaces describe velocities. Dual spaces carry forces, so a frame change has to preserve mechanical power.</p></div><b>↗</b></a>
<a href="/plan/23/#s-23.8"><span>Learning</span><div><h3>Train a controller where its observations live.</h3><p>The proposed gym keeps sensing, policy updates and rollouts on the device. FrankenTorch supplies the CPU reference for checking the gradients used in training.</p></div><b>↗</b></a>
<a href="/plan/21/"><span>Hardware</span><div><h3>Run many bodies in many worlds.</h3><p>The plan targets CUDA on dual RTX 4090s and Metal on Apple silicon. A CPU referee built on FrankenSim supplies a comparison for the accelerated physics.</p></div><b>↗</b></a>
<a href="/plan/12/"><span>Verification</span><div><h3>Trace an equation all the way to its checker.</h3><p>FrankenSymPy handles symbolic derivations and FrankenLean checks selected theorems. A separate auditor would let another researcher verify a design certificate offline.</p></div><b>↗</b></a>
<a href="/plan/39/"><span>Acceptance</span><div><h3>Time the answer an independent judge accepts.</h3><p>The proposed ten-fold test compares complete design studies on sealed missions. Search, simulation, rejected candidates and verification all consume the budget.</p></div><b>↗</b></a>
</div></section>
<section class="library-call"><div class="wrap"><div><p class="eyebrow"><i></i> Final hybrid edition</p><h2>The complete <em>research plan.</em></h2><p>Read the architecture, proofs, worked examples, milestones and all 15 appendices.</p></div><a class="primary" href="/plan/">Open the library <b>→</b></a></div></section>`;
}
