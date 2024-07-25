use anchor_lang::prelude::*;

use crate::error::MyErrors;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub is_whitelisted: bool,
    pub pool_address: Pubkey, //pool he's joining
    pub limit_amount: u64,
    pub user_bought: u64,
    pub bump: u8,
}

impl User {
    pub fn init(&mut self, pool_address: Pubkey, bump: u8) -> Result<()> {
        self.is_whitelisted = false;
        self.pool_address = pool_address;
        self.limit_amount = 0;
        self.user_bought = 0;
        self.bump = bump;
        Ok(())
    }

    pub fn whitelist_user(&mut self, is_whitelisted: bool, limit_amount: u64) -> Result<()> {
        require!(!self.is_whitelisted, MyErrors::AlreadyWhitelisted);
        self.is_whitelisted = is_whitelisted;
        self.limit_amount = limit_amount;
        Ok(())
    }

    pub fn remove_whitelist_user(&mut self) -> Result<()> {
        require!(self.is_whitelisted, MyErrors::NotWhitelisted);
        self.is_whitelisted = false;
        self.limit_amount = 0;
        Ok(())
    }

    pub fn transfer_sol_to_pool_vault() -> Result<()> {
        //user should send sol to pool sol address then he can get the tokens to his mint address
        Ok(())
    }
}
