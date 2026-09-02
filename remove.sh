#!/bin/bash
set -euo pipefail

BIN_NAME="FWDFF"
DEST_BIN="/usr/bin/${BIN_NAME}"
CRON_FILE="/etc/cron.d/fwdff"
LOG_FILE="/var/log/fwdff.log"

# Safety: only root can uninstall system-wide installed files
if [ "$(id -u)" -ne 0 ]; then
    echo "error: this uninstaller must be run as root (try: sudo uninstaller.sh)" >&2
    exit 1
fi

confirm() {
    read -r -p "$1 [y/N] " response
    case "$response" in
        [yY][eE][sS]|[yY]) return 0 ;;
        *) return 1 ;;
    esac
}

if ! confirm "Uninstall ${BIN_NAME} (remove binary, cron job, and log)?"; then
    echo "Aborted."
    exit 0
fi

# 1. Remove cron job (runs at boot as root) - do this first so the binary
#    can't be re-launched by cron while we're removing things.
if [ -e "${CRON_FILE}" ]; then
    echo "==> Removing cron job ${CRON_FILE}..."
    sudo rm -f "${CRON_FILE}"
else
    echo "==> cron job ${CRON_FILE} not present, skipping."
fi

# 2. Remove the installed binary
if [ -e "${DEST_BIN}" ]; then
    echo "==> Removing binary ${DEST_BIN}..."
    sudo rm -f "${DEST_BIN}"
else
    echo "==> binary ${DEST_BIN} not present, skipping."
fi

# 3. Remove the log file
if [ -e "${LOG_FILE}" ]; then
    echo "==> Removing log file ${LOG_FILE}..."
    sudo rm -f "${LOG_FILE}"
else
    echo "==> log file ${LOG_FILE} not present, skipping."
fi

# 4. Reload systemd's cron manager if present so removal takes effect immediately
if command -v systemctl >/dev/null 2>&1 && systemctl list-units --type=service >/dev/null 2>&1; then
    systemctl try-restart cron >/dev/null 2>&1 || systemctl try-restart crond >/dev/null 2>&1 || true
fi

echo "==> Done! ${BIN_NAME} has been uninstalled."