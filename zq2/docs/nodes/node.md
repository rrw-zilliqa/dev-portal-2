---
id: nodes/nodes
title:  Prototestnet Nodes 
---

# Prototestnet nodes

Both the proto-testnet and the proto-mainnet version of Zilliqa 2.0 allow users to setup a node and join the network.

## Zilliqa 2.0 (proto-testnet) node Prerequisites

### [Hardware requirements](#hardware-requirements-prototestnet)

- **CPU**:
    - 1 Core / 2 threads or more
- **RAM**:
    - 4 GB or more
- **Disk**:
    - 100 GB or more

## Zilliqa 2.0 (proto-mainnet) node Prerequisites

### [Hardware requirements](#hardware-requirements-protomainnet)

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

### [Setting Up Your Environment and Building ZQ2 Node](#setup-a-node)

To run a Zilliqa 2.0 node and join the proto-testnet, we provide the `z2`
utility as part of the [zq2](https://github.com/Zilliqa/zq2/blob/main/) code
base.

The `z2 join` command creates the node startup script and configuration
file that you can copy and paste on your Ubuntu VM, configured as per above specs,
and run.

### Step by step guide

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
8. Generate the node private key.
  ```bash
  openssl rand -hex 32 > node-private-key.txt
  export PRIVATE_KEY=$(cat node-private-key.txt)
  ```
  _NOTE: Please save the node key as described above. You may need it
  in the future to restart the node to generate the BLS public
  key of the node._
9. Now it's time to decide how the node will synchronize with the network. 
There are two methods for setting the synchronization rules for the node.

    - Start the node from a checkpoint.

    Starting from a checkpoint is a significantly faster option. This method leverages a 
    predefined checkpoint block number, enabling the node to sync with the network in just 
    a few hours, depending on the checkpoint's block height. Before proceeding to 
    [start the node](#start-the-node) section, you'll need to configure 
    the necessary settings to start the node from a checkpoint.  
    Detailed instructions for this configuration are available in 
    [syncing-from-checkpoints](../nodes/checkpoint.md#syncing-a-node-from-a-checkpoint). 
    Once the checkpoint is set up, your node will be ready to start.

    - Start the node from the genesis.

    This method initializes the node from the genesis block, ensuring that the node processes 
    the entire blockchain history. However, this process is time-consuming, as the node must 
    download and validate every block from the genesis block to the latest block height. 
    Syncing the node to the latest block may take a considerable amount of time, 
    potentially up to several days to complete fully. If you opt for this method, you can 
    proceed directly to the [Start the node](#start-the-node) section.

   

### [Start the node](#start-the-node)
Since only full archive nodes need to sync from the genesis block, all other nodes can be started from a checkpoint: 

* <b>start the node from a checkpoint:</br></b>
(fast, recommended)
  ```bash
  chmod +x start_node.sh && \
  ./start_node.sh -k $PRIVATE_KEY -p <checkpoint_block_num.dat>
  ```

* <b>start the node from the genesis:</br></b>
(slow, available after the next network upgrade)
  ```bash
  chmod +x start_node.sh && \
  ./start_node.sh -k $PRIVATE_KEY
  ```


_NOTE: The `<checkpoint_block_num.dat>` file is the one you previously downloaded. Refer to [syncing-from-checkpoint](../nodes/checkpoint.md#syncing-a-node-from-a-checkpoint)_

Great! The node should now be syncing with the network. It may
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

To upgrade your node, first edit the `ZQ_VERSION` variable in `start_node.sh` to refer to the newest release. Then run the following:

```bash
docker pull asia-docker.pkg.dev/prj-p-devops-services-tvwmrf63/zilliqa-public/zq2:${NEW_ZQ_VERSION} # This is optional, but recommended. Pulling the new image before stopping the old version will minimise the downtime of your node.
docker container ls # Identify the container ID of the existing node. This will look a 12 character hex-string (e.g. af6010f3f9ae).
docker stop <container id> # Stop the old version.
./start_node.sh # Start the new version.
```

You can validate the version your node is running by calling the `GetVersion` API.

