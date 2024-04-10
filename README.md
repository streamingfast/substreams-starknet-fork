# Substreams Starknet

Rust library for Starknet [substreams](https://substreams.streamingfast.io/) development.

[![crates-badge](https://img.shields.io/crates/v/substreams-starknet.svg)](https://crates.io/crates/substreams-starknet)

## Regenerate Protobuf Rust code

Whenever the [Protobuf types definition](./proto/zklend.starknet.type.v1.proto) changes, run the following command to update the generated Rust code:

```console
cargo codegen
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
