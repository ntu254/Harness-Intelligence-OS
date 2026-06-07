# Validation

## Proof Strategy

Validate the cookbook with the adoption verifier, then run Harness story proof.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Adoption verifier checks command groups and key commands. |
| Integration | README and docs map link the cookbook. |
| E2E | Cookbook covers intake, context, verify, trace, release, dashboard, and MCP. |
| Platform | Windows path note is present. |
| Logs/Audit | Trace records scope, validation, and non-goals. |

## Fixtures

- `docs/COMMAND_COOKBOOK.md`.

## Commands

```text
cargo fmt --check
cargo test --workspace
python scripts/verify-adoption-docs.py
harness-cli context --story US-044
harness-cli arch-check --story US-044
harness-cli story verify US-044
```

## Acceptance Criteria

- `docs/COMMAND_COOKBOOK.md` exists.
- Cookbook has intake commands.
- Cookbook has context commands.
- Cookbook has verify commands.
- Cookbook has trace commands.
- Cookbook has release commands.
- Cookbook has dashboard commands.
- Cookbook has MCP/provider commands.
- Cookbook includes short examples for command groups.
- Cookbook preserves provider `inconclusive`, not `pass`.
- README links to the cookbook.
- `docs/README.md` links to the cookbook.
- `python scripts/verify-adoption-docs.py` passes.
- Story governance gate passes.

## Acceptance Evidence

- GitHub issue created: `#22`.
- Intake recorded: `#33`.
- Story recorded: `US-044`, lane `normal`, verify command
  `python scripts/verify-adoption-docs.py`.
- Command cookbook added: `docs/COMMAND_COOKBOOK.md`.
- README links to `docs/COMMAND_COOKBOOK.md`.
- `docs/README.md` links to `docs/COMMAND_COOKBOOK.md`.
- Cookbook groups commands for setup/state, intake, stories, context, verify,
  trace, release, dashboard, MCP/provider evidence, and friction.
- Cookbook includes short examples for each command group.
- Cookbook includes Windows CLI path note:
  `.\scripts\bin\harness-cli.exe`.
- Cookbook preserves provider `inconclusive`, not `pass`.
- Cookbook preserves Google credential/provider session guardrails.
- Adoption docs verifier extended for cookbook contract:
  `scripts/verify-adoption-docs.py`.
- `cargo fmt --check` passed.
- `cargo test --workspace` passed: 49 tests.
- `python scripts/verify-adoption-docs.py` passed.
- `git diff --check` passed.
- `harness-cli context --story US-044` passed.
- `harness-cli arch-check --story US-044` passed.
- Detailed trace recorded: `#43`, score `3/3`.
- `harness-cli story verify US-044` passed.
- Story governance gate passed.
- No provider live call, release, tag, installer pin, or CLI behavior change.
