---
id: staking/delegatedstaking
title: Overview
---

<!-- markdownlint-disable MD013 MD051 -->

# Delegated Staking

This documentation provides the contracts and scripts required to create a staking pool where users can delegate their ZILs. The staking mechanism supports two variants:

---

## Staking Variants

### 1. **Liquid Variant**

- Users receive a **non-rebasing Liquid Staking Token (LST)** upon delegation.
- The LST can be sent to the validator's contract to withdraw the staked amount plus the corresponding share of validator rewards.

### 2. **Non-Liquid Variant**

- Users can withdraw their share of the rewards regularly without withdrawing the staked amount.

---

## Steps for Delegated Staking

### 1. **Install an RPC Node**

Set up and configure a compatible RPC node to interact with the blockchain.  
Refer to the [Node Installation Guide](../nodes/node.md) for details.

---

### 2. **Deploy the Staking Contract**

Choose a staking variant (Liquid or Non-Liquid) and deploy the appropriate contract.  
Refer to the [Contract Deployment Guide](https://github.com/Zilliqa/delegated_staking#contract-deployment).

---

### 3. **Manage Commission**

Set and adjust the commission rates for the staking pool as needed.  
Detailed instructions are available in the [Manage Commission Guide](https://github.com/Zilliqa/delegated_staking#contract-configuration).

---

### 4. **Validator Activation or Migration**

- **Activation**: Deposit **10M ZIL tokens** to activate the staking node as a validator.
- **Migration**: If the node is already a PoS node, activate it to participate in the staking pool.  
  Refer to the [Validator Activation or Migration Guide](https://github.com/Zilliqa/delegated_staking#validator-activation-or-migration) for more details.

---

### 5. **Stake and Unstake**

Enable users to:

- Delegate (stake) ZILs to your validator.
- Withdraw (unstake) their ZILs as needed, based on the selected staking variant.  
  Refer to [staking-unstaking](https://github.com/Zilliqa/delegated_staking?tab=readme-ov-file#staking-and-unstaking) section

---

### 6. **Withdrawing or Staking Rewards**

- Allow users to withdraw their rewards.
- Alternatively, let them reinvest the rewards back into the staking pool.  
  Refer to the [Rewards Withdrawal Guide](https://github.com/Zilliqa/delegated_staking?tab=readme-ov-file#withdrawing-or-staking-rewards) section.

---
