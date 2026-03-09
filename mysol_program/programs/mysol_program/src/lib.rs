use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    self, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked,
};

declare_id!("Ed3m1fhxygWysgyLSLryp3haQNcvMri8MkrqGvNDw4bt");

#[program]
pub mod mysol_program {
    use super::*;

    pub fn initialize_vault(
        ctx: Context<Initialize>,
        daily_limit_sol: u64,
        daily_limit_usdc: u64,
        enforce_days: i64,
    ) -> Result<()> {
        let vault        = &mut ctx.accounts.vault;
        let clock        = Clock::get()?;
        require!(enforce_days > 0, VaultError::InvalidEnforcementDays);
        let enforce_seconds = enforce_days
            .checked_mul(86_400)
            .ok_or(error!(VaultError::MathOverflow))?;
        let expiry_date = clock
            .unix_timestamp
            .checked_add(enforce_seconds)
            .ok_or(error!(VaultError::MathOverflow))?;

        vault.owner              = *ctx.accounts.user.key;
        vault.usdc_mint          = ctx.accounts.usdc_mint.key();
        vault.daily_limit_sol    = daily_limit_sol;
        vault.daily_limit_usdc   = daily_limit_usdc;
        vault.last_withdraw_sol  = 0;
        vault.last_withdraw_usdc = 0;
        vault.withdrawn_sol      = 0;
        vault.withdrawn_usdc     = 0;
        vault.expiry_date        = expiry_date;
        msg!("Vault initialised. SOL: {} lamports/day, USDC: {} micro/day, {} days.",
            daily_limit_sol, daily_limit_usdc, enforce_days);
        Ok(())
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
        let vault_ai = ctx.accounts.vault.to_account_info();
        let vault = &mut ctx.accounts.vault;
        let clock = Clock::get()?;
        let rent_min = Rent::get()?.minimum_balance(vault_ai.data_len());
        let cur_lamports = vault_ai.lamports();
        let post_lamports = cur_lamports
            .checked_sub(amount)
            .ok_or(error!(VaultError::InsufficientVaultSol))?;
        require!(post_lamports >= rent_min, VaultError::InsufficientVaultSol);

        if clock.unix_timestamp < vault.expiry_date {
            if clock.unix_timestamp - vault.last_withdraw_sol > 86400 {
                vault.withdrawn_sol = 0;
            }
            let new_withdrawn = vault
                .withdrawn_sol
                .checked_add(amount)
                .ok_or(error!(VaultError::MathOverflow))?;
            require!(
                new_withdrawn <= vault.daily_limit_sol,
                VaultError::SolLimitExceeded
            );
            vault.withdrawn_sol    = new_withdrawn;
            vault.last_withdraw_sol = clock.unix_timestamp;
        }
        **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }

