FROM ekidd/rust-musl-builder:stable AS builder

COPY . ./
RUN sudo chown rust:rust -R .

RUN cargo build --release \
    && strip /home/rust/src/target/x86_64-unknown-linux-musl/release/annict-profile-card

FROM scratch

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/annict-profile-card /

ENTRYPOINT ["/annict-profile-card"]
