# temis

temis is a cli utility for development on NEAR. It leverages official NEAR libraries such as `near-workspaces`, `cargo-near` and `near-abi-client-rs` to provide an unified development experience.

## Forks
This is a NEARCON 2023 hackathon project, and as such, a cli utility was decided for the deliverable format. However I do think the efforts that went into this should go back to the original libraries we leverage.

Currently some NEAR ABI libraries are not actively maintained, and for this project we patched them so they could work. We do intend on putting more effort into the contributions so they can be in a state that could be merged back into their original repositories.

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

Based on the ABI of the contract, it uses (a patch of) `near-abi-client-rs` to generate a struct with a similar interface as that of the contract, for emitting RPC calls. It is specially useful for integration test.

```
temis generate path/to/crate output/path
```

### Host a local Sandbox with your contracts deployed

By writing a single configuration file (see [sample-config.json](sample-config.json)) specifying which crates you which to build, you can use `temis` to run a local Sandbox with your contracts already deployed. Useful for iterating and testing via RPC calls
```
temis sandbox path/to/config.json
```

