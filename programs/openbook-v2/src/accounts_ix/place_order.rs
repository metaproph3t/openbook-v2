use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    #[account(mut)]
    pub open_orders_account: AccountLoader<'info, OpenOrdersAccountFixed>,
    pub owner: Signer<'info>,

    #[account(
        mut,
        has_one = bids,
        has_one = asks,
        has_one = event_queue,
        has_one = oracle,
    )]
    pub market: AccountLoader<'info, Market>,
    #[account(mut)]
    pub bids: AccountLoader<'info, BookSide>,
    #[account(mut)]
    pub asks: AccountLoader<'info, BookSide>,
    #[account(mut, constraint = payer.owner == owner.key())]
    pub payer: Account<'info, TokenAccount>,
    #[account(mut)]
    pub base_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub quote_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub event_queue: AccountLoader<'info, EventQueue>,

    /// CHECK: The oracle can be one of several different account types and the pubkey is checked above
    pub oracle: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
