all: run

build:
	rustc heckit.rs

run: build
	./heckit

test:
	rustc --test -o test_heckit heckit.rs
	./test_heckit
