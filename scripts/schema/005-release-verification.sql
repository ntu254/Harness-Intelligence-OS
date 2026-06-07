-- HI-OS schema - migration 005
-- Trusted distribution release verification evidence.

ALTER TABLE story ADD COLUMN release_proof_required INTEGER NOT NULL DEFAULT 0
    CHECK(release_proof_required IN (0, 1));

CREATE TABLE release_verification (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    checked_at     TEXT NOT NULL DEFAULT (datetime('now')),
    version        TEXT NOT NULL,
    origin         TEXT NOT NULL,
    tag            TEXT NOT NULL,
    platform       TEXT NOT NULL,
    result         TEXT NOT NULL
                   CHECK(result IN ('pass', 'fail', 'inconclusive')),
    report_path    TEXT NOT NULL,
    assets_checked INTEGER NOT NULL DEFAULT 0,
    failure        TEXT,
    story_id       TEXT REFERENCES story(id)
);

CREATE INDEX release_verification_story_result
    ON release_verification(story_id, result, id DESC);

INSERT INTO schema_version (version) VALUES (5);
