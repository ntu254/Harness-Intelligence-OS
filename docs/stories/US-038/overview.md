# Overview

## Current Behavior

US-034 through US-037 implemented governance report, maturity summary, and
static dashboard export, but no public v0.6.0 release exists yet.

## Target Behavior

HI-OS v0.6.0 is released publicly with trusted distribution evidence and
governance dashboard artifacts.

US-038 hardens and releases v0.6.0:

- bump CLI version to `0.6.0`;
- pin installer to `harness-cli-v0.6.0`;
- update installer payload for v0.6 governance docs/schema/verifier;
- add release notes;
- run local validation;
- publish GitHub release assets;
- run `release verify --version 0.6.0`;
- generate governance report and dashboard evidence.

## Affected Users

- Maintainers installing Harness from `main`.
- Agents verifying trusted distribution.
- Governance dashboard consumers.

## Affected Product Docs

- `RELEASE_NOTES_v0.6.0.md`
- `scripts/README.md`
- `docs/stories/US-038/validation.md`

## Non-Goals

- Adding new dashboard features beyond US-037.
- Changing older release tags or assets.
- Adding migrations.
