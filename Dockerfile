FROM alpine:edge AS builder

WORKDIR /app
COPY heckit.rs Makefile /app/

RUN apk update && apk add make rust
RUN make static

FROM scratch
COPY --from=builder /app/heckit /heckit
CMD ["/heckit"]
