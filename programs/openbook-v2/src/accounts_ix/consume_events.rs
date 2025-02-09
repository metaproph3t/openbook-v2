use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ConsumeEvents<'info> {
    #[account(
        mut,
        has_one = event_queue,
    )]
    pub market: AccountLoader<'info, Market>,

    #[account(mut)]
    pub event_queue: AccountLoader<'info, EventQueue>,
}
