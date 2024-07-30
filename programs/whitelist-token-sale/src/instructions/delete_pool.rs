use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{close_account, CloseAccount, Mint, TokenAccount, TokenInterface},
};

#[allow(unused_doc_comments)]
#[allow(unused_variables)]
use crate::state::Pool;

use crate::error::MyErrors;

#[derive(Accounts)]
pub struct DeletePool<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        mut,
        seeds=[b"POOL",author.key().as_ref(),token_mint.key().as_ref()],
        close=author,
        bump,
        constraint=pool_account.author==author.key() @MyErrors::UnAuthorised,
        constraint=pool_account.mint==token_mint.key() @MyErrors::UnAuthorised,
        constraint=pool_account.allow_to_buy==false @MyErrors::AlreadyStarted
    )]
    pub pool_account: Account<'info, Pool>,
    #[account(
        mut,
        associated_token::mint=token_mint,
        associated_token::authority=pool_account,
        associated_token::token_program=token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub assocated_token: Program<'info, AssociatedToken>,
}

impl<'info> DeletePool<'info> {
    pub fn delete_pool(&mut self) -> Result<()> {
        //close the vault account;
        let accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.author.to_account_info(),
            authority: self.pool_account.to_account_info(),
        };
        let context = CpiContext::new(self.token_program.to_account_info(), accounts);
        close_account(context)
        // Ok(())
    }
}
