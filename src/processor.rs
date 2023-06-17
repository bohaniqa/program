use mpl_token_metadata::state::{Metadata, TokenMetadataAccount};

use {
    borsh::{
        BorshDeserialize, 
        BorshSerialize, 
    },
    crate::{
        check::Check,
        instruction::BOQInstruction,
        state::*,
    },
    solana_program::{
        account_info::{
            next_account_info, 
            AccountInfo, 
        },
        clock::Clock,
        entrypoint::ProgramResult,
        program::{invoke_signed},
        program_pack::Pack, 
        pubkey::Pubkey,
        rent::Rent, 
        slot_history::{Slot},
        sysvar::Sysvar, 
        system_instruction,
    },
    spl_token::state::{
        Account, 
    },
    std::{
        cmp::min, 
    }
};

pub struct Processor;

impl Processor {

    pub fn process(
        program_id: &Pubkey, 
        accounts: &[AccountInfo], 
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = BOQInstruction::try_from_slice(instruction_data)?;
        match instruction {
            BOQInstruction::Test => {
                Self::process_test(
                    program_id, 
                    accounts, 
                )
            },
            
            BOQInstruction::CreateMintAuthority {
                bump,
            } => {
                Self::process_create_mint_authority(
                    program_id, 
                    accounts, 
                    bump,
                )
            },
            BOQInstruction::InitializeMintAuthority {
                bump,
            } => {
                Self::process_initialize_mint_authority(
                    program_id, 
                    accounts, 
                    bump,
                )
            },
            BOQInstruction::SetMintAuthority {
                bump,
                new_authority_pubkey,
            } => {
                Self::process_set_mint_authority(
                    program_id, 
                    accounts, 
                    bump,
                    &new_authority_pubkey, 
                )
            },

            BOQInstruction::CreateEmployer {
                bump,
            } => {
                Self::process_create_employer(
                    program_id, 
                    accounts, 
                    bump,
                )
            },
            BOQInstruction::InitializeEmployer { 
                bump,
                token_mint,
                collection_mint,
                max_shifts,
                max_employees,
                start_slot, 
                slots_per_shift, 
                base_rate_per_slot, 
             } => {
                Self::process_initialize_employer(
                    program_id, 
                    accounts, 
                    bump,
                    &token_mint,
                    &collection_mint,
                    max_shifts,
                    max_employees,
                    start_slot,
                    slots_per_shift,
                    base_rate_per_slot,
                )
             },

            BOQInstruction::CreateEmployee {
                bump,
            } => {
                Self::process_create_employee(
                    program_id, 
                    accounts, 
                    bump,
                )
            },
            BOQInstruction::InitializeEmployee {
                bump,
                nft_mint,
            } => {
                Self::process_initialize_employee(
                    program_id, 
                    accounts, 
                    bump,
                    &nft_mint,
                )
             },
             
             BOQInstruction::CreateShift {
                bump
             } => {
                Self::process_create_shift(
                    program_id, 
                    accounts,
                    bump,
                )
             },
             BOQInstruction::InitializeShift {
                bump,
                slot,
                owner,
             } => {
                Self::process_initialize_shift(
                    program_id, 
                    accounts,
                    bump,
                    slot,
                    &owner,
                )
             },
             BOQInstruction::Shift {
                number_of_employees,
             } => {
                Self::process_shift(
                    program_id, 
                    accounts,
                    number_of_employees,
                )
             }
        }
    }

    /**
     * Test instruction.
     */
    fn process_test(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
    ) -> ProgramResult {
        Ok(())
    }

