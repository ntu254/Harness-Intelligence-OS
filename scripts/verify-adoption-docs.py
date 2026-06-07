#!/usr/bin/env python3
"""Validate adoption docs contain adoption and agent instruction contracts."""

from __future__ import annotations

from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]


def read(path: str) -> str:
    full_path = ROOT / path
    if not full_path.exists():
        raise AssertionError(f"missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def require(text: str, needle: str, path: str) -> None:
    if needle not in text:
        raise AssertionError(f"{path}: missing required text: {needle}")


def main() -> int:
    walkthrough_path = "docs/adoption/clean-clone-walkthrough.md"
    walkthrough = read(walkthrough_path)
    example_path = "docs/examples/full-agent-workflow.md"
    example = read(example_path)
    agent_paths = [
        "docs/agents/codex.md",
        "docs/agents/claude-code.md",
        "docs/agents/cursor.md",
    ]
    agents = {path: read(path) for path in agent_paths}
    agents_text = "\n".join(agents.values())
    troubleshooting_path = "docs/troubleshooting.md"
    troubleshooting = read(troubleshooting_path)
    cookbook_path = "docs/COMMAND_COOKBOOK.md"
    cookbook = read(cookbook_path)
    agents_index = read("AGENTS.md")
    readme = read("README.md")
    docs_readme = read("docs/README.md")
    scripts_readme = read("scripts/README.md")

    readme_needles = [
        "# Harness Intelligence OS",
        "Sovereign Identity",
        "hios.toml",
        "harness-cli identity",
        "5-Minute Quickstart",
        "intake",
        "context",
        "story verify",
        "trace",
        "governance dashboard",
        "docs/adoption/clean-clone-walkthrough.md",
        "docs/examples/full-agent-workflow.md",
        "docs/agents/codex.md",
        "docs/agents/claude-code.md",
        "docs/agents/cursor.md",
        "docs/troubleshooting.md",
        "docs/COMMAND_COOKBOOK.md",
        "release verify --version 0.6.0",
        "Governance Dashboard",
        "CodeGraph",
        "NotebookLM",
        "inconclusive",
        "Google credentials",
        "ntu254/Harness-Intelligence-OS",
        "v0.7: Adoption Ready",
    ]

    for needle in readme_needles:
        require(readme, needle, "README.md")

    walkthrough_needles = [
        "git clone https://github.com/ntu254/Harness-Intelligence-OS.git",
        "cargo build --package harness-cli --release",
        "harness-cli.exe init",
        "import brownfield",
        "query matrix",
        "US-DEMO",
        "context --story US-DEMO",
        "arch-check --story US-DEMO",
        "trace",
        "story verify US-DEMO",
        "governance report",
        "governance dashboard",
        "release verify --version 0.6.0",
        "CodeGraph",
        "NotebookLM",
        "inconclusive",
        "harness.db",
        ".harness/",
        "Do not commit",
        "Google credentials",
    ]

    for needle in walkthrough_needles:
        require(walkthrough, needle, walkthrough_path)

    example_needles = [
        "# Full Agent Workflow Example",
        "US-EXAMPLE",
        "intake",
        "story add",
        "Optional Provider Context",
        "codegraph impact",
        "notebooklm brief",
        "context --story US-EXAMPLE",
        "cargo test --workspace",
        "story update",
        "trace",
        "story verify US-EXAMPLE",
        "governance report",
        "governance dashboard",
        "Expected output",
        "Provider Troubleshooting",
        "CodeGraph Unavailable",
        "NotebookLM Auth Or Session Missing",
        "Context ingest: inconclusive",
        "Context ingest: fail",
        "Google credentials",
        "provider session files",
    ]

    for needle in example_needles:
        require(example, needle, example_path)

    for path, text in agents.items():
        agent_needles = [
            "Startup Checklist",
            "context --story US-XXX",
            "Do not code before",
            "story verify",
            "inconclusive",
            "pass",
            "Google credentials",
            "provider session files",
            "Verification Discipline",
        ]
        for needle in agent_needles:
            require(text, needle, path)

    for path in agent_paths:
        require(readme, path, "README.md")
        require(agents_index, path, "AGENTS.md")

    troubleshooting_needles = [
        "# Troubleshooting",
        "Installer Fails",
        "Release Verify Fails",
        "CodeGraph Unavailable",
        "NotebookLM Auth Or Session Fails",
        "NotebookLM Output Fails Validation",
        "Governance Gate Fails",
        "Governance Report Or Dashboard Fails",
        "checksum mismatch",
        "release verify --version 0.6.0",
        "Context ingest: inconclusive",
        "Context ingest: fail",
        "story verify US-XXX",
        "verify-governance-report-schema.py",
        "harness.db",
        ".harness/",
        "Google credentials",
        "provider session files",
        "Do not weaken the story gate",
    ]

    for needle in troubleshooting_needles:
        require(troubleshooting, needle, troubleshooting_path)

    cookbook_needles = [
        "# Command Cookbook",
        "harness-cli identity",
        "## Intake",
        "## Context",
        "## Verify",
        "## Trace",
        "## Release",
        "## Dashboard",
        "## MCP / Provider Evidence",
        "harness-cli intake",
        "harness-cli context --story",
        "harness-cli story verify",
        "harness-cli trace",
        "release verify --version 0.6.0",
        "governance report",
        "governance dashboard",
        "codegraph impact",
        "notebooklm brief",
        "inconclusive",
        "Google credentials",
        "provider session files",
        ".\\scripts\\bin\\harness-cli.exe",
    ]

    for needle in cookbook_needles:
        require(cookbook, needle, cookbook_path)

    require(
        readme,
        "docs/adoption/clean-clone-walkthrough.md",
        "README.md",
    )
    require(readme, "docs/examples/full-agent-workflow.md", "README.md")
    require(readme, "docs/troubleshooting.md", "README.md")
    require(readme, "docs/COMMAND_COOKBOOK.md", "README.md")
    require(docs_readme, "adoption/", "docs/README.md")
    require(docs_readme, "examples/", "docs/README.md")
    require(docs_readme, "agents/", "docs/README.md")
    require(docs_readme, "troubleshooting.md", "docs/README.md")
    require(docs_readme, "COMMAND_COOKBOOK.md", "docs/README.md")
    require(docs_readme, "hios.toml", "docs/README.md")
    require(scripts_readme, "harness-cli identity", "scripts/README.md")
    require(scripts_readme, "hios.toml", "scripts/README.md")
    require(agents_text, "Codex", "docs/agents/*")
    require(agents_text, "Claude Code", "docs/agents/*")
    require(agents_text, "Cursor", "docs/agents/*")
    require(
        scripts_readme,
        "python scripts/verify-adoption-docs.py",
        "scripts/README.md",
    )

    print("Adoption docs verification passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as exc:
        print(f"Adoption docs verification failed: {exc}", file=sys.stderr)
        raise SystemExit(1)
