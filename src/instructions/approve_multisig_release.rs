use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use crate::{errors::BondrError, states::{load_acc_mut_unchecked, ClientMultisig, DataLen, Escrow}};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ApproveMultisigRelease {}

impl DataLen for ApproveMultisigRelease {
    const LEN: usize = core::mem::size_of::<ApproveMultisigRelease>();
}

pub fn approve_multisig_release(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [member, multisig_acc, escrow_acc, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !member.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Deserialize ix data
    let _ix = unsafe { crate::states::load_ix_data::<ApproveMultisigRelease>(data)? };

    // Load account state
    let multisig = unsafe {
        load_acc_mut_unchecked::<ClientMultisig>(multisig_acc.borrow_mut_data_unchecked())?
    };

    let escrow =
        unsafe { load_acc_mut_unchecked::<Escrow>(escrow_acc.borrow_mut_data_unchecked())? };

    if multisig.pending_escrow != *escrow_acc.key() {
        return Err(BondrError::MultisigPendingEscrowMismatch.into());
    }

    let mut found = false;
    let mut member_index: usize = 0;

    for i in 0..(multisig.member_count as usize) {
        if multisig.members[i] == *member.key() {
            found = true;
            member_index = i;
            break;
        }
    }

    if !found {
        return Err(BondrError::NotMultisigMember.into());
    }

    if multisig.approvals[member_index] == 1 {
        return Err(BondrError::AlreadyApproved.into());
    }

    // approve
    multisig.approvals[member_index] = 1;

    // check threshold
    let approvals_met = multisig.approvals
        .iter()
        .take(multisig.member_count as usize)
        .filter(|&&a| a == 1)
        .count() as u8;


    if approvals_met >= multisig.threshold {
        escrow.is_released = true;
    }
    
    Ok(())
}
