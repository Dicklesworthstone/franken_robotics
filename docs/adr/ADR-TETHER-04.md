# ADR-TETHER-04: Bounded Synchronous Wait with Job-Handle Fallback

- **ID:** `ADR-TETHER-04`
- **Status:** `accepted`
- **Governing bead:** `N32a-16`
- **First milestone:** `M2`
- **Plan references:** §32.3, §32.4, §32.6, §36

## Decision

Every `run` verb accepts a bounded synchronous wait parameter `sync_wait_ms` (`0 <= sync_wait_ms <= wait.mcp_max_ms <= 30_000`, `RL-09`). If the job finishes inside the wait window, the verb returns the completed result envelope directly (exit 0 or 2). If the wait elapses while the job is still running, the call returns cleanly with exit code 8 and a job handle (`job_id`, progress with denominator, `poll_after_ms`) without cancelling the job. Per-surface defaults are declared explicitly in `registries/verbs.toml` (`wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`).

## Reason

Most studio verbs finish within a few seconds (`TARGET`: `design.check` under 2 s, `control.synth` under 5 s), whereas an AI agent reasoning turn costs roughly 30 to 120 s of thinking (`HYPOTHESIS`, measured by `GATE-TETHER-13`). Returning fast completions in one round trip eliminates unnecessary polling turns while capping tool-call latency for long runs.

## Alternative excluded

Always blocking unconditionally until completion (which stalls MCP clients and triggers transport timeouts on multi-minute jobs), or always returning a job handle immediately (which forces a follow-up `job.wait` turn even for 2-second studio checks).

## Enforced by

- `GATE-TETHER-12` (pending at M2, `N32a-16`): verifies every deferred `run` call returns its exit-8 job handle within `sync_wait_ms + 500 ms` while the job continues cleanly in `frobd`.

## Revisit when

- `GATE-TETHER-13` measurements invalidate the 30–120 s agent-turn hypothesis, or `OPEN-TETHER-3` (`N32a-33`) revises the 30,000 ms fail-closed maximum together with the MCP server request timeout.
