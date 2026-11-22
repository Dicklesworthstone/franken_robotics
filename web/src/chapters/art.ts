/** The pictures on the six cards. Drawn here as SVG, so they are sharp at any size and weigh nothing. */
export type Art = "worlds" | "studio" | "family" | "crash" | "workshop" | "algebra";

const W = 480;
const H = 240;
const svg = (inner: string) => `<svg viewBox="0 0 ${W} ${H}" preserveAspectRatio="xMidYMid slice">${inner}</svg>`;

/* A floor of isometric cells, each a small world with its own robot and its own terrain. */
function worlds() {
  const tx = 46;
  const ty = 23;
  let s = "";
  const hash = (i: number, j: number) => ((i * 7 + j * 13 + ((i * j) % 5) * 11) % 17) / 17;
  for (let i = -2; i < 9; i++)
    for (let j = -2; j < 9; j++) {
      const x = W / 2 + (i - j) * tx * 1.06;
      const y = -6 + (i + j) * ty * 1.06;
      if (x < -60 || x > W + 60 || y < -50 || y > H + 50) continue;
      const h = hash(i, j);
      const on = h > 0.42;
      const c = h > 0.82 ? "#c6f25a" : h > 0.6 ? "#6cc8f7" : "#4ade80";
      s += `<path d="M${x} ${y}l${tx} ${ty}l${-tx} ${ty}l${-tx} ${-ty}z" fill="${on ? c : "#0e1b15"}" fill-opacity="${on ? (0.07 + h * 0.1).toFixed(2) : 0.7}" stroke="${on ? c : "#bef0d4"}" stroke-opacity="${on ? 0.42 : 0.1}"/>`;
      if (h > 0.2 && h < 0.42) s += `<path d="M${x - 16} ${y + 24}l10-9 9 5 8-8 8 9" fill="none" stroke="#bef0d4" stroke-opacity=".3"/>`;
      if (!on) continue;
      const cy = y + ty - 8;
      const legs = h > 0.7 ? 3 : 2;
      s += `<g stroke="${c}" stroke-width="1.8" stroke-linecap="round" fill="none"><rect x="${x - 10}" y="${cy - 7}" width="20" height="7" rx="3" fill="${c}" fill-opacity=".9" stroke="none"/>`;
      for (let k = 0; k < legs; k++) {
        const lx = x - 8 + (16 * k) / (legs - 1);
        s += `<path d="M${lx} ${cy}l${k % 2 ? 2.5 : -2.5} 5l${k % 2 ? -2 : 2} 5"/>`;
      }
      s += "</g>";
    }
  return svg(s);
}

/* A terminal, a check that passes and a body taking shape beside it. */
function studio() {
  const lines: [string, string, string][] = [
    ["$ frob design new carrier", "#e9f2ec", ""],
    ["$ frob design check carrier", "#e9f2ec", ""],
    ["  knee torque", "#98aba1", "31% short"],
    ["$ frob design edit … qdd-l", "#e9f2ec", ""],
    ["$ frob design check carrier", "#e9f2ec", ""],
    ["  eight stages", "#98aba1", "passed"],
  ];
  let s = `<rect x="26" y="26" width="300" height="214" rx="14" fill="#050c08" stroke="#bef0d4" stroke-opacity=".2"/><circle cx="46" cy="46" r="4" fill="#fb7185"/><circle cx="61" cy="46" r="4" fill="#fbbf24"/><circle cx="76" cy="46" r="4" fill="#4ade80"/><path d="M26 62H326" stroke="#bef0d4" stroke-opacity=".13"/>`;
  lines.forEach(([t, c, v], i) => {
    const y = 88 + i * 26;
    s += `<text x="44" y="${y}" fill="${c}" font-family="JetBrains Mono, monospace" font-size="13.500">${t}</text>`;
    if (v) s += `<text x="308" y="${y}" text-anchor="end" fill="${v === "passed" ? "#4ade80" : "#fbbf24"}" font-family="JetBrains Mono, monospace" font-size="13.500" font-weight="600">${v}</text>`;
  });
  // The robot, as a drawing with its joints marked.
  s += `<g transform="translate(398 118)" fill="none" stroke-linecap="round" stroke-linejoin="round"><circle r="78" stroke="#fbbf24" stroke-opacity=".16"/><circle r="56" stroke="#fbbf24" stroke-opacity=".1" stroke-dasharray="3 5"/><path d="M-34 4l-14 26 12 26M34 4l14 26-12 26" stroke="#7c9086" stroke-width="4"/><rect x="-46" y="-26" width="92" height="30" rx="13" fill="#132219" stroke="#e9f2ec" stroke-width="2.500"/><path d="M-26 4l-10 28 14 26M26 4l10 28-14 26" stroke="#e9f2ec" stroke-width="5"/><g fill="#030705" stroke="#fbbf24" stroke-width="2.500"><circle cx="-26" cy="4" r="6"/><circle cx="26" cy="4" r="6"/><circle cx="-36" cy="32" r="6"/><circle cx="36" cy="32" r="6"/></g></g>`;
  return svg(s);
}

