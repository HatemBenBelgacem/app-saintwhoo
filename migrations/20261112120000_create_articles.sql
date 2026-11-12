-- ANF-001: Magazin-Artikel erfassen
-- Die Regeln aus docs/features/ANF-001 werden zusätzlich in der DB abgesichert.
CREATE TABLE articles (
    id           BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title        TEXT        NOT NULL,
    body         TEXT        NOT NULL,
    published_on DATE,
    status       TEXT        NOT NULL DEFAULT 'draft',
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT articles_title_length CHECK (char_length(title) BETWEEN 1 AND 200),
    CONSTRAINT articles_title_trimmed CHECK (title = btrim(title)),
    CONSTRAINT articles_body_not_blank CHECK (body ~ '\S'),
    CONSTRAINT articles_status_valid CHECK (status IN ('draft', 'published')),
    CONSTRAINT articles_published_needs_date CHECK (status <> 'published' OR published_on IS NOT NULL)
);

CREATE INDEX articles_status_published_on_idx ON articles (status, published_on);
