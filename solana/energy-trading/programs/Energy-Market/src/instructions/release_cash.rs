use anchor_lang::prelude::*;
use crate::data_accounts::*;
use crate::errors::EnergyMarketErrors;

pub fn release_cash(ctx: Context<ReleaseCash>, amount: u16, price: u16) -> Result<()> {
    let target_account = &mut ctx.accounts.target;
    let bid_account = &mut ctx.accounts.bid_account;

    require!(**bid_account.to_account_info().try_borrow_lamports()? >= u64::from(price*amount), EnergyMarketErrors::NotEnoughLamports);

    **bid_account.to_account_info().try_borrow_mut_lamports()? -= u64::from(price*amount);
    **target_account.to_account_info().try_borrow_mut_lamports()? += u64::from(price*amount);

    Ok(())
}


#[derive(Accounts)]
pub struct ReleaseCash<'info> {
    #[account(mut,
        seeds = [b"bid", consumer.key().as_ref(), &[bid_account.bid_id]], bump = bid_account.bump
    )]
    pub bid_account: Account<'info, Bid>,
    /// CHECK: We only send SOL to this account, therefore we need no verification
    #[account(mut)]
    pub target: UncheckedAccount<'info>,
    /// CHECK: We only use this as to verify the bid account, and therefore need no verification of this account
    pub consumer: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}