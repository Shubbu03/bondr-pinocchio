use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::create_program_address,
    ProgramResult,
};

use crate::{
    errors::BondrError,
    states::{load_acc_mut_unchecked, load_ix_data, DataLen, Escrow},
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReleasePayment {
    reference_seed: u8,
}

impl DataLen for ReleasePayment {
    const LEN: usize = core::mem::size_of::<ReleasePayment>();
}

pub fn release_payment(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [client, escrow] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let ix_data = unsafe { load_ix_data::<ReleasePayment>(data) }?;

    if !client.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let escrow_state =
        unsafe { load_acc_mut_unchecked::<Escrow>(escrow.borrow_mut_data_unchecked())? };

    let expected_pda = create_program_address(
        &[
            b"escrow".as_ref(),
            escrow_state.sender.as_ref(),
            escrow_state.receiver.as_ref(),
            &[ix_data.reference_seed],
        ],
        &crate::ID,
    )?;

    if expected_pda != *escrow.key() {
        return Err(BondrError::PdaMismatch.into());
    }

    if escrow_state.sender != *client.key() {
        return Err(BondrError::UnauthorizedSender.into());
    }

    if escrow_state.is_released {
        return Err(BondrError::AlreadyReleased.into());
    }

    escrow_state.is_released = true;

    Ok(())
}
