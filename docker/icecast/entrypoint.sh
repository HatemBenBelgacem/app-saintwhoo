#!/bin/sh
# Erzeugt /etc/icecast/icecast.xml aus der Vorlage und startet Icecast.
set -eu

fail() {
    echo "Icecast: $1" >&2
    exit 1
}

for var in ICECAST_SOURCE_PASSWORD ICECAST_ADMIN_PASSWORD; do
    eval "value=\${$var:-}"
    [ -n "$value" ] || fail "$var ist nicht gesetzt"
    case "$value" in
        hackme | bitte-aendern) fail "$var: bitte ein eigenes Passwort setzen" ;;
    esac
done

: "${ICECAST_HOSTNAME:=localhost}"
: "${ICECAST_LOCATION:=Zürich}"
: "${ICECAST_ADMIN_EMAIL:=radio@localhost}"
: "${ICECAST_MAX_CLIENTS:=100}"
: "${ICECAST_MOUNT:=/live}"

case "$ICECAST_MOUNT" in
    /*) ;;
    *) fail "ICECAST_MOUNT muss mit / beginnen (z. B. /live)" ;;
esac
case "$ICECAST_MAX_CLIENTS" in
    '' | *[!0-9]*) fail "ICECAST_MAX_CLIENTS muss eine Zahl sein" ;;
esac

export ICECAST_SOURCE_PASSWORD ICECAST_ADMIN_PASSWORD ICECAST_HOSTNAME \
    ICECAST_LOCATION ICECAST_ADMIN_EMAIL ICECAST_MAX_CLIENTS ICECAST_MOUNT

# Platzhalter wörtlich ersetzen (kein Regex) und Werte für XML maskieren,
# damit Passwörter mit & < > " funktionieren.
awk '
function lit(s, key, val,   out, i) {
    out = ""
    while ((i = index(s, key)) > 0) {
        out = out substr(s, 1, i - 1) val
        s = substr(s, i + length(key))
    }
    return out s
}
function xml(v) {
    v = lit(v, "&", "&amp;")
    v = lit(v, "<", "&lt;")
    v = lit(v, ">", "&gt;")
    return lit(v, "\"", "&quot;")
}
BEGIN {
    n = split("ICECAST_SOURCE_PASSWORD ICECAST_ADMIN_PASSWORD ICECAST_HOSTNAME ICECAST_LOCATION ICECAST_ADMIN_EMAIL ICECAST_MAX_CLIENTS ICECAST_MOUNT", names, " ")
}
{
    line = $0
    for (k = 1; k <= n; k++) line = lit(line, "@" names[k] "@", xml(ENVIRON[names[k]]))
    print line
}
' /etc/icecast/icecast.xml.template > /etc/icecast/icecast.xml

echo "Icecast: Stream unter http://${ICECAST_HOSTNAME}:8000${ICECAST_MOUNT}"
exec icecast -c /etc/icecast/icecast.xml
