FROM lukemathwalker/cargo-chef:0.1.38-rust-1.62.0-bullseye@sha256:03dc114bb0e3e8a114105d3d47fc5ac9f894be3f2eab34ef8a3cd189bcdee1d1 AS chef
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

FROM --platform=$TARGETPLATFORM debian:11.3-slim@sha256:f6957458017ec31c4e325a76f39d6323c4c21b0e31572efa006baa927a160891 AS runtime
COPY --from=build /app/target/release/annict-profile-card /
ENTRYPOINT [ "/annict-profile-card" ]
