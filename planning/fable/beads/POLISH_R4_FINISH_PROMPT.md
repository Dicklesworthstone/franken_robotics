You are a reviewer completing polishing round {N} of the FrankenRobotics beads (about 3,640 self-contained work items in the project's `br` issue tracker). Your slice is **{S}**. Your full instructions are in /Users/jemanuel/projects/franken_robotics/planning/fable/beads/POLISH_R4_REST_PROMPT.md : read that file and follow it exactly, substituting {N} for {{N}} and {S} for {{S}}. It points to the general prompt, the brief and the conventions file (14 items), which you must also read in full. Your slice file is /Users/jemanuel/projects/franken_robotics/planning/fable/beads/polish/round-{N}/{S}.md and your result file is /Users/jemanuel/projects/franken_robotics/planning/fable/beads/polish/round-{N}/out/{S}.json .

**The owner's instruction for this pass**, which rules where it and the other files differ:

> We obviously need to fix outright mistakes in the existing beads. Once again, check over everything again with fresh eyes looking for any blunders, mistakes, errors, oversights, omissions, problems, misconceptions, bugs, etc. Be SUPER thorough and meticulous!

What that means here. The round was paused after the registry beads and the first twenty unit slices; implementation starts when it is complete, so what you leave wrong gets built wrong. Your beads have not been read since the registry and owner beads were revised, and the slice file lists every note addressed to them: apply each one after checking it. Then read every bead in full, slowly, as its implementer would, and check everything that can be checked:

- every command line in the bead against the row of its verb in the `VERBS-<cluster>` bead as it stands now: the verb exists at that milestone, each option exists and is spelled as the row spells it, required inputs are present, a run verb carries `--key`. Forms the registries no longer have are outright mistakes: a per-action flag or a positional action word where the row says `--action <name>` (and `--key` on an action of kind admin); `--run-seed` or `--worlds` beside `--battery` on `eval run`; the build feature `mutants` instead of `gauntlet-mutations`; "five" common run properties where there are six; "variations 10 to 13" where there are 10 to 16;
- every refusal code, exit code, trigger and repair order against its `…-REF` row; every known answer and counterexample number against its `…-KA` or `…-CX` row; every gate, part and recorder against the gate register;
- every number, formula, unit, sign and threshold, recomputed; every identifier and file path, spelled as its owner spells it;
- every acceptance criterion, unit test and script step: it can actually pass, it uses only what the bead and its dependencies deliver, and it would fail if the thing it guards were broken;
- every `deps` entry: real, pointing the right way, none missing, none on a later milestone;
- the plan sections in `plan_refs`: nothing the plan requires is missing, weakened or simplified.

Fix what is wrong by editing the staging file. The reasons for an edit in the instructions file are exactly the kinds of mistake to hunt; rewording that changes no meaning is still not an edit. Report truthfully, including the beads you found right and left alone.

`/Users/jemanuel/projects/franken_robotics/AGENTS.md` now says that implementation has begun. That does not change your task in this session: you review and correct beads, and you implement nothing. Never run a `br` command that writes, never delete a file, do not launch sub-agents.
