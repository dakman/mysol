use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

declare_id!("Ed3m1fhxygWysgyLSLryp3haQNcvMri8MkrqGvNDw4bt");

// ── Devnet USDC mint (passed as account from frontend) ───────────────────────
// Devnet:  4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU
// Mainnet: EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v  ← swap before mainnet

#[program]
pub mod mysol_program {
    use super::*;

    // ── Initialize vault with both SOL and USDC limits ────────────────────────
    //
    //  daily_limit_sol  – max lamports per 24h window
    //  daily_limit_usdc – max USDC micro-units (6 decimals) per 24h window
    //  enforce_days     – how many days rules are active
    //
    pub fn initialize_vault(
        ctx: Context<Initialize>,
        daily_limit_sol: u64,
        daily_limit_usdc: u64,
        enforce_days: i64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let clock = Clock::get()?;

        vault.owner              = *ctx.accounts.user.key;
        vault.daily_limit_sol    = daily_limit_sol;
        vault.daily_limit_usdc   = daily_limit_usdc;
        vault.last_withdraw_sol  = 0;
        vault.last_withdraw_usdc = 0;
        vault.withdrawn_sol      = 0;
        vault.withdrawn_usdc     = 0;
        vault.expiry_date        = clock.unix_timestamp + (enforce_days * 86400);

        msg!(
            "Vault initialised. SOL limit: {} lamports, USDC limit: {} micro-USDC, active for {} days.",
            daily_limit_sol,
            daily_limit_usdc,
            enforce_days
        );
        Ok(())
    }

    // ── Withdraw SOL ──────────────────────────────────────────────────────────
    pub fn withdraw_sol(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let clock = Clock::get()?;

        if clock.unix_timestamp < vault.expiry_date {
            // Rolling 24h reset
            if clock.unix_timestamp - vault.last_withdraw_sol > 86400 {
                vault.withdrawn_sol = 0;
            }
            require!(
                vault.withdrawn_sol + amount <= vault.daily_limit_sol,
                VaultError::SolLimitExceeded
            );
            vault.withdrawn_sol     += amount;
            vault.last_withdraw_sol  = clock.unix_timestamp;
        }

        // Direct lamport transfer — correct for PDA-owned SOL
        **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok(())
    }

    // ── Withdraw USDC ─────────────────────────────────────────────────────────
    //
    //  Moves USDC from the vault's ATA → user's ATA via SPL Token CPI.
    //  The vault PDA signs via invoke_signed through Anchor's seeds/bump.
    //
    pub fn withdraw_usdc(ctx: Context<WithdrawUsdc>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let clock = Clock::get()?;

        if clock.unix_timestamp < vault.expiry_date {
            // Rolling 24h reset
            if clock.unix_timestamp - vault.last_withdraw_usdc > 86400 {
                vault.withdrawn_usdc = 0;
            }
            require!(
                vault.withdrawn_usdc + amount <= vault.daily_limit_usdc,
                VaultError::UsdcLimitExceeded
            );
            vault.withdrawn_usdc     += amount;
            vault.last_withdraw_usdc  = clock.unix_timestamp;
        }

        // CPI into SPL Token program — vault PDA signs via invoke_signed
        let user_key = ctx.accounts.user.key();
        let seeds: &[&[u8]] = &[
            b"vault",
            user_key.as_ref(),
            &[ctx.bumps.vault],
        ];
        let signer_seeds = &[seeds];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from:      ctx.accounts.vault_token_account.to_account_info(),
                mint:      ctx.accounts.usdc_mint.to_account_info(),
                to:        ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
            },
            signer_seeds,
        );
        // USDC has 6 decimals
        token_interface::transfer_checked(cpi_ctx, amount, 6)?;

        Ok(())
    }

    // ── Close vault ───────────────────────────────────────────────────────────
    //  NOTE: expiry guard is COMMENTED OUT for devnet testing.
    //  Uncomment before mainnet deploy.
    pub fn close_vault(_ctx: Context<CloseVault>) -> Result<()> {
        // ── RE-ENABLE BEFORE MAINNET ─────────────────────────────────────────
        // let vault = &_ctx.accounts.vault;
        // let clock = Clock::get()?;
        // require!(
        //     clock.unix_timestamp >= vault.expiry_date,
        //     VaultError::EnforcementActive
        // );
        // ─────────────────────────────────────────────────────────────────────
        msg!("Vault closed. Rent returned to owner.");
        Ok(())
    }
}

// ── Account contexts ──────────────────────────────────────────────────────────

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        // 8 discriminator + 32 owner + 8*6 u64/i64 fields + 8 expiry = 96 bytes
        space = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 8,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawSol<'info> {
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
pub struct WithdrawUsdc<'info> {
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump,
        constraint = vault.owner == user.key() @ VaultError::Unauthorized
    )]
    pub vault: Account<'info, VaultState>,

    /// USDC mint — needed for transfer_checked
    pub usdc_mint: InterfaceAccount<'info, Mint>,

    /// Vault's USDC ATA — must be owned by vault PDA
    #[account(
        mut,
        token::mint      = usdc_mint,
        token::authority = vault,
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

    /// User's USDC ATA — receives the tokens
    #[account(
        mut,
        token::mint      = usdc_mint,
        token::authority = user,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
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
    pub owner: Pubkey,              // 32 — vault owner
    pub daily_limit_sol: u64,       //  8 — max lamports per 24h
    pub daily_limit_usdc: u64,      //  8 — max USDC micro-units per 24h
    pub last_withdraw_sol: i64,     //  8 — unix ts of last SOL withdrawal
    pub last_withdraw_usdc: i64,    //  8 — unix ts of last USDC withdrawal
    pub withdrawn_sol: u64,         //  8 — lamports withdrawn in current window
    pub withdrawn_usdc: u64,        //  8 — USDC micro-units withdrawn in current window
    pub expiry_date: i64,           //  8 — enforcement ends at this unix ts
    // Total: 8 (disc) + 32 + 7*8 = 96 bytes
}

// ── Errors ────────────────────────────────────────────────────────────────────

#[error_code]
pub enum VaultError {
    #[msg("SOL daily limit exceeded. Blockchain enforcement active.")]
    SolLimitExceeded,
    #[msg("USDC daily limit exceeded. Blockchain enforcement active.")]
    UsdcLimitExceeded,
    #[msg("Unauthorized. You are not the vault owner.")]
    Unauthorized,
    #[msg("Enforcement period still active. Cannot close vault early.")]
    EnforcementActive,
    #[msg("Invalid token mint. Only USDC is accepted.")]
    InvalidMint,
}