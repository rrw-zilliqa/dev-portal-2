---
id: nodes/nodes
title:  Node setup 
---

# Node setup

Both the proto-testnet and the proto-mainnet version of Zilliqa 2.0 allow users to setup a node and join the network.

## Prerequisites

### [Proto-testnet hardware requirements](#proto-testnet-hardware-requirements)

- **CPU**:
    - 1 Core / 2 threads or more
- **RAM**:
    - 4 GB or more
- **Disk**:
    - 100 GB or more

### [Proto-mainnet hardware requirements](#proto-mainnet-hardware-requirements)

- **CPU**:
    - 2 Core / 4 threads or more
- **RAM**:
    - 8 GB or more
- **Disk**:
    - 200 GB or more

We are running our Zilliqa 2.0 Nodes on Google Cloud Platform, GCP,
GCE VM `e2-standard-2` instance with 256 GB SSD (`pd-ssd`).

### [Software requirements](#software-requirements)

1. Operating System: We build and run on Ubuntu 22.04LTS or above
2. Docker: 27.0.3+

### [Port-forwarding](#port-forwarding)

The following TCP ports need to be open to the internet for both inbound and
outbound.

_NOTE: We don't recommend to run Nodes behind a NAT, if you're doing so
and you are facing any traversal issue you might have to debug on your own._

#### Required

3333/UDP - P2P protocol port: has to be opened on inbound and outbound to
public internet.

#### Optional

4201/TCP - JSONRPC over HTTP: API port, only necessary if you want your API to
be accessible via the internet.

## Installation

### [Setting up your node](#setting-up-your-node)

