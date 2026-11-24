---
key: "NAB-11"
title: "registries/sources.toml: local sources read at plan time (sibling files and commits, this machine, records)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "suite", "gauntlet", "delivery", "solid", "registry"]
parent: "WP-01"
deps: ["BD-001", "BD-025", "NAB-09"]
needs: []
related: ["NAB-15"]
gates: []
plan_refs: ["Appendix H.2", "§0.1 (rule 2)", "§1.7", "§33"]
---
## Background

FrankenRobotics is built on sibling projects that the same owner maintains: FrankenSim (geometry, multibody
dynamics, contact, statistics, structure), FrankenTorch (tensors and the Metal kernel gateway), FrankenNumPy,
FrankenSymPy (symbolic derivation and exact solvers), FrankenLean (proof checking), fastmcp_rust (the MCP
server library), asupersync (the async runtime and its lab), and others. The specification's statements about
what those siblings contain are FACTs read from their source and documents on 2026-11-21. Each such statement
is a fact about the commit it was read at; a sibling moves, so every one is re-read at the pinned commit of the
stack lock before anything depends on it, and a defect that a review *reported* in a sibling is written "FACT
(to recheck)" and becomes an admission test and an upstream work item.

This bead enters those readings into the source register (`registries/sources.toml`, schema `frob.sources/1`,
NAB-09): one record per file or crate read, with repository, path, line range where one was recorded, the
commit where one was recorded, and what the reading supports. It also enters two other kinds of local source:
the hardware facts of the development machine, and the planning records.

Why the list matters (DESIGN): the dependency firewall and the admission gates decide which sibling crates may
be linked. A file that was only *read as a report of a defect* (the stand-in training crate, for example) must
never be mistaken for an admitted dependency; the register marks the difference. Tier: Solid.

## Scope

In: the `[[source]]` records of kind `local`, `machine` and `planning-record` listed below. Out: the schema
(NAB-09); re-reading at the pinned commits (N00-16); re-running the machine probes whose scripts were not
retained (N00-35, and the hardware fingerprint BD-029); the public sources (NAB-12, NAB-13); the plan-time
checks (NAB-15).

## Technical approach

**Transcription rule.**

1. One record per backticked path. `kind = "local"`, `repo` = the sibling's directory name, `locator` =
   `<repo>:<path>`, `read_on = "2026-11-21"`, `licence = "owner"`, one `reading` with `mark = "P"` (a raw file
   was read) and `covers` = the line range where the text gives one, or "whole file". The two workspace rule
   files lie in no sibling repository, and `/Users/jemanuel/AGENTS.md` lies outside `local_root`: their records
   have `repo = ""`, `role = "document"`, `commit = ""` and a home-relative `locator` (`~/AGENTS.md`,
   `~/projects/AGENTS.md`), so that no locator carries the user name; NAB-09's loader accepts an empty `repo` on
   a `local` record only with a locator of that `~/` form, and refuses any other `local` record whose `repo` is
   empty with `src.local-path-missing-repo`.
2. A crate named without a file (`fs-archive`, `fs-bo`, `fs-topopt`) is one record with `locator =
   "<repo>:crates/<crate>"` and `covers = "crate surface, as read"`. A path printed without its crate belongs to
   the crate named last before it in the same item: a bare file name, or a path that does not start with `src/`
   or `tests/`, lies under that crate's `src/` (`lie_differential.rs` is
   `frankensim:crates/fs-ga/src/lie_differential.rs`; `articulated/generalized.rs` is
   `frankensim:crates/fs-mbd/src/articulated/generalized.rs`), while `src/…`, `tests/…`, `CONTRACT.md` and
   `Cargo.toml` lie under the crate root; `src/{kernel.h,cuda.rs,metal.rs,emulate.rs}` is four records. A name
   without an extension inside a crate's list (`tri_wheel_cluster`; `frictional_multicontact`, `certified_ccd`;
   `product_mlmc_allocation`, `product_control_variate`, `product_copula`) is a module of that crate: one record
   located at the crate, with `covers = "module <name>, as read"`. A path this rule resolves wrongly is reported
   by the existence check of the test plan, and the re-read corrects it.
