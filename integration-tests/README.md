[`parachains-integration-tests`](https://github.com/paritytech/parachains-integration-tests) is a tool designed to test interactions between Substrate based blockchains.

Is used here to develop tests rapidly describing them in a YAML file.

# Setup

Install `parachains-integration-tests` into your system:
```
$ yarn global add ts-node

$ yarn global add @parity/parachains-integration-tests
```

Review the [Run a local network](https://github.com/paritytech/extended-parachain-template#%EF%B8%8F-run-a-local-network) to set up Zombienet in your system.

Create a `bin` directory into the root of this repository and place the following binaries inside of it:
- `polkadot` (which you can download from [the releases](https://github.com/paritytech/polkadot/releases))
- `polkadot-parachain` (which you will build from [cumulus](https://github.com/paritytech/cumulus))

Use the following command in the root of this repository to build the node:

```sh
cargo build --release
```

# Usage

Please refer to the [project's `README.md`](https://github.com/paritytech/parachains-integration-tests#how-to-use) for an extensive description of how to write YAML test files and how to execute tests.


