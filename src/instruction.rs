use {
    borsh::{
        BorshDeserialize, 
        BorshSerialize, 
    },
    solana_program::{
        pubkey::Pubkey, 
        slot_history::Slot, 
    },
};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum BOQInstruction {
    
    Test,
    
    // MINT AUTHORITY

    /**
     * Create mint authority PDA account.
     */
    CreateMintAuthority {
        bump: u8,
    },
        
    /**
     * Initialize mint authority PDA account.
     */
    InitializeMintAuthority {
        bump: u8,
    },
        
    /**
     * Set [new_authority_pubkey] as the new mint authority.
     */
    SetMintAuthority {
        bump: u8,
        new_authority_pubkey: Pubkey,
    },

    // EMPLOYER
    
    /**
     * Create employer PDA account.
     */
    CreateEmployer {
        bump: u8,
    },
    
    /**
     * Initialize employer PDA account.
     */
    InitializeEmployer {
        bump: u8,
        token_mint: Pubkey,
        collection_mint: Pubkey,
        max_shifts: Option<u16>, 
        max_employees: Option<u16>,
        start_slot: Option<Slot>,
        slots_per_shift: Option<u64>,
        base_rate_per_slot: Option<u64>,
    },

    // EMPLOYEE
    
    /**
     * Create employee PDA account.
     */
    CreateEmployee {
        bump: u8,
    },
    
    /**
     * Initialize employee PDA account.
     */
    InitializeEmployee {
        bump: u8,
        nft_mint: Pubkey,
    },

    // SHIFTS
    
    /**
     * Create shift PDA account.
     */
    CreateShift {
        bump: u8,
    },
    
    /**
     * Initialize shift PDA account.
     */
    InitializeShift {
        bump: u8,
        slot: Slot,
        owner: Pubkey,
    },
    
    /**
     * Work shift.
     */
    Shift {
        number_of_employees: u8,
    },
}