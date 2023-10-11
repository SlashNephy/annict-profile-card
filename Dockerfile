FROM lukemathwalker/cargo-chef:0.1.38-rust-1.62.0-bullseye AS chef
WORKDIR /app

FROM chef AS recipe
COPY ./ /app/
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
# Build dependencies
COPY --from=recipe /app/recipe.json ./
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY ./ /app/
RUN cargo build --release --offline --workspace --verbose \
    && strip target/release/annict-profile-card

FROM --platform=$TARGETPLATFORM debian:12.2-slim@sha256:b55e2651b71408015f8068dd74e1d04404a8fa607dd2cfe284b4824c11f4d9bd AS runtime
COPY --from=build /app/target/release/annict-profile-card /
ENTRYPOINT [ "/annict-profile-card" ]
