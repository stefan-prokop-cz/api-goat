[![Build Status](https://travis-ci.com/stefan-prokop-cz/api-goat.svg?branch=master)](https://travis-ci.com/stefan-prokop-cz/api-goat)

# API Goat

- Simple REST API with vulnerabilities written in Rust

:construction: Under development :construction:

## How to run

### Use `docker-compose`

* Server port is exposed on `3000`
* MySQL port is exposed on `3306`

```
docker-compose up
```

### Use MySQL service

* MySQL port is exposed on `3306`

```
docker-compose up -d mysql
```

### Run manually

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install [MySQL v5.7](https://dev.mysql.com/doc/mysql-installation-excerpt/5.7/en/)
3. Clone this [repository](https://github.com/stefan-prokop-cz/api-goat)
4. Install [diesel cli](https://github.com/diesel-rs/diesel/tree/master/diesel_cli): `cargo install diesel_cli --no-default-features --features mysql`
5. Run MySQL on port 3306
6. Apply migrations `diesel migration run`
7. Run `cargo run --release` or `cargo build --release && ./target/release/api-goat`
8. Server is running on port `3000`
