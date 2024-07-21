use anchor_lang::prelude::*;

use anchor_spl::{associated_token::AssociatedToken, token::{Token, Transfer,transfer}, token_interface::{Mint,TokenAccount}};

use crate::state::Pool;


#[derive(Accounts)]
pub struct ApproveBuy<'info>{
    #[account(mut)]
    pub author:Signer<'info>,
    #[account(
        mut,
        seeds=[b"POOL",author.key().as_ref(),token_mint.key().as_ref()],
        bump=pool_account.bump,
    )]
    pub pool_account:Account<'info,Pool>,
    pub token_mint:InterfaceAccount<'info,Mint>,
     #[account(
        mut,
        associated_token::mint=token_mint,
        associated_token::authority=pool_account,
    )]
    pub vault_tokens:InterfaceAccount<'info,TokenAccount>,
    #[account(
        mut,
        associated_token::mint=token_mint,
        associated_token::authority=author
    )] 
    pub author_ata:InterfaceAccount<'info,TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program:Program<'info,Token>
}

impl<'info> ApproveBuy<'info>{
    pub fn make_it_buy(&mut self,amount:u64)->Result<()>{
        self.pool_account.allow_buy().expect("Something went wrong in approve buy");
        self.transfer_alloc_tokens_to_vault(amount)
    }
    pub fn transfer_alloc_tokens_to_vault(&mut self,amount:u64)->Result<()>{

        let transfer_instruction=Transfer{
            from:self.author_ata.to_account_info(),
            to:self.vault_tokens.to_account_info(),
            authority:self.author.to_account_info()
        };

       let cpi_context= CpiContext::new(self.token_program.to_account_info(), transfer_instruction);

       transfer(cpi_context, amount)

    }
}