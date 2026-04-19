# MySOL Vault: Immutable Spending Rules

**MySOL Vault** is a non-custodial Solana protocol that allows users to lock **SOL** and **USDC** into a Program Derived Address (PDA) governed by immutable, on-chain spending limits.

* **Live dApp:** [Launch MySOL Vault (GitHub Pages)](https://dakman.github.io/mysol/mysol.html) 
    * *Default network is now **Mainnet**. Switch to **Devnet** in the UI for free testing.*
    * *The app uses an RPC endpoint for blockchain reads and transaction relay. Users can change the RPC from the UI if they want to use their own provider.*
* **On-Chain Program:** [View on Solscan (Mainnet)](https://solscan.io/account/2EHg4iqQxpi5ZuftbDrTw2XoKR5HM56AEbo8Am4rSTRV)
* **Smart Contract Logic:** The core enforcement rules are defined in [`lib.rs`](./mysol_program/programs/mysol_program/src/lib.rs).

![MySOL Vault — Live Dashboard](./screenshot.png)

---

## ✅ Mainnet Status

**As of April 19, 2026, MySOL Vault is live on Solana mainnet.**

* **Mainnet Program ID:** `2EHg4iqQxpi5ZuftbDrTw2XoKR5HM56AEbo8Am4rSTRV`
* **Observed deploy cost:** about `1.6597 SOL`
* **Per-vault account creation rent:** about `0.00382 SOL` plus transaction fees
* **Devnet** is still available for testing and demo usage.

---

## Motivations: Why Use a Spending Vault?

The core motivation is **self-sovereign discipline for high-risk behavior**. This project is aimed at degens and gamblers who want hard on-chain friction between a win and the impulse to blow it back.

* **Anti-Chasing Guardrail:** After a big win, funds in a normal wallet can be redeposited instantly. MySOL adds daily withdrawal constraints so you cannot immediately loop winnings back into higher-risk bets.
* **Willpower as a Service:** By moving spending limits to the blockchain, you outsource discipline to deterministic program rules instead of emotion in the moment.
* **Damage Limiter:** This is a harm-reduction tool. It will not stop losses completely, but it can limit how fast you can lose money in a single day.

---

## 🛠 Multi-Asset Support: SOL & USDC

MySOL Vault handles both native Solana and SPL Tokens (specifically USDC).

* **Native SOL:** Handled via direct lamport reassignment from the Vault PDA to the user.
* **USDC (SPL Token):** The vault creates an Associated Token Account (ATA) owned by the Vault PDA. Withdrawals are executed via `transfer_checked` CPI (Cross-Program Invocation).
* **Mainnet USDC Mint:** `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`
* **RPC:** The frontend uses an RPC endpoint to read chain data and broadcast signed transactions. Users can override the RPC from the app UI.

---

## 🔎 How It Works

### 1. Program Derived Vault (PDA)
Each wallet gets a deterministic vault address derived from:
* seed `"vault"`
* the user wallet pubkey
* seed `"v2"`

This means the same wallet always maps to the same vault for this program.

### 2. No Private Key for the Vault
The vault is a PDA, not a normal wallet account. There is no seed phrase or private key to import/export for it. Funds can only move through valid program instructions signed by the owner wallet.

### 1. The Rolling 24-Hour Window
Unlike systems that reset at a fixed time, MySOL Vault uses a **Relative Rolling Window**:
* When you withdraw, the program records the `unix_timestamp`.
* On the next attempt, the program checks if `> 86,400 seconds` (24 hours) have passed since the *last* withdrawal.
* If yes, your "Spent Today" counter resets, allowing for a new withdrawal.

### 1b. Adjustable Enforcement Unit (Days or Minutes)
Vault creation supports an adjustable enforcement interval unit:
* **Days (`0`)**: default mode for normal usage.
* **Minutes (`1`)**: testing mode for rapid validation (for example, a 1-minute expiry on devnet).

This changes how `expiry_date` is computed during `initialize_vault`.

### 2. Logic Gatekeepers
* **`initialize_vault`**: Writes the rules. Once set, the `expiry_date` is a hard-coded deadline.
* **`withdraw_sol` / `withdraw_usdc`**: Enforce daily limits on-chain. Over-limit transactions fail at runtime even from custom frontends.
* **`close_vault`**: Closes only when conditions are met (post-enforcement and empty vault accounts).
* **`reset_vault_devnet` / `end_enforcement_devnet`**: Devnet testing helpers (feature-gated).

---

## 🎯 Use Cases

* **The Profit Protector:** Lock away daily trading profits in USDC so you don't trade them back into the market during a "tilt."
* **The Big-Win Lockbox:** After a large gambling/trading win, move funds into the vault so you can't immediately redeposit and chase bigger wins or recover losses impulsively.
* **The Living Allowance:** Deposit your monthly budget in USDC and set a daily limit (e.g., $50/day) to ensure your rent money lasts.
* **The Damage-Control Layer:** Keep a larger stack in the vault and expose only daily-sized amounts to impulsive decisions. This is a behavioral guardrail, not a full wallet-security solution.

---

## 🚀 Getting Started

### Prerequisites
* **Wallet:** Use the **Solflare** or **Phantom** mobile app.
* **Environment:** Open the [Live Link](https://dakman.github.io/mysol/mysol.html) within the **built-in dApp browser** of these wallets.
* **Network:** Mainnet is live by default. Switch to **Devnet** in the top-right selector if you want faucet-based testing first.
* **RPC:** Use the `RPC` link in the header if you want to point the app at your own RPC provider.

### Operation Steps
1.  **Connect Wallet:** Link your wallet via the UI.
2.  **Burn Rules:** Set your Daily Limit (for SOL and USDC), enforcement interval, and unit (Days or Minutes for testing).
3.  **Fund:** Send SOL or USDC to the Vault PDA address shown in your dashboard.
4.  **Spend:** Withdraw as needed. The dApp provides a live dashboard showing your "Remaining Today" balance.

---

## 📊 Program Metadata

| Field | Value |
| :--- | :--- |
| **Program ID** | `2EHg4iqQxpi5ZuftbDrTw2XoKR5HM56AEbo8Am4rSTRV` |
| **Framework** | Anchor 0.32.1 |
| **Account Seeds** | `[b"vault", user_pubkey, b"v2"]` |
| **Vault Account Space** | 128 bytes |

---

## ⚠️ Security Architecture

* **Non-Custodial:** Funds are held by the program code on-chain, not by a third-party developer.
* **On-Chain Enforcement:** Limits are checked by program logic, not by UI state.
* **Wallet-Scoped Authority:** Vault actions require the original wallet authority checks.
* **Devnet Helpers:** Reset/end-enforcement helpers are for testing workflows.

---

## ⚠️ Disclaimer

Use MySOL Vault at your own risk. Developer(s) are not liable for lost funds.  
Vault withdrawals/deposits require valid on-chain instructions signed by the original vault owner wallet; this can be done via this app or other compatible Solana clients.  
One vault per wallet.
This project is not financial advice.

---

## 💻 Local Development

**To run locally:**
1.  Clone this repository.
2.  Open `mysol.html` in your browser or dApp browser.
3.  Set your wallet to **Devnet** for testing.

```bash
# No dependencies required to view the UI
open mysol.html
