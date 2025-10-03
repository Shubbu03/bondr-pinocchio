use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use crate::{
    errors::BondrError,
    states::{load_acc_mut_unchecked, load_ix_data, DataLen, FreelancerBadge},
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UpdateFreelancerBadge {
    value: u64,
}

impl DataLen for UpdateFreelancerBadge {
    const LEN: usize = core::mem::size_of::<UpdateFreelancerBadge>();
}

pub fn update_freelancer_badge(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [freelancer, badge] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !freelancer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // deserialize instruction data
    let ix = unsafe { load_ix_data::<UpdateFreelancerBadge>(data)? };

    let badge =
        unsafe { load_acc_mut_unchecked::<FreelancerBadge>(badge.borrow_mut_data_unchecked())? };

    if badge.freelancer != *freelancer.key() {
        return Err(BondrError::UnauthorizedSender.into());
    }
    if ix.value == 0 {
        return Err(BondrError::InvalidAmountZero.into());
    }

    badge.completed_escrows = badge.completed_escrows.saturating_add(1);
    badge.total_value_completed = badge.total_value_completed.saturating_add(ix.value);

    Ok(())
}