To run a Zilliqa 2.0 node and join the proto-mainnet or the proto-testnet,
we provide the `z2` utility as part of the [zq2](https://github.com/Zilliqa/zq2/blob/main/) code
base. Follow the step by step guide to setup your node:

1. Cargo and Rust: You need to have Cargo and Rust installed on your system.
  You can install them using [rustup](https://rustup.rs/). Once rustup is installed,
  you can update Rust to the latest stable version.
2. Install the following requirements:
  ```bash
  sudo add-apt-repository ppa:ethereum/ethereum && sudo apt update && \
  sudo apt install -y solc build-essential pkg-config libssl-dev cmake \
  protobuf-compiler
  ```
3. Pick a directory. You'll need quite a lot of space. Let's call it `/my/dir`.
4. Clone [zq2](https://github.com/zilliqa/zq2) sourcecode into that directory to get `/my/dir/zq2`.

5. Build the code using `cargo build`.
6. Source the setenv file:
  ```bash
  source /my/dir/zq2/scripts/setenv
  ```
  This will give you access to the `z2` tool (in `zq2/z2`).
7. Generate the startup script and the configuration file for your node by running:
  ```bash
  z2 join --chain zq2-prototestnet
  ```
  _NOTE: You can replace `zq2-prototestnet` with `zq2-protomainnet` depending on
  which network you want your node to join._
8. Generate the node private key.
  ```bash
  openssl rand -hex 32 > node-private-key.txt
  export PRIVATE_KEY=$(cat node-private-key.txt)
  ```
  _NOTE: Please save the node key as described above. You may need it
  in the future to restart the node to generate the BLS public
  key of the node._
9. Now it's time to decide how the node will synchronize with the network. 
There are two options you can choose from:

    - Synchronization from a checkpoint.

    Starting from a checkpoint is a significantly faster option. This method leverages a 
    predefined checkpoint block number and hash, enabling the node to sync with the network 
     in justa few hours, depending on the checkpoint's block height. Before proceeding to 
    [start the node](#start-the-node) section, you'll need to configure the necessary
    settings to start the node from a checkpoint. Detailed instructions for this configuration
    are available in [syncing-from-checkpoints](../nodes/checkpoint.md#syncing-a-node-from-a-checkpoint).

    - Synchronization from the genesis.

    This method initializes the node from the genesis block, ensuring that the node processes 
    the entire blockchain history. However, this process is time-consuming, as the node must 
    download and validate every block from the genesis block to the latest block height. 
    Syncing the node to the latest block may take a considerable amount of time, 
    potentially up to several days to complete fully.
   

### [Starting your node](#starting-your-node)
Since only full archive nodes need to sync from the genesis block, all other nodes can be started from a checkpoint: 

* <b>start the node from a checkpoint:</br></b>
(fast, recommended)
  ```bash
  chmod +x start_node.sh && \
  ./start_node.sh -k $PRIVATE_KEY -p <checkpoint_block_num.dat>
  ```

* <b>start the node from the genesis:</br></b>
(slow, available in a future upgrade)
  ```bash
  chmod +x start_node.sh && \
  ./start_node.sh -k $PRIVATE_KEY
  ```


_NOTE: The `<checkpoint_block_num.dat>` file is the one you previously downloaded. Refer to [syncing-from-checkpoint](../nodes/checkpoint.md#syncing-a-node-from-a-checkpoint)_

Great! The node should now be syncing with the network. It may
take up to 1-2 hours for the node to fully synchronize. You can check the progress
of the node by running the following command, which should return the latest
block height after syncing.
```bash
curl --request POST \
  --url http://localhost:4201/ \
  --header 'Content-Type: application/json' \
  --data '{"method":"eth_blockNumber","params":[],"id":1,"jsonrpc":"2.0"}'
```

If you started your node from a checkpoint and it does not respond to
the above request, then it is still processing the checkpoint file
and has not started synchronizing yet.

For additional details on `z2` and the `join` capability refer to:

- <https://github.com/Zilliqa/zq2/blob/main/z2/docs/README.md>
- <https://github.com/Zilliqa/zq2/blob/main/z2/docs/join.md>

### [Becoming a Validator](#becoming-a-validator)

Under the consensus mechanism introduced in Zilliqa 2.0, nodes can stake ZIL to secure
the network and promote themselves as validator nodes. In return, they receive a 
share of the block rewards.

While becoming a validator on the Zilliqa 2.0 mainnet will be permissionless,
on the current proto-testnet you need to request the minimum required stake of
10 million ZIL in order for you to register as a validator.

To register as a validator on the Jasper proto-testnet, please complete and
submit validator join form.

Once you have sufficient $ZILs you can register your node as validator.

Below is a guide on how to register a validator node for Zilliqa 2.0:

<https://github.com/Zilliqa/zq2/blob/main/z2/docs/deposit.md>

### [Upgrading your node](#upgrading-your-node)

You should try to keep your node version up-to-date with the latest released version of Zilliqa 2.0.
You can stay informed of new releases via the [repository release page](https://github.com/Zilliqa/zq2/releases).

Sometimes a hard fork will be needed when the execution semantics of blocks or transactions have changed.
It is important to upgrade your node's version before the block height at which these hard forks are activated.
Not doing so may lead to your node going out of sync and losing rewards if it is a validator.

First, update your `start_node.sh` script and configuration file by re-running `z2 join`:

```bash
z2 join --chain zq2-prototestnet
```
_NOTE: Replace `zq2-prototestnet` with the chain you are running on._

To minimise the downtime of your node, we recommend pulling the new image locally before you stop your old node:

```bash
docker pull asia-docker.pkg.dev/prj-p-devops-services-tvwmrf63/zilliqa-public/zq2:${ZQ_VERSION} # You can copy the new ZQ_VERSION from inside `start_node.sh`
```

Stop your existing node:

```bash
docker container ls # Identify the container ID of the existing node. This will look a 12 character hex-string (e.g. af6010f3f9ae).
docker stop <container id>
```

Start your new node:

```bash
./start_node.sh
```

You can validate the version your node is running by calling the `GetVersion` API method:

```bash
curl --request POST --url http://localhost:4201 --header 'content-type: application/json' --data '{"method":"GetVersion","id":1,"jsonrpc":"2.0"}'
```
