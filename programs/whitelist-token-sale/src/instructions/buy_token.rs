use anchor_lang::{prelude::*, system_program};

use crate::{
    error::MyErrors,
    state::{Pool, User},
};

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

#[derive(Accounts)]
pub struct BuyToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds=[b"POOL",author.key().as_ref(),token_mint.key().as_ref()],
        bump=pool_account.bump
    )]
    pub pool_account: Account<'info, Pool>,
    /// CHECK : it's safe here we are not doing anything
    pub author: AccountInfo<'info>,
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds=[b"USER_ACCOUNT",user.key().as_ref(),pool_account.key().as_ref()],
        bump=user_account.bump
    )]
    pub user_account: Account<'info, User>,
    #[account(
        mut,
        associated_token::mint=token_mint,
        associated_token::authority=pool_account,
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        associated_token::mint=token_mint,
        associated_token::authority=user,
        payer=user
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds=[b"SOL_VAULT",author.key().as_ref(),token_mint.key().as_ref()],
        bump=pool_account.sol_vault_bump
    )]
    pub sol_vault: SystemAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> BuyToken<'info> {
    pub fn handler(&mut self, tokens: u64) -> Result<()> {
        let left_tokens = self
            .pool_account
            .allocation
            .checked_sub(self.pool_account.sold_tokens)
            .unwrap();
        if left_tokens == 0 {
            return Err(MyErrors::PoolEmpty.into());
        }
        if tokens < left_tokens {
            return Err(MyErrors::InsufficientPoolBalance.into());
        }
        let amount = self.pool_account.clone().price * tokens; //this will be in lamports
        self.transfer_sol_to_sol_vault(amount)
            .expect("Something went wrong while sending sol");
        msg!("sent sol successfuly");
        self.transfer_tokens_vault_to_user(tokens)
            .expect("Something went wrong while transfering tokens to user");
        self.pool_account.increase_sold_tokens(tokens)?;
        Ok(())
    }

    pub fn transfer_sol_to_sol_vault(&mut self, amount: u64) -> Result<()> {
        let accounts = system_program::Transfer {
            from: self.user.to_account_info(),
            to: self.sol_vault.to_account_info(),
        };

        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);

        system_program::transfer(ctx, amount)
    }

    pub fn transfer_tokens_vault_to_user(&mut self, tokens: u64) -> Result<()> {
        let author_key = self.author.key();
        let token_mint_key = self.token_mint.key();

        let seeds = &[
            b"POOL",
            author_key.as_ref(),
            token_mint_key.as_ref(),
            &[self.pool_account.bump],
        ];
        let signer_seed = &[&seeds[..]];

        let accounts = TransferChecked {
            from: self.vault_ata.to_account_info(),
            to: self.user_ata.to_account_info(),
            authority: self.pool_account.to_account_info(),
            mint: self.token_mint.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seed,
        );

        transfer_checked(ctx, tokens, self.token_mint.decimals)
    }
}
