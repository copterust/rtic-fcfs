## Prepare
```bash
cargo install probe-rs-tools && \
rustup target add thumbv7em-none-eabihf
```

nightly-2024-07-21 is used until [#862](https://github.com/knurling-rs/defmt/issues/862) will be fixed.

## Run
```bash
DEFMT_LOG=trace cargo run --bin fc
```