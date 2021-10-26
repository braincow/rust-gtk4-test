.PHONY: clean clean-all install uninstall

target/release/numberui: src
	SHAREDIR=/usr/share/numberui cargo build --release --bin numberui

install: target/release/numberui
	# Install the binary
	cp target/release/numberui /usr/bin/numberui
	cp numberui.desktop /usr/share/applications/numberui.desktop
	cp numberui.svg /usr/share/icons/hicolor/scalable/apps/numberui.svg
	mkdir -p /usr/share/numberui
	cp assets/number.ui /usr/share/numberui/number.ui

uninstall:
	rm -f /usr/bin/numberui
	rm -f /usr/share/applications/numberui.desktop
	rm -f /usr/share/icons/hicolor/scalable/apps/numberui.svg
	rm -f /usr/share/numberui/number.ui
	rmdir /usr/share/numberui/

clean-all:
	cargo clean
