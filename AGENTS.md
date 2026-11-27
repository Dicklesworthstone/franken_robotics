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

## Working rules

These twelve standing rules bind every implementing agent (adopted from the machine-level rules and `frankensim/AGENTS.md`, matching `N34-15`'s rule table):

| # | Rule | Why | Enforced by |
|---|---|---|---|
| 1 | Cargo runs through the build-offload shim (`rch`), which queues for a worker and never compiles locally; `cargo` is not invoked by absolute path. Projects stay under `~/projects`. | Local compiles starve every other agent on a shared host, and checkouts outside `~/projects` break sibling path resolution and offload rules. | `frob_envelope_run` in `scripts/lib/envelope.sh` refuses a command line whose first word is an absolute path ending in `/cargo`; every script in `scripts/` calls plain `cargo`. |
| 2 | Isolated target directories use `${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/<name>`; large scratch uses `${BIG_SCRATCH:-${TMPDIR:-/tmp}}/<name>`. | Parallel sessions and matrix cells contend on build locks and exhaust shared scratch if they share one target directory. | `frob_target_dir` and `frob_scratch_dir` in `scripts/lib/envelope.sh`; scripts use nothing else. |
| 3 | A disk preflight precedes heavy build and test lanes. | Heavy lanes fail mid-run and wedge shared workers when free space runs out. | `frob_disk_preflight` in `scripts/lib/envelope.sh`; a heavy lane that skips it fails its own self-test. |
| 4 | **The machine envelope.** No run on any host uses more than 16 GiB of memory, 8 cores, one hour of wall time, or one device without the owner's written approval given before it starts; builds and tests count; any run longer than a few minutes starts under an explicit timeout; when an agent stops, its jobs stop. | Unbounded or orphaned runs exhaust shared RAM, CPU, devices, or wall-clock capacity across concurrent sessions. | `frob_envelope_run` in `scripts/lib/envelope.sh` (covers runs outside the verbs) and the verbs' own admission (`N36-20`, via `frob` and `frob-launch`), both reading `registries/compute_envelope.toml`. |
| 5 | Lanes that need more than E0 (the world bank on the M5 Ultra, training campaigns on the two RTX 4090 cards, the ten-fold tournament) name their budget in their work package and wait for that approval. Until it is given, the default E0 envelope applies. | Prevents unapproved high-resource jobs from starving the fleet before their budget and kill rules are ratified. | `frob_envelope_run` in `scripts/lib/envelope.sh` reads an approval record in `registries/compute_envelope.toml` and refuses above-E0 runs without one; without an approval, the default applies. |
| 6 | No file or directory is deleted without explicit owner permission. No branches and no worktrees: work lands on `main`, and exclusive file reservations through Agent Mail replace branches. | Prevents unrecoverable file loss and week-long branch divergence across parallel agents. | `frob_envelope_selftest` in `scripts/lib/envelope.sh` greps `scripts/` for deleting commands outside a script's own scratch directory; Agent Mail file reservations are a working practice. |
| 7 | `ubs` runs on changed files before a commit. Formatting, lints, and tests run before a commit. Beads are managed with `br` and triaged with `bv --robot-*` (never bare `bv`). | Catches defects, lint regressions, and formatting drift before code lands on `main`, and prevents interactive TUI hangs. | `scripts/precommit.sh`: `ubs` on changed files, `cargo fmt --check`, `cargo clippy`, `cargo test` for touched crates, `scripts/lib/tests/*_test.sh`, and the six workspace checks. |
| 8 | Source that was never compiled does not land. The commit message carries the verification commands and their results. | Keeps `main` buildable for every agent and leaves an auditable verification trail on every commit. | `scripts/precommit.sh` prints the verification block to paste into the commit message; the first commit contains no crate that has not compiled. |
| 9 | Code is changed by focused patches, never by automated rewrite scripts or broad regular-expression rewrites. An existing file is revised in place; a new file must be new functionality or a required contract or test artifact. | Prevents noisy mechanical diffs, broken abstractions, and file sprawl. | Review; the exit review samples commits for file sprawl. |
| 10 | Before a build failure is attributed to this repository, the stack check runs. An off-pin sibling is classified as a fast-forward or a divergence before any remedy is proposed, and a sibling checkout is never moved back to its pin to make a check pass. | Sibling checkouts move under other agents; misattributing failures or rewinding a sibling destroys another agent's work. | `cargo xtask check-stack` (`N33a-05`; drift classes from `N33a-04`) is the first step of the build-failure recipe below; the check writes nothing to a sibling. |
| 11 | Process is never the product. Before non-product work an agent answers three questions: (1) which capability, defect, or blocker does this address, (2) is it the smallest direct action, and (3) is it worth its cost now. If any answer is unclear the work is not done. | Prevents multi-day tooling detours that ship no real capability. | The exactly-six rule for workspace checks; bead close reasons that cite a test's output; the exit review. |
| 12 | The machine-level rule on Apple Simulator audio applies to any use of a device simulator. This project uses none. | Preserves machine-level audio hygiene so any later simulator use inherits the rule automatically. | Nothing to enforce; recorded so that a later use inherits it. |

For rule 4, `frob_envelope_run` in `scripts/lib/envelope.sh` enforces per-run limits for runs outside the verbs (builds, tests, heavy shell lanes), while the verbs' own admission (`N36-20`) covers runs started through `frob` and `frob-launch`, both reading `registries/compute_envelope.toml`.

### Build-failure recipe (rule 10)

1. Run `cargo xtask check-stack` before attributing a build failure to this repository.
2. Classify any off-pin sibling as a fast-forward or a divergence (`N33a-04` drift classes) before proposing any remedy.
3. Never move a sibling checkout back to its pin to make a check pass; resolve drift forward through the owning upstream bead (`FD-…`).

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
