# US-022 Upgrade GitHub Actions For Node.js 24 Readiness

## Status

in_progress

## Lane

normal

## Product Contract

The official Harness CLI release workflow runs with Node.js 24-compatible
GitHub Actions while preserving the existing five-platform release artifact
contract.

## Relevant Product Docs

- `.github/workflows/harness-cli-release.yml`
- `scripts/README.md`
- `docs/decisions/0005-prebuilt-rust-harness-cli.md`
- `docs/decisions/0008-canonical-public-release-origin.md`

## Acceptance Criteria

- [x] Official JavaScript actions use Node.js 24-compatible major versions.
- [x] No Node.js 20-only official action remains in the release workflow.
- [x] The workflow opts into Node.js 24 before the hosted-runner default switch.
- [ ] A dispatch run passes verification and all five platform builds.
- [ ] The run produces five binaries and five SHA256 workflow artifacts.
- [ ] A clean installer smoke still passes against public v0.3.0.
- [ ] `release verify --version 0.3.0` still passes.
- [ ] The story governance gate and required trace pass.

## Design Notes

- Commands: no Harness CLI command changes.
- Queries: inspect workflow run jobs, annotations, and artifacts.
- API: GitHub Actions remains the official build and packaging system.
- Tables: no schema changes.
- Domain rules: the release asset names and five-platform matrix stay unchanged.
- UI surfaces: GitHub Actions workflow output only.

The workflow uses:

- `actions/checkout@v6`
- `actions/upload-artifact@v7`
- `actions/download-artifact@v8`
- `FORCE_JAVASCRIPT_ACTIONS_TO_NODE24: true`

`dtolnay/rust-toolchain@stable` remains unchanged because it is a composite
action and does not declare a JavaScript runtime.

## Validation

| Layer | Expected proof |
| --- | --- |
| Unit | Repository-local workflow readiness verifier passes. |
| Integration | Workflow verification job passes under forced Node.js 24. |
| E2E | Public v0.3.0 clean installer and release verification still pass. |
| Platform | macOS arm64/x64, Linux arm64/x64, and Windows x64 builds pass. |
| Release | Dispatch artifacts contain five binaries and five SHA256 files; no new release is published. |

## Harness Delta

Backlog #2 becomes US-022. A mechanical verifier prevents future workflow edits
from silently restoring Node.js 20-only official action versions.

## Evidence

- GitHub action manifests declare Node.js 24 for checkout v6,
  upload-artifact v7, and download-artifact v8.
- Runtime workflow, artifact, installer, release verification, governance, and
  trace evidence will be added after the dispatch run completes.
