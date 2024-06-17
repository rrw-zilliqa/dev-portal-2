---
id: changes
title: Changelog and transition plan
---

# Zilliqa 2.0 changes and transition plan

## Changes

There are a number of differences between Zilliqa 1.0 and Zilliqa 2.0 that you should be aware of:

- Zilliqa 2.0 uses proof of stake based Fast-Hotstuff as a consensus algorithm. Mining is no longer necessary.
- Zilliqa 2.0 has many fewer nodes, and is thus far cheaper to run, than Zilliqa 1.0 - a typical Zilliqa 2.0 mainnet can run comfortably in 32 nodes.
- Zilliqa 2.0 has a much faster block time (there is typically a hardwired minimum of 1s/block); dApp operators will need to make sure that where they use block number as a proxy for a timestamp, they allow sufficient blocks for users to react.
- Zilliqa 2 upgrades are seamless and relatively quick; you don't need to redownload persistence and there will be an auto-upgrader you can run if you wish which will run the newer version of zq2 and cut over when ready. We hope this will enable us to eliminate upgrade downtime and to make more frequent bug fixes.

## API differences

- There are no DS epochs any more (though some are faked to allow
  existing applications that retrieve the current DS epoch to work),
  so the transaction pool is no longer cleared wat the start of a DS
  epoch.
- `GetTransactionStatus` no longer depends on an off-chain transaction
  store, and therefore now works for any transaction.

## Continuity

There are also a number of things that have not changed:

- Zilliqa 2.0 is (or should be!) compatible with all the same dApps, tokens and sites as Zilliqa 1.
