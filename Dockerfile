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

CMD [ "sh" ]