3. `commit` is filled for the five repositories where a commit was recorded: frankensim `483d09c682`,
   franken_lean `4426f8ea7`, fastmcp_rust `117388bc`, frankentorch `8165d6a18028`,
   franken_drone_geometry_reconstruction `db45703afc5e`. For the others `commit = ""`, and the first re-read
   records the commit it was made at.
4. `role` (the local-record field of NAB-09's schema): `linked-candidate` (a crate the airlock may consume, subject
   to its admission gate), `document` (a plan, README or rule file), `build-fact` (a manifest, lock or toolchain
   file), `defect-report` (read only as a report of a defect and not linked), `pattern` (read for a pattern to
   follow, not to depend on).
5. Every local record has `reread_before_gate = true` until the fact-sheet script re-reads it at the stack-lock
   commit. The paths are stored repository-relative; the absolute prefix `/Users/jemanuel/projects/` is the
   value of a single `local_root` key, so a replication kit can rewrite it.
6. Notes carried on records: `fs-conform` is a converter-conformance kit, **not** conformal prediction (a name
   collision that has misled readers); `fs-g1-train` (`src/ppo.rs` lines 253–345, `standin_env.rs`) and
   `fs-cmaes-viz-wasm/src/g1_walking.rs` are `defect-report` records and are never linked.
7. A document named in words and not by path is one record per phrase, located at its repository (`locator =
   "<repo>:"`, followed by the crate path where one is printed), with `covers` = the phrase as printed; the first
   re-read records the exact path in its note. There are seven: franken_numpy's "README and manifests";
   asupersync's "README and manifests"; franken_lean's "S26 conformance receipt (per-module results)"; the README
   of frankenscipy's `fsci-opt`; the README of `frankenjax/crates/fj-egraph`; `franken_firmware`'s toolchain file;
   `franken_drone_geometry_reconstruction`'s status file and schemas.

**The machine record** (`kind = "machine"`, id `SRC-MACH-01`, one reading with mark P: the command output was
read directly): `sysctl` and `system_profiler` report an Apple
M5 Ultra with 36 CPU cores, an 80-core GPU with Metal 4, and 256 GB of memory; a Metal device query reports a
working-set limit of about 222.7 GiB and a largest single buffer of about 167.0 GiB. Only core counts, memory
size and these two Metal limits are recorded for this machine; bandwidths, launch costs and compile times are
not (they are in the not-verified list of NAB-14 and are measured by the hardware fingerprint).

**The planning records** (`kind = "planning-record"`, ids `SRC-REC-01` to `SRC-REC-05`, one reading each with
mark P, the files read directly): the verdict register of
24 hypotheses with its 23 evidence packs; two independent audits of 76 claims (36 in the first; 40 of 58 in the
second, with 18 left pending); a harvest of 18 primitive matches; a compilation of eight decision families; and
the planning reproduction archive (NAB-15). Their files are under `planning/` in this repository, and the
review log (N00-05) names them.

**The sources as read at plan time.**

Read on 2026-11-21; paths are under `/Users/jemanuel/projects/` unless noted. Commits where a chapter recorded one:
frankensim `483d09c682`, franken_lean `4426f8ea7`, fastmcp_rust `117388bc`, frankentorch `8165d6a18028`,
franken_drone_geometry_reconstruction `db45703afc5e`.

**frankensim, plans and repository files.** `COMPREHENSIVE_PLAN_FOR_FRANKENSIM.md`,
`COMPREHENSIVE_ADDENDUM_TO_FRANKENSIM_PLAN.md`, `COMPREHENSIVE_BRIDGE_PLAN_FOR_FRANKENSIM.md`,
`COMPREHENSIVE_PLAN_TO_EXTEND_FRANKENSIM_TO_NEW_DOMAINS.md`,
`COMPREHENSIVE_PLAN_FOR_REAL_TIME_WRIGHT_FLYER_SIM_WITH_FRANKENSIM.md`, `AGENTS.md`, `README.md`,
`docs/ACCELERATOR_DOCTRINE.md`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `constellation.lock`,
`capability-maturity.json`, `unsafe-capsules.json`.

**frankensim, crates** (under `frankensim/crates/`).

