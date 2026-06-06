# Architecture

No application stack is selected yet.

No application code exists yet. This document defines generic architecture
questions and boundary rules that future implementation should adapt after a
user-provided spec and stack decision exist.

## Discovery Before Shape

Before proposing implementation shape, identify:

- Product surfaces: browser, mobile, desktop, CLI, API, worker, or service.
- Runtime stack: language, framework, database, queues, providers, and hosting.
- Core domains: the product concepts that deserve stable names and contracts.
- Boundary inputs: user input, API requests, webhooks, jobs, files, credentials,
  provider payloads, and environment configuration.
- Validation ladder: the smallest checks that can prove the selected stack.

Record stack choices in `docs/decisions/` when they meaningfully constrain
future work.

## Default Layering

```text
domain
  <- application
      <- infrastructure
          <- interface
              <- app surfaces
```

## Candidate Structure

```text
app/
  domain/
    entities/
    value-objects/
    repositories/
    services/

  application/
    commands/
    queries/
    handlers/

  infrastructure/
    database/
    logging/
    notifications/

  interface/
    controllers/
    dto/
    presenters/
    routes/
    middlewares/

surfaces/
  browser/
  mobile/
  desktop/
  cli/
```

This is a thinking template, not a scaffold. Create real folders only when a
story enters implementation and the selected stack needs them.

## Dependency Rule

Inner layers must not depend on outer layers.

| Layer | May depend on | Must not depend on |
| --- | --- | --- |
| domain | nothing project-external except tiny pure utilities | framework, database, UI, provider, process/env |
| application | domain | framework, UI, provider, database concrete clients |
| infrastructure | domain, application | interface controllers or UI |
| interface | all backend layers | UI state or platform shell assumptions |
| app surfaces | API contracts and app-facing clients | domain internals directly |

## Automated Architecture Check

Projects encode executable dependency boundaries in
`harness-architecture.toml`:

```toml
[[layer]]
name = "domain"
path = "src/domain"
forbidden_imports = ["infrastructure", "interface"]

[[layer]]
name = "application"
path = "src/application"
forbidden_imports = ["interface"]
```

Use `files = ["domain.rs"]` when logical layers share one directory. The
scanner reads supported source files, extracts import/include statements, and
matches forbidden dependencies by complete path segment.

```bash
scripts/bin/harness-cli arch-check --story US-012
```

The MVP scanner supports Rust, TypeScript/JavaScript, Python, Go, Java, Kotlin,
and C# source extensions. It is a deterministic path/import gate, not a
replacement for a semantic dependency graph.

## Parse-First Boundary Rule

Unknown data must be parsed at boundaries before it enters inner code.

Boundaries include:

- HTTP request bodies, params, and query strings.
- Session payloads and identity claims.
- Environment variables.
- Database rows returned from external clients.
- Platform shell payloads.
- Deep links, tokens, and signed URLs.
- Provider webhooks, events, and async payloads.

Target flow:

```text
unknown input
  -> parser
  -> typed DTO or command
  -> application use case
  -> domain object/value object
```

External intelligence follows the same rule:

```text
MCP provider
  -> versioned file artifact
  -> Harness schema and semantic validation
  -> mapped context
  -> durable summary and governance use
```

MCP providers and adapters must not write directly into Harness SQLite.

Inner layers should work with meaningful product types such as `UserId`,
`AccountId`, `WorkspaceId`, `Role`, `DateRange`, or domain-specific IDs,
rather than repeatedly validating raw strings.

## Command/Query Boundary

If the product has both reads and writes, keep command/query separation clear at
the code level even when the storage layer is simple:

- Commands mutate state and own audit side effects.
- Queries read state and format for consumers.
- Shared domain rules live in domain/application, not controllers.

## Observability Contract

The future server should emit one canonical JSON log line per request with:

- timestamp
- level
- request_id
- user_id when known
- action
- duration_ms
- status_code
- message

Audit logs are product records. Application logs are operational records. Do not
use one as a substitute for the other.