    /**
     * Creates a PDA account derived from `seed` and `bump`.
     * 
     * Throws a [ProgramError] if the account already exists.
     */
    fn _process_create_pda(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        space: usize,
        seeds: &[&[u8]],
        sign: bool,
    ) -> ProgramResult {

        // The instruction accounts.
        let account_info_iter = &mut accounts.iter();
        
        // The transaction fee payer.
        let payer_info = next_account_info(account_info_iter)?;
        
        // The `PDA` account.
        let pda_info = next_account_info(account_info_iter)?;

        // The shift program.
        let shift_program_info = next_account_info(account_info_iter)?;
        if sign {
            Check::signer(shift_program_info)?;
        }
        
        // The system program.
        let system_program_info = next_account_info(account_info_iter)?;

        // The Sysvar rent.
        let rent = Rent::get()?;

        // Create a new PDA account of size `space`. 
        //
        // Throws an exception if the account already exists or `signers_seeds` does not derive 
        // the `pda_info` account.
        invoke_signed(
            &system_instruction::create_account(
                &payer_info.key,
                &pda_info.key,
                rent.minimum_balance(space), 
                space.try_into().unwrap(),
                shift_program_info.key, // DO NOT use program_id.
            ),
            &[
                payer_info.clone(),
                pda_info.clone(),
                system_program_info.clone(),
            ],
            &[
                seeds,
            ],
        )
    }

    /**
     * Creates the PDA for [BOQMintAuthority].
     * 
     * Throws a [ProgramError] if the account already exists.
     */
    fn process_create_mint_authority(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        bump: u8,
    ) -> ProgramResult {
        Self::_process_create_pda(
            program_id, 
            accounts, 
            BOQMintAuthority::MAX_SIZE, 
            &[BOQSeed::MINT_AUTHORITY.as_bytes(), &[bump]],
            true,
        )
    }

    /**
     * Initializes the [BOQMintAuthority] PDA account.
     * 
     * This instruction does not check for signers and MUST be sent in the same transaction as 
     * [BOQInstruction::CreateMintAuthority] to prevent another account from taking control.
     * 
     * Throws a [ProgramError] if the account has already been initialized.
     */
    fn process_initialize_mint_authority(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        bump: u8,
    ) -> ProgramResult {
        
        // The `mint authority` PDA account.
        let mint_authority_info = &accounts[0];
        let mint_authority_data = &mut mint_authority_info.data.borrow_mut();
        let mint_authority = BOQMintAuthority::try_from_slice(mint_authority_data)?;
        Check::uninitialized(&mint_authority, mint_authority_info)?;
        
        // Set account data.
        BOQMintAuthority::new(bump).serialize(&mut &mut mint_authority_data[..])?;

        Ok(())
    }

    /**
     * Sets `new_authority_pubkey` as the new mint authority of `token_mint`.
     * 
     * `Must be signed by the program account`.
     * 
     * Throws a [ProgramError] if the shift program is not the current mint authority.
     */
    fn process_set_mint_authority(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        bump: u8,
        new_authority_pubkey: &Pubkey,
    ) -> ProgramResult {

        // The instruction accounts.
        let account_info_iter = &mut accounts.iter();

        // The token mint.
        let token_mint_info = next_account_info(account_info_iter)?;

        // The `mint authority` PDA account.
        let mint_authority_info = next_account_info(account_info_iter)?;
        Check::owner(mint_authority_info, program_id)?;
        let seeds = &[BOQSeed::MINT_AUTHORITY.as_bytes(), &[bump]];
        Check::pda(program_id, mint_authority_info, seeds)?;

        // The shift program account.
        let shift_program_info = next_account_info(account_info_iter)?;
        Check::signer(shift_program_info)?;
        Check::pubkey(shift_program_info.key, program_id)?;

        // The token_program.
        let token_program_info = next_account_info(account_info_iter)?;

        invoke_signed(
            &spl_token::instruction::set_authority(
                token_program_info.key, 
                token_mint_info.key, 
                Some(&new_authority_pubkey), 
                spl_token::instruction::AuthorityType::MintTokens, 
                mint_authority_info.key, 
                &[],
            )?, 
            &[
                token_mint_info.clone(),
                mint_authority_info.clone(),
                token_program_info.clone(),
            ], 
            &[
                seeds,
            ],
        )
    }

