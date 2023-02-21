# Frontier Parachain Template 

A [Frontier](https://github.com/paritytech/frontier/) + [Cumulus](https://github.com/paritytech/cumulus/)-based Substrate node, ready for hacking ☁️..

### Description

This project is originally a fork of the [Substrate Parachain Template](https://github.com/substrate-developer-hub/substrate-parachain-template) which in turn is a fork of 
[Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template), modified to include dependencies required for registering this node as a **parathread** or **parachain**.

This repository also comes with Ethereum compatibility layer built using [Frontier](https://github.com/paritytech/frontier) and [Debug-Trace API from Moonbeam](https://github.com/PureStake/moonbeam/). 

## 🚀 Getting Started

### Rust Setup

Make sure you have Rust installed along with everything that's needed to compile a substrate node. More details [here](./docs/rust-setup.md).

### Build

1. Clone the template repository:

```sh
git clone https://github.com/paritytech/frontier-parachin-template
```

2. Use `cargo` to build the parachain node without launching it:

```sh
cargo build --release
```

### Run a local network
 You will need a compatible release of [Polkadot](https://github.com/paritytech/polkadot) to run a local network. You may also want to use [Zombienet](https://github.com/paritytech/zombienet/releases) (available for Linux and MacOS),  for spinning up a full fledged relay chain - parachain environment. You can find more information about running a local test network [HERE](./docs/zombienet.md)



👉 Learn more about parachains [here](https://wiki.polkadot.network/docs/learn-parachains), and parathreads [here](https://wiki.polkadot.network/docs/learn-parathreads).


🧙 Learn about how to use this template and run your own parachain testnet for it in the
[Devhub Cumulus Tutorial](https://docs.substrate.io/tutorials/v3/cumulus/start-relay/).