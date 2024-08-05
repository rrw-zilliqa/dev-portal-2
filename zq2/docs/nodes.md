---
id: nodes
title: Nodes and Validators
---

# [Nodes and Validators](#nodes-and-validators)

The current proto-testnet version of Zilliqa 2.0 allows users to setup a node
and join the network.

Both API nodes and validator nodes use the same software, though API nodes do
not participate in the consensus process. This guide provides instructions for
setting up both types of nodes. Further details on securing validator nodes
will be added as we approach the mainnet launch.

## Zilliqa 2.0 (proto-testnet) Prerequisites

### [Hardware requirements](#hardware-requirements)

- CPU:
  - 1 Core / 2 threads or more
- RAM:
  - 8 GB or more
- Disk:
  - 100 GB or more

### [Virtual machines on Cloud Platforms](#virtual-machines-on-cloud-platforms)

We are running our Zilliqa 2.0 validators on Google Cloud Platform, GCP,
GCE VM `e2-standard-2` instance with 256 GB SSD (`pd-ssd`).

If you running on other cloud provider, please do select an instance with
similar specs.

### [Software requirements](#software-requirements)

1. Operating System: We build and run on Ubuntu 20.04LTS
2. Docker: 27.0.3+

### [Port-forwarding](#port-forwarding)

The following TCP ports need to be open to the internet for both inbound and
outbound.

_NOTE: We don't recommend to run validators behind a NAT, if you're doing so
and you are facing any traversal issue you might have to debug on your own._

#### Required

3333/TCP - P2P protocol port: has to be opened on inbound and outbound to
public internet.

#### Optional

4201/TCP - JSONRPC over HTTP: API port, only necessary if you want your API to
be accessible via the internet.

### [Running a Node](#running-a-node)

To run a Zilliqa 2.0 node and join the proto-testnet, we provide the `z2`
utility as part of the [zq2](https://github.com/Zilliqa/zq2/blob/main/) code
base.

The `z2 join` command creates the validator node startup script and configuration
file that you can copy and paste on your Ubuntu VM, configured as per above specs,
and run.

### Step by step guide

- Cargo and Rust: You need to have Cargo and Rust installed on your system.
  You can install them using [rustup](https://rustup.rs/). Once rustup is installed,
  you can update Rust to the latest stable version.
- Install the following requirements:

  ```bash
  sudo add-apt-repository ppa:ethereum/ethereum && sudo apt update && \
  sudo apt install -y solc build-essential pkg-config libssl-dev cmake \
  protobuf-compiler
  ```

- Pick a directory. You'll need quite a lot of space. Let's call it `/my/dir`.
- Clone `git@github.com:zilliqa/zq2` into that directory to get `/my/dir/zq2`.
- Build the code using `cargo build`.
- Source the setenv file:

  ```bash
  source /my/dir/zq2/scripts/setenv
  ```

  This will give you access to the `z2` tool (in `zq2/z2`).

- Generate the startup script and the configuration file for your node by running:

  ```bash
  z2 join --chain zq2-prototestnet
  ```

- Generate the validator node private key.

  ```bash
  openssl rand -hex 32 > validator-private-key.txt
  export PRIVATE_KEY=$(cat validator-private-key.txt)

  ```

  _NOTE: Please save the validator node key as described above. You may need it
  in the future to restart the validator node to generate the BLS public
  key of the validator node._

- Start the validator node:

  ```bash
  chmod +x start_validator.sh && \
  ./start_validator.sh $PRIVATE_KEY
  ```

Great! The validator node should now be syncing with the network. It may
take up to 1.5 hours for the node to fully sync. You can check the progress
of the node by running the following command, which should return the latest
block height after syncing.

```bash
curl --request POST \
  --url http://localhost:4201/ \
  --header 'Content-Type: application/json' \
  --data '{"method":"eth_blockNumber","params":[],"id":1,"jsonrpc":"2.0"}'
```

For additional details on `z2` and the `join` capability refer to:

- <https://github.com/Zilliqa/zq2/blob/main/z2/docs/README.md>
- <https://github.com/Zilliqa/zq2/blob/main/z2/docs/join.md>

### [Becoming a Validator](#becoming-a-validator)

Under the consensus mechanism introduced in Zilliqa 2.0, validators stake ZIL
to secure the network, in return for which they receive a share of block
rewards.

While becoming a validator on the Zilliqa 2.0 mainnet will be permissionless,
on the current proto-testnet you need to request the minimum required stake of
10 million ZIL in order for you to register as a validator.

To register as a validator on the Jasper proto-testnet, please complete and
submit validator join form.

Once you have sufficient $ZILs you can register your node as validator.

Below is a guide on how to register a validator node for Zilliqa 2.0:

<https://github.com/Zilliqa/zq2/blob/main/z2/docs/deposit.md>
