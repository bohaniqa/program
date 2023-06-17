use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{pubkey::Pubkey, slot_history::Slot},
};

/***************************************************************************************************
 * ACCOUNTS
***************************************************************************************************/

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum BOQAccountType {
    Uninitialized,
    MintAuthority,
    Employer,
    Employee,
    Shift,
}

impl Default for BOQAccountType {
    fn default() -> Self {
        BOQAccountType::Uninitialized
    }
}

pub struct BOQSeed;
impl BOQSeed {
    pub const MINT_AUTHORITY: &'static str = "mint_authority";
    pub const EMPLOYER: &'static str = "employer";
    pub const EMPLOYEE: &'static str = "employee";
    pub const SHIFT: &'static str = "shift";
}

pub trait BOQAccount {
    fn is_initialized(&self) -> bool;
}

/***************************************************************************************************
 * MINT AUTHORITY
***************************************************************************************************/

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BOQMintAuthority {
    pub account_type: BOQAccountType,
    pub bump: u8,
}

impl BOQAccount for BOQMintAuthority {

    fn is_initialized(&self) -> bool { 
        self.account_type == BOQAccountType::MintAuthority 
    }
}

impl BOQMintAuthority {

    pub const MAX_SIZE: usize = 
        1 + 
        1;

    pub fn new(bump: u8) -> Self {
        Self { 
            account_type: BOQAccountType::MintAuthority,
            bump,
        }
    }
}

/***************************************************************************************************
 * EMPLOYER
***************************************************************************************************/

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BOQEmployer {

    pub account_type: BOQAccountType,
    pub bump: u8,
    pub is_active: bool,

    pub employees: u16,
    pub max_employees: u16,

    pub start_slot: Slot,
    pub end_slot: Slot,
    pub slots_per_shift: u64,
    pub base_rate_per_slot: u64,
    pub inflation_rate_per_slot: u64,

    pub token_mint: Pubkey,
    pub collection_mint: Pubkey,
}

impl BOQAccount for BOQEmployer {

    fn is_initialized(&self) -> bool { 
        self.account_type == BOQAccountType::Employer 
    }
}

impl BOQEmployer {

    pub const MAX_SIZE: usize = 
        1 + 
        1 +
        1 + 
        2 + 
        2 + 
        8 + 
        8 +
        8 +
        8 + 
        8 +
        32 +
        32;

    pub fn new(
        bump: u8,
        is_active: bool,
        max_shifts: u16,
        max_employees: u16,
        start_slot: Slot,
        slots_per_shift: u64,
        base_rate_per_slot: u64,
        token_mint: Pubkey,
        collection_mint: Pubkey,
    ) -> Self {
        Self { 
            account_type: BOQAccountType::Employer,
            bump,
            is_active,
            employees: 0,
            max_employees,
            start_slot,
            end_slot: start_slot + (u64::from(max_shifts) * slots_per_shift),
            slots_per_shift,
            base_rate_per_slot,
            inflation_rate_per_slot: base_rate_per_slot / 1000,
            token_mint,
            collection_mint,
        }
    }

    // pub fn current_shift(&self, slot: Slot) -> u64 {
    //     if slot < self.start_slot { 
    //         0 
    //     } else { 
    //         (slot - self.start_slot) / self.slots_per_shift 
    //     }
    // }
}

/***************************************************************************************************
 * EMPLOYEE
***************************************************************************************************/

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BOQEmployee {
    pub account_type: BOQAccountType,
    pub bump: u8,
    pub last_slot: Slot,
    pub total_slots: Slot,
    pub nft_mint: Pubkey,
}

impl BOQAccount for BOQEmployee {

    fn is_initialized(&self) -> bool { 
        self.account_type == BOQAccountType::Employee 
    }
}

impl BOQEmployee {

    pub const MAX_SIZE: usize = 
        1 + 
        1 +
        8 +
        8 +
        32;

    pub fn new(
        bump: u8,
        mint: Pubkey,
    ) -> Self {
        Self { 
            account_type: BOQAccountType::Employee,
            bump,
            last_slot: 0,
            total_slots: 0,
            nft_mint: mint,
        }
    }

    // pub fn total_shifts(&self, employer: &BOQEmployer) -> u64 {
    //     self.total_slots / employer.slots_per_shift
    // }
}

/***************************************************************************************************
 * SHIFT
***************************************************************************************************/

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BOQShift {
    pub account_type: BOQAccountType,
    pub bump: u8,
    pub slot: Slot,
    pub total_slots: u64,
    pub total_rewards: u64,
    pub owner: Pubkey,
}

impl BOQAccount for BOQShift {

    fn is_initialized(&self) -> bool { 
        self.account_type == BOQAccountType::Shift 
    }
}

impl BOQShift {

    pub const MAX_SIZE: usize = 
        1 + 
        1 +
        8 +
        8 +
        8 +
        32;

    pub fn new(
        bump: u8, 
        slot: Slot,
        owner: Pubkey,
    ) -> Self {
        Self { 
            account_type: BOQAccountType::Shift,
            bump,
            slot,
            total_slots: 0,
            total_rewards: 0,
            owner,
        }
    }
}