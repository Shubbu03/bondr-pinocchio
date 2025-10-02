use pinocchio::pubkey::Pubkey;

use crate::states::{DataLen, ReputationTier};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FreelancerBadge {
    pub tier: ReputationTier, //enum - Unranked -> Verified -> Professional -> Elite
    pub completed_escrows: u32,
    pub total_value_completed: u64,
    pub freelancer: Pubkey,
    pub bump: u8,
}

impl DataLen for FreelancerBadge {
    const LEN: usize = core::mem::size_of::<FreelancerBadge>();
}
