use anchor_lang::prelude::*;
use pda_signer::{
    cpi::{accounts::Increment, increment},
    Counter, PdaOne, PdaTwo,
};

declare_id!("Hbe9UDYix7KQwHZy96qT7Eb3vxvGJ1brmV1es7ox5igN");

#[program]
pub mod my_cpi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let pda_signer_program = &ctx.accounts.pda_signer.to_account_info();

        let (_, signer_pda_one_bump) = Pubkey::find_program_address(&[b"pda_one"], ctx.program_id);
        let (_, signer_pda_two_bump) = Pubkey::find_program_address(&[b"pda_two"], ctx.program_id);
        let cpi_ctx_accounts = Increment {
            counter: ctx.accounts.counter.to_account_info(),
            pda_one: ctx.accounts.pda_one.to_account_info(),
            pda_two: ctx.accounts.pda_two.to_account_info(),
            signer_pda_one: ctx.accounts.signer_pda_one.to_account_info(),
            signer_pda_two: ctx.accounts.signer_pda_two.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let pda_one_seeds: &[&[u8]] = &[b"pda_one", &[signer_pda_one_bump]];
        let pda_two_seeds: &[&[u8]] = &[b"pda_two", &[signer_pda_two_bump]];
        let signer_seeds: &[&[&[u8]]] = &[pda_one_seeds, pda_two_seeds];

        let cpi_ctx =
            CpiContext::new_with_signer(pda_signer_program.clone(), cpi_ctx_accounts, signer_seeds);
        // Proceed with the CPI call
        increment(cpi_ctx)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: pda
    #[account(address = pda_signer::ID)]
    pub pda_signer: AccountInfo<'info>,
    /// CHECK: pda
    // #[account(mut, signer)]
    pub pda_one: Account<'info, PdaOne>,
    /// CHECK: pda
    // #[account(signer)]
    pub pda_two: Account<'info, PdaTwo>,
    /// CHECK: pda
    #[account(init, payer = signer, space = 8 + 32, seeds = [b"pda_one"], bump)]
    pub signer_pda_one: AccountInfo<'info>,
    /// CHECK: pda
    #[account(init, payer = signer, space = 8 + 32, seeds = [b"pda_two"], bump)]
    pub signer_pda_two: AccountInfo<'info>,
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}
