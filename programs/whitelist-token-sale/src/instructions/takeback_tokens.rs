use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Transfer},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{error::MyErrors, state::Pool};

#[derive(Accounts)]
pub struct TakeBackTokens<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint=token_mint,
        associated_token::authority=token_mint
    )]
    pub author_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint=token_mint,
        associated_token::authority=pool_account
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds=[b"POOL",author.key().as_ref(),token_mint.key().as_ref()],
        bump=pool_account.bump
    )]
    pub pool_account: Account<'info, Pool>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token: Program<'info, AssociatedToken>,
}

impl<'info> TakeBackTokens<'info> {
    pub fn handle_it(&mut self) -> Result<()> {
        //take back the tokens which are left
        require!(!self.pool_account.allow_to_buy, MyErrors::OngoingPoolError);
        let clock = Clock::get()?;
        let time = clock.unix_timestamp;
        require!(
            self.pool_account.end_time < time,
            MyErrors::OngoingPoolError
        );
        let left_token = self
            .pool_account
            .allocation
            .checked_sub(self.pool_account.sold_tokens)
            .unwrap();
        if left_token == 0 {
            return Err(MyErrors::PoolEmpty.into());
        }
        let author = self.author.key();
        let token_mint = self.token_mint.key();
        let seed = &[
            b"POOL",
            author.as_ref(),
            token_mint.as_ref(),
            &[self.pool_account.bump],
        ];
        let signer_seeds = &[&seed[..]];
        let accounts = Transfer {
            from: self.vault_ata.to_account_info(),
            to: self.author_ata.to_account_info(),
            authority: self.pool_account.to_account_info(),
        };
        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        transfer(cpi_context, left_token)
    }
}
