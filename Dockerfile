FROM rust as build
WORKDIR /usr/src

RUN USER=root cargo new api
WORKDIR /usr/src/api
RUN CARGO_INSTALL_ROOT=. cargo install diesel_cli --no-default-features --features mysql

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm -rf target/release/*

RUN rm src/*.rs
COPY . .

RUN cargo build --release

FROM ubuntu

RUN apt-get update
RUN apt-get install -y libmariadbclient-dev

COPY --from=build /usr/src/api/target/release/api-goat .
COPY --from=build /usr/src/api/.env .
COPY --from=build /usr/src/api/migrations ./migrations
COPY --from=build /usr/src/api/docker-compose-entrypoint.sh .
COPY --from=build /usr/src/api/bin/diesel .
RUN chmod u+x docker-compose-entrypoint.sh

EXPOSE 3000
CMD ["./api-goat"]
