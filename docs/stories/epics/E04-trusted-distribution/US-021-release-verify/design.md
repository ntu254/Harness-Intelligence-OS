# Design

## Domain Model

Proposed release verification concepts:

- `ReleaseVersion`: normalized semantic version such as `0.2.0`.
- `ReleaseTag`: the accepted tag shape, currently
  `harness-cli-v<version>`.
- `ReleaseOrigin`: canonical owner/repository from decision `0008`.
- `ExpectedAsset`: platform binary plus matching `.sha256` file.
- `ReleaseCheck`: named check with pass/fail status and evidence.
- `ReleaseVerificationReport`: complete result for one version and origin.

The expected v0.2 asset contract is five native binaries and five SHA256 files.

## Application Flow

1. Parse and validate `--version`.
2. Resolve the canonical public origin and tag.
3. Query public release metadata without credentials.
4. Compare actual assets with the expected platform matrix.
5. Select the current-platform binary and checksum.
6. Download both into a temporary directory.
7. Parse the checksum file and hash the downloaded binary.
8. Run the binary with `--version`.
9. Run a non-mutating smoke command such as `arch-check --help`.
10. Write JSON evidence atomically.
11. Return success only when every required check passes.

## Interface Contract

Target command:

```text
harness-cli release verify --version <version>
```

Proposed options that require confirmation during implementation:

```text
--origin <owner/repo>   Override only for tests or explicit audit.
--out <path>            Override the default evidence path.
```

Default evidence path:

```text
.harness/release/harness-cli-v<version>-release-verify.json
```

Proposed report shape:

```json
{
  "version": "0.2.0",
  "origin": "ntu254/Harness-Intelligence-OS",
  "tag": "harness-cli-v0.2.0",
  "assets_checked": 10,
  "download": "pass",
  "checksum": "pass",
  "version_check": "pass",
  "smoke_install": "pass",
  "result": "pass"
}
```

The final schema should also include timestamps, the current platform, asset
names, expected and actual hashes, command output summaries, and failure
details.

## Data Model

The report is operational evidence under `.harness/release/` and is not tracked
in Git. Before implementation, decide whether story governance should:

1. Store a report path and result on the story.
2. Query the latest report directly.
3. Add a dedicated durable release verification table.

Do not add schema columns until this ownership decision is explicit.

## UI / Platform Impact

- Terminal output presents each check and the final result.
- Current-platform binary execution is required.
- Other platform assets are verified for presence and checksum metadata, not
  executed locally.
- Public downloads must work without GitHub authentication.

## Observability

- JSON evidence report under `.harness/release/`.
- Detailed Harness trace linked to `US-021`.
- No separate tracked Markdown trace file; SQLite remains trace source of
  truth.
- Failures identify the exact tag, asset, checksum, version, or smoke check.

## Alternatives Considered

1. Keep manual release verification. Rejected because it is not repeatable or
   mechanically enforceable.
2. Implement checks only in GitHub Actions. Rejected because local agents and
   installed Harness repos also need auditable verification.
3. Execute every platform binary from one host. Rejected because native
   execution requires platform-specific runners.
4. Hardcode private or local paths. Rejected because the canonical origin and
   public installer contract must remain portable.
