use anchor_lang::{prelude::*, system_program};
use anchor_spl::token_interface::Mint;

use crate::state::Pool;

#[derive(Accounts)]
pub struct WithdrawSol<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        mut,
        seeds=[b"POOL",author.key().as_ref(),token_mint.key().as_ref()],
        bump=pool_account.bump
    )]
    pub pool_account: Account<'info, Pool>,
    #[account(
        mut,
        seeds=[b"SOL_VAULT",author.key().as_ref(),token_mint.key().as_ref()],
        bump=pool_account.sol_vault_bump
    )]
    pub sol_vault: SystemAccount<'info>,
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawSol<'info> {
    pub fn handle_it(&mut self) -> Result<()> {
        let lamports = self.sol_vault.to_account_info().lamports();
        let author = self.author.key();
        let mint = self.token_mint.key();
        let seeds = &[
            b"SOL_VAULT",
            author.as_ref(),
            mint.as_ref(),
            &[self.pool_account.bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let accounts = system_program::Transfer {
            from: self.sol_vault.to_account_info(),
            to: self.author.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        system_program::transfer(ctx, lamports)
    }
}
