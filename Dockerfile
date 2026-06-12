FROM rust:1.75 as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/smartfo /usr/local/bin/
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD smartfo health check || exit 1
CMD ["smartfo", "--help"]
