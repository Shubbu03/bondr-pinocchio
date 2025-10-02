use pinocchio::pubkey::Pubkey;

use crate::states::DataLen;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UserStats {
    pub user: Pubkey,
    pub completed_escrows: u32,
    pub bump: u8,
}

impl DataLen for UserStats {
    const LEN: usize = core::mem::size_of::<UserStats>();
}
