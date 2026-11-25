# Project scope

FrankenRobotics is in the implementation stage. On 2026-11-23 the project owner changed the scope: the plan has
been converted into Beads, and implementing those Beads is now the work. Work on the plans, their research evidence
and their review continues beside it.

## Where the work is

- The work items are the Beads in this repository's `br` tracker (`.beads/`, prefix `frob`): about 3,640 of them,
  each self-contained, with its acceptance criteria, unit tests and end-to-end script. Find work with
  `br ready --json` or `bv --robot-triage` (never bare `bv`), take Beads in dependency order, and close a Bead only
  when everything its acceptance criteria name passes.
- `COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md` is the specification the Beads came from. A Bead is meant to be
  enough on its own; where a Bead and the plan disagree, say so in the Bead before building either reading.
- Registry Beads are the authority for what they register: the verb registries `VERBS-<cluster>`, the refusal,
  known-answer and counterexample registries (`…-REF`, `…-KA`, `…-CX`), and the owner Beads `NJL-01`, `STD-E2E`,
  `STD-TEST` and `BD-025`. Where a mechanism Bead and its registry row differ, the registry row holds. The
  program-wide conventions are in `planning/fable/beads/polish/KNOWN_DRIFTS.md`.
- A Bead with the label `review-notes-open` carries a tracker comment (shown by `br show <id>`) listing review
  notes that were addressed to it after its own review and are not yet in its description. Read the comment
  before starting the Bead, apply each note or confirm that it does not hold, and remove the label in the
  staging file when done.

## Changing Beads

- Every Bead has a staging file, `planning/fable/beads/defs/<unit>/<key>.md`, and the tracker is filled from those
  files by `planning/fable/beads/tools/sync_beads.py` (which uses only `br`). A change made with `br` alone to a
  Bead's description, labels or dependencies is overwritten by the next sync unless the staging file carries it too.
  Status changes (`br update --status`, `br close`) are not touched by the sync.
- Every sibling project (frankensim, fastmcp_rust, asupersync and the rest) is the owner's: a change a Bead needs
  in a sibling is made in that sibling's repository, by the upstream Beads (`FD-…`), not worked around here.

## Working rules (machine and suite)

These standing rules bind every implementing agent (adopted from the machine-level rules and `frankensim/AGENTS.md`):

- **Build offload only.** `cargo` runs through the build-offload shim (`rch`) and is never invoked by absolute path.
- **Canonical project tree.** Projects stay under `~/projects`.
- **Isolated target directories.** Use `${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/<name>` so parallel sessions and matrix cells never contend on build locks.
- **Disk preflight.** Run a free-space preflight before heavy build or test lanes.
- **Machine envelope and timeouts.** No run uses more than 16 GiB of memory, 8 cores, or one hour of wall time without the owner's written approval given before it starts. Any run longer than a few minutes starts under an explicit timeout.
- **No deletions without permission.** No file or directory is deleted without explicit owner permission.
- **No branches and no worktrees.** Work lands on `main`; exclusive file reservations through Agent Mail replace branches.
- **Pre-commit quality bar.** Run `ubs` on changed files, and run formatting, lints, and tests before a commit.
- **Issue tracking and triage.** Manage Beads with `br` and triage with `bv --robot-*` (never bare `bv`).
- **Compiled source and commit evidence.** Source that was never compiled does not land, and the commit message carries the verification commands and their results.
- **Focused patches only.** Code is changed by focused patches, never by automated rewrite scripts or broad regular-expression rewrites.
- **Stack attribution and sibling pins.** Before a build failure is blamed on this repository, run `xtask check-stack` (or `scripts/bootstrap/first_receipt.sh` until `xtask check-stack` lands). Never move a sibling checkout back to its pin to make a check pass.
- **Process is never the product.** Before any non-product work, an agent answers three questions: (1) which capability, defect, or blocker does this address, (2) is it the smallest direct action, and (3) is it worth its cost now.

## Swarm rules (binding on every agent)

The goal is working code. A Bead is done when its code and its tests exist and pass, not when a report says so.

- **Claim, one at a time.** `br update <id> --claim --actor <YourAgentMailName> --no-auto-flush`. Hold one Bead in
  `in_progress` at a time. Reserve the files you will edit through Agent Mail before editing them.
- **Real code, real tests, same Bead.** Write what the Bead's technical approach and test plan name. Forbidden:
  `todo!()`/`unimplemented!()` or stub bodies in a commit, tests that assert nothing or were weakened to pass,
  fixtures or mocks presented as live proof, hard-coded success paths, golden files regenerated to force green,
  silenced stderr in a command whose output is cited as evidence.
- **Code first, builds through rch only.** `cargo` is shimmed to the rch fleet; never call it by absolute path. An
  agent runs at most `cargo check -p <crate> --all-targets` on its own crate before handing off. The orchestrator
  runs the test suites centrally. Shell and Python tests that build nothing may be run by the author.
- **Commit only your own paths**, after `cargo check` passes (`git add <your paths>`, never `git add -A`). Never
  stash, reset, revert or rewrite another agent's changes; other agents commit to `main` all the time.
- **Hand off, never close.** When the code and tests are written and every acceptance item maps to a test, run
  `br update <id> --status batch_pending --actor <You> --no-auto-flush --transition-comment "<commit sha>; tests:
  <exact test names>; acceptance: <item -> test>; not done: <anything>"`. Only the orchestrator closes, after it
  re-runs the tests; the tracker refuses every other close. An honest "not done: X" keeps the Bead open and is
  worth more than a false close.
- **No process artifacts.** No new ledgers, dashboards, certificates, status reports or meta-Beads unless a Bead
  names them as its deliverable. Guard and refusal paths never close a Bead whose positive capability is missing.
- **Staging files and sync.** Do not edit `planning/fable/beads/defs/` or run `sync_beads.py`; report a wrong Bead
  to the orchestrator by Agent Mail instead. Do not change sibling repositories unless assigned an `FD-…` Bead.
- **Talk through Agent Mail** (project key: this repository's absolute path). Check your inbox between Beads.
