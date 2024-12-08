use anchor_lang::prelude::*;

declare_id!("8PMR15DmZ7SkbNbEGGTLHkufm7AtwwabHz5avs5PpM1a");

#[program]
pub mod pda_signer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.bump = ctx.bumps.counter;

        let pda_one = &mut ctx.accounts.pda_one;
        pda_one.bump = ctx.bumps.pda_one;

        let pda_two = &mut ctx.accounts.pda_two;
        pda_two.bump = ctx.bumps.pda_two;

        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init, payer = signer, space = 8 + 32 + 8, seeds = [b"pda_one"], bump)]
    pub pda_one: Account<'info, PdaOne>,
    #[account(init, payer = signer, space = 8 + 32 + 8, seeds = [b"pda_two"], bump)]
    pub pda_two: Account<'info, PdaTwo>,
    #[account(init, payer = signer, space = 8 + 32, seeds = [b"pda_three"], bump)]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [b"pda_three"], bump = counter.bump)]
    pub counter: Account<'info, Counter>,
    #[account(seeds = [b"pda_one"], bump = pda_one.bump)]
    pub pda_one: Account<'info, PdaOne>,
    #[account(seeds = [b"pda_two"], bump = pda_two.bump)]
    pub pda_two: Account<'info, PdaTwo>,
    /// CHECK: pda
    pub signer_pda_one: Signer<'info>,
    /// CHECK: pda
    pub signer_pda_two: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    pub count: u64,
    pub bump: u8,
}

#[account]
pub struct PdaOne {
    pub bump: u8,
}

#[account]
pub struct PdaTwo {
    pub bump: u8,
}
