#FROM rust:latest
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

# Issue : Current docker image size is min 3.49GB and max 20.64GB very high
#######################################################################
# Current size of image :
# a1.0.0 : size : 3.49GB  | ver.bullsys
# a1.0.1 : size : 11.33GB, build time : 11m 45s | ver.slim
# a1.0.2 : size : 13.95GB, build time : 14m 45s | ver.slim
# a1.0.3 : size : 20.64GB, build time : 23m 30s | ver.bullsys
# a1.0.4 : size : 4.07GB,  build time : 7m 03s  | ver.slim
# a1.0.5 : size : 4.07GB,  build time : 5m 40s  | ver.slim
#######################################################################