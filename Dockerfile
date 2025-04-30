ARG RUST_VERSION=1.86.0
ARG APP_NAME=slackit

# ---- Stage 1: Build ----
FROM rust:${RUST_VERSION}-alpine AS build

ARG APP_NAME
WORKDIR /app

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
cargo build --locked --release && \
cp ./target/release/$APP_NAME /bin/slackit


# ---- Stage 2: Runtime ----
FROM alpine:3.18 AS final

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage.
COPY --from=build /bin/server /bin/

# will be appended to this entrypoint command.
ENTRYPOINT ["./slackit"]

# Optional: Default command (can be overridden) - useful if entrypoint needs default args
CMD ["--help"]
