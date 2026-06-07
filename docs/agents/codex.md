# Codex Agent Pack

Use this pack when Codex works in a HI-OS repository.

## Startup Checklist

Before editing files, Codex must read or generate the current operating
context:

```text
AGENTS.md
README.md
docs/HARNESS.md
docs/FEATURE_INTAKE.md
docs/ARCHITECTURE.md
docs/CONTEXT_RULES.md
harness-cli query matrix
```

If the task already has a story, generate or read the context pack:

```bash
scripts/bin/harness-cli context --story US-XXX
```

On Windows:

```powershell
.\scripts\bin\harness-cli.exe context --story US-XXX
```

## Work Rules

- Do not code before the lane, affected docs, story, and validation path are
  understood.
- Use `docs/FEATURE_INTAKE.md` to classify the task as `tiny`, `normal`, or
  `high-risk`.
- Use a story packet for normal and high-risk work.
- Prefer the context pack over rereading broad historical docs once it exists.
- Never treat CodeGraph or NotebookLM `inconclusive` evidence as `pass`.
- Never store Google credentials, cookies, tokens, browser profiles, or
  provider session files in Harness.
- Do not change release tags, installer pins, schemas, or governance policy
  unless the story explicitly requires it.

## Verification Discipline

Codex must not claim completion until the relevant proof has actually run.

Minimum handoff for normal or high-risk work:

```text
harness-cli context --story US-XXX
harness-cli arch-check --story US-XXX
project validation command
harness-cli trace --story US-XXX ...
harness-cli story verify US-XXX
```

When reporting completion, include:

- what changed;
- which validation commands passed;
- whether the story governance gate passed;
- what was intentionally out of scope.

## Recovery Pattern

If the gate fails, read the missing evidence and fix that exact gap. Do not
weaken the proof requirement to make the story pass.

