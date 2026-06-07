# Design

## Domain Model

The cookbook is organized around repeated command jobs rather than story
history:

- intake;
- context;
- verify;
- trace;
- release;
- dashboard;
- MCP/provider evidence.

## Application Flow

Users should be able to move from a task question to a command example:

```text
What am I trying to do?
  -> find group
  -> copy minimal command
  -> replace story id / file path
  -> run validation
```

## Interface Contract

No CLI changes. The cookbook references existing commands and is checked by:

```text
python scripts/verify-adoption-docs.py
```

## Data Model

No SQLite migration is added.

## UI / Platform Impact

The cookbook primarily uses POSIX-style command examples and tells Windows
users to replace the CLI path with `.\scripts\bin\harness-cli.exe`.

## Observability

The cookbook includes commands for trace, query matrix, release verify,
governance report, and dashboard evidence.

## Alternatives Considered

1. Expand `scripts/README.md` instead.
   - Rejected because `scripts/README.md` is implementation detail; the
     cookbook is user-facing adoption material.
2. Merge cookbook with troubleshooting.
   - Rejected because cookbook answers "what command?" while troubleshooting
     answers "why did it fail?".

