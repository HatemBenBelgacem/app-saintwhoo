-- Erste Migration: Basis-Setup.
-- Hier kommen später die Tabellen aus docs/system-design.md hinein.
CREATE TABLE IF NOT EXISTS app_meta (
    key        TEXT PRIMARY KEY,
    value      TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
