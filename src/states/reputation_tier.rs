use crate::states::DataLen;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ReputationTier {
    Unranked = 0,
    Verified = 1,
    Professional = 2,
    Elite = 3,
}

impl DataLen for ReputationTier {
    const LEN: usize = core::mem::size_of::<ReputationTier>();
}