    /**
     * Creates the [BOQEmployer] account.
     * 
     * Throws a [ProgramError] if the account already exists.
     */
    fn process_create_employer(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        bump: u8,
    ) -> ProgramResult {
        Self::_process_create_pda(
            program_id, 
            accounts, 
            BOQEmployer::MAX_SIZE, 
            &[BOQSeed::EMPLOYER.as_ref(), &[bump]],
            true,
        )
    }

    /**
     * Initializes the [BOQEmployer] account.
     * 
     * This instruction does not check for signers and MUST be sent in the same transaction as 
     * [BOQInstruction::CreateEmployer] to prevent another account from taking control.
     * 
     * Throws a [ProgramError] if the account has already been initialized.
     */
    fn process_initialize_employer(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        bump: u8,
        token_mint: &Pubkey,
        collection_mint: &Pubkey,
        max_shifts: Option<u16>, 
        max_employees: Option<u16>, 
        start_slot: Option<Slot>,
        slots_per_shift: Option<u64>,
        base_rate_per_slot: Option<u64>,
    ) -> ProgramResult {
        
        // The `employer` account.
        let employer_info = &accounts[0];
        let employer_data = &mut employer_info.data.borrow_mut();
        let employer = BOQEmployer::try_from_slice(employer_data)?;
        Check::uninitialized(&employer, employer_info)?;
        
        // The Sysvar clock.
        let clock = Clock::get()?;

        // Set account data.
        BOQEmployer::new(
            bump,
            true,
            max_shifts.unwrap_or(10_000), 
            max_employees.unwrap_or(10_000), 
            start_slot.unwrap_or(clock.slot), 
            slots_per_shift.unwrap_or(250_000), 
            base_rate_per_slot.unwrap_or(100_000),
            *token_mint,
            *collection_mint,
        ).serialize(
            &mut &mut employer_data[..],
        )?;

        Ok(())
    }

    /**
     * Creates the PDA for [BOQEmployee].
     * 
     * Throws a [ProgramError] if the account already exists.
     */
    fn process_create_employee(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        bump: u8,
    ) -> ProgramResult {

        // The instruction accounts.
        let account_info_iter = &mut accounts.iter();

        // The `employer` account.
        let employer_info = next_account_info(account_info_iter)?;
        let employer_data = &mut employer_info.data.borrow_mut();
        let mut employer = BOQEmployer::try_from_slice(&employer_data)?;
        Check::initialized(&employer, employer_info)?;

        if employer.employees < employer.max_employees {

            let nft_token_info = next_account_info(account_info_iter)?;
            let nft_metadata_info = next_account_info(account_info_iter)?;
            Check::owner(nft_metadata_info, &mpl_token_metadata::ID)?;
            let nft_metadata = Metadata::from_account_info(nft_metadata_info)?;
            Check::pubkey(&nft_metadata.mint, nft_token_info.key)?;
            let collection = nft_metadata.collection.unwrap();
            Check::assert(collection.verified, "Unverified collection.")?;
            Check::pubkey(&collection.key, &employer.collection_mint)?; 

            Self::_process_create_pda(
                program_id, 
                &accounts[3..], 
                BOQEmployee::MAX_SIZE, 
                &[BOQSeed::EMPLOYEE.as_ref(), nft_token_info.key.as_ref(), &[bump]],
                false,
            )?;
            employer.employees += 1;
            employer.serialize(&mut &mut employer_data[..])?;
        } 

        Ok(())

        // // The `employer` account.
        // let employer_info = &accounts[0];
        // let employer_data = &mut employer_info.data.borrow_mut();
        // let mut employer = BOQEmployer::try_from_slice(&employer_data)?;
        // Check::initialized(&employer, employer_info)?;

        // if employer.employees < employer.max_employees {
        //     Self::_process_create_pda(
        //         program_id, 
        //         &accounts[1..], 
        //         BOQEmployee::MAX_SIZE, 
        //         &[BOQSeed::EMPLOYEE.as_ref(), nft_mint.as_ref(), &[bump]],
        //         true,
        //     )?;
        //     employer.employees += 1;
        //     employer.serialize(&mut &mut employer_data[..])?;
        // } 

        // Ok(())
    }

