#!/usr/bin/env python3
import copy
import json
from pathlib import Path

from jsonschema import Draft202012Validator, FormatChecker


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_DIR = ROOT / "docs" / "schemas"


def load_schema(name):
    with (SCHEMA_DIR / name).open(encoding="utf-8") as handle:
        schema = json.load(handle)
    Draft202012Validator.check_schema(schema)
    return Draft202012Validator(schema, format_checker=FormatChecker())


def expect_valid(validator, instance, label):
    errors = sorted(validator.iter_errors(instance), key=lambda error: list(error.path))
    if errors:
        raise AssertionError(f"{label} should be valid: {errors[0].message}")


def expect_invalid(validator, instance, label):
    if not list(validator.iter_errors(instance)):
        raise AssertionError(f"{label} should be invalid")


sha = "a" * 64
codegraph = {
    "schema_version": "1.0.0",
    "artifact_type": "codegraph-impact",
    "artifact_id": "11111111-1111-4111-8111-111111111111",
    "story_id": "US-023",
    "status": "pass",
    "generated_at": "2026-06-07T00:00:00Z",
    "provenance": {
        "provider": "codegraph",
        "adapter": "hios-codegraph-adapter",
        "adapter_version": "0.1.0",
        "invocation_id": "run-1",
        "repository": "ntu254/Harness-Intelligence-OS",
        "revision": "3fdc04c",
        "inputs": [{"uri": "git:HEAD", "sha256": sha}],
    },
    "impact": {
        "summary": "Defines contracts only.",
        "affected_files": [
            {
                "path": "docs/schemas/codegraph-impact.schema.json",
                "change_kind": "documentation",
                "reasons": ["New versioned contract"],
            }
        ],
        "risk_flags": ["public_contracts", "external_systems"],
        "claims": [
            {
                "claim_id": "CG-1",
                "statement": "The story changes an intelligence artifact contract.",
                "evidence_refs": ["docs/schemas/codegraph-impact.schema.json"],
            }
        ],
    },
}

notebooklm = {
    "schema_version": "1.0.0",
    "artifact_type": "notebooklm-brief",
    "artifact_id": "22222222-2222-4222-8222-222222222222",
    "story_id": "US-023",
    "status": "pass",
    "generated_at": "2026-06-07T00:00:00Z",
    "provenance": {
        "provider": "notebooklm",
        "adapter": "hios-notebooklm-adapter",
        "adapter_version": "0.1.0",
        "invocation_id": "run-2",
        "sources": [
            {
                "source_id": "SRC-1",
                "title": "Harness operating model",
                "uri": "docs/HARNESS.md",
                "sha256": sha,
                "retrieved_at": "2026-06-07T00:00:00Z",
            }
        ],
    },
    "brief": {
        "summary": "MCP tools produce files that Harness validates.",
        "constraints": ["Providers do not write directly to SQLite."],
        "open_questions": [],
        "affected_docs": ["docs/HARNESS.md"],
        "claims": [
            {
                "claim_id": "NL-1",
                "statement": "Harness owns durable ingestion.",
                "citations": [{"source_id": "SRC-1", "locator": "Durable Layer"}],
            }
        ],
    },
}

ingest = {
    "schema_version": "1.0.0",
    "artifact_type": "context-ingest-result",
    "ingest_id": "33333333-3333-4333-8333-333333333333",
    "story_id": "US-023",
    "source": "codegraph",
    "source_artifact": {
        "artifact_type": "codegraph-impact",
        "artifact_id": codegraph["artifact_id"],
        "schema_version": "1.0.0",
        "path": ".harness/artifacts/codegraph-impact.json",
        "sha256": sha,
    },
    "status": "pass",
    "checked_at": "2026-06-07T00:01:00Z",
    "mapped_context": {
        "risk_flags": ["public_contracts"],
        "affected_files": ["docs/schemas/codegraph-impact.schema.json"],
        "code_impact_summary": "Defines contracts only.",
        "claim_ids": ["CG-1"],
    },
    "governance": {
        "eligible_for_intake": True,
        "eligible_for_context_pack": True,
        "eligible_for_story_verify": True,
    },
}

validators = {
    "codegraph": load_schema("codegraph-impact.schema.json"),
    "notebooklm": load_schema("notebooklm-brief.schema.json"),
    "ingest": load_schema("context-ingest-result.schema.json"),
}

expect_valid(validators["codegraph"], codegraph, "passing CodeGraph artifact")
expect_valid(validators["notebooklm"], notebooklm, "passing NotebookLM artifact")
expect_valid(validators["ingest"], ingest, "passing ingest result")

failed_codegraph = copy.deepcopy(codegraph)
failed_codegraph["status"] = "fail"
failed_codegraph.pop("impact")
failed_codegraph["errors"] = [
    {"code": "GRAPH_QUERY_FAILED", "message": "Graph query failed.", "retryable": False}
]
expect_valid(validators["codegraph"], failed_codegraph, "failed CodeGraph artifact")

inconclusive_notebooklm = copy.deepcopy(notebooklm)
inconclusive_notebooklm["status"] = "inconclusive"
inconclusive_notebooklm.pop("brief")
inconclusive_notebooklm["provenance"]["sources"] = []
inconclusive_notebooklm["unavailable"] = {
    "reason": "provider_unavailable",
    "retryable": True,
}
expect_valid(
    validators["notebooklm"],
    inconclusive_notebooklm,
    "inconclusive NotebookLM artifact",
)

inconclusive_ingest = copy.deepcopy(ingest)
inconclusive_ingest["status"] = "inconclusive"
inconclusive_ingest.pop("mapped_context")
inconclusive_ingest["diagnostics"] = [
    {
        "code": "SOURCE_UNAVAILABLE",
        "message": "MCP source was unavailable.",
        "retryable": True,
    }
]
inconclusive_ingest["governance"] = {
    "eligible_for_intake": False,
    "eligible_for_context_pack": False,
    "eligible_for_story_verify": False,
}
expect_valid(validators["ingest"], inconclusive_ingest, "inconclusive ingest result")

invalid = copy.deepcopy(codegraph)
invalid["status"] = "inconclusive"
expect_invalid(validators["codegraph"], invalid, "inconclusive CodeGraph without unavailable")

invalid = copy.deepcopy(codegraph)
invalid.pop("provenance")
expect_invalid(validators["codegraph"], invalid, "CodeGraph without provenance")

invalid = copy.deepcopy(notebooklm)
invalid["brief"]["claims"][0]["citations"] = []
expect_invalid(validators["notebooklm"], invalid, "ungrounded NotebookLM claim")

invalid = copy.deepcopy(ingest)
invalid["status"] = "inconclusive"
expect_invalid(validators["ingest"], invalid, "inconclusive ingest marked governance eligible")

print("MCP artifact contract schemas and semantic fixtures passed.")
