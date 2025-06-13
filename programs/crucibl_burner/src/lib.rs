use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};
use anchor_spl::token::{self, Burn, CloseAccount, Token, TokenAccount, Mint};

// Crucibl treasury fee wallet (devnet)
const FEE_WALLET: &str = "8Qg2XJ7kdzMJFLfnhAqF9U9evP1xG3LheVkgCM2YPqVZ";
const FEE_LAMPORTS: u64 = 500_000; // 0.0005 SOL in lamports

declare_id!("HBeNwmxJ1rqXni9mKpaBgr5q7yMbQpBF6VNiBAyQ6dNz");

#[program]
pub mod crucibl_burner {
    use super::*;

    pub fn purify(ctx: Context<Purify>, burn_amount: u64) -> Result<()> {
        // 1. Transfer 0.0005 SOL to the fee wallet
        let ix = system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.fee_wallet.key(),
            FEE_LAMPORTS,
        );
        invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.fee_wallet.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // 2. Burn SPL tokens from user's ATA
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::burn(cpi_ctx, burn_amount)?;

        // 3. Close the user's token account (reclaim rent)
        let cpi_ctx_close = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.user_token_account.to_account_info(),
                destination: ctx.accounts.user.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::close_account(cpi_ctx_close)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Purify<'info> {
    /// CHECK: This is a hardcoded fee wallet, not written to
    #[account(mut, address = FEE_WALLET.parse::<Pubkey>().unwrap())]
    pub fee_wallet: UncheckedAccount<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, has_one = mint, close = user)]
    pub user_token_account: Box<Account<'info, TokenAccount>>,
    pub mint: Box<Account<'info, Mint>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
