-- HI-OS schema - migration 006
-- Validated external intelligence artifact ingestion.

ALTER TABLE story ADD COLUMN codegraph_ingest_required INTEGER NOT NULL DEFAULT 0
    CHECK(codegraph_ingest_required IN (0, 1));

ALTER TABLE story ADD COLUMN notebooklm_ingest_required INTEGER NOT NULL DEFAULT 0
    CHECK(notebooklm_ingest_required IN (0, 1));

CREATE TABLE context_ingest (
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    checked_at        TEXT NOT NULL DEFAULT (datetime('now')),
    story_id          TEXT NOT NULL REFERENCES story(id),
    source            TEXT NOT NULL
                      CHECK(source IN ('codegraph', 'notebooklm')),
    artifact_type     TEXT NOT NULL,
    artifact_id       TEXT,
    artifact_path     TEXT NOT NULL,
    artifact_sha256   TEXT NOT NULL,
    schema_version    TEXT,
    result            TEXT NOT NULL
                      CHECK(result IN ('pass', 'fail', 'inconclusive')),
    provenance_status TEXT NOT NULL
                      CHECK(provenance_status IN ('pass', 'fail', 'inconclusive')),
    summary           TEXT,
    report_path       TEXT NOT NULL,
    failure           TEXT
);

CREATE INDEX context_ingest_story_source_result
    ON context_ingest(story_id, source, result, id DESC);

INSERT INTO schema_version (version) VALUES (6);
