#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(ctx: Context<InitializePoll>, 
                            poll_id: u64, 
                            start_time: u64, 
                            end_time: u64,
                            description: String) -> Result<()> {
        ctx.accounts.poll_account.poll_id = poll_id;
        ctx.accounts.poll_account.poll_description = description;
        ctx.accounts.poll_account.poll_voting_start = start_time;
        ctx.accounts.poll_account.poll_voting_end = end_time;
        ctx.accounts.poll_account.poll_candidate_amount = 0;
        Ok(())
    }
    
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info, System>,
}
#[account]
#[derive(InitSpace)]
pub struct PollAccount{
    pub poll_id: u64,
    #[max_len(280)]
    pub poll_description: String,
    pub poll_voting_start: u64,
    pub poll_voting_end: u64,
    pub poll_candidate_amount: u64,
}

