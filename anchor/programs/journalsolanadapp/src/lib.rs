#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("VqwzQTMfMMn9Q1zFpzCKMhNxR3qgAXsRPeCi5vVAimP");

const ANCHOR_SIZE_DESCRIMINATOR: usize = 8;

#[program]
pub mod journalsolanadapp {
    use super::*;

    pub fn create_journal_entry(
        ctx: Context<CreateJournalEntry>,
        title: String,
        content: String,
    ) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.owner = *ctx.accounts.owner.key;
        journal_entry.title = title;
        journal_entry.content = content;
        msg!("Journal entry created with title: {}", journal_entry.title);
        msg!("Journal entry content: {}", journal_entry.content);
        msg!("Journal entry owner: {}", journal_entry.owner);
        msg!("Journal entry address: {}", journal_entry.key());
        Ok(())
    }

    pub fn update_journal_entry(
        ctx: Context<UpdateJournalEntry>,
        _title: String,
        content: String,
    ) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.content = content;
        Ok(())
    }

    pub fn delete_journal_entry(
      _ctx: Context<DeleteJournalEntry>,
    ) -> Result<()>{
      Ok(())
    }

}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateJournalEntry<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
    init,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
    payer = owner,
    space = ANCHOR_SIZE_DESCRIMINATOR + JournalEntryState::INIT_SPACE,
  )]
    pub journal_entry: Account<'info, JournalEntryState>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateJournalEntry<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
      mut,
      seeds = [title.as_bytes(), owner.key().as_ref()],
      bump,
      realloc = ANCHOR_SIZE_DESCRIMINATOR + JournalEntryState::INIT_SPACE,
      realloc::payer = owner,
      realloc::zero = true
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteJournalEntry<'info> {
  #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
      mut,
      seeds = [title.as_bytes(), owner.key().as_ref()],
      bump,
      close = owner
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct JournalEntryState {
    pub owner: Pubkey,
    #[max_len(50)]
    pub title: String,
    #[max_len(1000)]
    pub content: String,
}
