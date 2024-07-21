use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::{error::MyErrors, state::{Pool, User}};

#[derive(Accounts)]
pub struct LeaveWhiteList<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        mut,
        seeds=[b"USER_ACCOUNT",user.key().as_ref(),pool_account.key().as_ref()],
        bump=user_account.bump,
        close=user
    )]
    pub user_account:Account<'info,User>,
   #[account(
        mut,
        seeds=[b"POOL",author.key().as_ref(),token_mint.key().as_ref()],
        bump=pool_account.bump
    )]
    pub pool_account:Account<'info,Pool>,
    pub author:AccountInfo<'info>,
    pub token_mint:InterfaceAccount<'info,Mint>
}

impl<'info> LeaveWhiteList<'info>{
    pub fn leave_whitelist(&mut self)->Result<()>{
    require!(self.pool_account.allow_to_buy==true,MyErrors::CantLeaveWhiteList);
    self.pool_account.descrease_candidate()
    }
}

