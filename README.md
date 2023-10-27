# Rust WC

Just a barebones implementation of the unix `wc` command written in Rust. 
<br />
Uses [clap](https://docs.rs/clap/latest/clap/) as a cli framework.
<br />
<br />
Written for the first entry in these [coding challenges](https://codingchallenges.fyi/challenges/challenge-wc)

### Development
Depends on on [fmt](https://github.com/rust-lang/rustfmt), [clippy](https://github.com/rust-lang/rust-clippy), and [nightly rust](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) (for testing).
<br />
Or, the provided [Dockerfile](./Dockerfile) can be used as a dev environment:
```
docker build -t rwc-dev .
docker run --rm -it -v $PWD:/rwc rwc-dev sh
```
The docker compose file also provides lint, formatting, and build services to use as quick build scripts. 