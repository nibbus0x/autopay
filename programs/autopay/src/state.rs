use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub enum  AcceptedTriggers {
    Immediate,
    Cron
}