    /**
     * Initializes a [BOQEmployee] account.
     * 
     * This instruction does not check for signers and MUST be sent in the same transaction as 
     * [BOQInstruction::CreateEmployee] to prevent another account from taking control.
     * 
     * Throws a [ProgramError] if the account has already been initialized.
     */
    fn process_initialize_employee(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        bump: u8,
        nft_mint: &Pubkey,
    ) -> ProgramResult {

        // The `employee` PDA account to initialize.
        let employee_info = &accounts[0];
        let employee_data = &mut employee_info.data.borrow_mut();
        let employee = BOQEmployee::try_from_slice(&employee_data)?;
        Check::uninitialized(&employee, employee_info)?;
        
        let seeds = &[BOQSeed::EMPLOYEE.as_ref(), nft_mint.as_ref(), &[bump]];
        Check::pda(program_id, employee_info, seeds)?;

        // Set account data.
        BOQEmployee::new(bump, *nft_mint).serialize(&mut &mut employee_data[..])?;

        Ok(())
    }

    /**
     * Creates a [BOQShift] PDA account.
     * 
     * Throws a [ProgramError] if the account already exists.
     */
    fn process_create_shift(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        bump: u8,
    ) -> ProgramResult {
        let owner_info = &accounts[0];
        Self::_process_create_pda(
            program_id, 
            accounts, 
            BOQShift::MAX_SIZE, 
            &[BOQSeed::SHIFT.as_ref(), owner_info.key.as_ref(), &[bump]],
            false,
        )

        // // The instruction accounts.
        // let account_info_iter = &mut accounts.iter();
        
        // // The shift account owner.
        // let owner_info = next_account_info(account_info_iter)?;

        // // The ATA of `owner_info` for `token_mint_info`.
        // let ata_info = next_account_info(account_info_iter)?;

        // // The ATA's token mint account.
        // let token_mint_info = next_account_info(account_info_iter)?;

        // // The `employer` to create the shift account for.
        // let employer_info = next_account_info(account_info_iter)?;
        // Check::owner(employer_info, program_id)?;
        // let employer = BOQEmployer::try_from_slice(&employer_info.data.borrow())?;
        // Check::initialized(&employer, employer_info)?;
        
        // // Check that token_mint_info matches the employer's token mint.
        // Check::pubkey(token_mint_info.key, &employer.token_mint)?;

        // // The shift PDA account to create.
        // let shift_info = next_account_info(account_info_iter)?;

        // // The system program.
        // let system_program_info = next_account_info(account_info_iter)?;

        // // The token program.
        // let token_program_info = next_account_info(account_info_iter)?;

        // // The associated token program.
        // let associated_token_program_info = next_account_info(account_info_iter)?;

        // // The Sysvar rent.
        // let rent = Rent::get()?;

        // // The new account's data size.
        // let space = BOQShift::MAX_SIZE;

        // // Creates an ATA if it does not exist.
        // invoke(
        //     &spl_associated_token_account::instruction::create_associated_token_account_idempotent(
        //         owner_info.key, 
        //         owner_info.key, 
        //         token_mint_info.key, 
        //         &token_program_info.key,
        //     ), 
        //     &[
        //         owner_info.clone(),
        //         ata_info.clone(),
        //         owner_info.clone(),
        //         token_mint_info.clone(),
        //         system_program_info.clone(),
        //         token_program_info.clone(),
        //         associated_token_program_info.clone(),
        //     ],
        // )?;

        // // Create a new PDA account of size `space`.
        // //
        // // Throws an exception if the account already exists or `signers_seeds` does not derive the 
        // // `shift_info` PDA account.
        // invoke_signed(
        //     &system_instruction::create_account(
        //         &owner_info.key,
        //         &shift_info.key,
        //         rent.minimum_balance(space), 
        //         space.try_into().unwrap(),
        //         program_id,
        //     ),
        //     &[
        //         owner_info.clone(),
        //         shift_info.clone(),
        //     ],
        //     &[&[
        //         BOQSeed::SHIFT.as_ref(),
        //         employer_info.key.as_ref(), 
        //         owner_info.key.as_ref(), 
        //         &[bump],
        //     ]],
        // )
    }

