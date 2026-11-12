-- ANF-002: Zeitpunkt der letzten Änderung (R-8)
ALTER TABLE articles
    ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT now();

UPDATE articles SET updated_at = created_at;

-- Liste: neueste zuerst (R-2)
CREATE INDEX articles_created_at_idx ON articles (created_at DESC, id DESC);
