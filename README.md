# MySOL | Blockchain Vault Burner

**MySOL** is a zero-dependency, hardware-grade discipline tool for the Solana blockchain. It allows users to "burn" immutable spending rules into the ledger for any specific wallet.

## 🛠 How It Works
1. **The Burn:** Using the MySOL Dashboard, you connect a wallet and upload a **Daily Spending Limit** and an **Enforcement Period**.
2. **On-Chain Enforcement:** These rules are stored in a Program Derived Address (PDA). The Smart Contract intercepts all withdrawal requests. If a request exceeds the daily limit during the enforcement period, the blockchain rejects it at the protocol level.
3. **The Self-Destruct:** Once the enforcement term (e.g., 90 days) expires, the contract's restrictive logic **automatically self-destructs**. The vault effectively "dissolves" the rules, restoring the wallet to a normal, unrestricted state.

## 🚀 Quick Start
1. **Prepare:** Create a new wallet in Solflare/Phantom (e.g., "Gamba Funds").
2. **Configure:** Set your Asset (SOL/USDC), Limit, and Lock Duration.
3. **Deploy:** Click **Sign & Burn**. The rules are now permanent for the chosen duration.
4. **Clean Up:** Once the transaction is confirmed, the local project files can be deleted. The rules now exist only on the blockchain.

## 🛡 Strategies
* **Gamba Guard:** Set a $50/day limit for 90 days to prevent tilt.
* **Fort Knox:** Set a 0 limit for 365 days for absolute HODLing.
* **Gas Saver:** Set a 0.05 SOL limit to ensure you always have transaction fees.

---
**WARNING:** During the enforcement period, there is no "undo" button. Not even the creator of the contract can bypass the rules once they are burned to the ledger.
