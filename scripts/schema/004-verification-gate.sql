-- HI-OS schema - migration 004
-- HI-OS v0.2 architecture and story governance gate evidence.

ALTER TABLE intake ADD COLUMN auto_generated INTEGER NOT NULL DEFAULT 0;

ALTER TABLE story ADD COLUMN arch_checked_at TEXT;
ALTER TABLE story ADD COLUMN gate_checked_at TEXT;
ALTER TABLE story ADD COLUMN gate_result TEXT
    CHECK(gate_result IN ('pass','fail') OR gate_result IS NULL);

INSERT INTO schema_version (version) VALUES (4);
