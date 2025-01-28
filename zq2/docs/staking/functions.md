---
id: staking/functions
title: Staking Functions
---

# Essential Functions for Node Operators

This section outlines the core functions that node operators need to manage staking, delegation, and rewards.

---

## **1. Staking & Unstaking**

| **Function**              | **Contract**                 | **Description**           |
| ------------------------- | ---------------------------- | ------------------------- |
| `stake()`                 | Liquid/Non-Liquid Delegation | Delegate ZILs for staking |
| `unstake(uint256 shares)` | Liquid/Non-Liquid Delegation | Unstake ZILs              |
| `claim()`                 | Liquid/Non-Liquid Delegation | Withdraw unstaked ZILs    |

---

## **2. Validator Deposit & Migration**

| **Function**                        | **Contract**   | **Description**                            |
| ----------------------------------- | -------------- | ------------------------------------------ |
| `depositFirst(bytes, bytes, bytes)` | BaseDelegation | First-time node deposit                    |
| `depositLater(bytes, bytes, bytes)` | BaseDelegation | Top-up stake for validator                 |
| `migrate(bytes)`                    | BaseDelegation | Migrate existing validator to staking pool |

---

## **3. Commission Management**

| **Function**                      | **Contract**   | **Description**              |
| --------------------------------- | -------------- | ---------------------------- |
| `setCommissionNumerator(uint256)` | BaseDelegation | Set commission percentage    |
| `collectCommission()`             | BaseDelegation | Withdraw commission earnings |

---

## **4. Querying Staking Information**

| **Function**          | **Contract**          | **Description**                          |
| --------------------- | --------------------- | ---------------------------------------- |
| `getStake()`          | BaseDelegation        | Returns total stake in the contract      |
| `getRewards()`        | BaseDelegation        | Returns claimable rewards                |
| `getDelegatedStake()` | Non-Liquid Delegation | Returns the user's current staked amount |
| `getPrice()`          | Liquid Delegation     | Returns ZIL equivalent for 1 LST token   |

---
