-- ANF-008: Rubrik pro Artikel (News-Filter aus dem Designvorschlag „Techno App“)
ALTER TABLE articles
    ADD COLUMN category TEXT NOT NULL DEFAULT 'news',
    ADD CONSTRAINT articles_category_valid CHECK (category IN ('news', 'interview', 'release', 'event', 'mix'));

CREATE INDEX articles_category_idx ON articles (category, published_on DESC);
