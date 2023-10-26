### Stable environment to run any linting, building and tests

FROM rust:alpine3.18

# Nightly build required for mocktopus testing library
RUN rustup override set nightly

# Adds some headers that are required
RUN apk add musl-dev

# Add clippy
RUN rustup update
RUN rustup component add clippy
RUN rustup component add rustfmt --toolchain nightly-x86_64-unknown-linux-musl

WORKDIR /rwc

# Copy in requirements and install
COPY Cargo.lock .
COPY Cargo.toml .

# silly limitation of cargo currently - there needs to be a lib or bin entrypoint for install to run.
# sidestep this by creating a dummy placeholder bin file.
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

RUN cargo install --path .

# Copy all other source files
COPY . .
