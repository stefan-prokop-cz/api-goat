[![Build Status](https://travis-ci.com/stefan-prokop-cz/api-goat.svg?branch=master)](https://travis-ci.com/stefan-prokop-cz/api-goat)

# API Goat

- Simple REST API with vulnerabilities written in Rust

:construction: Under development :construction:

## How to run

### Use `docker-compose`

* Server port is exposed on `3000`
* Postgres port is exposed on `5432`

```
docker-compose up
```

### Use Postgres service

* Postgres port is exposed on `5432`

```
docker-compose up -d postgres
```

### Run manually

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install [Postgres](https://www.postgresql.org/download/)
3. Clone this [repository](https://github.com/stefan-prokop-cz/api-goat)
4. Install [diesel_cli](https://github.com/diesel-rs/diesel/tree/master/diesel_cli): `cargo install diesel_cli --no-default-features --features postgres`
- If there is a problem with `diesel_cli` compilation, make sure the postgres client is installed
5. Run Postgres on port `5432`
6. Apply migrations `diesel migration run`
7. Run `cargo run --release` or `cargo build --release && ./target/release/api-goat`
8. Server is running on port `3000`
