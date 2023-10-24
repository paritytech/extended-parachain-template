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
- `polkadot` (which you can download from [the releases](https://github.com/paritytech/polkadot/releases)).

Note: Downloading polkadot will automatically download also the binaries of `polkadot-prepare-worker`, `polkadot-execute-worker`. Since Polkadot v1.0 all 3 binaries are needed for the node to run as a validator
- `polkadot-parachain` (which you will build from [cumulus](https://github.com/paritytech/cumulus))

Use the following command in the root of this repository to build the node:

```sh
cargo build --release
```

# Usage

Please refer to the [project's `README.md`](https://github.com/paritytech/parachains-integration-tests#how-to-use) for an extensive description of how to write YAML test files and how to execute tests.

In `integration-tests/force_hrmp_open_channels.yml` you can find a test to open a HRMP channel between this parachain and an asset-hub local network.

In `integration-tests/transact.yml` you can find a test to make a transfer from this parachain sovereign account to asset-hub local network sovereign account via a XCM Transact in the Relay Chain.

Run zombienet and wait until both parachains are propertly onboarded (producing blocks):
```
$ zombienet-macos spawn zombienet-config/integration-tests-config.toml -p native
```

Run the tests:
```
$ parachains-integration-tests -m test -t integration-tests/force_hrmp_open_channels.yml
$ parachains-integration-tests -m test -t integration-tests/transact.yml
```