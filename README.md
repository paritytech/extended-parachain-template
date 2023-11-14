# Extended Parachain Template

[![Check Build](https://github.com/paritytech/extended-parachain-template/actions/workflows/build.yml/badge.svg)](https://github.com/paritytech/extended-parachain-template/actions/workflows/build.yml)


The **Extended Parachain Template** is a ready-to-use parachain template, pre-configured with the [Assets](https://paritytech.github.io/substrate/master/pallet_assets/index.html) pallet, a simple Governance system ([Collective](https://paritytech.github.io/substrate/master/pallet_collective/index.html) & [Motion](https://github.com/paritytech/extended-parachain-template/tree/main/pallets/motion) pallets), and other useful base features.

This is a solid starting point for most Parachain projects as it is a more feature-rich alternative to the base [Substrate Parachain Template](https://github.com/substrate-developer-hub/substrate-parachain-template) (which it is derived from).

This template is maintained by the **Delivery Services** team at **Parity**.

## 🚀 Getting Started

### 🦀 Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### 🔧 Build

Clone the extended parachain template repository: 

```sh
git clone https://github.com/paritytech/extended-parachain-template
```

Use the following command to build the node without launching it:

```sh
cargo build --release
```

### 🕸️ Run a local network 
Next you will need a compatible release of [Polkadot](https://github.com/paritytech/polkadot-sdk) to run a testnet. You may also want to use [Zombienet (available for Linux and MacOS)](https://github.com/paritytech/zombienet/releases) for spinning up a testnet: 


You can find linux and macOS executables of the Zombienet CLI here:

https://github.com/paritytech/zombienet/releases
Download the Zombienet CLI according to your operating system.

Tip: If you want the executable to be available system-wide then you can follow these steps (otherwise just download the executable to your working directory):
```sh
wget https://github.com/paritytech/zombienet/releases/download/v1.3.30/zombienet-macos
chmod +x zombienet-macos 
cp zombienet-macos /usr/local/bin
```
Make sure Zombienet CLI is installed correctly:
```sh
./zombienet-macos --help
```
You should see some similar output:
```sh
Usage: zombienet [options] [command]

Options:
  -c, --spawn-concurrency <concurrency>  Number of concurrent spawning process to launch, default is 1
  -p, --provider <provider>              Override provider to use (choices: "podman", "kubernetes", "native")
  -m, --monitor                          Start as monitor, do not auto cleanup network
  -h, --help                             display help for command

Commands:
  spawn <networkConfig> [creds]          Spawn the network defined in the config
  test <testFile> [runningNetworkSpec]   Run tests on the network defined
  setup <binaries...>                    Setup is meant for downloading and making dev environment of Zombienet ready
  version                                Prints zombienet version
  help [command]                         display help for command

```
You may use a reference implementation from the folder `zombienet-config` or make your own. More instructions here: [Simulate parachains in a test network
](https://docs.substrate.io/test/simulate-parachains/)

👉 Learn more about parachains [here](https://wiki.polkadot.network/docs/learn-parachains), and
parathreads [here](https://wiki.polkadot.network/docs/learn-parathreads).


🧙 Learn about how to use this template and run your own parachain testnet for it in the
[Devhub Cumulus Tutorial](https://docs.substrate.io/tutorials/v3/cumulus/start-relay/).

 ## 🏛 Governance

 Parachain governance is a very crucial topic that goes beyond using `sudo` for privileged calls. [Read our Governance Explainer here](./docs/governance.md)