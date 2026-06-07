# Validation

## Proof Strategy

US-042 is docs-only, but agent instructions are operational. Validate that the
three packs exist, are linked, and keep the required HI-OS guardrails.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Adoption verifier checks all three agent packs and links. |
| Integration | README, AGENTS, and docs map point to the packs. |
| E2E | Packs describe read context -> validate -> trace -> story verify. |
| Platform | Packs mention Windows command form where relevant. |
| Logs/Audit | Trace records scope, validation, and non-goals. |

## Fixtures

- `docs/agents/cursor.md`
- `docs/agents/claude-code.md`
- `docs/agents/codex.md`

## Commands

```text
cargo fmt --check
cargo test --workspace
python scripts/verify-adoption-docs.py
harness-cli context --story US-042
harness-cli arch-check --story US-042
harness-cli story verify US-042
```

## Acceptance Criteria

- `docs/agents/cursor.md` exists.
- `docs/agents/claude-code.md` exists.
- `docs/agents/codex.md` exists.
- Each pack requires reading context before implementation.
- Each pack says not to code before lane/story/validation path is known.
- Each pack requires context pack use for story work.
- Each pack says not to claim pass until verification has run.
- Each pack mentions `story verify`.
- Each pack preserves provider `inconclusive`, not `pass`.
- Each pack forbids storing Google credentials or provider session files.
- `AGENTS.md` links the packs.
- `README.md` links the packs.
- `docs/README.md` maps `docs/agents/`.
- `python scripts/verify-adoption-docs.py` passes.
- Story governance gate passes.

## Acceptance Evidence

- GitHub issue created: `#20`.
- Intake recorded: `#31`.
- Story recorded: `US-042`, lane `normal`, verify command
  `python scripts/verify-adoption-docs.py`.
- Agent packs added:
  - `docs/agents/codex.md`;
  - `docs/agents/claude-code.md`;
  - `docs/agents/cursor.md`.
- `AGENTS.md` links the three packs.
- `README.md` links the three packs.
- `docs/README.md` maps `docs/agents/`.
- Each pack requires context reading before implementation.
- Each pack requires context pack use for story work.
- Each pack says not to code before lane, story, context, and validation path
  are understood.
- Each pack says not to claim completion before verification.
- Each pack mentions `story verify`.
- Each pack preserves provider `inconclusive`, not `pass`.
- Each pack forbids storing Google credentials, cookies, tokens, browser
  profiles, or provider session files in Harness.
- Adoption docs verifier extended for agent pack contract:
  `scripts/verify-adoption-docs.py`.
- `cargo fmt --check` passed.
- `cargo test --workspace` passed: 49 tests.
- `python scripts/verify-adoption-docs.py` passed.
- `git diff --check` passed.
- `harness-cli context --story US-042` passed.
- `harness-cli arch-check --story US-042` passed.
- Detailed trace recorded: `#41`, score `3/3`.
- `harness-cli story verify US-042` passed.
- Story governance gate passed.
- No installer, release, tag, or CLI behavior change.
