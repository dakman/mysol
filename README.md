# MySOL Vault: Immutable Spending Rules

**MySOL Vault** is a non-custodial Solana protocol that allows users to lock SOL into a Program Derived Address (PDA) governed by immutable, on-chain spending limits. 

This repository contains:
1.  **Smart Contract:** An Anchor-based program (`mysol_program`) that enforces limits.
2.  **Frontend:** A lightweight dApp (`mysol.html`) designed for mobile wallet browsers.

---

## 🔒 The Philosophy
Traditional "self-control" in crypto relies on willpower. **MySOL Vault** replaces willpower with code. By setting permanent rules on the blockchain, you create a financial environment where overspending is physically impossible at the protocol level.

---

## 🛠 Smart Contract Breakdown (Rust/Anchor)

The contract logic is built on **"Code is Law"** principles. It uses a **Program Derived Address (PDA)**, meaning the vault has no private key—it is controlled entirely by the logic of the program.



### 1. State Management
The program stores your rules in a `VaultState` account (72 bytes):
* **Daily Limit:** The maximum SOL allowed per 24-hour window.
* **Rolling Reset:** The limit isn't tied to a specific time of day. It uses a rolling 24-hour window based on your `last_withdraw_ts`.
* **Expiry Date:** A Unix timestamp that acts as a "Freedom Date." After this point, the program ignores the spending limit.

### 2. Core Logic: `withdraw`
When a withdrawal is requested, the program performs three critical checks:
1.  **Ownership:** Only the `owner` key that initialized the vault can sign for it.
2.  **Time Check:** If the current time is before `expiry_date`, enforcement is **ACTIVE**.
3.  **Math Check:** `withdrawn_today + request <= daily_limit`. If this is false, the **Solana Runtime** kills the transaction before any SOL can move.

---

## 🚀 Getting Started (User Guide)

### Prerequisites
* **Wallet:** Use the **Solflare** or **Phantom** mobile app.
* **Access:** Open `mysol.html` within the **built-in dApp browser** of these wallets.

### Operation Steps
1.  **Connect Wallet:** Link your wallet via the UI.
2.  **Burn Rules:** Set your Daily Limit and Enforcement Days. Once initialized, the blockchain enforces these rules—no exceptions.
3.  **Fund:** Send SOL to the Vault PDA. 
4.  **Spend:** Withdraw as needed. The dApp provides a live dashboard showing your "Remaining Today" balance and a usage progress bar.

---

## ⚠️ Security Architecture

* **Non-Custodial:** Funds are held by the program code on-chain, not by a third-party developer.
* **No Private Key:** PDA-owned SOL is moved via direct lamport adjustment, making it immune to traditional seed-phrase theft targeting the vault itself.
* **Mainnet Guard:** The `close_vault` function currently allows for early closing (Devnet mode). **Note:** Before Mainnet deployment, the `EnforcementActive` check must be enabled to prevent users from "deleting" their rules to bypass limits.

---

## 💻 Local Development

The frontend is a single-file SPA for transparency and speed.

**To run locally:**
1.  Clone this repository.
2.  Open `mysol.html` in your browser.
3.  Set your wallet to **Devnet** for testing.

```bash
# No dependencies required
open mysol.html