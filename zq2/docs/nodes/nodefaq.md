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

## [Validator node FAQ](#validator-faq)
## How do I sync my validator node? Should I use genesis sync or checkpoint sync ?

Currently, only genesis mode is supported for syncing validator nodes. We are actively working
on a fix for this issue. Issue link: [issue link]. Once the fix is live, you can use both genesis
sync and checkpoint sync.

## [API node FAQ](#api-node-faq)
## How do i sync my API node? Should i use genesis sync or checkpoint sync ?
For API node, both genesis sync and checkpoint sync option are supported.

## My API node s stuck at a specific block and is not progressing. What should i do ?
Try restarting the docker container, and the node should resume syncing from the last block.
```bash
docker restart <container_id>
```
## My API node starts, but the docker container stops immediately or after sometime.
This happens when running the api node from checkpoint.We observed that when a node is started from a
checkpoint , because of network latency issues, some nodes may experience node crash problem,
Issue link: [issue link].
The work around at present is as follows.
In the `zq2-prototestnet.toml` file, append following settings in the `[nodes.consensus]` section and
[start the node](../nodes/prototestnetnodes.md#start-the-node).
```bash
... # some content
[nodes.consensus]
... # some content
consensus_timeout = { secs = 3600, nanos = 0 }
```