all: run

build:
	rustc heckit.rs

run: build
	./heckit
