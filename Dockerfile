FROM rust:1.24

RUN apt-get update 
RUN apt-get install -y postgresql
RUN rustup default nightly
# This forces cargo to update the cached registry, plus we sometimes want
# access to diesel when doing admin tasks.
RUN cargo install diesel_cli --no-default-features --features postgres

RUN mkdir /code
WORKDIR /code

# Manually copy across the source code
COPY ./Cargo.toml /code/
COPY ./Cargo.lock /code/
COPY ./Rocket.toml /code/
COPY ./static /code/static
COPY ./migrations /code/migrations
COPY ./templates /code/templates
COPY ./website-cli /code/website-cli
COPY ./src /code/src

RUN pwd && ls -l
# pre-download dependencies
RUN cargo fetch --manifest-path /code/Cargo.toml

RUN cargo build --manifest-path /code/Cargo.toml --all --release
RUN cargo install --path /code --root /usr/local
RUN cargo install --path /code/website-cli --root /usr/local

EXPOSE 5000

ENTRYPOINT [ "website-server" ]
