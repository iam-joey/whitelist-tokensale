use anchor_lang::prelude::*;

use crate::instructions::*;

pub mod state;
pub mod instructions;
pub mod error;

declare_id!("8TCZHu8NamtbsbsrLHi9kRxphoYhJ7yAxuSoi66epsyz");
#[program]
pub mod whitelist_token_sale {
    
    use super::*;

    pub fn initialize(ctx: Context<InitializePool>,allocation:u64,start:i64,end:i64,price:u64) -> Result<()> {
        ctx.accounts.handler(allocation, start, end, price,ctx.bumps.pool_account,ctx.bumps.sol_vault)
    }

    pub fn delete_pool(ctx:Context<DeletePool>)->Result<()>{
        ctx.accounts.delete_pool()
    }

    pub fn close_pool(ctx:Context<ClosePool>)->Result<()>{
        ctx.accounts.stop_pool()
    }

    pub fn join_whitelist(ctx:Context<JoinWhitelist>)->Result<()>{
        ctx.accounts.init(ctx.bumps.user_account)
    }

    pub fn leave_whitelist(ctx:Context<LeaveWhiteList>)->Result<()>{
        ctx.accounts.leave_whitelist()
    }

    pub fn add_user_whitelist(ctx:Context<WhiteListUser>,limit:u64)->Result<()>{
        ctx.accounts.whitelist_user(limit)
    }

    pub fn approve_buy(ctx:Context<ApproveBuy>,amount:u64)->Result<()>{
        ctx.accounts.make_it_buy(amount)
    }

    pub fn buy_tokens(ctx:Context<BuyToken>,tokens:u64)->Result<()>{
        ctx.accounts.handler(tokens)
    }
    
    pub fn transfer_remaining_tokens(ctx:Context<TakeBackTokens>)->Result<()>{

        ctx.accounts.handle_it()
    }

    pub fn withdraw_sol(ctx:Context<WithdrawSol>)->Result<()>{
        ctx.accounts.handle_it()
    }

}


