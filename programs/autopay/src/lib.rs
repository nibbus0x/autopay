pub mod error;
pub mod state;
pub mod util;

use crate::error::AutoPayError;
use crate::util::{get_transfer_ix, verify_trigger};
use crate::state::AcceptedTriggers;

use anchor_lang::prelude::*;
use clockwork_sdk::{
    cpi::{thread_create, ThreadCreate},
    ThreadProgram,
};
use clockwork_thread_program::{state::SEED_THREAD, ID as THREAD_PROGRAM_ID};

declare_id!("5FLhLmmJFpLeKyqwHC34bmELNLZFutKTqdxtqmimiMSo");

#[program]
pub mod autopay {    
    use super::*;

    pub fn create_thread(
        ctx: Context<CreateThread>,
        thread_id: String,
        transfer_amount: u64,
        thread_trigger: AcceptedTriggers,
        thread_schedule: Option<String>
    ) -> Result<()> {
        let authority_key = ctx.accounts.authority.key();
        let thread_bump = *ctx.bumps.get("thread").ok_or(AutoPayError::MissingBump)?;
        let thread_seeds = &[
            SEED_THREAD,
            authority_key.as_ref(),
            thread_id.as_bytes(),
            &[thread_bump],
        ];
        let signer = &[&thread_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.thread_program.to_account_info(),
            ThreadCreate {
                authority: ctx.accounts.authority.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.authority.to_account_info(),
                thread: ctx.accounts.authority.to_account_info(),
            },
            signer,
        );

        let trigger = verify_trigger(thread_trigger, thread_schedule)?;
        let kickoff_ix = get_transfer_ix(
            &ctx.accounts.payer.to_account_info(),
            &ctx.accounts.receiver.to_account_info(),
            transfer_amount,
        )?;

        thread_create(cpi_ctx, thread_id.clone(), kickoff_ix, trigger)?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(thread_id: String)]
pub struct CreateThread<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
    /// CHECK: Seeds checked in constraint
    #[account(
        mut,
        seeds = [
            SEED_THREAD,
            authority.key().as_ref(),
            thread_id.as_bytes()
        ],
        bump,
        seeds::program = THREAD_PROGRAM_ID
    )]
    pub thread: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub thread_program: Program<'info, ThreadProgram>,
}
// Init/create daemon ix
// Update init jar and `update_tippees` ixs to shedule cronos tasks
// Write distribute ix
