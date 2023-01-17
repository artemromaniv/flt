set shell := ["bash", "-uc"]

build:
	cargo build --release

install:
	cp target/release/flt /usr/local/bin/

uninstall:
	rm /usr/local/bin/flt