# Validation

## Acceptance Criteria

- v0.4 release notes describe US-023 through US-027.
- CLI help/smoke covers `context ingest`, `codegraph impact`,
  `notebooklm brief`, and auto-intake evidence defaults.
- `cargo fmt --check` passes.
- `cargo test --workspace` passes.
- `cargo clippy --workspace --all-targets -- -D warnings` passes.
- `python scripts/verify-mcp-artifact-contracts.py` passes.
- Five platform binaries and five SHA256 assets are built.
- GitHub release `harness-cli-v0.4.0` is published.
- `harness-cli release verify --version 0.4.0 --story US-028` passes.
- CodeGraph evidence example is valid and ingested.
- NotebookLM provider pass evidence is valid, or provider unavailable is
  recorded as honest inconclusive without becoming pass.
- Context pack is generated.
- Trace is Detailed `3/3`.
- Story governance gate passes only after release proof exists.

## Commands

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
python scripts/verify-mcp-artifact-contracts.py
harness-cli context --story US-028
harness-cli release verify --version 0.4.0 --story US-028
harness-cli story verify US-028
```

## Evidence

- Version bump and installer pin:
  - `harness-cli` crate version: `0.4.0`.
  - Installer release tag pin: `harness-cli-v0.4.0`.
  - Release notes: `RELEASE_NOTES_v0.4.0.md`.
- Local validation:
  - `cargo fmt --check`: pass.
  - `cargo test --workspace`: pass, 42 tests.
  - `cargo clippy --workspace --all-targets -- -D warnings`: pass.
  - `python scripts/verify-mcp-artifact-contracts.py`: pass.
  - `cargo build --package harness-cli --release`: pass.
  - `target/release/harness-cli --version`: `harness-cli 0.4.0`.
- CLI smoke:
  - `context ingest --help`: pass.
  - `codegraph impact --help`: pass.
  - `notebooklm brief --help`: pass with required `--notebook`.
  - `intake --help`: pass with `--auto`.
- Local package smoke:
  - `scripts/build-harness-cli-release.sh --target x86_64-pc-windows-msvc`
    produced `dist/harness-cli-windows-x64.exe` and `.sha256`.
  - Packaged Windows CLI reports `harness-cli 0.4.0`.
- MCP adapter evidence:
  - CodeGraph `0.9.9` produced passing US-028 ingest evidence at
    `.harness/context/US-028-codegraph-ingest-result.json`.
  - NotebookLM `nlm` `0.7.1` provider smoke produced schema-valid
    inconclusive evidence at
    `.harness/context/US-028-notebooklm-ingest-result.json` because the default
    provider profile is missing.
  - Inconclusive NotebookLM evidence is not treated as pass.
- Public release:
  - Tag: `harness-cli-v0.4.0`.
  - GitHub Actions workflow run: `27072575966`.
  - Verify job passed.
  - Five platform build jobs passed: macOS arm64, macOS x64, Linux arm64,
    Linux x64, Windows x64.
  - Publish job passed.
  - GitHub release:
    `https://github.com/ntu254/Harness-Intelligence-OS/releases/tag/harness-cli-v0.4.0`.
  - Release contains ten assets: five binaries and five SHA256 files.
- Release verification:
  - `harness-cli release verify --version 0.4.0 --story US-028`: pass.
  - Release report:
    `.harness/release/harness-cli-v0.4.0-release-verify.json`.
  - Assets checked: 10.
  - Download, checksum, version, and smoke checks: pass.
- Governance:
  - Context pack generated:
    `.harness/context/US-028-context.md`.
  - Architecture check: pass.
  - `harness-cli story verify US-028`: mechanical verification pass and
    governance gate pass.

## Planning Evidence

- Intake: `#17`.
- Story: `US-028`, High-Risk.
- GitHub issue: `#6`.
- Release proof required: yes.
- Milestone: `HI-OS v0.4.0: Real MCP Adapter Layer`.
- Architecture check: pass.
- Context pack: `.harness/context/US-028-context.md`.
- Planning trace: `#23`, Detailed `3/3`.
