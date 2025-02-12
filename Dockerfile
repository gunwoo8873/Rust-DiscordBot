#FROM rust:latest
FROM rust:1.84.1-bullseye AS builder

# Rust using for default version
RUN rustup default nightly

WORKDIR /usr/src/Discord-Bot
COPY . .

VOLUME [ "/usr/src/Discord-Bot/data" ]

RUN cargo install --path .

CMD ["cargo", "run"]
# Note : Current dockerfile is image size of 3.56GB
#        but i think dockerfile sizing minimal size object of 2GB down
# Current size is 3.49GB