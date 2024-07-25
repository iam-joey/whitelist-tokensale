use anchor_lang::prelude::*;

use anchor_spl::token_interface::Mint;

use crate::state::{Pool, User};

#[derive(Accounts)]
pub struct JoinWhitelist<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer=user,
        seeds=[b"USER_ACCOUNT",user.key().as_ref(),pool_account.key().as_ref()],
        bump,
        space=8+User::INIT_SPACE
    )]
    pub user_account: Account<'info, User>,
    #[account(
        mut,
        seeds=[b"POOL",author.key().as_ref(),token_mint.key().as_ref()],
        bump=pool_account.bump
    )]
    pub pool_account: Account<'info, Pool>,
    pub author: AccountInfo<'info>,
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

impl<'info> JoinWhitelist<'info> {
    pub fn init(&mut self, bump: u8) -> Result<()> {
        self.user_account.init(self.pool_account.key(), bump)
    }
}
