use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenAccount;

use crate::state::Pool;


#[derive(Accounts)]
pub struct ClosePool<'info>{
    #[account(mut)]
    pub author:Signer<'info>,
    pub token_mint:InterfaceAccount<'info,TokenAccount>,
    #[account(
        mut,
        seeds=[b"POOL",author.key().as_ref(),token_mint.key().as_ref()],
        bump=pool_account.bump,
    )]
    pub pool_account:Account<'info,Pool>,
}

impl<'info>  ClosePool<'info>{
    pub fn stop_pool(&mut self)->Result<()> {
        self.pool_account.stop_pool()
    }
}