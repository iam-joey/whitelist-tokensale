use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{Mint,TokenAccount,TokenInterface},
    associated_token::AssociatedToken
};
use crate::state::Pool;

#[derive(Accounts)]
pub struct InitializePool<'info>{
    #[account(mut)]
    pub author:Signer<'info>,
    #[account(
        init,
        payer=author,
        space=8+Pool::INIT_SPACE,
        seeds=[b"POOL",author.key().as_ref(),token_mint.key().as_ref()],
        bump
    )]
    pub pool_account:Account<'info,Pool>,
    pub token_mint:InterfaceAccount<'info,Mint>,
    #[account(
        init,
        payer=author,
        associated_token::mint=token_mint,
        associated_token::authority=pool_account,
    )]
    pub vault_tokens:InterfaceAccount<'info,TokenAccount>, //for sending tokens to users who got whitelisted
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program:Program<'info,System>,
}

impl<'info> InitializePool<'info>{
    pub fn handler(&mut self,allocation:u64,start:i64,end:i64,price:u64,bump:u8)->Result<()>{
        self.pool_account.init_pool(self.author.key(), self.token_mint.key(), allocation, start, end, price,bump)
    }
}