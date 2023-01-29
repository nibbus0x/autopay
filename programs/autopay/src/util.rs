use crate::state::AcceptedTriggers;
use crate::error::AutoPayError;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction::transfer;
use clockwork_sdk::state::{AccountMetaData, InstructionData, Trigger};
use clockwork_cron::Schedule;
use std::str::FromStr;

pub fn verify_trigger(trigger: AcceptedTriggers, schedule_str: Option<String>) -> Result<Trigger> {
    match trigger {
        AcceptedTriggers::Immediate => {
            Ok(Trigger::Immediate)
        },
        AcceptedTriggers::Cron => {
          if let Some(str) = schedule_str {
              let schedule = Schedule::from_str(&str).unwrap();

              msg!("{:?}", schedule);
              
              Ok(Trigger::Cron { schedule: schedule.to_string(), skippable: false })
          } else {
              Err(error!(AutoPayError::InvalidScheduleString))
          }
        }
    }

    // Ok(Trigger::Immediate)
}

pub fn get_transfer_ix(
    payer: &AccountInfo,
    receiver: &AccountInfo,
    amount: u64,
) -> Result<InstructionData> {
    let transfer_ix = transfer(payer.key, receiver.key, amount);

    let ix_data = InstructionData {
        program_id: transfer_ix.program_id,
        accounts: transfer_ix
            .accounts
            .iter()
            .map(|meta| AccountMetaData {
                pubkey: meta.pubkey,
                is_signer: meta.is_signer,
                is_writable: meta.is_writable,
            })
            .collect(),
        data: transfer_ix.data,
    };

    Ok(ix_data)
}
