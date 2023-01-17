set shell := ["bash", "-uc"]

build:
	cargo build --release

install:
	cp target/release/flt /usr/local/bin/

uninstall:
	rm /usr/local/bin/flt

install-win:
	cp target/release/flt.exe C:\Windows\System32\

uninstall-win:
	del C:\Windows\System32\flt.exe