---

id: changes
title: What's new in Zilliqa 2.0
---

# What's new in Zilliqa 2.0

Zilliqa 2.0 is designed as an entirely new protocol which will be backwards-compatible with Zilliqa 1.0, the original version of Zilliqa currently in production. When ready, the state and all accounts on the current production mainnet will seamlessly be migrated to Zilliqa 2.0.
</br> </br>
__There are a number of differences between Zilliqa 1.0 and Zilliqa 2.0:__

-   Zilliqa 2.0 uses Proof-of-Stake based on Pipelined Fast-Hotstuff as a consensus algorithm. Mining is no longer necessary.

-   There are no DS epochs, no long delay at TX block 99, and the transaction pool is no longer cleared at the start of an epoch.

-   Zilliqa 2.0 has much fewer nodes and is thus far cheaper to run than Zilliqa 1.0 - a typical Zilliqa 2.0 mainnet can be operated efficiently and securely by 32 validator nodes.

-   Zilliqa 2.0 has a much faster block time; dApps will need to make sure that where they use block number as a proxy for a timestamp, they allow sufficient blocks for users to react.

-   In Zilliqa 1.0, account balances are stored in Qa and scaled up by 1,000,000 to report Wei; in Zilliqa 2.0, they are stored in Wei and scaled down by 1,000,000 to report Qa in non-EVM APIs.

-   Zilliqa 2.0 upgrades are seamless and deployed relatively quickly without the requirement to redownload persistence. This will help eliminate upgrade downtime and accommodate more frequent bug fixes.
</br> </br>


__There are also things that have not changed:__

-   Zilliqa 2.0 will be compatible with all the same dApps, tokens and sites as Zilliqa 1.0 (EVM -> Scilla contract interoperability is currently unavailable on proto-testnet but will be reintroduced in an upgrade soon).

-   A non-EVM native token transfer uses 50 gas; an EVM transfer uses 21000 gas. To make both cost the same amount of ZIL we divide EVM gas costs by 420 (== 21000/50). That is why EVM transactions require a minimum gas price of 4761.9048 Gwei.

-   Non-EVM addresses are derived from the SHA256 of the public key, giving a base16 hex string, e.g. 0x70b16b656fc1759193366dab9a56bee486feffda, which is then conventionally expressed in bech32 form zil1wzckket0c96eryekdk4e5447ujr0al76fd6ax4

-   EVM addresses are derived from the Keccak256 of the public key, giving a base16 hex string, possibly with a checksum embedded in the capitalisation, e.g. 0xB85fF091342e2e7a7461238796d5224fA81ca556.

-   Though both EVM and non-EVM transactions use the secp256k1 curve, non-EVM transactions use Schnorr signatures, whilst EVM transactions use ECDSA.

### Converting address formats and transferring between wallets

EVM DEX Plunderswap offers a free tool that allows you to easily transfer ZIL from a non-EVM Zilliqa wallet to an EVM wallet, and vice versa.

Plunderswap’s EVM Token Transfer tool is available [here](https://plunderswap.com/transfer).

You can also use the tool below to swap addresses between base16 and bech32 formats. Note this does not produce an EVM address.

``` py
[embed tool]
```