/* The map of a family: boxes that work, boxes that can't, and the edge refined between them. */
function family() {
  const edge = (x: number) => 0.2 + 0.62 * Math.exp(-2.6 * x);
  let s = "";
  const put = (x: number, y: number, w: number, depth: number) => {
    const worst = y > edge(x);
    const best = y + w > edge(x + w);
    const state = worst ? "#4ade80" : !best ? "#fb7185" : "#fbbf24";
    if (!worst && best && depth < 5) {
      for (const [dx, dy] of [[0, 0], [1, 0], [0, 1], [1, 1]] as const) put(x + (dx * w) / 2, y + (dy * w) / 2, w / 2, depth + 1);
      return;
    }
    const X = 30 + x * 420;
    const Y = 226 - (y + w) * 212;
    s += `<rect x="${(X + 0.8).toFixed(1)}" y="${(Y + 0.8).toFixed(1)}" width="${(w * 420 - 1.6).toFixed(1)}" height="${(w * 212 - 1.6).toFixed(1)}" rx="${depth < 3 ? 4 : 1.5}" fill="${state}" fill-opacity="${state === "#fbbf24" ? 0.95 : worst ? 0.3 : 0.22}"/>`;
  };
  for (let i = 0; i < 4; i++) for (let j = 0; j < 4; j++) put(i / 4, j / 4, 0.25, 1);
  return svg(s);
}

/* A replay rewound to the moment before the fall, then forked. */
function crash() {
  let s = `<path d="M24 146H214" stroke="#bef0d4" stroke-opacity=".35" stroke-width="2"/>`;
  for (let i = 0; i < 6; i++) s += `<circle cx="${40 + i * 30}" cy="146" r="4.500" fill="${i < 5 ? "#7c9086" : "#b7a2fb"}"/>`;
  s += `<circle cx="214" cy="146" r="13" fill="none" stroke="#b7a2fb" stroke-width="2"/><circle cx="214" cy="146" r="5" fill="#b7a2fb"/>`;
  const fork = (y: number, c: string, ok: boolean, text: string) => {
    s += `<path d="M226 146C270 146 268 ${y} 316 ${y}H352" fill="none" stroke="${c}" stroke-width="2.500" stroke-opacity=".85"/><circle cx="366" cy="${y}" r="12" fill="${c}" fill-opacity=".16" stroke="${c}" stroke-width="2"/><path d="${ok ? `M360 ${y}l4.500 4.500 8-9` : `M361 ${y - 5}l10 10M371 ${y - 5}l-10 10`}" fill="none" stroke="${c}" stroke-width="2.400" stroke-linecap="round" stroke-linejoin="round"/><text x="388" y="${y + 5}" fill="#e9f2ec" font-family="Inter, sans-serif" font-size="14" font-weight="600">${text}</text>`;
  };
  fork(64, "#4ade80", true, "stronger");
  fork(118, "#fb7185", false, "grippier");
  fork(174, "#fb7185", false, "as it was");
  s += `<g transform="translate(112 92)" stroke-linecap="round" stroke-linejoin="round" fill="none"><path d="M-30 6l-10 18 8 18M22 -2l16 18-4 22" stroke="#7c9086" stroke-width="4"/><g transform="rotate(-9)"><rect x="-36" y="-22" width="76" height="25" rx="11" fill="#171327" stroke="#e9f2ec" stroke-width="2.400"/></g><path d="M-22 6l-8 20 12 18M26 -2l12 18-16 12" stroke="#e9f2ec" stroke-width="4.600"/><circle cx="38" cy="16" r="6" fill="#030705" stroke="#fb7185" stroke-width="2.600"/><path d="M52 6l9-5M54 16h11M52 26l9 5" stroke="#fb7185" stroke-width="2.200"/></g><text x="24" y="204" fill="#98aba1" font-family="JetBrains Mono, monospace" font-size="12.500">rewind to just before the fall</text>`;
  return svg(s);
}

