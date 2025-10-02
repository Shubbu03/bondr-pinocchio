use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::create_program_address,
    sysvars::rent::Rent, ProgramResult,
};

use pinocchio_system::instructions::CreateAccount;

use crate::{
    errors::BondrError,
    states::{
        load_acc_mut_unchecked,
        utils::{load_ix_data, DataLen},
        FreelancerBadge, ReputationTier,
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InitializeFreelancerBadge {
    pub bump: u8,
}

impl DataLen for InitializeFreelancerBadge {
    const LEN: usize = core::mem::size_of::<InitializeFreelancerBadge>();
}

pub fn init_freelancer_badge(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [freelancer, badge, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !freelancer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !badge.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let ix_data = unsafe { load_ix_data::<InitializeFreelancerBadge>(data)? };

    let seeds = &[
        b"badge".as_ref(),
        freelancer.key().as_ref(),
        &[ix_data.bump],
    ];
    let derived_pda = create_program_address(seeds, &crate::ID)?;
    if derived_pda != *badge.key() {
        return Err(BondrError::PdaMismatch.into());
    }

    let min_lamports = system_program
        .lamports()
        .min(Rent::default().minimum_balance(FreelancerBadge::LEN));

    CreateAccount {
        from: freelancer,
        to: badge,
        lamports: min_lamports,
        space: FreelancerBadge::LEN as u64,
        owner: &crate::ID,
    }
    .invoke()?;

    let badge_state =
        unsafe { load_acc_mut_unchecked::<FreelancerBadge>(badge.borrow_mut_data_unchecked())? };

    *badge_state = FreelancerBadge {
        tier: ReputationTier::Unranked,
        completed_escrows: 0,
        total_value_completed: 0,
        freelancer: *freelancer.key(),
        bump: ix_data.bump,
    };

    Ok(())
}
