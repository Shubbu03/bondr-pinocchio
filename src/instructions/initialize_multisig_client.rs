use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{create_program_address, Pubkey},
    sysvars::rent::Rent,
    ProgramResult,
};

use pinocchio_system::instructions::CreateAccount;

use crate::{
    errors::BondrError,
    states::{
        load_acc_mut_unchecked,
        utils::{load_ix_data, DataLen},
        ClientMultisig, MAX_MULTISIG_MEMBERS,
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InitializeMultisigClient {
    members: [Pubkey; MAX_MULTISIG_MEMBERS],
    member_count: u8,
    threshold: u8,
    bump: u8,
}

impl DataLen for InitializeMultisigClient {
    const LEN: usize = core::mem::size_of::<InitializeMultisigClient>();
}

pub fn init_multisig_client(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [client, multisig, _system_program, sysvar_rent_acc] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !client.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !multisig.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let ix_data = unsafe { load_ix_data::<InitializeMultisigClient>(data)? };

    let seeds = &[
        b"client_multisig".as_ref(),
        client.key().as_ref(),
        &[ix_data.bump],
    ];

    let derived_pda = create_program_address(seeds, &crate::ID)?;
    if derived_pda != *multisig.key() {
        return Err(BondrError::PdaMismatch.into());
    }

    if ix_data.member_count == 0 || (ix_data.member_count as usize) > MAX_MULTISIG_MEMBERS {
        return Err(BondrError::InvalidMultisigConfig.into());
    }

    if ix_data.threshold == 0 || ix_data.threshold > ix_data.member_count {
        return Err(BondrError::InvalidMultisigConfig.into());
    }

    let active_members = &ix_data.members[..ix_data.member_count as usize];
    if !active_members.iter().any(|&m| m == *client.key()) {
        return Err(BondrError::InvalidMultisigConfig.into());
    }

    for i in 0..(ix_data.member_count as usize) {
        for j in (i + 1)..(ix_data.member_count as usize) {
            if ix_data.members[i] == ix_data.members[j] {
                return Err(BondrError::DuplicateMember.into());
            }
        }
    }

    let rent = Rent::from_account_info(sysvar_rent_acc)?;
    let min_lamports = rent.minimum_balance(ClientMultisig::LEN).max(1); // ensure >0

    CreateAccount {
        from: client,
        to: multisig,
        lamports: min_lamports,
        space: ClientMultisig::LEN as u64,
        owner: &crate::ID,
    }
    .invoke()?;

    let multisig_state =
        unsafe { load_acc_mut_unchecked::<ClientMultisig>(multisig.borrow_mut_data_unchecked())? };

    *multisig_state = ClientMultisig {
        members: ix_data.members,
        member_count: ix_data.member_count,
        threshold: ix_data.threshold,
        approvals: [0u8; MAX_MULTISIG_MEMBERS],
        pending_escrow: Pubkey::default(),
        bump: ix_data.bump,
    };

    Ok(())
}
