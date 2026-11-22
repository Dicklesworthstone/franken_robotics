import { entries, inline, ref } from "./plan";

/** Two catalogues read out of the plan at build time, so the page cannot drift from it. */
export interface Story {
  id: string;
  title: string;
  milestone: string;
  later: string;
  href: string;
  /** HTML. A story without a "before" in the plan has none here. */
  before: string;
  after: string;
}
export interface Construction {
  id: string;
  title: string;
  tiers: ("S" | "F" | "M")[];
  href: string;
  crates: string;
  /** HTML, by the plan's own bullet names. */
  replaces: string;
  payoff: string;
  test: string;
}

const body = (id: string) => entries.find((e) => e.id === id)!.body;
const flat = (s: string) => s.replace(/\s+/g, " ").trim();
/** "*After:* the mission is…" opens a panel here, so it opens with a capital. */
const capital = (s: string) => s.replace(/^[a-z]/, (c) => c.toUpperCase());

export const stories: Story[] = [];
for (const [i, t] of body("7").entries()) {
  if (t.type !== "list_item_open") continue;
  const text = flat(body("7")[i + 2]!.content);
  const m = text.match(/^\*\*(NS-\d+) (.+?) \[(M\d)(.*?)\]\.\*\* (.+)$/);
  if (!m) throw Error("A North-Star scenario has a new shape: " + text.slice(0, 80));
  const parts = m[5]!.match(/^(.*?)(?:\*Before:\* (.+?) \*After:\* (.+))?$/)!;
  stories.push({
    id: m[1]!,
    title: m[2]!,
    milestone: m[3]!,
    later: m[4]!.trim(),
    href: ref(m[1]!),
    before: parts[2] ? inline(capital(parts[2])) : "",
    after: inline(capital(flat(`${parts[1] ?? ""} ${capital(parts[3] ?? "")}`))),
  });
}

export const constructions: Construction[] = [];
{
  const b = body("10");
  const rows = new Map<string, { title: string; tier: string; crates: string }>();
  for (const [i, t] of b.entries()) {
    if (t.type !== "tr_open" || b[i + 1]?.type !== "td_open") continue;
    const cells: string[] = [];
    for (let k = i + 1; b[k]!.type !== "tr_close"; k++) if (b[k]!.type === "inline") cells.push(b[k]!.content);
    if (/^C-\d+$/.test(cells[0] ?? "")) rows.set(cells[0]!, { title: cells[1]!, tier: cells[2]!, crates: cells[4]! });
  }
  let at: Construction | undefined;
  for (const [i, t] of b.entries()) {
    if (t.type === "heading_open") {
      const id = b[i + 1]!.content.match(/^(C-\d+)\b/)?.[1];
      const row = id && rows.get(id);
      at = undefined;
      if (!id || !row) continue;
      at = {
        id,
        title: row.title,
        tiers: (["S", "F", "M"] as const).filter((k) => row.tier.includes(`[${k}]`)),
        href: ref(id),
        crates: row.crates.replaceAll("`", ""),
        replaces: "",
        payoff: "",
        test: "",
      };
      constructions.push(at);
    }
    if (!at || t.type !== "list_item_open") continue;
    const m = flat(b[i + 2]!.content).match(/^\*\*(Replaces|Payoff|Test)\.\*\* (.+)$/);
    if (m) at[m[1]!.toLowerCase() as "replaces" | "payoff" | "test"] = inline(m[2]!);
  }
  if (constructions.length !== rows.size || !stories.length) throw Error("The catalogue did not match the plan");
}
for (const c of constructions)
  if (!c.payoff || !c.test) throw Error(`${c.id} has no payoff or no test in the plan`);
