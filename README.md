# Dev

## Watch

```shell
cargo watch -x check -x test -x run
```


# CI Pipeline

## Test coverage

### Install
```shell
cargo install cargo-tarpaulin
```

### Run 
`cargo tarpaulin --ignore-tests`


## Linter

`cargo clippy -- -D warnings`

## Formatter

`cargo fmt -- --check`

## Security Vulnerabilities

### Install
`cargo install cargo-audit`

### Run 
`cargo audit`


# DB Migration

## Install sqlx-cli
`cargo install --version="~0.8" sqlx-cli --no-default-features --features rustls,postgres`