    /**
     * Initializes a [BOQShift] account.
     * 
     * This instruction does not check for signers and MUST be sent in the same transaction as 
     * [BOQInstruction::CreateShift] to prevent another account from taking control.
     * 
     * Throws a [ProgramError] if the account has already been initialized.
     */
    fn process_initialize_shift(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        bump: u8,
        slot: Slot,
        owner: &Pubkey,
    ) -> ProgramResult {
        
        // The `shift` PDA account to initialize.
        let shift_info = &accounts[0];
        let shift_data = &mut shift_info.data.borrow_mut();
        let shift = BOQShift::try_from_slice(&shift_data)?;
        Check::uninitialized(&shift, shift_info)?;

        // Set account data.
        BOQShift::new(bump, slot, *owner).serialize(&mut &mut shift_data[..])?;

        Ok(())
    }

    /**
     * Pays out the available wage to the current NFT holder.
     * 
     * This instruction does not check for signers as the NFT holder is always the correct recipient 
     * of the available payment.
     * 
     * Throws a [ProgramError] for an invalid request.
     */
    fn process_shift(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        number_of_employees: u8,
    ) -> ProgramResult {

        // The instruction accounts.
        let account_info_iter = &mut accounts.iter();

        // The `mint authority` PDA account.
        let mint_authority_info = next_account_info(account_info_iter)?;
        Check::owner(&mint_authority_info, program_id)?;
        let mint_authority = BOQMintAuthority::try_from_slice(&mint_authority_info.data.borrow())?;
        Check::initialized(&mint_authority, mint_authority_info)?;
        let mint_seeds = &[BOQSeed::MINT_AUTHORITY.as_bytes(), &[mint_authority.bump]];

        // The `employer` PDA account.
        let employer_info = next_account_info(account_info_iter)?;
        Check::owner(&employer_info, program_id)?;
        let employer = BOQEmployer::try_from_slice(&employer_info.data.borrow())?;
        Check::initialized(&employer, employer_info)?;

        // A `shift` account.
        let shift_info = next_account_info(account_info_iter)?;
        Check::owner(&shift_info, program_id)?;
        let shift_data = &mut shift_info.data.borrow_mut();
        let mut shift = BOQShift::try_from_slice(&shift_data)?;
        Check::initialized(&shift, shift_info)?;

        // The `salary` token's mint account.
        let token_mint_info = next_account_info(account_info_iter)?;
        Check::pubkey(token_mint_info.key, &employer.token_mint)?;

        // The `salary` token's receiver account.
        let ata_info = next_account_info(account_info_iter)?;
        Check::ata(ata_info, token_mint_info.key, &shift.owner)?;

        // // The shift program.
        // let shift_program_info = next_account_info(account_info_iter)?;
        // Check::pubkey(shift_program_info.key, program_id)?;

        // The token program.
        let token_program_info = next_account_info(account_info_iter)?;

        // Get the slot information.
        let slot = Clock::get()?.slot;
        let start_slot = employer.start_slot;
        let end_slot = employer.end_slot;
        let slots_per_shift = employer.slots_per_shift;

        // Check that the employer is still running.
        let active_message = "Mining not available.";
        Check::assert(slot >= start_slot && slot <= end_slot, active_message)?;

        let mut total_amount = 0;

        for _i in 0..number_of_employees {
            
            // The user's NFT token account.
            let nft_token_info = next_account_info(account_info_iter)?;

            // The `employee` PDA account.
            let employee_info = next_account_info(account_info_iter)?;

            // Unpack the NFT SPL token.
            let nft_token = Account::unpack(&nft_token_info.data.borrow())?;

            // Check that `nft_token_info` is potentially an NFT (amount == 1) and is owned by the 
            // provided shift account.
            if nft_token.amount == 1 && nft_token.owner.eq(&shift.owner)  {

                // Check that `employee_info` is a valid PDA account.
                Check::owner(&employee_info, program_id)?;
                let employee_data = &mut employee_info.data.borrow_mut();
                let mut employee = BOQEmployee::try_from_slice(&employee_data)?;
                Check::initialized(&employee, employee_info)?;

                // Check that the provided NFT token account and employee PDA account are for the 
                // same token mint.
                Check::pubkey(&nft_token.mint, &employee.nft_mint)?;

                // Calculate the base rate.
                let elapsed_slots = slot - employee.last_slot;
                let available_slots = min(elapsed_slots, slots_per_shift);
                if available_slots > 0 {

                    let base_rate = employer.base_rate_per_slot * available_slots;
                    let employee_total_slots = employee.total_slots + available_slots;

                    // Calculate the inflation rate.
                    let inflation_rate = if employee_total_slots > slots_per_shift {
                        let current_shift = employee.total_slots / slots_per_shift;
                        let next_shift = current_shift + 1;
                        let shift_boundary = next_shift * employer.slots_per_shift;
                        let next_shift_slots = if employee_total_slots > shift_boundary { 
                            employee_total_slots % slots_per_shift
                        } else {
                            0
                        };
                        let current_shift_slots = available_slots - next_shift_slots;
                        (employer.inflation_rate_per_slot * current_shift_slots * current_shift)
                        + (employer.inflation_rate_per_slot * next_shift_slots * next_shift)
                    } else {
                        0
                    };

                    // // Calculate the bonus.
                    // let employee_shifts = employee.total_shifts(&employer);
                    // let completed_shifts = min(employee_shifts, current_shift);
                    // let bonus_rate = completed_shifts * employer.rate_increase_per_shift;

                    let amount = base_rate + inflation_rate;

                    shift.total_slots += available_slots;
                    shift.total_rewards += amount;
                    shift.serialize(&mut &mut shift_data[..])?;

                    employee.last_slot = slot;
                    employee.total_slots = employee_total_slots;
                    employee.serialize(&mut &mut employee_data[..])?;

                    total_amount += amount;

                    // let ix = spl_token::instruction::mint_to(
                    //     &spl_token::ID, 
                    //     &token_mint_info.key, 
                    //     &owner_info.key, 
                    //     &shift_program_info.key, 
                    //     &[], 
                    //     amount,
                    // )?;
                    // invoke_signed(
                    //     &ix, 
                    //     &[
                    //         token_program_info.clone(),
                    //         token_mint_info.clone(),
                    //         owner_info.clone(),
                    //         shift_program_info.clone(),
                    //     ], 
                    //     &[

                    //     ],
                    // )
                }
            }
        }

        if total_amount > 0 {
            invoke_signed(
                &spl_token::instruction::mint_to(
                    token_program_info.key, 
                    token_mint_info.key, 
                    ata_info.key, 
                    mint_authority_info.key, 
                    &[], 
                    total_amount, 
                )?,
                &[
                    token_mint_info.clone(),
                    ata_info.clone(),
                    mint_authority_info.clone(),
                    token_program_info.clone(),
                ],
                &[
                    mint_seeds,
                ]
            )
        } else {
            Ok(())
        }
    }
}