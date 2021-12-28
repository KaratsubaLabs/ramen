
INSTALL_DIR := /usr/local/bin

.PHONY: dev-build build install uninstall

dev-build:
	cargo build

build:
	cargo build --release

install: target/release/ramen
	cp $< $(INSTALL_DIR)

uninstall:
	rm -f $(INSTALL_DIR)/ramen