    pub fn withdraw_usdc(ctx: Context<WithdrawUsdc>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let clock = Clock::get()?;
        require_keys_eq!(
            ctx.accounts.usdc_mint.key(),
            vault.usdc_mint,
            VaultError::InvalidUsdcMint
        );

        if clock.unix_timestamp < vault.expiry_date {
            if clock.unix_timestamp - vault.last_withdraw_usdc > 86400 {
                vault.withdrawn_usdc = 0;
            }
            let new_withdrawn = vault
                .withdrawn_usdc
                .checked_add(amount)
                .ok_or(error!(VaultError::MathOverflow))?;
            require!(
                new_withdrawn <= vault.daily_limit_usdc,
                VaultError::UsdcLimitExceeded
            );
            vault.withdrawn_usdc    = new_withdrawn;
            vault.last_withdraw_usdc = clock.unix_timestamp;
        }

        // CPI into SPL Token / Token-2022 — vault PDA signs via invoke_signed
        let user_key = ctx.accounts.user.key();
        let seeds: &[&[u8]] = &[b"vault", user_key.as_ref(), &[ctx.bumps.vault]];

        token_interface::transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from:      ctx.accounts.vault_token_account.to_account_info(),
                    mint:      ctx.accounts.usdc_mint.to_account_info(),
                    to:        ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.vault.to_account_info(),
                },
                &[seeds],
            ),
            amount,
            6, // USDC decimals
        )?;
        Ok(())
    }

    pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
        let vault = &ctx.accounts.vault;
        let clock = Clock::get()?;
        require!(
            clock.unix_timestamp >= vault.expiry_date,
            VaultError::EnforcementActive
        );
        require_keys_eq!(
            ctx.accounts.usdc_mint.key(),
            vault.usdc_mint,
            VaultError::InvalidUsdcMint
        );
        require!(
            ctx.accounts.vault_token_account.amount == 0,
            VaultError::VaultUsdcNotEmpty
        );
        let vault_ai = ctx.accounts.vault.to_account_info();
        let rent_min = Rent::get()?.minimum_balance(vault_ai.data_len());
        require!(
            vault_ai.lamports() <= rent_min,
            VaultError::VaultSolNotEmpty
        );

        let user_key = ctx.accounts.user.key();
        let seeds: &[&[u8]] = &[b"vault", user_key.as_ref(), &[ctx.bumps.vault]];
        token_interface::close_account(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                CloseAccount {
                    account: ctx.accounts.vault_token_account.to_account_info(),
                    destination: ctx.accounts.user.to_account_info(),
                    authority: ctx.accounts.vault.to_account_info(),
                },
                &[seeds],
            ),
        )?;
        msg!("Vault closed.");
        Ok(())
    }

    #[cfg(feature = "devnet-reset")]
    pub fn reset_vault_devnet(ctx: Context<CloseVault>) -> Result<()> {
        let vault = &ctx.accounts.vault;
        require_keys_eq!(
            ctx.accounts.usdc_mint.key(),
            vault.usdc_mint,
            VaultError::InvalidUsdcMint
        );
        require!(
            ctx.accounts.vault_token_account.amount == 0,
            VaultError::VaultUsdcNotEmpty
        );
        let vault_ai = ctx.accounts.vault.to_account_info();
        let rent_min = Rent::get()?.minimum_balance(vault_ai.data_len());
        require!(
            vault_ai.lamports() <= rent_min,
            VaultError::VaultSolNotEmpty
        );

        let user_key = ctx.accounts.user.key();
        let seeds: &[&[u8]] = &[b"vault", user_key.as_ref(), &[ctx.bumps.vault]];
        token_interface::close_account(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                CloseAccount {
                    account: ctx.accounts.vault_token_account.to_account_info(),
                    destination: ctx.accounts.user.to_account_info(),
                    authority: ctx.accounts.vault.to_account_info(),
                },
                &[seeds],
            ),
        )?;
        msg!("Devnet reset complete.");
        Ok(())
    }
}

// ── Contexts ──────────────────────────────────────────────────────────────────

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 32,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultState>,
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = user,
        associated_token::mint = usdc_mint,
        associated_token::authority = vault,
        associated_token::token_program = token_program,
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
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

    /// USDC mint — used by transfer_checked to validate decimals
    pub usdc_mint: InterfaceAccount<'info, Mint>,

    /// Vault USDC ATA — must be owned by vault PDA, same mint
    #[account(
        mut,
        token::mint      = usdc_mint,
        token::authority = vault,
        token::token_program = token_program,
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

    /// User USDC ATA — receives tokens, same mint
    #[account(
        mut,
        token::mint      = usdc_mint,
        token::authority = user,
        token::token_program = token_program,
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
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        token::mint      = usdc_mint,
        token::authority = vault,
        token::token_program = token_program,
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}

// ── State ─────────────────────────────────────────────────────────────────────

#[account]
pub struct VaultState {
    pub owner: Pubkey,           // 32
    pub daily_limit_sol: u64,    //  8
    pub daily_limit_usdc: u64,   //  8
    pub last_withdraw_sol: i64,  //  8
    pub last_withdraw_usdc: i64, //  8
    pub withdrawn_sol: u64,      //  8
    pub withdrawn_usdc: u64,     //  8
    pub expiry_date: i64,        //  8
    pub usdc_mint: Pubkey,       // 32
    // Total: 8 (disc) + 32 + 7*8 + 32 = 128 bytes
}

// ── Errors ────────────────────────────────────────────────────────────────────

#[error_code]
pub enum VaultError {
    #[msg("SOL daily limit exceeded.")]
    SolLimitExceeded,
    #[msg("USDC daily limit exceeded.")]
    UsdcLimitExceeded,
    #[msg("Unauthorized.")]
    Unauthorized,
    #[msg("Enforcement period still active.")]
    EnforcementActive,
    #[msg("Invalid USDC mint for this vault.")]
    InvalidUsdcMint,
    #[msg("Vault still holds SOL above rent reserve.")]
    VaultSolNotEmpty,
    #[msg("Vault token account still holds USDC.")]
    VaultUsdcNotEmpty,
    #[msg("Insufficient SOL in vault.")]
    InsufficientVaultSol,
    #[msg("Invalid enforcement period.")]
    InvalidEnforcementDays,
    #[msg("Math overflow.")]
    MathOverflow,
}