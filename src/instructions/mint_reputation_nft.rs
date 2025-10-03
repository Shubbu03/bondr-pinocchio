use mpl_core::instructions::CreateV2;
use mpl_core::types::DataState;
use pinocchio::{
    account_info::AccountInfo, program::invoke, program_error::ProgramError, ProgramResult,
};

use crate::constants::*;
use crate::errors::BondrError;
use crate::states::{load_acc_mut_unchecked, utils::DataLen, FreelancerBadge};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MintReputationNft {}

impl DataLen for MintReputationNft {
    const LEN: usize = core::mem::size_of::<MintReputationNft>();
}

pub fn mint_rep_nft(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [freelancer, badge_acc, asset, collection, mpl_core_program, system_program] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !freelancer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let badge = unsafe {
        load_acc_mut_unchecked::<FreelancerBadge>(badge_acc.borrow_mut_data_unchecked())?
    };

    // check freelancer matches badge
    if badge.freelancer != *freelancer.key() {
        return Err(BondrError::UnauthorizedSender.into());
    }

    // tier calculation
    let tier = match badge.completed_escrows {
        0..=2 => return Err(BondrError::InsufficientEscrows.into()),
        3..=9 => crate::states::ReputationTier::Verified,
        10..=24 => crate::states::ReputationTier::Professional,
        _ => crate::states::ReputationTier::Elite,
    };

    if badge.tier >= tier {
        return Err(BondrError::NFTAlreadyMinted.into());
    }

    let (name, uri) = match tier {
        crate::states::ReputationTier::Verified => ("Bondr Verified Badge", VERIFIED_METADATA_URI),
        crate::states::ReputationTier::Professional => {
            ("Bondr Professional Badge", PROFESSIONAL_METADATA_URI)
        }
        crate::states::ReputationTier::Elite => ("Bondr Elite Badge", ELITE_METADATA_URI),
        _ => return Err(BondrError::InsufficientEscrows.into()),
    };

    // Build the CPI instruction manually
    let ix = CreateV2 {
        asset: *asset.key(),
        collection: Some(*collection.key()),
        authority: Some(*freelancer.key()),
        payer: *freelancer.key(),
        owner: Some(*freelancer.key()),
        system_program: *system_program.key(),
        update_authority: Some(*freelancer.key()),
        log_wrapper: *mpl_core_program.key(),
        name: name.to_string(),
        uri: uri.to_string(),
        data_state: DataState::AccountState,
    }
    .instruction(&[
        asset.clone(),
        collection.clone(),
        freelancer.clone(),
        system_program.clone(),
        mpl_core_program.clone(),
    ]);

    invoke(
        &ix,
        &[
            asset.clone(),
            collection.clone(),
            freelancer.clone(),
            system_program.clone(),
            mpl_core_program.clone(),
        ],
    )?;

    // update badge tier
    badge.tier = tier;

    Ok(())
}
