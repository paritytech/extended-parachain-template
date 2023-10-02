## Zombienet Installation:

You can download executables of the Zombienet CLI from [paritytech/zombienet/releases](https://github.com/paritytech/zombienet/releases)


- Download the Zombienet CLI according to your operating system.

 💡 Tip: If you want the executable to be available system-wide then make sure you place it in one of your `$PATH` directories.
```sh
wget https://github.com/paritytech/zombienet/releases/download/v1.3.30/zombienet-macos
chmod +x zombienet-macos 
cp zombienet-macos /usr/local/bin
```
Then invoke it anywhere like :
```sh 
zombienet-macos --help
```
Otherwise you can just place it in your working directory.

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

## Setting up Zombienet config:

You may use a reference implementation from the folder `zombienet-config` or make your own. We provide a simple configuration for you called [zombienet-config.toml](../zombienet-config.toml) which spins up two validators for the relay chain, and one collator for your parachain to get you quickly upto speed.

⚠️ Note: the path of the polkadot executable used there is `./bin/polkadot` which means you need to have your compiled polkadot binary inside a folder `./bin`. Also since `polkadot-v1.1.0`, two additional binaries would be needed to run the relay chain node called `polkadot-prepare-worker` and `polkadot-execute-worker` both of which can be found in the [Polkadot-Sdk release page](https://github.com/paritytech/polkadot-sdk/releases/tag/polkadot-v1.1.0). Place these binaries alongside the main `polkadot` binary.

More instructions here: [Simulate parachains in a test network
](https://docs.substrate.io/test/simulate-parachains/)