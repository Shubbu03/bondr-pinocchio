use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use crate::{
    constants::MAX_MULTISIG_MEMBERS, errors::BondrError, states::{
        load_acc_mut_unchecked, load_ix_data, transfer_spl_tokens_from_escrow, ClientMultisig,
        DataLen, Escrow, UserStats, 
    }
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClaimPayment {
    reference_seed: u8,
    receiver_stats_bump: u8,
}

impl DataLen for ClaimPayment {
    const LEN: usize = core::mem::size_of::<ClaimPayment>();
}

pub fn claim_payment(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [_client, freelancer, escrow_acc, receiver_stats_acc, multisig_acc, escrow_token_acc, receiver_token_acc, _token_mint_acc, _token_program, _system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !freelancer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let ix_data = unsafe { load_ix_data::<ClaimPayment>(data) }?;

    let escrow_state =
        unsafe { load_acc_mut_unchecked::<Escrow>(escrow_acc.borrow_mut_data_unchecked())? };

    if escrow_state.receiver != *freelancer.key() {
        return Err(BondrError::UnauthorizedReceiver.into());
    }

    if !escrow_state.is_released {
        return Err(BondrError::NotReleased.into());
    }

    if !multisig_acc.data_is_empty() {
        let multisig_state = unsafe {
            load_acc_mut_unchecked::<ClientMultisig>(multisig_acc.borrow_mut_data_unchecked())?
        };

        // pending escrow must match
        if multisig_state.pending_escrow != *escrow_acc.key() {
            return Err(BondrError::MultisigPendingEscrowMismatch.into());
        }

        // approvals >= threshold
        let approvals_met = multisig_state
            .approvals
            .iter()
            .take(multisig_state.member_count as usize)
            .filter(|&&a| a == 1)
            .count() as u8;

        if approvals_met < multisig_state.threshold {
            return Err(BondrError::MultisigThresholdNotMet.into());
        }

        // reset pending escrow + approvals
        multisig_state.pending_escrow = Pubkey::default();
        multisig_state.approvals = [0u8; MAX_MULTISIG_MEMBERS];
    }

    transfer_spl_tokens_from_escrow(
        escrow_token_acc,
        receiver_token_acc,
        escrow_acc,
        &escrow_state.sender,
        &escrow_state.receiver,
        ix_data.reference_seed,
        escrow_state.bump,
        escrow_state.amount,
    )?;

    let receiver_stats = unsafe {
        load_acc_mut_unchecked::<UserStats>(receiver_stats_acc.borrow_mut_data_unchecked())?
    };

    if receiver_stats.user == Pubkey::default() {
        *receiver_stats = UserStats {
            user: *freelancer.key(),
            completed_escrows: 1,
            bump: ix_data.receiver_stats_bump,
        };
    } else {
        receiver_stats.completed_escrows = receiver_stats.completed_escrows.saturating_add(1);
    }

    escrow_acc.close();

    Ok(())
}
