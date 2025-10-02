use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    sysvars::rent::Rent,
    ProgramResult,
};

use pinocchio_system::instructions::CreateAccount;

use crate::{
    errors::BondrError,
    states::{
        utils::{load_ix_data, DataLen},
        Escrow,
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InitializeEscrow {
    pub amount: u64,
    pub bump: u8,
    pub stats_bump: u8,
    pub is_multisig: bool,
}

impl DataLen for InitializeEscrow {
    const LEN: usize = core::mem::size_of::<InitializeEscrow>();
}

pub fn init_escrow(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [sender, receiver, escrow_acc, _sender_stats, client_multisig, _sender_token_account, _escrow_token_account, sysvar_rent_acc, ..] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !sender.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !escrow_acc.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let rent = Rent::from_account_info(sysvar_rent_acc)?;

    let ix_data = unsafe { load_ix_data::<InitializeEscrow>(data)? };

    if ix_data.amount == 0 {
        return Err(BondrError::InvalidAmountZero.into());
    }
    if sender.key() == receiver.key() {
        return Err(BondrError::SelfTransfer.into());
    }

    Escrow::validate_pda(
        ix_data.bump,
        &escrow_acc.key(),
        &sender.key(),
        &receiver.key(),
    )?;

    let pda_bump_bytes = [ix_data.bump];

    // signer seeds
    let signer_seeds = [
        Seed::from(Escrow::SEED.as_bytes()),
        Seed::from(sender.key().as_ref()),
        Seed::from(receiver.key().as_ref()),
        Seed::from(&pda_bump_bytes[..]),
    ];
    let signers = [Signer::from(&signer_seeds[..])];

    CreateAccount {
        from: sender,
        to: escrow_acc,
        lamports: rent.minimum_balance(Escrow::LEN),
        space: Escrow::LEN as u64,
        owner: &crate::ID,
    }
    .invoke_signed(&signers)?;

    let multisig_pubkey = if ix_data.is_multisig {
        Some(*client_multisig.key())
    } else {
        None
    };

    Escrow::initialize(
        escrow_acc,
        *sender.key(),
        *receiver.key(),
        ix_data.amount,
        ix_data.bump,
        multisig_pubkey,
    )?;

    Ok(())
}
