PREFIX?=/usr/local

test:
	RUST_BACKTRACE=1 cargo test

run:
	RUST_LOG=warn RUST_BACKTRACE=1 cargo run -- --no-fork

install: install-resources
	cargo install --path . --force --root $(DESTDIR)$(PREFIX)

install-debug: install-resources
	cargo install --debug --path . --force --root $(DESTDIR)$(PREFIX)

install-resources:
	mkdir -p $(DESTDIR)$(PREFIX)/share/nvim-gtk/
	cp -r runtime $(DESTDIR)$(PREFIX)/share/nvim-gtk/
	mkdir -p $(DESTDIR)$(PREFIX)/share/applications/
	sed -e "s|Exec=nvim-gtk|Exec=$(PREFIX)/bin/nvim-gtk|" \
		desktop/org.daa.NeovimGtk.desktop \
		>$(DESTDIR)$(PREFIX)/share/applications/org.daa.NeovimGtk.desktop
	mkdir -p $(DESTDIR)$(PREFIX)/share/icons/hicolor/128x128/apps/
	cp desktop/org.daa.NeovimGtk_128.png $(DESTDIR)$(PREFIX)/share/icons/hicolor/128x128/apps/org.daa.NeovimGtk.png
	mkdir -p $(DESTDIR)$(PREFIX)/share/icons/hicolor/48x48/apps/
	cp desktop/org.daa.NeovimGtk_48.png $(DESTDIR)$(PREFIX)/share/icons/hicolor/48x48/apps/org.daa.NeovimGtk.png
	mkdir -p $(DESTDIR)$(PREFIX)/share/icons/hicolor/scalable/apps/
	cp desktop/org.daa.NeovimGtk.svg $(DESTDIR)$(PREFIX)/share/icons/hicolor/scalable/apps/
	mkdir -p $(DESTDIR)$(PREFIX)/share/icons/hicolor/symbolic/apps/
	cp desktop/org.daa.NeovimGtk-symbolic.svg $(DESTDIR)$(PREFIX)/share/icons/hicolor/symbolic/apps/

.PHONY: all clean test
