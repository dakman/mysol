use anchor_lang::prelude::*;

declare_id!("Ed3m1fhxygWysgyLSLryp3haQNcvMri8MkrqGvNDw4bt");

const SECONDS_PER_DAY: i64 = 86400;

#[program]
pub mod mysol_program {
    use super::*;

    pub fn initialize_vault(ctx: Context<Initialize>, daily_limit: u64, enforce_days: i64) -> Result<()> {
        require!(daily_limit > 0, VaultError::InvalidAmount);
        require!(enforce_days > 0, VaultError::InvalidDuration);

        let vault = &mut ctx.accounts.vault;
        let clock = Clock::get()?;

        vault.owner = *ctx.accounts.user.key;
        vault.daily_limit = daily_limit;
        vault.last_withdraw_ts = 0;
        vault.withdrawn_today = 0;
        vault.expiry_date = clock
            .unix_timestamp
            .checked_add(
                enforce_days
                    .checked_mul(SECONDS_PER_DAY)
                    .ok_or(VaultError::ArithmeticOverflow)?,
            )
            .ok_or(VaultError::ArithmeticOverflow)?;

        msg!("Vault initialized. Rules live for {} days.", enforce_days);
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::InvalidAmount);

        let vault = &mut ctx.accounts.vault;
        let clock = Clock::get()?;

        if clock.unix_timestamp < vault.expiry_date {
            // Rolling 24h reset
            let elapsed = clock
                .unix_timestamp
                .checked_sub(vault.last_withdraw_ts)
                .ok_or(VaultError::ArithmeticOverflow)?;

            if elapsed > SECONDS_PER_DAY {
                vault.withdrawn_today = 0;
            }

            let new_total = vault
                .withdrawn_today
                .checked_add(amount)
                .ok_or(VaultError::ArithmeticOverflow)?;

            require!(
                new_total <= vault.daily_limit,
                VaultError::DailyLimitExceeded
            );

            vault.withdrawn_today = new_total;
            vault.last_withdraw_ts = clock.unix_timestamp;
        }

        let vault_lamports = vault.to_account_info().lamports();
        require!(vault_lamports >= amount, VaultError::InsufficientFunds);

        // Direct lamport transfer — correct approach for PDA-owned SOL
        **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += amount;

        msg!("Withdrew {} lamports.", amount);
        Ok(())
    }

    pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
        let vault = &ctx.accounts.vault;
        let clock = Clock::get()?;

        require!(
            clock.unix_timestamp >= vault.expiry_date,
            VaultError::EnforcementActive
        );

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
    #[msg("Amount must be greater than zero.")]
    InvalidAmount,
    #[msg("Vault has insufficient funds for this withdrawal.")]
    InsufficientFunds,
    #[msg("Arithmetic overflow detected.")]
    ArithmeticOverflow,
    #[msg("Enforcement duration must be greater than zero days.")]
    InvalidDuration,
}
