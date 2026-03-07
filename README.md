# MySOL Vault: Immutable Spending Rules

**MySOL Vault** is a non-custodial Solana protocol that allows users to lock SOL into a Program Derived Address (PDA) governed by immutable, on-chain spending limits. 

This repository contains:
1. **Smart Contract:** An Anchor-based program (`mysol_program`) that enforces limits.
2. **Frontend:** A lightweight dApp (`mysol.html`) designed for mobile wallet browsers.

---

## 🧠 Motivations: Why Use a Spending Vault?

The core motivation is **Self-Sovereign Discipline**. In a 24/7 liquid market, the greatest risk to a user's capital is often their own impulsive behavior.

* **Eliminating "Hot Wallet" Risk:** Traditional wallets allow you to drain 100% of your funds in seconds. If your phone is snatched or you experience a moment of poor judgment, your capital is gone.
* **Willpower as a Service:** By moving enforcement to the blockchain, you outsource your discipline to an immutable auditor. You don't have to "try" to spend less; the network simply won't let you.
* **Anti-Extortion:** In a hypothetical "wrench attack," an attacker can only force you to withdraw up to your daily limit. The rest of your capital remains locked behind a time-based wall they cannot break.

---

## 🛠 How It Works: Technical Deep Dive

The protocol uses a **Program Derived Address (PDA)**. Unlike a standard wallet, a PDA has no private key. It can only be moved by the logic defined in the smart contract.



### 1. The Rolling 24-Hour Window
Unlike systems that reset at a fixed time (like Midnight UTC), MySOL Vault uses a **Relative Rolling Window**:
* When you withdraw, the program records the `unix_timestamp`.
* On the next withdrawal attempt, the program calculates the time elapsed.
* If the difference is `> 86,400 seconds` (24 hours), your "Spent Today" counter resets to zero.

### 2. Logic Gatekeepers
* **`initialize_vault`**: Writes the rules. Once set, the `expiry_date` is a hard-coded deadline.
* **`withdraw`**: The gatekeeper. It checks your balance, your limit, and the clock before executing a direct lamport transfer.



---

## 🎯 Use Cases

| User Profile | Use Case |
| :--- | :--- |
| **The Trader** | Lock away "Profit" into the vault so it cannot be revenge-traded back into the market the same day. |
| **The Student** | Set a daily allowance for living expenses while keeping the bulk of your SOL locked until the semester ends. |
| **The Security Conscious** | Keep your primary stack in a Vault. If your mobile wallet is compromised, a thief can only "trickle" out small amounts daily. |
| **The Long-Term HODLer** | Use a 365-day enforcement period with a low limit to ensure you don't panic-sell during a pump. |

---

## 🚀 Getting Started

### Prerequisites
* **Wallet:** Use the **Solflare** or **Phantom** mobile app.
* **Environment:** Open `mysol.html` within the **built-in dApp browser** of these wallets.

### Operation Steps
1. **Connect Wallet:** Link your wallet via the UI.
2. **Burn Rules:** Set your Daily Limit and Enforcement Days. Once initialized, the blockchain enforces these rules—no exceptions.
3. **Fund:** Send SOL to the Vault PDA. 
4. **Spend:** Withdraw as needed. The dApp provides a live dashboard showing your "Remaining Today" balance and a usage progress bar.

---

## ⚠️ Security Architecture

* **Non-Custodial:** Funds are held by the program code on-chain, not by a third-party developer.
* **Mainnet Guard:** The `close_vault` function currently allows for early closing (Devnet mode). 
* **Note:** Before Mainnet deployment, the `EnforcementActive` check in the Rust code must be enabled to prevent users from "deleting" their rules to bypass limits.

---

## 💻 Local Development

**To run locally:**
1. Clone this repository.
2. Open `mysol.html` in your browser or dApp browser.
3. Set your wallet to **Devnet** for testing.

```bash
# No dependencies required to view the UI
open mysol.html

Field,Value
Program ID,Ed3m1fhxygWysgyLSLryp3haQNcvMri8MkrqGvNDw4bt
Framework,Anchor 0.30.1
Account Seeds,"[b""vault"", user_pubkey]"
Account Space,8 + 32 + 8 + 8 + 8 + 8 (72 bytes)