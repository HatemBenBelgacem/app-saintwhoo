# Icecast-Streaming-Server für den Livestream von Saintwhoo Radio (ANF-007).
# Passwörter kommen zur Laufzeit aus Umgebungsvariablen, nie ins Image.
FROM alpine:3.21

# Das Paket legt den Benutzer "icecast" (ohne Root-Rechte) selbst an.
RUN apk add --no-cache icecast \
    && mkdir -p /etc/icecast /var/log/icecast \
    # Logs auf stdout/stderr, damit `docker compose logs icecast` sie zeigt
    && ln -sf /dev/stdout /var/log/icecast/access.log \
    && ln -sf /dev/stderr /var/log/icecast/error.log \
    && chown -R icecast /etc/icecast /var/log/icecast

COPY icecast.xml.template /etc/icecast/icecast.xml.template
COPY --chmod=755 entrypoint.sh /usr/local/bin/entrypoint.sh

USER icecast
EXPOSE 8000

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD wget -q -O /dev/null http://127.0.0.1:8000/status-json.xsl || exit 1

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
