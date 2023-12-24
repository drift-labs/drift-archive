use anchor_lang::prelude::*;

use drift::state::state::State;
use drift::state::user::User;
use crate::state::ArchivedUser;
use drift::state::traits::Size;
use arrayref::array_ref;

pub fn archive_user<'info>(ctx: Context<'_, '_, '_, 'info, ArchiveUser<'info>>) -> Result<()> {
    let archived_user = &mut ctx.accounts.archived_user.load_init()?;
    let account_info = ctx.accounts.drift_user.to_account_info();
    let account_info_data = account_info.try_borrow_data()?;
    let data = array_ref![account_info_data, 8, 4368];

    for i in 0..data.len() {
        archived_user.data[i] = data[i];
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    sub_account_id: u16,
)]
pub struct ArchiveUser<'info> {
    #[account(
        constraint = drift_state.signer == *drift_signer.key,
    )]
    pub drift_state: Box<Account<'info, State>>,
    #[account(
        constraint = drift_user.load()?.sub_account_id == sub_account_id,
    )]
    pub drift_user: AccountLoader<'info, User>,
    #[account(mut)]
    payer: Signer<'info>,
    drift_signer: Signer<'info>,
    #[account(
        init,
        seeds = [b"user",  drift_user.load()?.authority.as_ref(), drift_user.load()?.sub_account_id.to_le_bytes().as_ref()],
        space = ArchivedUser::SIZE,
        bump,
        payer = payer
    )]
    pub archived_user: AccountLoader<'info, ArchivedUser>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}