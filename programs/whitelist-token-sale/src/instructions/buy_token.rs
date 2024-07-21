use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct BuyToken<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    // #[account(

    // )]
}