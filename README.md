# 💎 MySol: Immutable Utility for SOL & USDC Vaults

**MySol** is a high-security, single-file utility built for the Solana ecosystem. It is not a wallet—it is a **Vault Factory**. It helps you create self-custody "Piggy Banks" with on-chain rules that protect your SOL and USDC from impulsive spending or gambling.

## 🎯 Project Goals
* **Financial Guardrails:** Use on-chain Solana programs to enforce daily withdrawal limits and time-locks.
* **Pure Solana Ecosystem:** Exclusively designed for **SOL** and native **USDC**.
* **Utility, Not a Wallet:** MySol creates the vaults; you can still import the keys into any standard wallet (Phantom, Solflare) for easy access.
* **Zero-Footprint:** A single, portable HTML/JS file under 1,000 lines of code.

---

## 🚀 Core Features
* **Vault Factory:** Easily generate new "Banks" for Rent, Savings, or Daily Allowances.
* **On-Chain Enforcement:** Withdrawal limits are handled by the Solana Cluster. If you try to exceed your limit, the transaction fails at the protocol level.
* **Time-Window Lock:** Define an "Enforcement Window" where vault rules cannot be modified.
* **Local-First Security:** Vault data and keys are encrypted locally using AES-256-GCM via the Web Crypto API.

---

## 🛠️ Getting Started
1.  **Download:** Save `mysol.html` to your local machine.
2.  **Open:** Run it in any modern browser.
3.  **Create:** Initialize a new Piggy Bank by setting your **Daily Limit** and **Lock Period**.
4.  **Connect:** Export the generated keys to your favorite Solana wallet for daily use, or keep them inside MySol for maximum discipline.

---

## ⛓️ Technical Architecture



MySol utilizes **Program Derived Addresses (PDAs)**. The SOL/USDC is held by the Solana Program logic:
1. The **Solana Program** checks the network timestamp and your `daily_withdrawn` balance.
2. If the request is valid, the program authorizes the transfer to your spending wallet.
3. You cannot "override" the code, even with your own private keys, until the lock period expires.

---

## 📝 TODOs & Future Roadmap
* [ ] **Convert Existing Wallets:** Implementation of a "Wrap" feature to easily import existing Solana addresses and convert them into MySol-managed smart contracts.
* [ ] **One-Click Export:** Seamlessly push vault keys to Phantom/Solflare via standard wallet adapter protocols.
* [ ] **Mobile Optimization:** Ensure the single-file UI is fully responsive for mobile browser injections.

---

## ⚠️ Security Warning
* **No Password Reset:** All data is encrypted locally. If you lose your Master Password, your local vault access cannot be recovered.
* **Finalized Authority:** To achieve true immutability, the program's upgrade authority must be revoked, making your rules permanent laws of the blockchain.

---

## 📜 License
MIT - Open Source and free to use for personal financial protection on Solana.
