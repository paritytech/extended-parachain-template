- ### Install Rust: 

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```
- ### Install essential dependencies for building a substrate node:

Ubuntu: 
```sh
sudo apt update
sudo apt install -y cmake pkg-config libssl-dev git gcc build-essential git clang libclang-dev protobuf-compiler
```
Arch Linux:
```sh
pacman -Syu --needed --noconfirm cmake gcc openssl-1.0 pkgconf git clang
export OPENSSL_LIB_DIR="/usr/lib/openssl-1.0";
export OPENSSL_INCLUDE_DIR="/usr/include/openssl-1.0"
```
Mac OS:
```sh
brew update
brew install openssl cmake llvm
```

- ### Install the `wasm` target for your rust toolchain

```sh
rustup update
rustup target add wasm32-unknown-unknown
```