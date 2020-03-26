all: run

build:
	rustc --target x86_64-unknown-linux-musl heckit.rs

run: build
	./heckit

test:
	rustc --test -o test_heckit heckit.rs
	./test_heckit

docker:
	docker build -t shockham/heckit:latest .
