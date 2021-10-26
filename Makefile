.PHONY: clean clean-all install uninstall

target/release/numberui: src
	cargo build --release --bin numberui

install: target/release/numberui
	# Install the binary
	cp target/release/numberui /usr/bin/numberui
	cp numberui.desktop /usr/share/applications/numberui.desktop
	cp numberui.svg /usr/share/icons/hicolor/scalable/apps/numberui.svg

uninstall:
	rm -f /usr/bin/numberui
	rm -f /usr/share/applications/numberui.desktop
	rm -f /usr/share/icons/hicolor/scalable/apps/numberui.svg

clean-all:
	cargo clean
