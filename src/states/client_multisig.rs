use pinocchio::pubkey::Pubkey;

use crate::states::DataLen;

pub const MAX_MULTISIG_MEMBERS: usize = 5;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClientMultisig {
    pub members: [Pubkey; MAX_MULTISIG_MEMBERS], // max 5 members allowed as per now
    pub member_count: u8,                        // how many active entries in `members`
    pub threshold: u8,                           // approvals required
    pub approvals: [u8; MAX_MULTISIG_MEMBERS], // 0 = not approved, 1 = approved; parallel to members
    pub pending_escrow: Pubkey,                // escrow PDA tied to this multisig
    pub bump: u8,
}

impl DataLen for ClientMultisig {
    const LEN: usize = core::mem::size_of::<ClientMultisig>();
}
