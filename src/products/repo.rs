//! Datenbankzugriff für Produkte (nur Server).

use chrono::{DateTime, Utc};
use sqlx::PgPool;

use super::model::{
    CreateOutcome, DeleteOutcome, FieldErrors, PAGE_SIZE, Product, ProductInput, ProductPage,
    ProductStatus, ProductSummary, SHOP_PAGE_SIZE, ShopItem, ShopPage, ShopSort, UpdateOutcome,
    ProductImage, ValidProduct, msg, normalize_sku, total_pages, validate,
};

#[derive(sqlx::FromRow)]
struct ProductRow {
    id: i64,
    name: String,
    sku: String,
    short_description: String,
    description: String,
    price_cents: i32,
    stock: i32,
    status: String,
    image_url: Option<String>,
    image_credit: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<ProductRow> for Product {
    fn from(r: ProductRow) -> Self {
        Product {
            id: r.id,
            name: r.name,
            sku: r.sku,
            short_description: r.short_description,
            description: r.description,
            price_cents: r.price_cents,
            stock: r.stock,
            // Die DB erlaubt per CHECK nur gültige Werte
            status: ProductStatus::parse(&r.status).unwrap_or_default(),
            image: ProductImage::from_columns(r.image_url, r.image_credit),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct SummaryRow {
    id: i64,
    name: String,
    sku: String,
    price_cents: i32,
    stock: i32,
    status: String,
    updated_at: DateTime<Utc>,
}

impl From<SummaryRow> for ProductSummary {
    fn from(r: SummaryRow) -> Self {
        ProductSummary {
            id: r.id,
            name: r.name,
            sku: r.sku,
            price_cents: r.price_cents,
            stock: r.stock,
            status: ProductStatus::parse(&r.status).unwrap_or_default(),
            updated_at: r.updated_at,
        }
    }
}

// ---------- Prüfen ----------

/// Gibt es die Artikelnummer schon (bei einem anderen Produkt als `except`)?
pub async fn sku_taken(pool: &PgPool, sku: &str, except: Option<i64>) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM products WHERE sku = $1 AND ($2::bigint IS NULL OR id <> $2))",
    )
    .bind(sku)
    .bind(except)
    .fetch_one(pool)
    .await
}

/// Alle Regeln inkl. „Artikelnummer schon vergeben“ – alle Fehler auf einmal.
#[allow(clippy::result_large_err)]
async fn check(
    pool: &PgPool,
    input: &ProductInput,
    except: Option<i64>,
) -> Result<Result<ValidProduct, FieldErrors>, sqlx::Error> {
    let result = validate(input);
    let sku_ok = match &result {
        Ok(_) => true,
        Err(e) => e.sku.is_none(),
    };
    if sku_ok && sku_taken(pool, &normalize_sku(&input.sku), except).await? {
        let mut errors = result.err().unwrap_or_default();
        errors.sku = Some(msg::SKU_TAKEN.into());
        return Ok(Err(errors));
    }
    Ok(result)
}

/// Zwei gleichzeitige Anfragen können beide die Prüfung bestehen – dann
/// greift der UNIQUE-Constraint. Diesen Fall als normalen Feldfehler melden.
fn is_sku_conflict(e: &sqlx::Error) -> bool {
    matches!(e, sqlx::Error::Database(db) if db.constraint() == Some("products_sku_unique"))
}

fn sku_taken_errors() -> FieldErrors {
    FieldErrors { sku: Some(msg::SKU_TAKEN.into()), ..Default::default() }
}

// ---------- Create ----------

pub async fn create(pool: &PgPool, input: &ProductInput) -> Result<CreateOutcome, sqlx::Error> {
    let p = match check(pool, input, None).await? {
        Ok(p) => p,
        Err(errors) => return Ok(CreateOutcome::Invalid(errors)),
    };
    let res: Result<i64, sqlx::Error> = sqlx::query_scalar(
        "INSERT INTO products
             (name, sku, short_description, description, price_cents, stock, status, image_url, image_credit)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING id",
    )
    .bind(&p.name)
    .bind(&p.sku)
    .bind(&p.short_description)
    .bind(&p.description)
    .bind(p.price_cents)
    .bind(p.stock)
    .bind(p.status.as_str())
    .bind(p.image.as_ref().map(|i| &i.url))
    .bind(p.image.as_ref().map(|i| &i.credit))
    .fetch_one(pool)
    .await;
    match res {
        Ok(id) => Ok(CreateOutcome::Created { id }),
        Err(e) if is_sku_conflict(&e) => Ok(CreateOutcome::Invalid(sku_taken_errors())),
        Err(e) => Err(e),
    }
}

// ---------- Read (Verwaltung) ----------

pub async fn find(pool: &PgPool, id: i64) -> Result<Option<Product>, sqlx::Error> {
    let row: Option<ProductRow> = sqlx::query_as(
        "SELECT id, name, sku, short_description, description, price_cents, stock, status,
                image_url, image_credit, created_at, updated_at
         FROM products WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(Into::into))
}

/// Eine Seite der Verwaltungsliste, neueste zuerst. Eine zu große
/// Seitenzahl wird auf die letzte Seite begrenzt.
pub async fn list(
    pool: &PgPool,
    page: u32,
    status: Option<ProductStatus>,
) -> Result<ProductPage, sqlx::Error> {
    let status = status.map(ProductStatus::as_str);

    let total: i64 =
        sqlx::query_scalar("SELECT count(*) FROM products WHERE ($1::text IS NULL OR status = $1)")
            .bind(status)
            .fetch_one(pool)
            .await?;

    let total_pages = total_pages(total, PAGE_SIZE);
    let page = page.clamp(1, total_pages);
    let offset = i64::from(page - 1) * PAGE_SIZE;

    let rows: Vec<SummaryRow> = sqlx::query_as(
        "SELECT id, name, sku, price_cents, stock, status, updated_at
         FROM products
         WHERE ($1::text IS NULL OR status = $1)
         ORDER BY created_at DESC, id DESC
         LIMIT $2 OFFSET $3",
    )
    .bind(status)
    .bind(PAGE_SIZE)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(ProductPage { items: rows.into_iter().map(Into::into).collect(), page, total_pages, total })
}

// ---------- Update ----------

/// Prüft und speichert Änderungen. `NotFound`, wenn es das Produkt nicht
/// (mehr) gibt – dann wird nichts angelegt.
pub async fn update(pool: &PgPool, id: i64, input: &ProductInput) -> Result<UpdateOutcome, sqlx::Error> {
    let p = match check(pool, input, Some(id)).await? {
        Ok(p) => p,
        Err(errors) => return Ok(UpdateOutcome::Invalid(errors)),
    };
    let res: Result<Option<i64>, sqlx::Error> = sqlx::query_scalar(
        "UPDATE products
         SET name = $1, sku = $2, short_description = $3, description = $4,
             price_cents = $5, stock = $6, status = $7,
             image_url = $8, image_credit = $9, updated_at = now()
         WHERE id = $10
         RETURNING id",
    )
    .bind(&p.name)
    .bind(&p.sku)
    .bind(&p.short_description)
    .bind(&p.description)
    .bind(p.price_cents)
    .bind(p.stock)
    .bind(p.status.as_str())
    .bind(p.image.as_ref().map(|i| &i.url))
    .bind(p.image.as_ref().map(|i| &i.credit))
    .bind(id)
    .fetch_optional(pool)
    .await;
    match res {
        Ok(Some(id)) => Ok(UpdateOutcome::Updated { id }),
        Ok(None) => Ok(UpdateOutcome::NotFound),
        Err(e) if is_sku_conflict(&e) => Ok(UpdateOutcome::Invalid(sku_taken_errors())),
        Err(e) => Err(e),
    }
}

// ---------- Delete ----------

/// Löscht endgültig (Hard Delete).
pub async fn delete(pool: &PgPool, id: i64) -> Result<DeleteOutcome, sqlx::Error> {
    let res = sqlx::query("DELETE FROM products WHERE id = $1").bind(id).execute(pool).await?;
    Ok(if res.rows_affected() == 0 { DeleteOutcome::NotFound } else { DeleteOutcome::Deleted })
}

// ---------- Shop (öffentlich) ----------

/// Eine Seite des Shops – nur aktive Produkte. Die Sortierung wird als
/// Parameter übergeben (sqlx erlaubt nur feste SQL-Strings).
pub async fn shop(pool: &PgPool, page: u32, sort: ShopSort) -> Result<ShopPage, sqlx::Error> {
    let total: i64 = sqlx::query_scalar("SELECT count(*) FROM products WHERE status = 'active'")
        .fetch_one(pool)
        .await?;

    let total_pages = total_pages(total, SHOP_PAGE_SIZE);
    let page = page.clamp(1, total_pages);
    let offset = i64::from(page - 1) * SHOP_PAGE_SIZE;

    #[derive(sqlx::FromRow)]
    struct Row {
        id: i64,
        name: String,
        short_description: String,
        price_cents: i32,
        stock: i32,
        image_url: Option<String>,
        image_credit: Option<String>,
    }
    let rows: Vec<Row> = sqlx::query_as(
        "SELECT id, name, short_description, price_cents, stock, image_url, image_credit
         FROM products
         WHERE status = 'active'
         ORDER BY
             CASE WHEN $1 = 'preis-auf' THEN price_cents END ASC,
             CASE WHEN $1 = 'preis-ab'  THEN price_cents END DESC,
             CASE WHEN $1 = 'name'      THEN lower(name) END ASC,
             created_at DESC, id DESC
         LIMIT $2 OFFSET $3",
    )
    .bind(sort.as_str())
    .bind(SHOP_PAGE_SIZE)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(ShopPage {
        items: rows
            .into_iter()
            .map(|r| ShopItem {
                id: r.id,
                name: r.name,
                short_description: r.short_description,
                price_cents: r.price_cents,
                stock: r.stock,
                image: ProductImage::from_columns(r.image_url, r.image_credit),
            })
            .collect(),
        page,
        total_pages,
        total,
    })
}

/// Ein Produkt für die öffentliche Produktseite – `None` für Entwürfe und
/// unbekannte IDs (→ 404).
pub async fn find_active(pool: &PgPool, id: i64) -> Result<Option<Product>, sqlx::Error> {
    let row: Option<ProductRow> = sqlx::query_as(
        "SELECT id, name, sku, short_description, description, price_cents, stock, status,
                image_url, image_credit, created_at, updated_at
         FROM products WHERE id = $1 AND status = 'active'",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(Into::into))
}
