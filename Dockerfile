# Rust
FROM rust:latest as build

# Install dependencies
RUN apt-get -qq update

RUN apt-get install -y -q \
    clang \
    llvm-dev \
    libclang-dev \
    cmake \
    openssl

RUN cargo install diesel_cli --no-default-features --features postgres

# Set default user
RUN USER=root cargo new --bin health_rules_engine
WORKDIR /health_rules_engine

# Copy over manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Copy over migrations
COPY ./migrations ./migrations
COPY ./templates ./templates

# This build to cache dependencies
RUN cargo build --release
RUN rm src/*.rs 

# Copy source tree
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/health_rules_engine*
RUN cargo build --release

# Final base
FROM rust:latest

# Copy final build artifact
COPY --from=build /health_rules_engine/target/release/health_rules_engine .

EXPOSE 8080

# Set startup command

CMD ["./health_rules_engine"]