- Geometry, dynamics and contact: `fs-ga/src/lie.rs`, `lie_differential.rs`, `lie_opt.rs`; `fs-mbd/src/articulated.rs`,
  `articulated/generalized.rs`, `articulated/point_motion.rs`, `point_motion.rs`, `robot_models.rs`,
  `planar_drive.rs` (lines 117–133), `articulated_contacts.rs`, `articulated_contacts/step.rs`,
  `articulated_contacts/step/sustained.rs`, `tri_wheel_cluster`, `CONTRACT.md`; `fs-mbd-material/src/geometry.rs`;
  `fs-contact/CONTRACT.md`, `src/broad_phase.rs`, `src/sustained/solve.rs` (line 26), `frictional_multicontact`,
  `certified_ccd`; `fs-time/src/hybrid_sensitivity.rs`; `fs-query/src/convex.rs`; `fs-scene/src/lib.rs`;
  `fs-tribo/src/lib.rs`, `src/stribeck_friction.rs`; `fs-orbit`; `fs-spectral/src/floquet.rs`; `fs-symmetry/src/lib.rs`.
- Ports, actuators and materials: `fs-ports/src/component.rs` (line 52); `fs-phs/src/interconnection.rs`,
  `src/circuit.rs`, `src/circuit/battery.rs`; `fs-control/Cargo.toml`, `src/synchronous.rs`, `src/thermal.rs`;
  `fs-em/src/winding.rs`; `fs-matdb/src/lib.rs`; `fs-qty/src/lib.rs` (lines 20–21), `src/parse.rs`; `fs-ir/src/lib.rs`.
- Statistics, search and evidence: `fs-eproc` (`CONTRACT.md`, `src/bernoulli.rs`, `src/hardening.rs`,
  `tests/conformal_shift_receipt.rs`); `fs-race` (`CONTRACT.md`, `tests/battery.rs`); `fs-voi` (`CONTRACT.md`);
  `fs-uq/src/mlmc.rs`, `src/product_importance.rs`, `src/product_importance/proposal.rs`, `src/product_shapley.rs`,
  `product_mlmc_allocation`, `product_control_variate`, `product_copula`; `fs-archive`; `fs-bo`, `fs-dfo`, `fs-ascent`;
  `fs-recompute` (`CONTRACT.md`); `fs-package`, `fs-checker`; `fs-sos/src/lib.rs`, `src/roa.rs`; `fs-ivl`; `fs-regime`;
  `fs-adjoint`; `fs-ad/src/ift.rs`, `gradcheck.rs`; `fs-surrogate`.
- Structure, reality and multiphysics: `fs-assimilate`, `fs-asbuilt`, `fs-topopt`, `fs-topols`, `fs-toleralloc`,
  `fs-geocon`, `fs-fab`, `fs-io`, `fs-mesh`, `fs-solid/src/rod.rs`, `fs-material`, `fs-lbm/src/lib.rs`, `fs-couple`.
- Build and execution: `fs-la/build.rs`; `fs-exec/Cargo.toml`; `fs-tilelang/src/lib.rs`; `fs-conform` (a
  converter-conformance kit, not conformal prediction).
- Not linked, read as reports of defects: `fs-g1-train/src/ppo.rs` (lines 253–345), `standin_env.rs`;
  `fs-cmaes-viz-wasm/src/g1_walking.rs`.
- Applications: `apps/instrument-lab/render-gpu/README.md`;
  `apps/instrument-lab/render-gpu-piano/src/{kernel.h,cuda.rs,metal.rs,emulate.rs}`.

**frankentorch.** `crates/ft-kernel-metal/src/compute.rs`, `src/lib.rs` and `Cargo.toml`; `crates/ft-core/src/lib.rs`
(lines 397 and 2460); `ft-api`, `ft-dispatch`, `ft-nn`, `ft-optim`, `ft-serialize`, `ft-autograd`;
`docs/planning/FEATURE_PARITY.md`.

**franken_numpy.** `crates/fnp-ndarray/src/lib.rs` (lines 258 and 390); `fnp-io`, `fnp-linalg`; README and manifests.

**frankensympy.** `README.md`, `COMPREHENSIVE_PLAN_FOR_FRANKENSYMPY.md`, `Cargo.toml` (lines 95–96),
`rust-toolchain.toml`; `crates/fsym-calculus/src/sparse_jacobian.rs`, `compile.rs`; `crates/fsym-formal/src/checker.rs`,
`envelope.rs`; `crates/fsym-solvers/src/real_rur.rs`; `crates/fsym-polys/src/groebner.rs`.

