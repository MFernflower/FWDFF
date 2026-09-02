#!/bin/bash
set -euo pipefail

BIN_NAME="FWDFF"
DEST_BIN="/usr/bin/${BIN_NAME}"
CRON_FILE="/etc/cron.d/fwdff"
LOG_FILE="/var/log/fwdff.log"

# 1. Build
echo "==> Building (release)..."
cargo build --release

# 2. Locate the compiled binary (single binary -> deterministic path)
BIN_PATH="target/release/${BIN_NAME}"
if [ ! -x "${BIN_PATH}" ]; then
    echo "error: could not find compiled '${BIN_NAME}' at ${BIN_PATH}" >&2
    exit 1
fi
echo "    found binary: ${BIN_PATH}"

# 3. Install binary
echo "==> Installing binary to ${DEST_BIN}..."
sudo install -m 0755 -o root -g root "${BIN_PATH}" "${DEST_BIN}"

# 4. Install cron job (runs once at boot as root)
echo "==> Installing cron job to ${CRON_FILE}..."
sudo tee "${CRON_FILE}" > /dev/null <<EOF
# FWDFF - Framework Desktop Fan Fix (runs once at boot as root)
@reboot root /usr/bin/${BIN_NAME} >> ${LOG_FILE} 2>&1
EOF

echo "==> Done! Please Reboot System!"
echo "    binary: ${DEST_BIN}"
echo "    cron:   ${CRON_FILE}"
echo "    log:    ${LOG_FILE}"
