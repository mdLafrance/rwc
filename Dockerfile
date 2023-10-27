### Stable environment to run any linting, building and tests

FROM rust:alpine3.18

# Nightly build required for mocktopus testing library
RUN rustup override set nightly

# Adds some required headers and libs
RUN apk add musl-dev
RUN apk add libressl-dev

# Add clippy for code linting
RUN rustup update
RUN rustup component add clippy

# Add fmt for format linting
RUN rustup component add rustfmt --toolchain nightly-x86_64-unknown-linux-musl

# Adds llvm-cov for code coverage
RUN rustup component add llvm-tools-preview --toolchain nightly-x86_64-unknown-linux-musl
RUN cargo install cargo-llvm-cov --locked

WORKDIR /rwc

COPY . .

CMD [ "sh" ]
