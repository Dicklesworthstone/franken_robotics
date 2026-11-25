# ADR-PERF-01 to ADR-PERF-05: Performance Program Target & Measurement Decisions

- **IDs:** `ADR-PERF-01`, `ADR-PERF-02`, `ADR-PERF-03`, `ADR-PERF-04`, `ADR-PERF-05`
- **Status:** `accepted`
- **Governing bead:** `N36-01`
- **First milestone:** `M0`
- **Plan references:** §36.1, §36.2, §36.3, §36.5, §36.6, §36.15, §36.16
- **Data register:** `registries/perf_targets.toml` (`frob.perf.targets/1`), validated by `schemas/registries/perf_targets.schema.json` and `frob_conformance::perf::targets`

Engine throughput is a parity requirement, not the source of the ten-fold design-loop claim. The five decisions below fix how every target in `registries/perf_targets.toml` is stated, compared, and judged so measurements cannot flatter the candidate.

---

## ADR-PERF-01: Engine Targets Are Ratios to Incumbents Measured on Our Own Hardware

### Decision

Engine throughput and training-time targets are stated as binding ratios to the strongest pinned incumbent measured on our own hardware (`trj` and the Mac); published incumbent figures provide planning values only until the parity baseline (`GATE-SPARK-21`) replaces them with measured figures and disclosure block identities.

### Reason

Published incumbent figures exist for the reference tasks on different GPUs (RTX 5090, RTX 6000 Ada, A100); an absolute target chosen without a measured comparator hides a parity gap when hardware or software shifts.

### Alternative excluded

Absolute targets chosen by us without a comparator on the same hardware. Such targets can sit comfortably below published or reproduced incumbent rates without anyone noticing that the engine is a net liability.

---

## ADR-PERF-02: Two Units (`world_steps_per_s` and `env_steps_per_s`), Each With Its Decimation

### Decision

Step-family throughput is always reported in typed units (`world_steps_per_s`, `env_steps_per_s`, or `world_s_per_wall_s` from `frob_core::perf::units::RateUnit`), and every step-family row carries its explicit rational decimation `d` (`frob_core::perf::units::Decimation`). A step-rate row without a decimation is rejected at load time.

### Reason

An `env_steps_per_s` figure means nothing without the number of physics steps inside each control tick: incumbents do not share a decimation (`d = 4` in mjlab, `d = 5` in MuJoCo Playground, `d = 10` in our reference workloads Q and H).

### Alternative excluded

A single unlabeled unit `"steps/s"` or omitting decimation from target rows and reports, which permits unit-ordering contradictions (`INV-PERF-01`, `CX-374`).

---

## ADR-PERF-03: Every Target Number Declares Whether It Is a Tolerance or an Ambition

### Decision

Every numeric threshold in `registries/perf_targets.toml` carries an explicit status (`tolerance`, `interim-floor`, `ambition`, `planning-value`, or `reported-only`). The `threshold` helper returns the status beside the number so an interim floor (such as row 1's 2.0 M world-steps/s) can never be recorded or cited as a binding tolerance.

### Reason

A reader and every automated gate must know which numbers the plan's argument actually depends on (tolerances, whose breach triggers `KILL-PARITY`, a defect, or a gate failure) versus stretch goals (ambitions) and temporary pre-measurement floors.

### Alternative excluded

One unlabeled number per target row with no status field, which conflates planning anchors, interim floors, binding parity bands, and stretch ambitions.

---

## ADR-PERF-04: Ratios Are Admitted on a Paired Lower Bound, Never a Point Estimate

### Decision

Every ratio row (`ratio_to_incumbent` or `ratio_to_row`) is judged solely on a one-sided 95% paired lower confidence bound (`Figure::LowerBound`) oriented so larger is better (for `at_most` time rows of ratio `v`, the threshold on the oriented bound is `1 / v`: 0.80 for row 4, 0.667 for row 9). Offering a `Figure::Point` to `judge` on a ratio row returns the typed error `JudgeError::PointEstimateOnRatioRow`, and offering a bound on the wrong side returns `JudgeError::WrongSide`.

### Reason

A mean or point estimate of noisy runs on a loaded host (`trj` or a thermally throttling workstation) flatters whichever arm ran last and fails to control false promotions.

### Alternative excluded

Judging ratio rows by sample means, medians, or best-of-`n` point estimates.

---

## ADR-PERF-05: Ceiling First — No Speed Experiment Without Its Computed Ceiling

### Decision

Before any optimization or speed experiment is funded, its theoretical speedup ceiling is computed from the current end-to-end profile using the Amdahl / family-batch bound (`1 / (1 - f + f/r + h)`, §36.5), and any experiment whose ceiling lies below its goal is refused.

### Reason

A recorded planning pilot ran a specialized-solver speed test whose 575.7 us untouched residual capped a zero-cost solver at ~1.46x, well below its goal; that ceiling was computable beforehand from the profile.

### Alternative excluded

"Measure first, explain later" optimization passes that spend engineering and compute time on kernels whose share of wall-clock cannot reach the target.
