# Claude Code Agent Pack

Use this pack when Claude Code works in a HI-OS repository.

Claude Code loads `CLAUDE.md`, but it does not automatically load `AGENTS.md`
unless the project imports it. In this repo, `CLAUDE.md` imports `AGENTS.md`
and `docs/FEATURE_INTAKE.md`. Still, Claude should use this pack as the
operating checklist for HI-OS work.

## Startup Checklist

Before editing files:

```text
Confirm CLAUDE.md imports AGENTS.md.
Read README.md.
Read docs/HARNESS.md.
Read docs/ARCHITECTURE.md when the task touches code or boundaries.
Read docs/CONTEXT_RULES.md for lane-specific retrieval.
Run harness-cli query matrix.
```

If a story id exists:

```bash
scripts/bin/harness-cli context --story US-XXX
```

On Windows:

```powershell
.\scripts\bin\harness-cli.exe context --story US-XXX
```

## Work Rules

- Do not code before the lane, story, context pack, and validation path are
  understood.
- Classify the work through `docs/FEATURE_INTAKE.md`.
- For normal or high-risk work, create or update the story packet first.
- Read the generated context pack before implementation.
- Keep provider outputs behind the file-based CodeGraph/NotebookLM artifact
  boundary.
- Treat provider unavailable as `inconclusive`, not `pass`.
- Do not store Google credentials, cookies, tokens, browser profiles, or
  provider session files in Harness.

## Verification Discipline

Claude Code must not say a story is done until proof exists.

Run the relevant validation ladder:

```text
context pack
architecture check
project tests or docs verifier
trace
story verify
```

Expected final gate:

```bash
scripts/bin/harness-cli story verify US-XXX
```

The final answer should name the story, validation commands, gate result, and
any out-of-scope items.

## Recovery Pattern

If the context pack or story gate is missing evidence, create the missing
artifact or run the missing proof. Do not summarize unverified work as complete.
