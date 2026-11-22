use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("9gBkptN51e1vf9VvjcQVqxobhWdcozcQ8Jbezhh6jYxb");

#[program]
pub mod liquidity_vault {
    use super::*;

    // Инициализация настроек Vault, хранилища токенов и share-mint
    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let config = &mut ctx.accounts.vault_config;
        config.authority = ctx.accounts.authority.key();
        config.asset_mint = ctx.accounts.asset_mint.key();
        config.share_mint = ctx.accounts.share_mint.key();
        config.vault_token_account = ctx.accounts.vault_token_account.key();
        config.total_assets = 0;
        config.bump = ctx.bumps.vault_config;

        msg!("Liquidity Vault каркас инициализирован");
        Ok(())
    }

    // Инициализация PDA-аккаунта позиции конкретного пользователя
    pub fn init_user_position(ctx: Context<InitUserPosition>) -> Result<()> {
        let position = &mut ctx.accounts.user_position;
        position.owner = ctx.accounts.user.key();
        position.vault = ctx.accounts.vault_config.key();
        position.deposited_amount = 0;
        position.bump = ctx.bumps.user_position;

        msg!("PDA позиции пользователя создан: {}", position.owner);
        Ok(())
    }
}

#[account]
pub struct VaultConfig {
    pub authority: Pubkey,           // 32
    pub asset_mint: Pubkey,          // 32 (входной токен)
    pub share_mint: Pubkey,          // 32 (share-токен)
    pub vault_token_account: Pubkey, // 32 (хранилище входных токенов)
    pub total_assets: u64,           // 8
    pub bump: u8,                    // 1
}

#[account]
pub struct UserPosition {
    pub owner: Pubkey,           // 32
    pub vault: Pubkey,           // 32
    pub deposited_amount: u64, // 8
    pub bump: u8,                // 1
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 32 + 32 + 8 + 1,
        seeds = [b"vault_config", asset_mint.key().as_ref()],
        bump
    )]
    pub vault_config: Account<'info, VaultConfig>,

    pub asset_mint: Account<'info, Mint>,

    // Share-токен, которым управляет PDA vault_config
    #[account(
        init,
        payer = authority,
        mint::decimals = asset_mint.decimals,
        mint::authority = vault_config,
        seeds = [b"share_mint", vault_config.key().as_ref()],
        bump
    )]
    pub share_mint: Account<'info, Mint>,

    // Токен-аккаунт самого vault для хранения входных токенов
    #[account(
        init,
        payer = authority,
        token::mint = asset_mint,
        token::authority = vault_config,
        seeds = [b"vault_token", vault_config.key().as_ref()],
        bump
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitUserPosition<'info> {
    #[account(
        seeds = [b"vault_config", vault_config.asset_mint.as_ref()],
        bump = vault_config.bump
    )]
    pub vault_config: Account<'info, VaultConfig>,

    #[account(
        init,
        payer = user,
        space = 8 + 32 + 32 + 8 + 1,
        seeds = [b"user_pos", vault_config.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub user_position: Account<'info, UserPosition>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}