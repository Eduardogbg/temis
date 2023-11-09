# temis

temis is a cli utility for development on NEAR. It leverages official NEAR libraries such as `near-workspaces`, `cargo-near` and `near-abi-client-rs` to provide an unified development experience.

## Installation
```
cargo install cargo-near
cargo install temis
```

## Usage

### Generate ABI for contracts

Specify the path of the contract crate and the output directory where you wish the ABI JSON file to be stored
```
temis abi path/to/crate output/path
```

### Generate `near-workspaces` client for contracts

Based on the ABI of the contract, it uses (a patch of) `near-abi-client-rs` to generate a struct with a similar interface as that of the contract, for emitting RPC calls. It is specially useful for integration te


```
cargo run abi crate_path output_path
cargo run generate crate_path output_path
cargo run sandbox config_path
```

