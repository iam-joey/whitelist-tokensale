use anchor_lang::prelude::*;

use crate::error::MyErrors;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub author: Pubkey,     //maker of the pool
    pub mint: Pubkey,       //token mint address of the token we're raising funds for
    pub allocation: u64,    //token allocation of the poool
    pub start_time: i64,    //start time of pool
    pub end_time: i64,      //end time of pool
    pub users: u64,         //users participating in the poool
    pub price: u64,         //price per token in lamports
    pub allow_to_buy: bool, //pool is allowed to buy
    pub sold_tokens: u64,   //total number of tokens bought from the allocation
    pub bump: u8,
    pub sol_vault_bump: u8,
}

impl Pool {
    pub fn init_pool(
        &mut self,
        author: Pubkey,
        mint: Pubkey,
        alloc: u64,
        start: i64,
        end: i64,
        price: u64,
        bump: u8,
        bump2: u8,
    ) -> Result<()> {
        let clock = Clock::get()?;
        let time = clock.unix_timestamp;
        require!(start > time && end > time, MyErrors::InvalidTimeRange);
        require!(start < end, MyErrors::InvalidTimeRange);
        self.author = author;
        self.mint = mint;
        self.allocation = alloc;
        self.start_time = start;
        self.end_time = end;
        self.price = price;
        self.allow_to_buy = false;
        self.bump = bump;
        self.sol_vault_bump = bump2;
        self.sold_tokens = 0;
        Ok(())
    }

    pub fn increase_candidate(&mut self) -> Result<()> {
        self.users = self.users.checked_add(1).unwrap();
        Ok(())
    }
    pub fn descrease_candidate(&mut self) -> Result<()> {
        self.users = self.users.checked_sub(1).unwrap();
        Ok(())
    }

    pub fn allow_buy(&mut self) -> Result<()> {
        require!(!self.allow_to_buy, MyErrors::AlreadyInitialized);
        self.allow_to_buy = true;
        Ok(())
    }

    pub fn stop_pool(&mut self) -> Result<()> {
        // require!(!self.allow_to_buy, MyErrors::AlreadyInitialized);
        self.allow_to_buy = false;
        Ok(())
    }

    pub fn increase_sold_tokens(&mut self, tokens: u64) -> Result<()> {
        self.sold_tokens = self.sold_tokens.checked_add(tokens).unwrap();
        Ok(())
    }
}
