use anchor_lang::prelude::*;

use crate::{
    error::MyErrors,
    state::{Pool, User},
};
use anchor_spl::token_interface::Mint;

#[derive(Accounts)]
pub struct WhiteListUser<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        mut,
        seeds=[b"USER_ACCOUNT",user.key().as_ref(),pool_account.key().as_ref()],
        bump=user_account.bump
    )]
    pub user_account: Account<'info, User>,
    pub user: AccountInfo<'info>,
    #[account(
        mut,
        seeds=[b"POOL",author.key().as_ref(),token_mint.key().as_ref()],
        bump=pool_account.bump
    )]
    pub pool_account: Account<'info, Pool>,
    pub token_mint: InterfaceAccount<'info, Mint>,
}

impl<'info> WhiteListUser<'info> {
    pub fn whitelist_user(&mut self, limit: u64) -> Result<()> {
        require!(
            self.user_account.pool_address == self.pool_account.key(),
            MyErrors::InvalidPoolUser
        );
        self.user_account.whitelist_user(true, limit)
    }
}
