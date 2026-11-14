-- ANF-005 Erweiterung (Designvorschlag „Techno App“): Kategorie und Marke für den Merch-Shop
ALTER TABLE products
    ADD COLUMN category TEXT NOT NULL DEFAULT 'other',
    ADD COLUMN brand    TEXT NOT NULL DEFAULT '',
    ADD CONSTRAINT products_category_valid CHECK (category IN ('hoodie', 'shirt', 'vinyl', 'accessory', 'other')),
    ADD CONSTRAINT products_brand_length CHECK (char_length(brand) <= 80 AND brand = btrim(brand));

CREATE INDEX products_category_idx ON products (status, category, created_at DESC);
