# Decision 0013: HI-OS sovereign identity

## Status

Accepted

## Context

HI-OS began as repository-harness, but the project now has its own release
trust chain, governance dashboard, MCP artifact contracts, adoption docs, and
public distribution origin. Keeping identity split across README prose,
installer defaults, release config, and report output makes it harder for a new
user or agent to tell what the system is and which public origin is canonical.

US-046 establishes a single tracked identity file for the product:

```text
hios.toml
```

That file defines the product name, short name, slug, repository, and default
public release origin.

## Decision

HI-OS uses `hios.toml` as the tracked sovereign identity contract.

The Harness CLI reads that file for identity display and governance evidence.
Governance reports include the HI-OS identity, and dashboards render it for
human audit. Release verification continues to use `harness-release.toml` for
release policy, but the default release origin must align with the HI-OS
identity origin.

The canonical public origin is:

```text
ntu254/Harness-Intelligence-OS
```

## Guardrails

- HI-OS identity is tracked in source control, not inferred only from Git
  remotes.
- `hios.toml` does not contain credentials, sessions, tokens, or runtime
  evidence.
- `harness-release.toml` remains the release policy file.
- `release verify` fails fast if tracked HI-OS identity and release policy
  disagree on the default public origin.
- Governance reports and dashboards display identity but do not mutate it.
- Legacy repository-harness cleanup remains US-047, not US-046.

## Consequences

- A clean clone has an explicit product identity before any runtime database is
  initialized.
- Agents can query the identity through `harness-cli identity`.
- Governance report schema advances to include `identity`.
- Future installer and release hardening can validate payload identity drift.
