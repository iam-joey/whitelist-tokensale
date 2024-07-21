use anchor_lang:: prelude::*;
use anchor_spl::{token_interface::{Mint,TokenAccount,TokenInterface,CloseAccount},
associated_token::AssociatedToken};

#[allow(unused_doc_comments)]
#[allow(unused_variables)]
use crate::state::Pool;

use crate::error::MyErrors;

#[derive(Accounts)]
pub struct DeletePool<'info>{
    #[account(mut)]
    pub author:Signer<'info>,
    #[account(
        mut,
        seeds=[b"POOL",author.key().as_ref(),token_mint.key().as_ref()],
        close=author,
        bump,
        constraint=pool_account.author==author.key() @MyErrors::UnAuthorised,
        constraint=pool_account.mint==token_mint.key() @MyErrors::UnAuthorised,
        constraint=pool_account.allow_to_buy ==false @MyErrors::AlreadyStarted
    )]
    pub pool_account:Account<'info,Pool>,
    #[account(
        mut,
        associated_token::mint=token_mint,
        associated_token::authority=pool_account,
        associated_token::token_program=token_program
    )]
    pub vault:InterfaceAccount<'info,TokenAccount>,
    pub token_mint:InterfaceAccount<'info,Mint>,
    pub token_program:Interface<'info,TokenInterface>,
    pub system_program:Program<'info,System>,
    pub assocated_token:Program<'info,AssociatedToken>
}

impl<'info> DeletePool<'info> {
    pub fn delete_pool(&mut self)->Result<()>{
        let signer_seeds: [&[&[u8]]; 1] =[&[
            b"POOL",
            self.author.to_account_info().key().as_ref(),
            self.token_mint.to_account_info().key().as_ref(),
            &[self.pool_account.bump],
        ]];

        ///need to complete this some logic is left what if the author started and in between mint he wanted to delete the pool

        Ok(())
    }
}