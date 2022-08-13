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

FROM --platform=$TARGETPLATFORM debian:11.3-slim AS runtime
COPY --from=build /app/target/release/annict-profile-card /
ENTRYPOINT [ "/annict-profile-card" ]
