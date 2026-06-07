#!/usr/bin/env python3
"""Validate adoption docs contain the README and walkthrough contracts."""

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
    readme = read("README.md")
    docs_readme = read("docs/README.md")
    scripts_readme = read("scripts/README.md")

    readme_needles = [
        "# Harness Intelligence OS",
        "5-Minute Quickstart",
        "intake",
        "context",
        "story verify",
        "trace",
        "governance dashboard",
        "docs/adoption/clean-clone-walkthrough.md",
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

    require(
        readme,
        "docs/adoption/clean-clone-walkthrough.md",
        "README.md",
    )
    require(docs_readme, "adoption/", "docs/README.md")
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
