# Validation

## Proof Strategy

Prove the taxonomy as a stable contract before any CLI or SQLite implementation
uses it. The verifier must accept representative valid friction events and
reject unknown types, provider-unavailable events without provider identity, and
high-severity events without evidence.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | JSON Schema validates required fields, enums, conditionals, and fixtures. |
| Integration | Decision and taxonomy docs align with schema event types. |
| E2E | Story gate verifies with schema checker, context pack, architecture check, and Detailed trace. |
| Platform | Python verifier runs on the local workspace without provider calls. |
| Performance | Not applicable. |
| Logs/Audit | Trace records taxonomy decisions and no automatic policy mutation. |

## Fixtures

- Valid high-severity `provider_unavailable` event with provider and evidence.
- Valid medium `repeated_manual_step` event without evidence.
- Invalid provider-unavailable event missing provider.
- Invalid high-severity event missing evidence.
- Invalid event with unknown type.

## Commands

```text
python scripts/verify-friction-taxonomy.py
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
python scripts/verify-mcp-artifact-contracts.py
harness-cli context --story US-029
harness-cli arch-check --story US-029
harness-cli story verify US-029
```

## Acceptance Criteria

- Friction taxonomy document exists.
- Friction event schema exists and validates as Draft 2020-12.
- Decision 0011 is accepted and recorded durably.
- Valid taxonomy fixtures pass.
- Invalid taxonomy fixtures fail.
- No SQLite migration is added.
- No CLI friction capture command is added.
- No automatic policy mutation is introduced.
- Context pack is generated.
- Trace is Detailed `3/3`.
- Story governance gate passes.

## Acceptance Evidence

- GitHub milestone created: `HI-OS v0.5.0: Harness Learning Loop`.
- GitHub issue created: `#7`.
- Intake recorded: `#18`.
- Story recorded: `US-029`, lane `high-risk`.
- Decision recorded and verified: `0011-harness-friction-taxonomy`.
- `docs/FRICTION_TAXONOMY.md` defines 8 friction event types:
  `missing_context`, `ambiguous_policy`, `weak_validation`,
  `provider_unavailable`, `schema_gap`, `release_gap`,
  `architecture_rule_gap`, and `repeated_manual_step`.
- `docs/schemas/friction-event.schema.json` validates as Draft 2020-12.
- `scripts/verify-friction-taxonomy.py` pass:
  - validates the schema;
  - checks taxonomy and Decision 0011 contain the canonical event types;
  - accepts representative valid events;
  - rejects provider-unavailable without provider;
  - rejects high severity without evidence;
  - rejects unknown event types.
- `python scripts/verify-mcp-artifact-contracts.py` pass.
- `cargo fmt --check` pass.
- `cargo test --workspace` pass: 42 tests.
- `cargo clippy --workspace --all-targets -- -D warnings` pass.
- `harness-cli context --story US-029` pass.
- `harness-cli arch-check --story US-029` pass.
- `harness-cli decision verify 0011-harness-friction-taxonomy` pass.
- Detailed trace recorded: `#26`, score `3/3`.
- `harness-cli story verify US-029` pass.
- Story governance gate pass.
- No SQLite migration was added.
- No CLI friction capture command was added.
- No backlog/rule suggestion behavior was added.
- No automatic policy mutation was introduced.
- Follow-up backlog recorded: `#3` for v0.5 release-hardening review of
  installer payload coverage.
