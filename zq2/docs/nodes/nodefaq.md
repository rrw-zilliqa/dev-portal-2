---
id: nodes/nodefaq
title: Frequently Asked Questions
keywords:
  - FAQ
  - Questions
  - NODE 
description: Frequently asked questions
---

# Frequently Asked Questions
## [Validator nodes](#validator-faq)

## How do I sync my validator node? Should I use genesis sync or checkpoint sync ?

Currently, only genesis mode is supported for syncing validator nodes.
We are actively working on checkpoint sync support feature for validator node. It will be activated through a
network upgrade coming soon.

## [Non-Validator nodes](#non-validator-node-faq)
## How do I sync my node if I'm not a validator node?
For non-validator node, both genesis sync and checkpoint sync option are supported.

## My node is stuck at a specific block and is not progressing. What should I do ?
Try restarting the docker container, and the node should resume syncing from the last block.
```bash
docker restart <container_id>
```
## My node starts, but the Docker container stops immediately or after sometime.
This is a temporary issue that will be fixed soon. It can happen to a node started from a checkpoint.
If you see the following error message in the logs.
```bash
{"timestamp":"2024-09-10T09:35:07.897651Z","level":"ERROR","fields":{"thread_name":"tokio-runtime-worker","message":"called `Result::unwrap()` on an `Err` value: database error: trie error: missing node\n\nC    aused by:\n    trie error: missing node\n\nStack backtrace:\n   0: anyhow::error::<impl core::convert::From<E> for anyhow::Error>::from\n             at ./usr/local/cargo/registry/src/index.crates.io-6f17d22    bba15001f/anyhow-1.0.87/src/backtrace.rs:27:14\n 
```
The workaround at present is as follows.

In the `zq2-prototestnet.toml` file, append following settings in the `[nodes.consensus]` section and
[start the node](../nodes/prototestnetnodes.md#start-the-node).
```bash
... # some content
[nodes.consensus]
... # some content
consensus_timeout = { secs = 3600, nanos = 0 }
```
If you notice any other issues not listed above, please bring them to our attention.