# ---- Simple build + install as a boot-starting systemd service -------------
BIN_NAME := FWDFF
PKG_NAME := $(shell echo "$(BIN_NAME)" | tr '[:upper:]' '[:lower:]')

DEST_BIN  := /usr/bin/$(BIN_NAME)
DEST_UNIT := /usr/lib/systemd/system/$(PKG_NAME).service

.PHONY: all build install enable status clean help

all: install

## build: compile the binary (release mode)
build:
	cargo build --release
	@BIN_PATH=$$(ls target/release/$(BIN_NAME) target/*/release/$(BIN_NAME) 2>/dev/null | head -n1); \
	if [ -z "$$BIN_PATH" ]; then \
		echo "error: could not locate compiled '$(BIN_NAME)' binary from Cargo output" >&2; exit 1; \
	fi; \
	echo "found binary: $$BIN_PATH"; \
	cp "$$BIN_PATH" "$(DEST_BIN)"

## install: build, install the binary, and register it as a boot service (default)
install: build
	sudo chown root:root $(DEST_BIN)
	sudo chmod 755 $(DEST_BIN)
	# Emit the systemd unit that runs at every boot
	@printf '%s\n' \
	'[Unit]' \
	'Description=Framework Desktop Fan Fix (FWDFF) - reset bugged Chromium EC fan controller' \
	'After=local-fs.target systemd-udevd.service' \
	'' \
	'[Service]' \
	'Type=oneshot' \
	'User=root' \
	'ExecStart=/usr/bin/$(BIN_NAME)' \
	'RemainAfterExit=yes' \
	'Restart=on-failure' \
	'RestartSec=3' \
	'' \
	'[Install]' \
	'WantedBy=multi-user.target' | sudo tee $(DEST_UNIT) > /dev/null
	sudo chown root:root $(DEST_UNIT)
	sudo chmod 644 $(DEST_UNIT)
	systemctl daemon-reload
	echo "installed $(BIN_NAME) -> $(DEST_BIN)"
	echo "installed service -> $(DEST_UNIT)"

## enable: turn the service on and start it now
enable:
	systemctl daemon-reload
	systemctl enable --now $(PKG_NAME)

## status: show service status
status:
	systemctl status $(PKG_NAME) || true

## clean: remove build artifacts (keeps installed system files intact)
clean:
	rm -rf target/release/$(BIN_NAME)

help:
	@echo "Targets:"
	@echo "  make          build + install binary and service"
	@echo "  make build    just compile the binary"
	@echo "  make enable   daemon-reload + enable --now (runs on next boot as root)"
	@echo "  make status   show service status"
	
