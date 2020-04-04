FROM alpine:latest AS builder

EXPOSE 8080
RUN apk update && apk add make rust

WORKDIR /app
COPY heckit.rs Makefile /app/

RUN make static

RUN cp ./heckit /usr/local/bin/heckit

FROM scratch
COPY --from=builder /usr/local/bin/heckit /
CMD ["/heckit"]
