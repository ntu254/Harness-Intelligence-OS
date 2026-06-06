# HI-OS v0.4.0: Real MCP Adapter Layer

This release upgrades Harness Intelligence OS from trusted distribution
evidence to a file-based MCP adapter layer with schema validation, ingest
summaries, provider adapters, and auto-intake evidence defaults.

## Added

- Versioned MCP artifact contracts:
  - `codegraph-impact`
  - `notebooklm-brief`
  - `context-ingest-result`
- `harness-cli context ingest` for validating external intelligence artifacts
  before they enter Harness governance state.
- `harness-cli codegraph impact` for producing and ingesting CodeGraph impact
  artifacts through the file boundary.
- `harness-cli notebooklm brief` for producing NotebookLM grounded brief
  artifacts through the local `nlm query notebook --json` provider boundary.
- Auto intake evidence defaults that prefer the latest passing CodeGraph and
  NotebookLM ingest summaries.
- Governance requirements for explicit CodeGraph and NotebookLM context ingest
  proof.

## Verified

- 42 Rust tests passed.
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- MCP schema verifier passed.
- Release build passed.
- CodeGraph adapter evidence passed through context ingest.
- NotebookLM provider unavailable evidence remained inconclusive and did not
  satisfy governance.
- Trace #24 scored Detailed 3/3 for the NotebookLM provider-contract
  correction.

## Notes

- MCP providers do not write directly into Harness SQLite.
- Passing artifacts can seed context and governance; failed or inconclusive
  artifacts remain auditable but never count as proof.
- The local NotebookLM provider is installed, but the default `nlm` profile is
  missing in this environment. v0.4.0 preserves that as honest inconclusive
  evidence rather than converting it into pass.
- Runtime evidence examples:
  `.harness/context/US-026-notebooklm-brief.json`
  `.harness/context/US-026-notebooklm-ingest-result.json`

## Release Assets

The release workflow publishes native CLI binaries and SHA256 files for:

- macOS arm64
- macOS x64
- Linux arm64
- Linux x64
- Windows x64

## Next Milestone

HI-OS v0.5.0 should start the learning loop:

- friction taxonomy
- failure attribution
- harness improvement proposals
- scorecard or reporting primitives
