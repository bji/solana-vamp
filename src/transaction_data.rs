// vamp program instruction data is C structs

// rust is a pain in that it doesn't allow direct memory access to the C struct data, so use zerocopy, but that
// requires manual padding bytes.  Derp.

// Note that this user of zerocopy only works for little endian systems.  But I'm not sure that anyone even uses
// any big endian systems any more.

#[repr(C)]
#[derive(zerocopy::AsBytes, Debug)]
pub struct EnterData
{
    pub instruction_code : u8, // 0 = Enter

    pub administrator : [u8; 32],

    pub use_commission_caps : bool,

    pub max_commission : u8,

    pub max_commission_increase_per_epoch : u8
}

#[repr(C)]
#[derive(zerocopy::AsBytes)]
pub struct SetLeaveEpochData
{
    pub instruction_code : u8, // 1 = SetLeaveEpoch

    pub padding : [u8; 7],

    pub leave_epoch : u64
}

#[repr(C)]
#[derive(zerocopy::AsBytes)]
pub struct LeaveData
{
    pub instruction_code : u8 // 2 = Leave
}

#[repr(C)]
#[derive(zerocopy::AsBytes)]
pub struct SetAuthorityData
{
    pub instruction_code : u8, // 3 = SetAdministrator, 4 = SetOperationAuthority, 5 = SetRewardsAuthority
    pub new_authority : [u8; 32]
}

#[repr(C)]
#[derive(zerocopy::AsBytes)]
pub struct SetVoteAuthorityData
{
    pub instruction_code : u8, // 6 = SetVoteAuthority
    pub new_authority : [u8; 32]
}

#[repr(C)]
#[derive(zerocopy::AsBytes)]
pub struct SetValidatorIdentityData
{
    pub instruction_code : u8 // 7 = SetValidatorIdentity
}

#[repr(C)]
#[derive(zerocopy::AsBytes)]
pub struct WithdrawData
{
    pub instruction_code : u8, // 8 = Withdraw

    pub padding : [u8; 7],

    pub lamports : u64
}

#[repr(C)]
#[derive(zerocopy::AsBytes)]
pub struct SetCommissionData
{
    pub instruction_code : u8, // 9 = SetCommission

    pub new_commission : u8
}
