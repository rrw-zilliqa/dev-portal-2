---
id: staking
title: Staking Migration Guide
---

Zilliqa 1.0 (ZQ1) is transitioning to the new Zilliqa 2.0 (ZQ2) platform, introducing significant changes to the staking process. Delegators and operators must adapt their systems to support the updated delegated staking mechanism on the new blockchain network. This document outlines the steps exchanges and users need to follow to ensure a smooth transition.

---

## Entities Impacted

1. **External Staked Users**  
   Users who are currently staking in Zilliqa through one of the SSN nodes.

2. **Exchanges and Wallet Providers**  
   Platforms offering staking services for the Zilliqa blockchain.

---

## Key Changes

1. **ZQ1 Staking Termination**  
   The ZQ1 staking mechanism (SSN List contract) will no longer earn rewards after the transition to ZQ2.

2. **Stake Withdrawal**

   - Users and exchanges/wallet providers must withdraw their stake from the SSN List contract using [Old Zillion](https://stake.zilliqa.com).
   - They must move their stake to the new EVM-based delegated staking mechanism.

3. **New Delegated Staking Implementation**

   - Exchanges and wallet providers must implement the new delegated staking model.
   - Users can stake using the [New Zillion](#link-goes-here) portal, selecting one of the staking nodes to transfer their stake.

4. **Technical Reference**  
   The new staking model is detailed in the GitHub repository: [Delegated Staking Repository](https://github.com/Zilliqa/delegated_staking).

5. **Additional Resources**
   - **For External Users**: Refer to the [User Guide](../staking/users.md).
   - **For Exchanges/Wallet Providers**: Refer to the [Exchanges Guide](#exchange.md).

---
