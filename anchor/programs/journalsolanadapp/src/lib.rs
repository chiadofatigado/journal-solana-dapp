#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod journalsolanadapp {
    use super::*;

  pub fn close(_ctx: Context<CloseJournalsolanadapp>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.journalsolanadapp.count = ctx.accounts.journalsolanadapp.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.journalsolanadapp.count = ctx.accounts.journalsolanadapp.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeJournalsolanadapp>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.journalsolanadapp.count = value.clone();
    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeJournalsolanadapp<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + Journalsolanadapp::INIT_SPACE,
  payer = payer
  )]
  pub journalsolanadapp: Account<'info, Journalsolanadapp>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseJournalsolanadapp<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub journalsolanadapp: Account<'info, Journalsolanadapp>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub journalsolanadapp: Account<'info, Journalsolanadapp>,
}

#[account]
#[derive(InitSpace)]
pub struct Journalsolanadapp {
  count: u8,
}
