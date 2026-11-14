-- ANF-007: Radio – Sendeplan und Archiv
-- Die Regeln aus docs/features/ANF-007 werden zusätzlich in der DB abgesichert.

-- Sendungen im Sendeplan. Zeiten in Ortszeit (Europe/Zurich), ohne Zeitzone.
CREATE TABLE radio_shows (
    id         BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title      TEXT        NOT NULL,
    host       TEXT        NOT NULL,
    starts_at  TIMESTAMP   NOT NULL,
    ends_at    TIMESTAMP   NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT radio_shows_title_length CHECK (char_length(title) BETWEEN 1 AND 120 AND title = btrim(title)),
    CONSTRAINT radio_shows_host_length CHECK (char_length(host) BETWEEN 1 AND 120 AND host = btrim(host)),
    CONSTRAINT radio_shows_duration CHECK (ends_at > starts_at AND ends_at <= starts_at + interval '12 hours')
);

CREATE INDEX radio_shows_starts_at_idx ON radio_shows (starts_at);

-- Aufgezeichnete Sets zum Nachhören.
CREATE TABLE radio_mixes (
    id               BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title            TEXT        NOT NULL,
    host             TEXT        NOT NULL,
    duration_minutes INTEGER     NOT NULL,
    audio_url        TEXT        NOT NULL,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT radio_mixes_title_length CHECK (char_length(title) BETWEEN 1 AND 120 AND title = btrim(title)),
    CONSTRAINT radio_mixes_host_length CHECK (char_length(host) BETWEEN 1 AND 120 AND host = btrim(host)),
    CONSTRAINT radio_mixes_duration CHECK (duration_minutes BETWEEN 1 AND 600),
    CONSTRAINT radio_mixes_audio_url_format
        CHECK (audio_url ~ '^(https://|/[^/])' AND audio_url !~ '\s' AND char_length(audio_url) <= 500)
);

CREATE INDEX radio_mixes_created_at_idx ON radio_mixes (created_at DESC, id DESC);
