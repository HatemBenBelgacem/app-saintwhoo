-- ANF-005: Shop – Produkte
-- Die Regeln aus docs/features/ANF-005 werden zusätzlich in der DB abgesichert.
CREATE TABLE products (
    id                BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name              TEXT        NOT NULL,
    sku               TEXT        NOT NULL,
    short_description TEXT        NOT NULL DEFAULT '',
    description       TEXT        NOT NULL DEFAULT '',
    -- Preis in Rappen (CHF × 100), damit nicht gerundet werden muss
    price_cents       INTEGER     NOT NULL,
    stock             INTEGER     NOT NULL DEFAULT 0,
    status            TEXT        NOT NULL DEFAULT 'draft',
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT products_sku_unique UNIQUE (sku),
    CONSTRAINT products_name_length CHECK (char_length(name) BETWEEN 1 AND 120),
    CONSTRAINT products_name_trimmed CHECK (name = btrim(name)),
    CONSTRAINT products_sku_format CHECK (sku ~ '^[A-Z0-9-]{1,40}$'),
    CONSTRAINT products_short_description_length CHECK (char_length(short_description) <= 200),
    CONSTRAINT products_price_range CHECK (price_cents BETWEEN 0 AND 9999999),
    CONSTRAINT products_stock_not_negative CHECK (stock >= 0),
    CONSTRAINT products_status_valid CHECK (status IN ('draft', 'active'))
);

-- Verwaltung: neueste zuerst
CREATE INDEX products_created_at_idx ON products (created_at DESC, id DESC);
-- Shop: nur aktive Produkte
CREATE INDEX products_active_idx ON products (status, created_at DESC);
