# MySOL Vault: Immutable Spending Rules

**MySOL Vault** is a non-custodial Solana protocol that allows users to lock **SOL** and **USDC** into a Program Derived Address (PDA) governed by immutable, on-chain spending limits.

* **Live dApp:** [Launch MySOL Vault (GitHub Pages)](https://dakman.github.io/mysol/mysol.html) 
    * *Note: The dApp is set to **Devnet** mode by default for safe testing.*
* **On-Chain Program:** [View on Solscan (Devnet)](https://solscan.io/account/Ed3m1fhxygWysgyLSLryp3haQNcvMri8MkrqGvNDw4bt?cluster=devnet)
* **Smart Contract Logic:** The core enforcement rules are defined in [`lib.rs`](./lib.rs).

---

## Motivations: Why Use a Spending Vault?

The core motivation is **Self-Sovereign Discipline**. In a 24/7 liquid market, the greatest risk to a user's capital is often their own impulsive behavior.

* **Eliminating "Hot Wallet" Risk:** Traditional wallets allow you to drain 100% of your funds in seconds. If your phone is snatched or you experience a moment of poor judgment, your capital is gone.
* **Willpower as a Service:** By moving enforcement to the blockchain, you outsource your discipline to an immutable auditor. The network simply won't let you overspend.
* **Anti-Extortion:** In a "wrench attack," an attacker can only force you to withdraw up to your daily limit. The bulk of your capital remains locked behind a time-based wall they cannot break.

---

## 🛠 Multi-Asset Support: SOL & USDC

MySOL Vault handles both native Solana and SPL Tokens (specifically USDC).

* **Native SOL:** Handled via direct lamport reassignment from the Vault PDA to the user.
* **USDC (SPL Token):** The vault creates an Associated Token Account (ATA) owned by the Vault PDA. Withdrawals are executed via `transfer_checked` CPI (Cross-Program Invocation).



### 1. The Rolling 24-Hour Window
Unlike systems that reset at a fixed time, MySOL Vault uses a **Relative Rolling Window**:
* When you withdraw, the program records the `unix_timestamp`.
* On the next attempt, the program checks if `> 86,400 seconds` (24 hours) have passed since the *last* withdrawal.
* If yes, your "Spent Today" counter resets, allowing for a new withdrawal.

### 2. Logic Gatekeepers
* **`initialize_vault`**: Writes the rules. Once set, the `expiry_date` is a hard-coded deadline.
* **`withdraw`**: The gatekeeper. It checks your balance, your limit, and the clock before executing the transfer.



---

## 🎯 Use Cases

* **The Profit Protector:** Lock away daily trading profits in USDC so you don't trade them back into the market during a "tilt."
* **The Living Allowance:** Deposit your monthly budget in USDC and set a daily limit (e.g., $50/day) to ensure your rent money lasts.
* **The Security Layer:** Keep your primary stack in the Vault. If your mobile wallet is compromised, a thief can only "trickle" out small amounts daily, giving you time to respond.

---

## 🚀 Getting Started

### Prerequisites
* **Wallet:** Use the **Solflare** or **Phantom** mobile app.
* **Environment:** Open the [Live Link](https://dakman.github.io/mysol/mysol.html) within the **built-in dApp browser** of these wallets.

### Operation Steps
1.  **Connect Wallet:** Link your wallet via the UI.
2.  **Burn Rules:** Set your Daily Limit (for SOL and USDC) and Enforcement Days.
3.  **Fund:** Send SOL or USDC to the Vault PDA address shown in your dashboard.
4.  **Spend:** Withdraw as needed. The dApp provides a live dashboard showing your "Remaining Today" balance.

---

## 📊 Program Metadata

| Field | Value |
| :--- | :--- |
| **Program ID** | `Ed3m1fhxygWysgyLSLryp3haQNcvMri8MkrqGvNDw4bt` |
| **Framework** | Anchor 0.30.1 |
| **Account Seeds** | `[b"vault", user_pubkey]` |
| **Account Space** | 8 + 32 + 8 + 8 + 8 + 8 (72 bytes) |

---

## ⚠️ Security Architecture

* **Non-Custodial:** Funds are held by the program code on-chain, not by a third-party developer.
* **Mainnet Guard:** The `close_vault` function currently allows for early closing (Devnet mode). 
* **Crucial:** Before Mainnet deployment, the `EnforcementActive` check in [`lib.rs`](./lib.rs) must be enabled to prevent users from "deleting" their rules to bypass limits.

---

## 💻 Local Development

**To run locally:**
1.  Clone this repository.
2.  Open `mysol.html` in your browser or dApp browser.
3.  Set your wallet to **Devnet** for testing.

```bash
# No dependencies required to view the UI
open mysol.html
