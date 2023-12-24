mod state;
mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod drift_archive {
    use super::*;

    pub fn archive_user<'info>(ctx: Context<'_, '_, '_, 'info, ArchiveUser<'info>>) -> Result<()> {
        instructions::archive_user(ctx)
    }
}
