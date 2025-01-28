---
id: staking/delegatedstaking
title: Delegated Staking
---

<!-- markdownlint-disable MD013 MD051 -->

[In Progress]

## Delegated Staking

This repository provides the contracts and scripts required to create a staking pool that users can delegate their ZILs to. The staking mechanism supports two variants:

1. **Liquid Variant**

   - Users receive a non-rebasing Liquid Staking Token (LST) when delegating their stake.
   - The LST can be sent to the validator's contract to withdraw the staked amount plus the corresponding share of validator rewards.

2. **Non-Liquid Variant**
   - Users can regularly withdraw their share of the rewards without withdrawing their staked amount.

---

## Steps for Delegated Staking

1. **Install an RPC Node**  
   Set up and configure a compatible RPC node to interact with the blockchain.

2. **Deploy the Staking Contract**  
   Choose the staking variant (Liquid or Non-Liquid) and deploy the appropriate contract.

3. **Manage Commission**  
   Set and adjust the commission rates for the staking pool as required.

4. **Stake and Unstake**  
   Allow users to delegate (stake) or withdraw (unstake) their ZILs based on the chosen staking variant.

---

This process ensures that validators and pool operators can provide a seamless staking experience for users while managing rewards and commissions effectively.
