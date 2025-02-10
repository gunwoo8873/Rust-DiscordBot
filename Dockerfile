FROM rust:latest

RUN rustup default nightly

WORKDIR /usr/src/Discord-Bot
COPY . .

RUN cargo install --path .

CMD ["cargo", "run"]