FROM ekidd/rust-musl-build:1.57.0 AS build

ENV DEBIAN_FRONTEND noninteractive
RUN sudo apt-get update \
    && sudo apt-get install -y ca-certificates

COPY --chown=rust:rust . ./
RUN cargo build --release \
    && strip /home/rust/src/target/x86_64-unknown-linux-musl/release/annict-profile-card

FROM scratch
ENV SSL_CERT_FILE /etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR /etc/ssl/certs

COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/annict-profile-card /

ENTRYPOINT ["/annict-profile-card"]
