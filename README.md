# 💎 MySol: Immutable Self-Custody Vaults

**MySol** is a high-security, single-file utility for the Solana blockchain. It acts as a financial "seatbelt" by allowing you to create self-custody piggy banks with rules that are physically impossible to break—protecting you from impulsive spending or gambling.

## 🎯 Project Goals
* **Immutable Discipline:** Use on-chain smart contracts to enforce daily withdrawal limits and time-locks.
* **Zero-Footprint:** A single, portable HTML/JS file under 1,000 lines of code.
* **True Self-Custody:** You own the keys. No middleman, no central server, and no "admin" access.
* **Local-First Security:** All sensitive data is encrypted locally using AES-256-GCM via the Web Crypto API.

---

## 🚀 Core Features
* **Multi-Vault Setup:** Create separate "Banks" for Rent, Savings, or Daily Allowance.
* **On-Chain Enforcement:** If you try to withdraw more than your daily limit, the **Solana Cluster** rejects the transaction at the protocol level.
* **Time-Window Lock:** Define an "Enforcement Window" (e.g., 90 days) where limits cannot be changed or removed.
* **Local Encryption:** Your master password derives a key that locks your private keys in `localStorage`.

---

## 🛠️ Getting Started
1.  **Download:** Save `mysol.html` to your local machine.
2.  **Open:** Double-click the file to run it in any modern browser (Chrome/Brave recommended).
3.  **Unlock:** Set a Master Password to initialize your local encrypted vault.
4.  **Create:** Initialize a new Piggy Bank by setting a **Daily Limit** and a **Lock Period**.
5.  **Deposit:** Send SOL to the generated Vault address.

---

## ⛓️ Technical Architecture


MySol utilizes **Program Derived Addresses (PDAs)**. Unlike a standard wallet, a PDA has no private key; it is controlled entirely by the logic of the smart contract. 
1. The **Smart Contract** checks the current timestamp and your `daily_withdrawn` balance.
2. If the request is within your limits, the contract signs the move of SOL to your main wallet.
3. If you exceed the limit, the transaction fails—even if you have your master password.

---

## ⚠️ Security Warning
* **No Password Reset:** There is no "Forgot Password" on the blockchain. If you lose your Master Password, your local vault cannot be decrypted.
* **Finalized Authority:** To achieve true immutability, the smart contract's upgrade authority must be revoked. This ensures even the developer cannot change the rules of your vault.

---

## 📜 License
MIT - Open Source and free to use for personal financial protection.
