# US-045 Validation

## Acceptance Criteria

- All v0.7 content stories are implemented and gated.
- CLI version and installer pin are 0.7.0.
- Release notes exist.
- Local tests and all tracked verifiers pass.
- The production payload rebuilds deterministically.
- Previous public v0.6.0 verification still passes.
- GitHub workflow publishes twelve v0.7.0 assets.
- Public v0.7.0 payload and host binary checks pass.
- Public installer smoke passes in a clean target.
- Governance report and dashboard remain trusted.
- Detailed trace and story governance gate pass.

## Evidence

### Pre-Publication

- GitHub issue #26 and intake #37 recorded.
- US-039 through US-044 and US-046 through US-048 are implemented with passing
  story gates.
- CLI version: `0.7.0`.
- Installer pin: `harness-cli-v0.7.0`.
- `RELEASE_NOTES_v0.7.0.md` added.
- Release workflow requires five native builds and a separate production
  payload job, then uploads twelve assets.
- `cargo fmt --check`: pass.
- `cargo test --workspace`: 50 passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- Adoption, governance report, MCP contract, and friction taxonomy verifiers:
  pass.
- Bash and PowerShell syntax/readiness checks: pass.
- Release build reports `harness-cli 0.7.0`.
- Production payload contains 73 operating files.
- Python, PowerShell, and Bash payload builds produced identical SHA256:
  `ca26f3535be41a3aea1664e68439ae0425c3b83cceb30b85c594cb9bcdf7814e`.
- Source-matched production payload verification: pass.
- Clean pre-publication payload install copied 70 files and installed
  `harness-cli 0.7.0`.
- Installed adoption, MCP, friction, identity, SQLite, governance report
  schema, and dashboard checks: pass.
- The v0.7 CLI verified immutable public v0.6.0 using its ten-asset legacy
  contract.
- US-045 context pack and architecture check: pass.
- Pre-publication dashboard: trusted maturity.

### Public Release

- Release prep commit: `552953b`.
- Annotated tag: `harness-cli-v0.7.0`.
- GitHub Actions workflow:
  `https://github.com/ntu254/Harness-Intelligence-OS/actions/runs/27081479651`.
- Workflow jobs passed:
  - Verify;
  - Build Production Payload;
  - macOS arm64;
  - macOS x64;
  - Linux arm64;
  - Linux x64;
  - Windows x64;
  - Publish GitHub Release.
- Public release:
  `https://github.com/ntu254/Harness-Intelligence-OS/releases/tag/harness-cli-v0.7.0`.
- Release is public, non-draft, and non-prerelease.
- Public assets: 12.
- Public production payload SHA256:
  `91599e1c177744c3ee7c045ef81729c329e16f833a3f4ea7e4dd5abb578d9c5a`.
- `release verify --version 0.7.0 --story US-045` passed:
  - all twelve expected assets discovered;
  - host binary download and SHA256 passed;
  - production payload download and SHA256 passed;
  - binary version passed;
  - smoke command passed.
- Public raw installer from `main` installed and verified `harness-cli 0.7.0`
  in an empty target.
- Public production ZIP was downloaded, externally and internally verified,
  extracted, and used to install `harness-cli 0.7.0` in another empty target.
- Both public install paths passed identity, SQLite initialization, adoption,
  MCP, friction, governance report schema, and dashboard checks.
- Trace #48: Detailed 3/3.
- US-045 mechanical verification: pass.
- US-045 governance gate: pass.
- Final governance report/dashboard:
  - 30 stories;
  - 29 gates pass;
  - 0 gates fail;
  - trusted maturity score 93.
- Older public release assets remain unchanged.
