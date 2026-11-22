You are one of several drafters converting the FrankenRobotics plan into beads (self-contained work items). Your drafting unit is **{U}**.

Read, in this order and in full:
1. /Users/jemanuel/projects/franken_robotics/planning/fable/beads/BEADS_CHARTER.md : the binding rules: what a bead is, the file format, what every section must contain, the testing standard.
2. /Users/jemanuel/projects/franken_robotics/planning/fable/beads/briefs/{U}.md : what you own: plan line ranges, seed beads, registries, gates, and any instructions specific to your unit.
3. The plan text your brief assigns to you, by line range (use Read with offset and limit; the plan file is far too large to read whole), and then the other material the brief points to (appendix rows, the §40 rows of your gates, type sketches, verbs, the constructions your mechanisms implement).

Then write the bead definition files and `coverage.md` into your output directory, exactly as the charter specifies.

Rules of work:
- Write each bead to disk as soon as it is drafted (one Write call per bead), so that partial work survives an interruption. Before you start, list your output directory: if bead files are already there, keep them and continue with what is missing.
- Write files only inside your output directory. Do not run `br` or `bv`. Do not launch sub-agents. Do not build or implement anything: the project is at the planning stage and beads only describe work.
- The standard: a bead is implementable by an agent who has never seen the plan. Carry the content itself: formulas with every symbol defined, thresholds with their numbers, types, algorithms as numbered steps, refusal codes with meanings and repairs, the reasons for decisions and the alternatives rejected. DO NOT OVERSIMPLIFY and DO NOT LOSE ANY FEATURE OR FUNCTIONALITY: everything in your subsections lands in some bead. Every bead has a real test plan: named unit tests with inputs and expected values, an end-to-end script with the structured logging of charter section 6, known answers and negative controls.
- If a line range in the brief does not begin at the heading it names, find the heading with grep and adjust.
- Work through your subsections in order and keep a running list for `coverage.md` as you go.

When finished, run the self-check the charter's last section describes, fix what it finds, then reply in at most 150 words with the counts the charter asks for.
