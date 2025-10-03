use pinocchio::program_error::ProgramError;

pub mod approve_multisig_release;
pub mod claim_payment;
pub mod initialize_escrow;
pub mod initialize_freelancer_badge;
pub mod initialize_multisig_client;
pub mod mint_reputation_nft;
pub mod release_payment;
pub mod update_freelancer_badge;

pub use approve_multisig_release::*;
pub use claim_payment::*;
pub use initialize_escrow::*;
pub use initialize_freelancer_badge::*;
pub use initialize_multisig_client::*;
pub use mint_reputation_nft::*;
pub use release_payment::*;
pub use update_freelancer_badge::*;

#[repr(u8)]
pub enum BondrInstruction {
    InitializeEscrow,
    InitializeFreelancerBadge,
    InitializeMultisigClient,
    ReleasePayment,
    ClaimPayment,
    ApproveMultisigRelease,
    UpdateFreelancerBadge,
    MintReputationNft,
}

impl TryFrom<&u8> for BondrInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(BondrInstruction::InitializeEscrow),
            1 => Ok(BondrInstruction::InitializeFreelancerBadge),
            2 => Ok(BondrInstruction::InitializeMultisigClient),
            3 => Ok(BondrInstruction::ReleasePayment),
            4 => Ok(BondrInstruction::ClaimPayment),
            5 => Ok(BondrInstruction::ApproveMultisigRelease),
            6 => Ok(BondrInstruction::UpdateFreelancerBadge),
            7 => Ok(BondrInstruction::MintReputationNft),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
