# syntax=docker/dockerfile:1
# Saintwhoo – App-Image (ANF-010).
# Stufe 1 baut Server + WASM + CSS mit cargo-leptos, Stufe 2 enthält nur das Ergebnis.
#
#   docker compose up --build        # App + PostgreSQL (siehe docker-compose.yml)
#   docker build -t saintwhoo-app .  # nur das Image

# ---------------------------------------------------------------------------
# Stufe 1: Build
# ---------------------------------------------------------------------------
# trixie statt bookworm: die fertigen wasm-bindgen-Programme brauchen glibc ≥ 2.38
FROM rust:1.96-trixie AS builder

# Node.js für Tailwind/DaisyUI (npm ci) – Version wie lokal
COPY --from=node:24-trixie-slim /usr/local/bin/node /usr/local/bin/node
COPY --from=node:24-trixie-slim /usr/local/lib/node_modules /usr/local/lib/node_modules
RUN ln -s /usr/local/lib/node_modules/npm/bin/npm-cli.js /usr/local/bin/npm

RUN rustup target add wasm32-unknown-unknown

# cargo-leptos lädt wasm-bindgen (passend zu Cargo.lock), wasm-opt und Tailwind selbst herunter
ARG CARGO_LEPTOS_VERSION=0.3.10
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install cargo-leptos --version ${CARGO_LEPTOS_VERSION} --locked

WORKDIR /app

# npm-Abhängigkeiten zuerst (eigene Schicht, ändert sich selten)
COPY package.json package-lock.json ./
RUN npm ci --no-audit --no-fund

COPY . .

# Cache-Mounts für Registry und target/: ein geänderter Quelltext baut nicht alles neu.
# Das Ergebnis wird aus dem Cache nach /out kopiert, weil Cache-Mounts nicht im Image landen.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/root/.cache/cargo-leptos \
    cargo leptos build --release \
    && mkdir -p /out \
    && cp target/release/app-saintwhoo /out/app-saintwhoo \
    && strip /out/app-saintwhoo \
    && cp -r target/site /out/site

# ---------------------------------------------------------------------------
# Stufe 2: Laufzeit
# ---------------------------------------------------------------------------
FROM debian:trixie-slim AS runtime

# ca-certificates für TLS zur DB. Kein curl: den Healthcheck macht das Binary selbst.
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 999 --no-create-home --shell /usr/sbin/nologin saintwhoo

WORKDIR /app
COPY --from=builder --chown=root:root /out/app-saintwhoo /app/app-saintwhoo
COPY --from=builder --chown=root:root /out/site /app/site

# Leptos-Konfiguration (Cargo.toml liegt nicht im Image)
ENV LEPTOS_OUTPUT_NAME=app-saintwhoo \
    LEPTOS_SITE_ROOT=site \
    LEPTOS_SITE_PKG_DIR=pkg \
    LEPTOS_SITE_ADDR=0.0.0.0:3000 \
    LEPTOS_RELOAD_PORT=3001 \
    LEPTOS_ENV=PROD \
    RUST_LOG=info

USER saintwhoo
EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD ["/app/app-saintwhoo", "healthcheck"]

ENTRYPOINT ["/app/app-saintwhoo"]
