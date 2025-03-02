# Dockerfile image build
```Dockerfile
FROM rust:1.84.1-slim AS builder

#### Rust toolchain env variables
ENV RUST_TOOLCHAIN="nightly"
ENV RUST_TOOLCHAIN_UPDATE="rustup update"

#### Rust using for default version
RUN rustup default ${RUST_TOOLCHAIN}
RUN ${RUST_TOOLCHAIN_UPDATE}

#### Default working directory
WORKDIR /usr/src/Discord-Bot
COPY . .

#### Volume commented
# VOLUME [ "/usr/src/Discord-Bot/data" ]

#### Project build env variables
ENV CARGO_BUILD_PROFILE_RELEASE="--release"

#### Application build script line
RUN cargo install --path .
RUN cargo build ${CARGO_BUILD_PROFILE_RELEASE}

#### Application run command line
CMD ["cargo", "run", "--release"]
```

> [!NOTE]
> **Dockerfile base image to bullsys and latest version is very big and very hearvy size, then im used base image to slim from the rust 1.84.1 latest version. Discord Bot is use library for rust toolchain need to nightly version.**