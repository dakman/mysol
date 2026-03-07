use anchor_lang::prelude::*;

declare_id!("Ed3m1fhxygWysgyLSLryp3haQNcvMri8MkrqGvNDw4bt");

#[program]
pub mod mysol_program {
    use super::*;

    pub fn initialize_vault(ctx: Context<Initialize>, daily_limit: u64, enforce_days: i64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let clock = Clock::get()?;

        vault.owner = *ctx.accounts.user.key;
        vault.daily_limit = daily_limit;
        vault.last_withdraw_ts = 0;
        vault.withdrawn_today = 0;
        vault.expiry_date = clock.unix_timestamp + (enforce_days * 86400);

        msg!("Burn Complete. Rules live for {} days.", enforce_days);
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let clock = Clock::get()?;

        if clock.unix_timestamp < vault.expiry_date {
            // Rolling 24h reset
            if clock.unix_timestamp - vault.last_withdraw_ts > 86400 {
                vault.withdrawn_today = 0;
            }

            require!(
                vault.withdrawn_today + amount <= vault.daily_limit,
                VaultError::DailyLimitExceeded
            );

            vault.withdrawn_today += amount;
            vault.last_withdraw_ts = clock.unix_timestamp;
        }

        // Direct lamport transfer — correct approach for PDA-owned SOL
        **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok(())
    }

    /// Close vault and return rent to owner.
    /// NOTE: No expiry guard here — this is the DEVNET/TESTING version.
    /// Before mainnet deploy, re-add the expiry guard (see comment below).
    pub fn close_vault(_ctx: Context<CloseVault>) -> Result<()> {
        // ── ADD THIS BACK BEFORE MAINNET DEPLOY ──────────────────────────────
        // let vault = &_ctx.accounts.vault;
        // let clock = Clock::get()?;
        // require!(
        //     clock.unix_timestamp >= vault.expiry_date,
        //     VaultError::EnforcementActive
        // );
        // ─────────────────────────────────────────────────────────────────────
        msg!("Vault closed.");
        Ok(())
    }
}

// ── Account contexts ──────────────────────────────────────────────────────────

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 8 + 8 + 8,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump,
        constraint = vault.owner == user.key() @ VaultError::Unauthorized
    )]
    pub vault: Account<'info, VaultState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseVault<'info> {
    #[account(
        mut,
        close = user,
        seeds = [b"vault", user.key().as_ref()],
        bump,
        constraint = vault.owner == user.key() @ VaultError::Unauthorized
    )]
    pub vault: Account<'info, VaultState>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// ── State ─────────────────────────────────────────────────────────────────────

#[account]
pub struct VaultState {
    pub owner: Pubkey,          // 32
    pub daily_limit: u64,       // 8
    pub last_withdraw_ts: i64,  // 8
    pub withdrawn_today: u64,   // 8
    pub expiry_date: i64,       // 8
}

// ── Errors ────────────────────────────────────────────────────────────────────

#[error_code]
pub enum VaultError {
    #[msg("Daily limit exceeded. Blockchain enforcement active.")]
    DailyLimitExceeded,
    #[msg("Unauthorized. You are not the vault owner.")]
    Unauthorized,
    #[msg("Enforcement period still active. Cannot close vault early.")]
    EnforcementActive,
}