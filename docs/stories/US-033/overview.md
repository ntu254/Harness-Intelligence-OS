# Overview

## Current Behavior

HI-OS v0.5 learning-loop features are implemented across US-029 through
US-032, but no v0.5.0 public release exists yet. Installer payload coverage for
newer schema and decision contracts also needs release-hardening review.

## Target Behavior

HI-OS v0.5.0 is released publicly with trusted distribution evidence.

US-033 hardens and releases v0.5.0:

- bump CLI version to `0.5.0`
- update installer release pin
- update installer payload for v0.4/v0.5 schema and decision contracts
- publish public release assets
- verify release with `harness-cli release verify`
- close the installer payload backlog follow-up if resolved

## Affected Users

- Users installing Harness from the public release.
- Maintainers relying on trusted release evidence.
- Agents using v0.5 learning-loop commands.

## Affected Product Docs

- `RELEASE_NOTES_v0.5.0.md`
- `scripts/README.md`
- `scripts/harness-cli-release-tag`
- `scripts/install-harness.ps1`
- `scripts/install-harness.sh`
- `docs/stories/US-033/validation.md`

## Non-Goals

- v0.6 dashboard work.
- NotebookLM live-provider authentication setup.
- New learning-loop features beyond release hardening.
