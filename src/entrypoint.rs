#![allow(unexpected_cfgs)]

use crate::instructions::{self, BondrInstruction};
use pinocchio::{
    account_info::AccountInfo, default_panic_handler, no_allocator, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

// This is the entrypoint for the program.
program_entrypoint!(process_instruction);
//Do not allocate memory.
no_allocator!();
// Use the no_std panic handler.
default_panic_handler!();

#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (ix_disc, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match BondrInstruction::try_from(ix_disc)? {
        BondrInstruction::InitializeEscrow => instructions::init_escrow(accounts, instruction_data),
        BondrInstruction::InitializeFreelancerBadge => {
            instructions::init_freelancer_badge(accounts, instruction_data)
        }
        BondrInstruction::InitializeMultisigClient => {
            instructions::init_multisig_client(accounts, instruction_data)
        }
        BondrInstruction::ReleasePayment => {
            instructions::release_payment(accounts, instruction_data)
        }
        BondrInstruction::ClaimPayment => instructions::claim_payment(accounts, instruction_data),
        BondrInstruction::ApproveMultisigRelease => {
            instructions::approve_multisig_release(accounts, instruction_data)
        }
        BondrInstruction::UpdateFreelancerBadge => {
            instructions::update_freelancer_badge(accounts, instruction_data)
        }
        BondrInstruction::MintReputationNft => {
            instructions::mint_rep_nft(accounts, instruction_data)
        }
    }
}
