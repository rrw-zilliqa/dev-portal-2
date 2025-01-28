---
id: staking/exchange
title: Exchanges & Wallets
---

## Transitioning to Zilliqa 2.0 Staking for Exchanges

Currently, exchanges have integrated Zilliqa 1.0 staking on their platforms. With the launch of Zilliqa 2.0, the Zilliqa 1.0 staking protocol will become inoperational. Exchanges will need to transition to the new EVM-based staking mechanism to ensure seamless staking functionality.

Below are the steps exchanges must follow to adopt the new Zilliqa 2.0 delegated staking mechanism.

---

## Overview of Zilliqa 2.0 Staking

Exchanges or wallets running validator nodes will validate transactions and earn both block rewards and staking commissions. Users delegating ZILs to these validator nodes will earn staking rewards. Depending on the staking variant supported by the validator, users can either:

- Claim their rewards at any time.
- Receive rewards when they unstake their liquid staking tokens.

For technical implementation details, refer to the [Zilliqa 2.0 Whitepaper](#whitepaper-link) and [Node Documentation](#node-docs-link).

---

## Steps to Transition

### 1. Withdrawing Stake from Zilliqa 1.0

- Claim all rewards and withdraw the staked amount delegated to your SSN.
- This can be done via [Old Zillion](https://stake.zilliqa.com) or your in-house implementation mechanism.

### 2. Implementing the New EVM-Based Delegated Staking

- Integrate the new EVM-based delegated staking mechanism as outlined in the [Delegated Staking Documentation](#page-link).

---
