# Overview

## Current Behavior

Release publication, asset enumeration, checksum inspection, public download,
CLI version checks, and smoke installation are verified through separate manual
commands. The results exist in traces and release metadata but are not produced
by one repeatable Harness command.

## Target Behavior

An agent can run:

```powershell
.\scripts\bin\harness-cli.exe release verify --version 0.2.0
```

Harness verifies the canonical release chain:

```text
publish -> download -> checksum -> version -> smoke install -> evidence report
```

The command exits non-zero on any failed trust check and writes a structured
report for governance and audit.

External availability failures are reported as `inconclusive`, never `pass`.
Trust failures such as a missing asset, checksum mismatch, or wrong version are
reported as `fail`.

## Scope Boundary

This story verifies an already-published release. It does not create tags,
publish assets, mutate releases, change installer defaults, or redesign the
canonical origin.

## Affected Users

- AI coding agents preparing or auditing releases.
- Maintainers reviewing installer and release trust.
- Humans consuming the public one-line installer.

## Affected Product Docs

- `docs/decisions/0008-canonical-public-release-origin.md`
- `docs/HARNESS.md`
- `docs/ARCHITECTURE.md`
- `scripts/README.md`

## Non-Goals

- Releasing v0.3.0 as part of this story.
- Replacing GitHub Releases.
- Adding artifact signatures or provenance attestations.
- Verifying every platform binary by executing it on one host.
- Treating network availability as evidence that a release is valid or invalid.
