# 0009 Release Verification Evidence Storage

Date: 2026-06-06

## Status

Accepted

## Context

US-021 must produce evidence that is both detailed enough for release audits
and stable enough for Harness queries and the blocking story governance gate.

An operational JSON file is portable and inspectable, but file discovery alone
is a weak governance contract. A SQLite row is queryable and durable, but a
summary cannot preserve every asset, hash, command output, and failure detail.

## Decision

Use hybrid evidence storage:

1. `release verify` writes a complete JSON report under `.harness/release/`.
2. It writes a summary to a dedicated SQLite release verification table.
3. The summary may link to a story.
4. Stories gain an explicit durable `release_proof_required` flag.
5. `story verify` requires a passing linked summary when that flag is set.
6. The canonical origin is read from tracked `harness-release.toml`, initialized
   to match decision `0008`.
7. Network or GitHub unavailability is stored as `inconclusive`, exits
   non-zero, and never satisfies governance.

The JSON report is the audit artifact. SQLite is the query, statistics, and
governance surface.

## Alternatives Considered

1. JSON only. Rejected because story verification would depend on filesystem
   discovery and ad hoc parsing.
2. SQLite only. Rejected because a summary cannot preserve the full release
   audit trail.
3. JSON plus SQLite summary. Accepted because it preserves both audit depth and
   governance stability.

## Consequences

Positive:

- Detailed evidence remains portable and human-inspectable.
- Story verification can query one durable result.
- Governance reports can aggregate release trust without reparsing files.

Tradeoffs:

- Implementation must keep report and summary writes consistent.
- A schema migration and repository API are required.
- Cleanup and retention rules must account for both storage surfaces.

## Follow-Up

- Define the migration and summary fields during US-021 implementation.
- Add consistency tests for report-write and database-write failures.
- Add tests proving release evidence is required only by the explicit story
  flag, never inferred from names or paths.
- Do not move US-021 to In Progress until implementation explicitly begins.
