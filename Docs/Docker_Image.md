# Dockerfile image build

```Dockerfile
FROM rust:1.58.1-slim AS builder

WORKDIR /app
COPY . .

ENV PROFILE=release

RUN cargo install --path .
RUN cargo build --{{profile}}
RUN cargo run --{{profile}}

CMD ["./target/{{profile}}/app"]
```

* FROM
* WORKDIR
* COPY
* ENV
* RUN
* CMD