**franken_lean.** `README.md`, `IMPLEMENTATION_STATUS.md`, `CERTIFICATE_FORMAT.md`, `CARTRIDGE_FORMAT.md`,
`SUITE.lock`, the S26 conformance receipt (per-module results); `crates/fln-checker/src/admit.rs`, `policy.rs`;
`crates/fln-kernel/src/lib.rs`; `crates/fln-verdict`; `crates/fln-anvil`.

**fastmcp_rust.** `Cargo.toml`, `rust-toolchain.toml`, `LICENSE`, `README.md`, `UPGRADE_LOG.md`;
`crates/fastmcp/src/lib.rs`; `crates/fastmcp-macros/src/lib.rs`; `crates/fastmcp-server/src/handler.rs`,
`builder.rs`, `router.rs`, `lib.rs`; `crates/fastmcp-protocol/src/types.rs`, `messages.rs`;
`crates/fastmcp-core/src/context.rs`; `crates/fastmcp-transport/src/lib.rs`.

**mcp_agent_mail_rust.** `Cargo.toml`, `Cargo.lock`; `crates/mcp-agent-mail-server/src/lib.rs`;
`crates/mcp-agent-mail-tools/src/lib.rs`, `resources.rs`, `idempotency.rs`, `messaging.rs`.

**asupersync.** README and manifests; `src/lab/`, `src/lab/runtime.rs`, `src/lab/conformal.rs`;
`franken_kernel/src/lib.rs`.

**Other siblings.** `frankensqlite/crates/fsqlite-pager/src/s3_fifo.rs`; `frankenscipy/crates/fsci-opt/src/linprog.rs`
and its README; `frankentui/README.md`, `crates/ftui-runtime/Cargo.toml` (lines 56 and 81);
`franken_threed/COMPREHENSIVE_PLAN_FOR_THE_DESIGN_OF_FRANKENTHREED.md`, `README.md`, `DEPENDENCY_ADMISSION.md`,
`Cargo.toml`, `crates/f3d-runtime/Cargo.toml`; `frankenjax/crates/fj-egraph` and its README;
`franken_textiles/COMPREHENSIVE_PLAN_FOR_THE_DESIGN_OF_FRANKENTEXTILES.md`, `AGENTS.md`, `README.md`,
`crates/ftx-drape`; `franken_mining/COMPREHENSIVE_PLAN_FOR_THE_DESIGN_OF_FRANKENMINING.md`, `AGENTS.md`,
`franken-stack.lock`; `franken_firmware` (toolchain file); `franken_drone_geometry_reconstruction` (status file and
schemas); `franken_surveillance_system/BLENDER_TWIN_INTEGRATION_PLAN.md`.

**Workspace rules.** `/Users/jemanuel/AGENTS.md`, `/Users/jemanuel/projects/AGENTS.md`.

**This machine.** `sysctl` and `system_profiler` report an Apple M5 Ultra, 36 CPU cores, an 80-core GPU with Metal 4
and 256 GB of memory. A Metal device query reports a working-set limit of about 222.7 GiB and a largest single buffer
of about 167.0 GiB.

**Planning records.** The verdict register of 24 hypotheses and its 23 evidence packs; two independent audits of 76
claims (36 in the first, 40 of 58 in the second, 18 left pending); a harvest of 18 primitive matches; a compilation
of eight decision families; and the planning reproduction archive of H.5. Their locations are in the planning review log.

## Acceptance criteria

- [ ] Every backticked path, crate name and module name above (resolved by rules 1 and 2) and each of the seven
      documents of rule 7 has exactly one `[[source]]` record of kind `local`; the
      thirteen groups (frankensim plans and repository files; frankensim crates; frankentorch; franken_numpy;
      frankensympy; franken_lean; fastmcp_rust; mcp_agent_mail_rust; asupersync; other siblings; workspace
      rules; this machine; planning records) are the `topic` values.
- [ ] The five recorded commits are on the records of their repositories.
- [ ] The three `defect-report` paths carry that role, and a test fails if any of them appears among the
      dependencies of a workspace manifest.
