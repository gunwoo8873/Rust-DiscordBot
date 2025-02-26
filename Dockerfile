#FROM rust:latest
FROM rust:1.84.1-slim AS builder

# Rust using for default version
RUN rustup default nightly

WORKDIR /usr/src/Discord-Bot
COPY . .

#### Volume commented
# VOLUME [ "/usr/src/Discord-Bot/data" ]

RUN cargo install --path .
RUN cargo build --release

CMD ["cargo", "run", "--release"]

# Issue : Current docker image size is min 3.49GB and max 20.64GB very high

#######################################################################
# Current size of image :
# a1.0.0 : size : 3.49GB  | ver.bullsys
# a1.0.2 : size : 13.95GB, build time : 14m 45s | ver.slim
# a1.0.3 : size : 20.64GB, build time : 23m 30s | ver.bullsys
#######################################################################