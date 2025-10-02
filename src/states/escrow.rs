use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{create_program_address, Pubkey},
    ProgramResult,
};

use crate::{
    errors::BondrError,
    states::{load_acc_mut_unchecked, DataLen},
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Escrow {
    pub sender: Pubkey,
    pub receiver: Pubkey,
    pub amount: u64,
    pub is_released: bool,
    pub bump: u8,
    pub client_multisig: Pubkey, // store Pubkey::default() if not set
    pub has_multisig: bool,
}

impl DataLen for Escrow {
    const LEN: usize = core::mem::size_of::<Escrow>();
}

impl Escrow {
    pub const SEED: &'static str = "escrow";

    pub fn validate_pda(
        bump: u8,
        pda: &Pubkey,
        sender: &Pubkey,
        receiver: &Pubkey,
    ) -> Result<(), ProgramError> {
        let seeds = &[
            Self::SEED.as_bytes(),
            sender.as_ref(),
            receiver.as_ref(),
            &[bump],
        ];
        let derived = create_program_address(seeds, &crate::ID)?;

        if derived != *pda {
            return Err(BondrError::PdaMismatch.into());
        }
        Ok(())
    }

    pub fn initialize(
        escrow_acc: &AccountInfo,
        sender: Pubkey,
        receiver: Pubkey,
        amount: u64,
        bump: u8,
        client_multisig: Option<Pubkey>,
    ) -> ProgramResult {
        let my_state =
            unsafe { load_acc_mut_unchecked::<Escrow>(escrow_acc.borrow_mut_data_unchecked()) }?;

        my_state.sender = sender;
        my_state.receiver = receiver;
        my_state.amount = amount;
        my_state.is_released = false;
        my_state.bump = bump;

        if let Some(ms) = client_multisig {
            my_state.client_multisig = ms;
            my_state.has_multisig = true;
        } else {
            my_state.client_multisig = Pubkey::default();
            my_state.has_multisig = false;
        }

        Ok(())
    }
}
