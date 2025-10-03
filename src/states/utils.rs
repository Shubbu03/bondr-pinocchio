use pinocchio::{
    account_info::AccountInfo, instruction::Signer, program_error::ProgramError, pubkey::Pubkey,
    seeds, ProgramResult,
};

use crate::errors::BondrError;
use pinocchio_token::instructions::Transfer;

pub trait DataLen {
    const LEN: usize;
}

#[inline(always)]
pub unsafe fn load_acc_unchecked<T: DataLen>(bytes: &[u8]) -> Result<&T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(&*(bytes.as_ptr() as *const T))
}

#[inline(always)]
pub unsafe fn load_acc_mut_unchecked<T: DataLen>(bytes: &mut [u8]) -> Result<&mut T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(&mut *(bytes.as_mut_ptr() as *mut T))
}

#[inline(always)]
pub unsafe fn load_ix_data<T: DataLen>(bytes: &[u8]) -> Result<&T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(BondrError::InvalidInstructionData.into());
    }
    Ok(&*(bytes.as_ptr() as *const T))
}

pub unsafe fn to_bytes<T: DataLen>(data: &T) -> &[u8] {
    core::slice::from_raw_parts(data as *const T as *const u8, T::LEN)
}

pub unsafe fn to_mut_bytes<T: DataLen>(data: &mut T) -> &mut [u8] {
    core::slice::from_raw_parts_mut(data as *mut T as *mut u8, T::LEN)
}

pub fn transfer_spl_tokens_from_escrow(
    escrow_token_acc: &AccountInfo,
    receiver_token_acc: &AccountInfo,
    escrow_acc: &AccountInfo, // PDA authority account (escrow PDA)
    client_pub: &Pubkey,
    freelancer_pub: &Pubkey,
    reference_seed: u8,
    escrow_bump: u8,
    amount: u64,
) -> ProgramResult {
    // Build the signer seeds exactly as your PDA derivation:
    let pda_ref = &[reference_seed];
    let bump_ref = &[escrow_bump];

    let seeds_arr = seeds!(b"escrow", client_pub, freelancer_pub, pda_ref, bump_ref);

    let signer = Signer::from(&seeds_arr);

    // Construct the Transfer struct literal
    let ix = Transfer {
        from: escrow_token_acc,
        to: receiver_token_acc,
        authority: escrow_acc,
        amount,
    };

    // Invoke signed with the PDA signer
    ix.invoke_signed(&[signer])?;
    Ok(())
}
