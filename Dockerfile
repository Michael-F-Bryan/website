FROM rust:1.20

EXPOSE 8000

RUN rustup default nightly
RUN cargo install diesel_cli

# COPY . /code
WORKDIR /code

RUN cargo build

CMD cargo run