
INSTALL_DIR := /usr/local/bin

.PHONY: debug build devsetup install uninstall

debug:
	cargo run

build:
	cargo build --release

devsetup:
	cp dev/hooks/* .git/hooks

install: target/release/ramen
	cp $< $(INSTALL_DIR)

uninstall:
	rm -f $(INSTALL_DIR)/ramen

