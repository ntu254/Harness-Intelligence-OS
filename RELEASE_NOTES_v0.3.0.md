# HI-OS v0.3.0: Trusted Distribution & Evidence Trail

This release upgrades Harness Intelligence OS from a blocking governance gate
to a verifiable public distribution chain with durable release evidence.

## Added

- `harness-cli release verify`.
- Release asset discovery for all five supported platforms and ten expected
  binary and SHA256 assets.
- SHA256 verification of the selected platform binary.
- Downloaded CLI version validation.
- Non-mutating smoke command execution.
- Detailed JSON evidence reports under `.harness/release/`.
- SQLite release verification summaries for queries and governance.
- Explicit `release_proof_required` integration with `story verify`.

## Verified

- 27 Rust tests passed.
- `cargo fmt` passed.
- `cargo clippy` passed.
- Architecture check passed.
- Story governance gate passed.
- Installer smoke schema 5 passed.
- Trace #10 scored Detailed 3/3.

## Notes

- The v0.2.0 tag and release assets remain unchanged.
- The public v0.2.0 release was used as immutable verification input while
  implementing US-021.
- Runtime evidence example:
  `.harness/release/harness-cli-v0.2.0-release-verify.json`.

## Release Assets

The release workflow publishes native CLI binaries and SHA256 files for:

- macOS arm64
- macOS x64
- Linux arm64
- Linux x64
- Windows x64

## Next Milestone

HI-OS v0.4.0 begins with MCP artifact contracts before real provider calls:

- US-022: Define MCP artifact contracts.
- US-023: Implement CodeGraph impact adapter.
- US-024: Implement NotebookLM grounded brief adapter.
- US-025: Add context ingest validation.
