-- ANF-006: Events (mit Erweiterung aus dem Designvorschlag „Techno App“:
-- Beginn, Ort, Stadt, Lineup, Tickets)
-- Die Regeln aus docs/features/ANF-006 werden zusätzlich in der DB abgesichert.
CREATE TABLE events (
    id            BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title         TEXT        NOT NULL,
    description   TEXT        NOT NULL,
    -- Ortszeit (Europe/Zurich), ohne Zeitzone gespeichert
    starts_at     TIMESTAMP   NOT NULL,
    venue         TEXT        NOT NULL,
    city          TEXT        NOT NULL,
    lineup        TEXT        NOT NULL DEFAULT '',
    ticket_url    TEXT,
    ticket_status TEXT        NOT NULL DEFAULT 'available',
    image_url     TEXT,
    image_credit  TEXT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT events_title_length CHECK (char_length(title) BETWEEN 1 AND 150),
    CONSTRAINT events_title_trimmed CHECK (title = btrim(title)),
    CONSTRAINT events_description_not_blank CHECK (description ~ '\S'),
    CONSTRAINT events_description_length CHECK (char_length(description) <= 10000),
    CONSTRAINT events_venue_length CHECK (char_length(venue) BETWEEN 1 AND 120 AND venue = btrim(venue)),
    CONSTRAINT events_city_length CHECK (char_length(city) BETWEEN 1 AND 80 AND city = btrim(city)),
    CONSTRAINT events_lineup_length CHECK (char_length(lineup) <= 300),
    CONSTRAINT events_ticket_url_format
        CHECK (ticket_url IS NULL OR (ticket_url ~ '^https://' AND ticket_url !~ '\s' AND char_length(ticket_url) <= 500)),
    CONSTRAINT events_ticket_status_valid CHECK (ticket_status IN ('available', 'few', 'sold_out')),
    CONSTRAINT events_image_url_format
        CHECK (image_url IS NULL OR (image_url ~ '^(https://|/[^/])' AND image_url !~ '\s' AND char_length(image_url) <= 500)),
    CONSTRAINT events_image_credit_length
        CHECK (image_credit IS NULL OR char_length(image_credit) <= 200),
    CONSTRAINT events_image_needs_credit
        CHECK (image_url IS NULL OR (image_credit IS NOT NULL AND btrim(image_credit) <> '')),
    CONSTRAINT events_credit_needs_image
        CHECK (image_credit IS NULL OR image_url IS NOT NULL)
);

-- Verwaltung: zuletzt geändert zuerst
CREATE INDEX events_updated_at_idx ON events (updated_at DESC, id DESC);
-- Eventliste: nach Beginn
CREATE INDEX events_starts_at_idx ON events (starts_at);