/* A leg pulled apart along its axis, dimensioned like a drawing. */
function workshop() {
  const y = 116;
  const part = (x: number, inner: string) => `<g transform="translate(${x} ${y})">${inner}</g>`;
  let s = `<path d="M18 ${y}H462" stroke="#6cc8f7" stroke-opacity=".55" stroke-dasharray="16 5 3 5"/>`;
  s += part(62, `<rect x="-28" y="-40" width="56" height="80" rx="10" fill="#12202a" stroke="#e9f2ec" stroke-width="2.200"/><circle r="22" fill="#0a151c" stroke="#e9f2ec" stroke-width="1.800"/><circle r="8" fill="none" stroke="#6cc8f7" stroke-width="1.800"/>${[45, 135, 225, 315].map((a) => `<circle cx="${(15 * Math.cos((a * Math.PI) / 180)).toFixed(1)}" cy="${(15 * Math.sin((a * Math.PI) / 180)).toFixed(1)}" r="2.500" fill="#6cc8f7"/>`).join("")}`);
  s += part(136, `<path d="M-15 -30h30v60h-30z" fill="#0e1b15" stroke="#e9f2ec" stroke-width="2"/>${[-20, -10, 0, 10, 20].map((v) => `<path d="M-15 ${v}h30" stroke="#e9f2ec" stroke-opacity=".5"/>`).join("")}`);
  s += part(262, `<path d="M-84 -24C-50 -24 -44 -15 -10 -15H24C52 -15 58 -21 84 -21V21C58 21 52 15 24 15H-10C-44 15 -50 24 -84 24Z" fill="#12202a" stroke="#e9f2ec" stroke-width="2.200"/><circle cx="-64" r="9" fill="#050c08" stroke="#6cc8f7" stroke-width="1.800"/><circle cx="66" r="8" fill="#050c08" stroke="#6cc8f7" stroke-width="1.800"/><path d="M-40 0C-20 -9 -8 -8 2 0C12 8 30 8 44 0" fill="none" stroke="#6cc8f7" stroke-opacity=".7"/>`);
  s += part(392, `<circle r="25" fill="#0e1b15" stroke="#e9f2ec" stroke-width="2.200"/><circle r="11" fill="none" stroke="#e9f2ec" stroke-opacity=".6"/>`);
  s += `<g stroke="#6cc8f7" fill="none"><path d="M178 60V50M346 60V50M178 54H346"/><path d="M184 50l-6 4 6 4M340 50l6 4-6 4"/><path d="M367 160v12M417 160v12M367 167H417"/></g><g fill="#6cc8f7" font-family="JetBrains Mono, monospace" font-size="13"><text x="262" y="45" text-anchor="middle">210.00 ±0.05</text><text x="392" y="190" text-anchor="middle">Ø 50</text></g><g fill="#98aba1" font-family="JetBrains Mono, monospace" font-size="12"><text x="62" y="190" text-anchor="middle">qdd-l</text><text x="262" y="190" text-anchor="middle">shank, −18% mass</text></g>`;
  return svg(s);
}

/* Two motions taken in either order. They miss by exactly the bracket. */
function algebra() {
  let s = "";
  for (let i = 0; i < 9; i++) s += `<path d="M${-60 + i * 74} 250C${i * 74} 170 ${20 + i * 74} 90 ${i * 74 - 10} -10" fill="none" stroke="#bef0d4" stroke-opacity=".06"/><path d="M-10 ${-20 + i * 36}C120 ${i * 36 - 46} 320 ${i * 36 + 30} 490 ${i * 36 - 18}" fill="none" stroke="#bef0d4" stroke-opacity=".06"/>`;
  s += `<defs><marker id="al-a" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6.500" markerHeight="6.500" orient="auto"><path d="M0 0 10 5 0 10Z" fill="#c6f25a"/></marker><marker id="al-b" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6.500" markerHeight="6.500" orient="auto"><path d="M0 0 10 5 0 10Z" fill="#4ade80"/></marker><marker id="al-c" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto"><path d="M0 0 10 5 0 10Z" fill="#fbbf24"/></marker></defs>`;
  s += `<g fill="none" stroke-width="3" stroke-linecap="round"><path d="M96 194C148 168 198 178 252 188" stroke="#c6f25a" marker-end="url(#al-a)"/><path d="M256 182C286 142 288 104 296 66" stroke="#4ade80" marker-end="url(#al-b)"/><path d="M92 186C104 148 120 108 142 80" stroke="#4ade80" stroke-opacity=".75" stroke-dasharray="1 7" marker-end="url(#al-b)"/><path d="M148 74C196 48 250 40 344 42" stroke="#c6f25a" stroke-opacity=".75" stroke-dasharray="1 7" marker-end="url(#al-a)"/><path d="M302 60L340 48" stroke="#fbbf24" marker-end="url(#al-c)"/></g><circle cx="92" cy="195" r="6.500" fill="#e9f2ec"/><g font-family="JetBrains Mono, monospace" font-size="17" font-weight="600"><text x="172" y="206" fill="#c6f25a">X</text><text x="296" y="128" fill="#4ade80">Y</text><text x="90" y="126" fill="#4ade80">Y</text><text x="222" y="30" fill="#c6f25a">X</text><text x="356" y="72" fill="#fbbf24">[X, Y]</text></g><text x="356" y="94" fill="#98aba1" font-family="Inter, sans-serif" font-size="13">how a car parks</text>`;
  return svg(s);
}

export const art: Record<Art, string> = {
  worlds: worlds(),
  studio: studio(),
  family: family(),
  crash: crash(),
  workshop: workshop(),
  algebra: algebra(),
};
