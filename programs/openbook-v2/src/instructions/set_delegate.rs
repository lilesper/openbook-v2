use anchor_lang::prelude::*;

use crate::accounts_ix::*;
use crate::pod_option::PodOption;
use crate::state::*;

pub fn set_delegate(ctx: Context<SetDelegate>) -> Result<()> {
    let mut account = ctx.accounts.open_orders_account.load_full_mut()?;

    let delegate_account: PodOption<Pubkey> = ctx
        .accounts
        .delegate_account
        .as_ref()
        .map(|account| account.key())
        .into();

    account.fixed.delegate = delegate_account;

    Ok(())
}