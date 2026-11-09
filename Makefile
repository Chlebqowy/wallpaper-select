build:
	cargo build --release
	mkdir -pv target/universal/
	cp -v target/release/chlebqowy-wallpaper-select target/universal/
install:
	sudo mkdir -pv $(DESTDIR)/usr
	sudo mkdir -pv $(DESTDIR)/usr/bin 
	sudo install -m 0755 target/universal/chlebqowy-wallpaper-select $(DESTDIR)/usr/bin/chlebqowy-wallpaper-select
	sudo mkdir -pv $(DESTDIR)/usr/share
	sudo mkdir -pv $(DESTDIR)/usr/share/chlebqowy-wallpaper-select
	sudo install -m 0644 default.png $(DESTDIR)/usr/share/chlebqowy-wallpaper-select/default.png
	sudo mkdir -pv $(DESTDIR)/usr/share/licenses
	sudo mkdir -pv $(DESTDIR)/usr/share/licenses/chlebqowy-wallpaper-select
	sudo install -m 0644 LICENSE $(DESTDIR)/usr/share/licenses/chlebqowy-wallpaper-select/LICENSE
		
