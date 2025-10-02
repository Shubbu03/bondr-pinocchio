use pinocchio::program_error::ProgramError;

#[derive(Clone, PartialEq, shank::ShankType)]
pub enum BondrError {
    InvalidInstructionData,
    PdaMismatch,
    InvalidOwner,
    InvalidAmount,
    InvalidAmountZero,
    SelfTransfer,
    InvalidReferenceSeed,
    AmountTooLarge,
    InsufficientBalance,
    MissingTokenAccounts,
    MissingTokenProgram,
    UnauthorizedSender,
    AlreadyReleased,
    NotReleased,
    InsufficientEscrows,
    NFTAlreadyMinted,
    InvalidMplKey,
    InvalidMultisigConfig,
    DuplicateMember,
    MultisigBusy,
    NotMultisigMember,
    AlreadyApproved,
    MultisigPendingEscrowMismatch,
    MultisigThresholdNotMet,
}

impl From<BondrError> for ProgramError {
    fn from(e: BondrError) -> Self {
        Self::Custom(e as u32)
    }
}
