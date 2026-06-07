-- HI-OS schema - migration 007
-- Durable structured friction events for the HI-OS v0.5 learning loop.

CREATE TABLE friction_event (
    id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    captured_at          TEXT NOT NULL DEFAULT (datetime('now')),
    schema_version       TEXT NOT NULL DEFAULT '1.0.0'
                         CHECK(schema_version = '1.0.0'),
    artifact_type        TEXT NOT NULL DEFAULT 'friction-event'
                         CHECK(artifact_type = 'friction-event'),
    event_id             TEXT NOT NULL UNIQUE,
    story_id             TEXT REFERENCES story(id),
    trace_id             INTEGER REFERENCES trace(id),
    friction_type        TEXT NOT NULL
                         CHECK(friction_type IN (
                            'missing_context',
                            'ambiguous_policy',
                            'weak_validation',
                            'provider_unavailable',
                            'schema_gap',
                            'release_gap',
                            'architecture_rule_gap',
                            'repeated_manual_step'
                         )),
    severity             TEXT NOT NULL
                         CHECK(severity IN ('low', 'medium', 'high')),
    source               TEXT NOT NULL
                         CHECK(source IN (
                            'trace',
                            'agent',
                            'review',
                            'workflow',
                            'provider'
                         )),
    summary              TEXT NOT NULL,
    observed_at          TEXT NOT NULL,
    provider             TEXT,
    affected_paths       TEXT,
    evidence_json        TEXT,
    proposed_action_json TEXT,
    notes                TEXT,
    CHECK(friction_type != 'provider_unavailable' OR provider IS NOT NULL),
    CHECK(severity != 'high' OR evidence_json IS NOT NULL)
);

CREATE INDEX friction_event_story
    ON friction_event(story_id, id DESC);

CREATE INDEX friction_event_type_severity
    ON friction_event(friction_type, severity, id DESC);

INSERT INTO schema_version (version) VALUES (7);
