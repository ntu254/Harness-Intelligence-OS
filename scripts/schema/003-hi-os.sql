-- HI-OS schema - migration 003
-- Harness Intelligence OS additions.

-- Extends intake with impact data from CodeGraph and semantic context from NotebookLM
ALTER TABLE intake ADD COLUMN code_impact_summary TEXT;
ALTER TABLE intake ADD COLUMN grounded_context TEXT;

-- Extends story with context pack references and architecture check results
ALTER TABLE story ADD COLUMN context_pack_path TEXT;
ALTER TABLE story ADD COLUMN arch_check_result TEXT 
    CHECK(arch_check_result IN ('pass','fail') OR arch_check_result IS NULL);

INSERT INTO schema_version (version) VALUES (3);
