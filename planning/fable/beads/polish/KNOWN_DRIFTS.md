# Program-wide conventions settled during polishing

Reviewers of rounds 1 and 2 kept finding the same drifts in beads of every chapter. Each is settled below, with the
bead that owns the authoritative statement. Check every bead of your slice against this list and bring it into
line; where a bead needs more than the convention (a row in a registry, a new option on a verb), read the owning
bead and follow it exactly, or record a request to its slice.

1. **Running the Gauntlet** (owners `BD-025`, `N38a-02`, `N38b-19`). There is no `frob conformance`,
   `frob-conformance run`, `frob gauntlet` or `frob gates` command, verb or binary. A gate or a known-answer row is
   run as `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test <tier> -- <gate|row>`, where `<tier>` is one
   of the six test targets `g0_laws`, `g1_known_answers`, `g2_parity`, `g3_metamorphic`, `g4_chaos`,
   `g5_determinism` (so `--test g1` is not a target name), with `--features gauntlet-mutations` when the negative
   controls are run (the feature that builds planted defects has this one name everywhere: not `mutants`).
   A gate value is recorded with
   `cargo run -p frob-conformance --example record_gate -- --gate <id> --receipt <hash>`.
2. **Registry files** (owner `STD-TEST`). The physical files are
   `crates/frob-conformance/registry/known_answers.toml`, `controls.toml`, `relations.toml`, `pilots.toml` and
   `comparators.toml`. `registries/known_answers.toml` and `registries/negative_controls.toml` are logical aliases
   of the first two; a second physical copy, or a chapter's private file or private row format, is a lint failure.
   Rows that the plan states without a number get one from the allocation beads: known answers and
   counterexamples from `N38a-40`, refusal codes from `N35-33` and `N35x-02`. A bead never invents a number.
3. **The pilot register** (owner `N38a-09`). One file, `crates/frob-conformance/registry/pilots.toml`, schema
   `frob.gauntlet.pilot/1`. The name `frob.pilot/1` is retired.
4. **Decision records** (lead decision, DESIGN: one place to look is better for users than two). Every decision
   record lives in `docs/adr/`: architecture decisions, bet decisions, open-question decisions and kill-gate
   decisions. `docs/delivery/adr/` is retired. A kill-gate decision is `docs/adr/ADR-<gate-name>-<milestone>.md`
   with the required fields `N40-03` lists.
5. **Run verbs take an idempotency key.** A call of a run-kind verb in a script or an example carries `--key`
   (the conventions section of the `VERBS-<cluster>` beads); two runs that a test compares carry distinct keys, or
   the second returns the first one's stored receipt.
6. **Overflow refusals differ by tier** (owners `N16-REF`, `N21-REF`): `FROB-REF-722` on `batch-cpu`,
   `FROB-REF-610` on the GPU tiers. They are not merged.
7. **No bead waits on what waits on it.** A bead's acceptance criteria, unit tests and its own end-to-end script
   use only what the bead itself delivers and what its dependencies deliver. A check that needs a later bead
   belongs to that later bead (or to a new bead after both); the earlier bead exposes a hook, a fixture or a script
   stage for it.
8. **Scripts call what exists.** An end-to-end script calls only verbs and options that a bead defines and that
   the script's bead depends on, and every option it uses appears in that verb's row in its `VERBS-<cluster>` bead.
   An option a mechanism needs and no row has is added to the mechanism bead and requested of the registry bead.
9. **One recorder per gate** (owner `N38b-19`). A script never writes a gate record itself. It produces a receipt,
   and the gate's value is recorded from that receipt with the `record_gate` example of convention 1, by the one
   bead that measures the gate (named in the gate register row).
10. **Kill-gate decisions have one form** (owner `N40-03`): the decision record at
    `docs/adr/ADR-<gate-name>-<milestone>.md` with its required fields, one of the registered outcomes, and the
    scope the register lists for the gate. A decision bead closes whichever way the measurement falls: it never
    depends on a bead that can only be finished if the gate passes.
11. **Each mechanism bead owns its own end-to-end script** (`scripts/e2e/<name>.sh`), built only on its own
    dependencies. A later end-to-end bead may run that script as a stage. A script shared by several beads has
    one owning bead, named in each of the others.
12. **The registries are the authority.** After the first half of round 3 the verb registries (`VERBS-<cluster>`),
    the refusal, known-answer and counterexample registries and the owner beads named above carry every option,
    code and row that mechanism beads asked for. Where a bead and its registry row disagree, the bead changes,
    unless the row is plainly wrong against the plan: then the bead keeps the plan's form and a request goes to
    the registry.
13. **A bead states what is, not what was asked.** A bead never narrates the review: no "requested of `X`", no
    "until `X` registers it", no step that logs "not run" while a request is open, no interim code "until `X`
    allocates a number". Where a bead needs a row, an option, a field or a code that another bead owns, it states
    it as a fact about that bead, in the owner's exact form, after checking that the owner carries it. If the owner
    does not carry it, the reviewer records a request (in the result file, not in the bead) and the bead still
    states the form it needs; the owner's reviewer then adds it or answers with the form that holds. The value
    `to-register` of the pilot register's kill-gate field is a real register value (`N38a-09`) and is not narration.
    A bead is not edited merely to remove the words "round 2" from a note on where a split came from.
14. **Envelope refusals, their order, and how a job cites its envelope** (owners `N32-REF` row 704, `N35-REF` row
    761, `N36-20`, `NJL-01`; plan §32.11.2 and §36.14; settled by the lead after round 3). The daemon's size
    classifier refuses a `campaign`-class job that has no approved envelope with `FROB-REF-704` when the job is
    submitted, before envelope admission. A job above the class in force for its scope, or above an approved
    scope's per-run or cumulative limit, is refused before it starts with `FROB-REF-761`. The envelope checks come
    before the session-budget check `FROB-REF-701` (DESIGN: the envelope depends only on the job and the ratified
    limits, so every caller gets the same answer, and an agent is never sent to raise a budget for a job the
    envelope refuses anyway). A run verb names the approved scope it belongs to with one common run input,
    `envelope_scope` (command line `--envelope-scope <gate:…|campaign:…|pretrain:…>`), listed with the key, the
    budget, the wait, `--estimate` and `on_cancel` among the inputs every `run` verb has (DESIGN: the repair of
    704 is "resubmit citing it", and an agent over MCP has no launcher, so without this input no agent could ever
    run above E0). The scope of a demand is, in this order: the token of a launcher it runs under (`frob-launch`,
    the campaign runner), a cited `envelope_scope`, a scope the verb derives from its own inputs (`train.family`),
    otherwise `adhoc`, whose class in force is E0. A citation that contradicts the launcher's token is refused. No
    new capability scope is introduced: the session's ordinary run scope and budget apply, and the envelope meter
    bounds what a scope can consume.
