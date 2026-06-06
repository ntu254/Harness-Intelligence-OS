# Feature Intake

Every implementation prompt enters the intake gate before code changes. A new
project spec also enters through this gate before it becomes product docs,
stories, or implementation work.

The human does not need to classify risk. The harness does.

## Intake Flow

```text
User prompt
    |
    v
Classify input type
    |
    v
Restate as work item
    |
    v
Find affected product docs and stories
    |
    v
Run risk checklist
    |
    v
Choose lane: tiny, normal, or high-risk
```

## Input Types

Use the input type to decide where the work should land before choosing the risk
lane.

| Type | Use when | Typical artifact |
| --- | --- | --- |
| New spec | Turning a user-provided project spec into harness-ready docs | Product docs, candidate epics, decisions |
| Spec slice | Implementing selected behavior from an accepted spec | Story packet |
| Change request | Changing, fixing, or refining accepted behavior | Story packet or direct patch |
| New initiative | Adding a larger product area that needs multiple stories | Initiative notes plus story packets |
| Maintenance request | Changing technical, operational, or dependency behavior | Story packet, validation report, or decision |
| Harness improvement | Improving how humans and agents collaborate | Direct docs update or `scripts/bin/harness-cli backlog add` |

Do not create or extend a monolithic spec by default after intake. Use product
docs, stories, decisions, and initiative notes as the living surface.

## Automated Intake (HI-OS)

For projects using Harness Intelligence OS, the intake checklist can be automated:

```bash
scripts/bin/harness-cli intake \
  --summary "<text>" \
  --story <id> \
  --auto \
  --impact-report <path-to-codegraph-impact-json> \
  --business-context <path-to-notebooklm-brief-md>
```

This automates the following steps:
1.  **Risk Flag Identification:** Scans the CodeGraph impact report for affected files (e.g. database schema changes, auth files, or public contracts) to automatically apply appropriate risk flags.
2.  **Risk Lane Determination:** Calculates whether the story is `tiny`, `normal`, or `high-risk` based on the detected flags.
3.  **Context Pack Seeding:** Stores CodeGraph and NotebookLM insights directly inside the SQLite durable database, ready to be packaged by `harness-cli context`.

The MVP JSON reader supports top-level string fields and string arrays only.
Nested CodeGraph reports are unsupported input rather than complete impact
evidence. Risk detection uses normalized path segments, so names such as
`author` do not count as `auth`.

## MCP Artifact Boundary

HI-OS v0.4 defines the real-provider boundary as versioned JSON files:

- `docs/schemas/codegraph-impact.schema.json`
- `docs/schemas/notebooklm-brief.schema.json`
- `docs/schemas/context-ingest-result.schema.json`

MCP tools do not write directly to Harness SQLite. `context ingest` validates a
provider artifact, verifies provenance and semantic links, maps accepted
fields, and emits an auditable ingest result. The full operational artifact and
result remain files; SQLite stores the governance summary. `fail` and
`inconclusive` artifacts never count as successful intake or governance
evidence. The current `intake --auto` flags remain unchanged; live provider
adapters and broader governance integration are separate stories.

The CodeGraph CLI adapter is available through `harness-cli codegraph impact`.
It preserves the file boundary and composes with `context ingest`; it does not
change `intake --auto` defaults.

The NotebookLM grounded-brief adapter is available through `harness-cli
notebooklm brief`. It invokes the accepted `notebooklm-mcp-cli` CLI boundary,
using `nlm query notebook --json`, requires an explicit provider notebook id
or alias, normalizes only citation-backed provider output into a
`notebooklm-brief` artifact, and composes with `context ingest`.
Provider/session/network unavailability is inconclusive, while uncited
summaries or malformed provider output fail and cannot satisfy governance.

When `intake --auto --story <id>` runs after context evidence has been ingested,
Harness prefers the latest passing CodeGraph and NotebookLM `context_ingest`
reports for that story. Passing CodeGraph evidence can seed risk flags,
affected docs, and code impact summary; passing NotebookLM evidence can seed
affected docs and grounded context. Failed or inconclusive evidence remains
audit-visible but never seeds intake as proof. Missing required evidence is
blocked by story governance rather than by intake creation, avoiding a circular
bootstrap between intake and `context ingest`.

## Lanes

### Tiny

Use for low-risk docs, copy, names, or narrow edits.

Also use for initial project setup when the work is limited to installing
declared dependencies, wiring a server entrypoint, adding a health/smoke
endpoint, or opening a local development database connection without creating
domain schema, CRUD behavior, auth, authorization, provider integration, or
data migration. A health endpoint in a new benchmark or scaffolded project is
smoke proof, not a public contract escalation by itself.

Requirements:

- Patch directly.
- Keep affected docs current.
- Run available quick checks.
- Update the harness only if friction was found.

### Normal

Use for story-sized behavior with bounded blast radius.

Requirements:

- Create or update one story file from `docs/templates/story.md`.
- Link relevant product docs.
- Add or update validation expectations.
- Implement the smallest vertical slice when implementation exists.
- Record or update proof status with `scripts/bin/harness-cli story add` and
  `scripts/bin/harness-cli story update`.

### High-Risk

Use when the work can affect security, data, scope, contracts, or multiple
roles/platforms.

Requirements:

- Create a story folder using `docs/templates/high-risk-story/`.
- Fill in `execplan.md`, `overview.md`, `design.md`, and `validation.md`.
- Ask for human confirmation before implementation if direction is ambiguous.
- Record a durable decision when behavior, architecture, authorization, data
  ownership, API shape, or validation requirements change meaningfully. Use a
  `docs/decisions/NNNN-*.md` file from `docs/templates/decision.md`, then add
  or refresh the durable row with `scripts/bin/harness-cli decision add`.
  Decision text in a trace is not a durable decision record.

## Risk Checklist

Mark one flag for each item that applies:

| Risk flag | Applies when the work touches |
| --- | --- |
| Auth | login, logout, sessions, JWT, password, refresh token |
| Authorization | roles, permissions, tenant or company scope |
| Data model | schema, migrations, uniqueness, deletion, retention |
| Audit/security | audit logs, privacy, sensitive data, access logs |
| External systems | email, payments, cloud services, provider SDKs, queues, webhooks |
| Public contracts | API shape, response envelope, client-visible behavior |
| Cross-platform | desktop/mobile/browser split, native shell behavior, deep links |
| Existing behavior | already implemented or test-covered behavior changes |
| Weak proof | unclear or missing tests around the affected area |
| Multi-domain | more than one product domain changes at once |

## Classification

```text
0-1 flags:
  tiny or normal, based on code impact

2-3 flags:
  normal with stronger validation

4+ flags:
  high-risk

Any hard gate:
  high-risk unless the human explicitly narrows scope
```

Hard gates:

- Auth.
- Authorization.
- Data loss or migration.
- Audit/security.
- External provider behavior.
- Removing or weakening validation requirements.

## Output

At the end of intake, the agent should be able to say:

```text
Lane: normal
Reason: touches authorization, API contract, and audit behavior.
Docs: permissions, account-settings, audit-log.
Story: docs/stories/epics/E02-access-control/US-014-manager-updates-role.md.
Validation: unit, integration, E2E.
```
