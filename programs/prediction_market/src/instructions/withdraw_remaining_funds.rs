use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};
use crate::state::market::Market;
use crate::error::CustomError;

pub fn handler(ctx: Context<WithdrawRemainingFunds>) -> Result<()> {
    let market = &mut ctx.accounts.market;
  

    // Ensure there are funds to withdraw
    let remaining_funds = ctx.accounts.market_token_account.amount;
    require!(remaining_funds > 0, CustomError::NoRemainingFunds);

    // Transfer payout tokens to user
    let market_id_bytes: [u8; 8] = market.market_id.to_le_bytes();
    let seeds = &[b"market", &market_id_bytes[..], &[market.bump]];
    let signer_seeds = &[&seeds[..]];

    // Transfer remaining funds to the admin or treasury
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.market_token_account.to_account_info(),
            to: ctx.accounts.fee_recipient_token_account.to_account_info(),
            authority: market.to_account_info(),
        },
        signer_seeds
    );
    token::transfer(transfer_ctx, remaining_funds)?;

    msg!(
        "Transferred {} remaining tokens to admin account",
        remaining_funds
    );

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawRemainingFunds<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,

    #[account(mut)]
    pub market_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub fee_recipient_token_account: Account<'info, TokenAccount>,

    pub fee_recipient: Signer<'info>,

    pub token_program: Program<'info, Token>,
}
