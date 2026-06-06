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
- `ReleaseVerificationSummary`: durable SQLite projection used by queries and
  story governance.

The expected v0.2 asset contract is five native binaries and five SHA256 files.

## Trust Boundaries

- Decision `0008` is authoritative for the default public origin.
- GitHub release metadata and downloaded assets are untrusted external input.
- SHA256 files are release assertions; the verifier must parse them and compare
  them with a locally computed digest.
- Downloaded binaries are untrusted until checksum validation passes.
- Binary output is untrusted process output and must be parsed exactly.
- Filesystem output paths must remain inside the repository or an explicitly
  supplied output directory.
- Network and GitHub availability failures produce `inconclusive`, never
  `pass`.

## Application Flow

1. Parse and validate `--version`.
2. Resolve the canonical public origin from `harness-release.toml` and build
   the tag.
3. Query public release metadata without credentials.
4. Compare actual assets with the expected platform matrix.
5. Select the current-platform binary and checksum.
6. Download both into a temporary directory.
7. Parse the checksum file and hash the downloaded binary.
8. Run the binary with `--version`.
9. Run a non-mutating smoke command such as `arch-check --help`.
10. Write JSON evidence atomically.
11. Persist a durable SQLite summary only after the report write succeeds.
12. Return success only when every required check passes.

## Interface Contract

Target command:

```text
harness-cli release verify --version <version>
```

Accepted options:

```text
--origin <owner/repo>  Override only for tests or an explicit audit.
--platform <platform>  Select the asset contract; defaults to the host platform.
--output <path>        Override the default evidence path.
--story <id>           Link the durable summary to a story.
```

The default origin is read from tracked `harness-release.toml`, whose initial
value must match decision `0008`. It must not require user input. An override is
recorded in evidence and cannot silently replace the canonical origin for story
governance.

Proposed tracked config:

```toml
origin = "ntu254/Harness-Intelligence-OS"
tag_prefix = "harness-cli-v"
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

Decision `0009` accepts hybrid storage:

- Write the complete operational JSON report under `.harness/release/`.
- Store a summary row in a dedicated SQLite release verification table.
- Link the summary to a story when `--story <id>` is supplied.
- Let `story verify` query the durable summary and require `pass` for stories
  whose durable `release_proof_required` flag is set.

The summary should include version, origin, tag, platform, result, report path,
checked timestamp, and linked story. The JSON report remains the detailed audit
artifact; SQLite remains the stable query and governance surface.

The implementation migration should add:

- A dedicated release verification table.
- A `release_proof_required` story flag, defaulting to false.
- Story add/update CLI input for explicitly enabling the requirement.

The gate must never infer this requirement from title or path substrings.

Report and summary consistency rule:

1. Write the report to a temporary file.
2. Atomically rename it to the final path.
3. Commit the SQLite summary.
4. If SQLite persistence fails, return non-zero and report the evidence as not
   durably recorded.

## UI / Platform Impact

- Terminal output presents each check and the final result.
- Current-platform binary execution is required.
- Other platform assets are verified for presence and checksum metadata, not
  executed locally.
- Public downloads must work without GitHub authentication.
- `--platform` may validate a foreign asset contract, but binary execution is
  permitted only for the host platform.

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
5. Store only JSON. Rejected because story governance and queries need a stable
   durable summary.
6. Store only SQLite. Rejected because audits need a detailed portable report.
