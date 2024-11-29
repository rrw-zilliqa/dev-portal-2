---
id: nodes/checkpoint
title: Checkpoint
---

# Checkpoints in Nodes

Checkpoints in nodes serve as specific reference points within the blockchain, allowing nodes to synchronize more efficiently when joining or rejoining the network. Instead of processing the entire blockchain from the genesis block, nodes can start from a known, validated state. Below is a detailed guide on how to set up checkpoints.

## Syncing a Node from a Checkpoint

Before proceeding, ensure you have completed the [Node Setup](../nodes/nodes.md#setting-up-your-environment-and-building-zq2-node) section.

For **prototestnet**, you will need the `zq2-prototestnet.toml` configuration file and the `start_node.sh` script, both generated during the setup process. Similarly, for **protomainnet**, you will use the `zq2-protomainnet.toml` configuration file.

The following steps apply to both networks.

### Step-by-Step Guide to Configure Checkpoints

1. **Ensure Proper Directory**
   Navigate to the directory you created during the setup process, e.g., `/my/dir/zq2`.

2. **Download the Checkpoint File**
   Follow these steps to download the latest checkpoint file for your chosen network:

    - **Prototestnet**:
     Visit the public [checkpoint URL](https://checkpoints.zq2-prototestnet.zilliqa.com/).

    - **Protomainnet**:
     Visit the public [checkpoint URL](https://checkpoints.zq2-protomainnet.zilliqa.com/).

    From the XML file at the respective URL:
        - Look for the `<key>` tag, which contains the checkpoint file's name. The file follows the `block_num.dat` format (e.g., `000291600.dat`).
        - Copy the file name of the latest checkpoint from the topmost `<key>` tag. For older checkpoints, explore the `previous/` directory.
        - Download the checkpoint file using the `wget` command or paste the link in your browser:
      ```bash
      wget https://checkpoints.zq2-<network>.zilliqa.com/<block_num.dat>
      ```
      Replace `<network>` with `prototestnet` or `protomainnet` based on your selected network.

      **Note**: It is recommended to use the latest checkpoint file to speed up synchronization.

3. **Configure Checkpoints in the Configuration File**
   Open the respective configuration file (`zq2-prototestnet.toml` or `zq2-protomainnet.toml`) and add the following lines to enable checkpoint settings:
   ```toml
   [nodes.load_checkpoint]
   file = "xxxxx..." # File name of the checkpoint block. for eg: 3000.dat
   hash = "xxxxx..." # Block hash corresponding to the file block (Remove '0x' prefix from hash if present)
   ```

    `file` : This parameter specifies the name of the checkpoint or block number file, which
    can be obtained from the public GCS bucket. It’s recommended to download the latest checkpoint
    file from this source.

    `hash` : The hash is used to verify the validity of the state data and ensure that no
    tampering has occurred. You can obtain the block hash corresponding to the checkpoint height from the
    public explorer of your chosen network. For example, if the downloaded
    checkpoint file is 3000, you can use the `eth_getBlockByNumber` API to query the block hash:

    ```bash
    curl --request POST --url https://api.zq2-prototestnet.zilliqa.com/ \
    --header 'Content-Type: application/json' \
    --data '{"method":"eth_getBlockByNumber","params":["0xBB8",false],"id":1,"jsonrpc":"2.0"}' \
    | grep -o '"hash":"[^"]*"' | awk -F':' '{print $2}' | tr -d '"'
    ```
  Alternatively, you can retrieve the block hash directly from the public explorer of your chosen network by searching for the block number.
  Refer to [block explorers](../endpoints.md#block-explorer) section for public explorer.
  By this stage, your checkpoints configuration should be set up in the `zq2-prototestnet.toml` or `zq2-protomainnet.toml` file.

4. **Launch the node**  
Now the node is ready to launch. Follow the instructions in the [Start the Node](../nodes/node.md#start-the-node) section to start your node.
