FROM rust:1.84-alpine3.21 as builder

WORKDIR /usr/src/discord_bot

COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y \