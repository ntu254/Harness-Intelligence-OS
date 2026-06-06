# Overview

## Current Behavior

US-036 reports include maturity summary data, but maintainers still need to
inspect raw JSON to review governance state.

## Target Behavior

Harness CLI can export a standalone static HTML dashboard from a governance
report JSON artifact.

US-037 adds:

- `harness-cli governance dashboard`
- default dashboard output under `.harness/dashboard/index.html`
- static rendering for maturity, gate, release, friction, and story rows
- no external assets or live server

## Affected Users

- Maintainers reviewing governance state.
- Agents preparing v0.6 release hardening.
- Future release evidence consumers.

## Affected Product Docs

- `docs/GOVERNANCE_REPORT.md`
- `docs/stories/US-037/validation.md`

## Non-Goals

- v0.6 release/tag.
- Installer pin changes.
- SQLite migration.
- Live web server.
- External CSS or JavaScript assets.
