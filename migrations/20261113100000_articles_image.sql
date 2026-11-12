-- ANF-004: Artikelbild mit Bildnachweis
ALTER TABLE articles
    ADD COLUMN image_url    TEXT,
    ADD COLUMN image_credit TEXT,
    ADD CONSTRAINT articles_image_url_format
        CHECK (image_url IS NULL OR (image_url ~ '^(https://|/[^/])' AND image_url !~ '\s' AND char_length(image_url) <= 500)),
    ADD CONSTRAINT articles_image_credit_length
        CHECK (image_credit IS NULL OR char_length(image_credit) <= 200),
    ADD CONSTRAINT articles_image_needs_credit
        CHECK (image_url IS NULL OR (image_credit IS NOT NULL AND btrim(image_credit) <> '')),
    ADD CONSTRAINT articles_credit_needs_image
        CHECK (image_credit IS NULL OR image_url IS NOT NULL);
