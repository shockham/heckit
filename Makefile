all: run

build:
	rustc -O heckit.rs
	strip heckit

static:
	rustc -C target-feature=+crt-static heckit.rs
	ldd heckit

run: build
	./heckit

test:
	rustc --test -o test_heckit heckit.rs
	./test_heckit

docker-build:
	docker build -t shockham/heckit:latest .

docker-run:
	docker run -p 8080:80 -it shockham/heckit:latest

docker: docker-build docker-run
