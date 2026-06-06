#!/usr/bin/env python3
import copy
import json
from pathlib import Path

from jsonschema import Draft202012Validator, FormatChecker


ROOT = Path(__file__).resolve().parents[1]
SCHEMA = ROOT / "docs" / "schemas" / "friction-event.schema.json"
TAXONOMY_DOC = ROOT / "docs" / "FRICTION_TAXONOMY.md"
DECISION_DOC = ROOT / "docs" / "decisions" / "0011-harness-friction-taxonomy.md"
EVENT_TYPES = [
    "missing_context",
    "ambiguous_policy",
    "weak_validation",
    "provider_unavailable",
    "schema_gap",
    "release_gap",
    "architecture_rule_gap",
    "repeated_manual_step",
]


def expect_valid(validator, instance, label):
    errors = sorted(validator.iter_errors(instance), key=lambda error: list(error.path))
    if errors:
        raise AssertionError(f"{label} should be valid: {errors[0].message}")


def expect_invalid(validator, instance, label):
    if not list(validator.iter_errors(instance)):
        raise AssertionError(f"{label} should be invalid")


def expect_event_types_documented(schema):
    schema_types = schema["properties"]["friction_type"]["enum"]
    if schema_types != EVENT_TYPES:
        raise AssertionError("schema friction_type enum does not match canonical order")

    taxonomy = TAXONOMY_DOC.read_text(encoding="utf-8")
    decision = DECISION_DOC.read_text(encoding="utf-8")
    for event_type in EVENT_TYPES:
        if f"`{event_type}`" not in taxonomy:
            raise AssertionError(f"{event_type} missing from taxonomy doc")
        if f"`{event_type}`" not in decision:
            raise AssertionError(f"{event_type} missing from Decision 0011")


event = {
    "schema_version": "1.0.0",
    "artifact_type": "friction-event",
    "event_id": "44444444-4444-4444-8444-444444444444",
    "story_id": "US-029",
    "trace_id": 25,
    "friction_type": "provider_unavailable",
    "severity": "high",
    "source": "trace",
    "summary": "NotebookLM default profile was unavailable during release hardening.",
    "observed_at": "2026-06-07T00:00:00Z",
    "provider": "notebooklm-mcp-cli",
    "affected_paths": [
        "docs/stories/US-026/validation.md",
        "docs/stories/US-028/validation.md",
    ],
    "evidence": {
        "command": "nlm login --check",
        "exit_code": 1,
        "artifact_path": ".harness/context/US-028-notebooklm-brief.json",
        "report_path": ".harness/context/US-028-notebooklm-ingest-result.json",
        "trace_id": 25,
        "details": "Profile not found: default",
    },
    "proposed_action": {
        "action_type": "provider_preflight",
        "title": "Add NotebookLM provider preflight",
        "target_path": "docs/stories/US-030/overview.md",
    },
}


with SCHEMA.open(encoding="utf-8") as handle:
    loaded_schema = json.load(handle)
Draft202012Validator.check_schema(loaded_schema)
expect_event_types_documented(loaded_schema)

validator = Draft202012Validator(loaded_schema, format_checker=FormatChecker())
expect_valid(validator, event, "provider unavailable friction event")

manual_step = copy.deepcopy(event)
manual_step["event_id"] = "55555555-5555-4555-8555-555555555555"
manual_step["friction_type"] = "repeated_manual_step"
manual_step["severity"] = "medium"
manual_step["provider"] = "local-shell"
manual_step.pop("evidence")
manual_step["summary"] = "Release packaging required repeated manual command checks."
manual_step["proposed_action"] = {
    "action_type": "backlog",
    "title": "Add release packaging preflight checklist",
}
expect_valid(validator, manual_step, "medium repeated manual step")

invalid = copy.deepcopy(event)
invalid["friction_type"] = "provider_unavailable"
invalid.pop("provider")
expect_invalid(validator, invalid, "provider unavailable without provider")

invalid = copy.deepcopy(event)
invalid["severity"] = "high"
invalid.pop("evidence")
expect_invalid(validator, invalid, "high severity without evidence")

invalid = copy.deepcopy(event)
invalid["friction_type"] = "made_up_type"
expect_invalid(validator, invalid, "unknown friction type")

print("Friction taxonomy schema and semantic fixtures passed.")