- [ ] `SRC-MACH-01` holds the six machine figures with units; the two Metal limits are marked "about".
- [ ] The five planning records exist with counts 24, 23, 76 (36 + 40), 18 pending, 18 matches, 8 families.
- [ ] Every local record reports `reread_before_gate = true` until a re-read at the stack-lock commit exists.

## Test plan

### Unit tests

`crates/frob-conformance/tests/sources_content.rs`, module `local`:

- `every_listed_path_has_a_record`: the frozen list of (`locator`, `covers`) pairs, derived from this bead's
  text by rules 1, 2 and 7, equals the set of those pairs over the records of kind `local`; spot checks:
  `lie_differential.rs` resolves to `frankensim:crates/fs-ga/src/lie_differential.rs`, `certified_ccd` to
  `frankensim:crates/fs-contact` with covers "module certified_ccd, as read", and the seven rule-7 phrases are
  present.
- `commits_recorded_for_five_repos`: the five (repository, commit) pairs above.
- `line_ranges_kept`: `fs-mbd/src/planar_drive.rs` has lines 117–133; `fs-contact/src/sustained/solve.rs` line
  26; `fs-ports/src/component.rs` line 52; `fs-qty/src/lib.rs` lines 20–21; `fs-g1-train/src/ppo.rs` lines
  253–345; `ft-core/src/lib.rs` lines 397 and 2460; `fnp-ndarray/src/lib.rs` lines 258 and 390; frankensympy
  `Cargo.toml` lines 95–96; `ftui-runtime/Cargo.toml` lines 56 and 81.
- `defect_reports_are_not_dependencies`: parses every `Cargo.toml` of the workspace and annexes; none names
  `fs-g1-train` or `fs-cmaes-viz-wasm`.
- `fs_conform_note_present`.
- `machine_record_values`: 36 cores, 80 GPU cores, 256 GB, 222.7 GiB, 167.0 GiB.
- `audit_counts_add_up`: 36 + 40 = 76 and 58 − 40 = 18.
- `paths_exist_when_siblings_present`: for each record whose repository is checked out under `local_root`, the
  path exists at the stack-lock commit (`git cat-file -e <git_rev>:<path>`, `git_rev` of the repository's
  `[[foundation]]` row); a repository with no pinned `git_rev` (a row whose pin enters only at its admission, a
  pattern row such as frankenjax's, or a repository with no row, such as mcp_agent_mail_rust and
  franken_mining) is checked at its `HEAD`, and the two `~/` workspace rule files on disk. A missing path is a finding `src.local-path-gone` for the fact-sheet
  review, reported and not failed in this test (the sibling may have moved the file).
- `empty_repo_only_for_workspace_files`: the two workspace rule records load with `repo = ""`; a copy of a
  frankensim record with its `repo` emptied fails `src.local-path-missing-repo`.

### End-to-end script and logging

`scripts/e2e/sources.sh` (NAB-16) prints the per-repository counts and runs the path-existence check against the
checked-out siblings. Logged per STD-E2E: repository, pinned commit, records, paths found and missing, as
`metric` lines.

### Known answers and negative controls

Known answers: thirteen topic groups; five recorded commits; three defect-report paths; the machine figures
above; the audit arithmetic 36 + 40 = 76 and 58 − 40 = 18. Negative controls: a `local` record whose `repo`
is empty and whose locator is not of the `~/` form (`src.local-path-missing-repo`); a defect-report crate added
to a manifest (the dependency test must fail). A local reading cited by a gate before its re-read fails NAB-10's
`cite.local-not-at-pin`; that control needs the citation lint, which comes after this bead, and is a plant of
NAB-16's script.

## Considerations

- The siblings are the owner's projects and change daily. A path that has moved is not an error of the register;
  it is a reading task for the fact sheet, and the statement that cited it is rechecked.
- A limitation or defect found in a sibling is fixed in that sibling's repository as an upstream work item,
  never by a local patch; this register only records what was read.
- The planning records include material written by several independent passes; the register names them by what
  they are (a verdict register, two audits, a harvest, a compilation, an archive) and by path.

## References

Appendix H.2; §0.1 rule 2 (a reported defect in a foundation is a FACT to recheck); §1.7; §33 (admission gates
and the dependency firewall).
