use {
    crate::state::BOQAccount,
    solana_program::{
        account_info::AccountInfo, 
        program_error::ProgramError, 
        pubkey::Pubkey, 
        msg,
    },
};

pub struct Check;

impl Check {

    // /// Check that `length` is gte `min_length`.
    // pub fn min_length(
    //     length: usize, 
    //     min_length: usize, 
    // ) -> Result<(), ProgramError> {
    //     if length < min_length {
    //         msg!(
    //             "The length {} is less that the minimum required length {}", 
    //             length,
    //             min_length,
    //         );
    //         Err(ProgramError::AccountDataTooSmall)
    //     } else {
    //         Ok(())
    //     }
    // }

    // /// Check that the NFT token (`token_info`) and its metadata (`metadata_info`) accounts are 
    // /// valid.
    // pub fn nft<'a>(
    //     token_info: &AccountInfo,
    //     metadata_info: &AccountInfo,
    //     metadata_bump: u8,
    //     creator_key: &Pubkey,
    //     collection_key: Option<&Pubkey>,
    // ) -> Result<(), ProgramError> {
    //     let metadata_program_id = &mpl_token_metadata::ID;
    //     // Check owners.
    //     Check::owner(token_info, &spl_token::ID)?;
    //     Check::owner(metadata_info, &metadata_program_id)?;
    //     // Check relationship between token and metadata.
    //     let metadata = Metadata::from_account_info(metadata_info)?;
    //     Check::pubkey(token_info.key, &metadata.mint)?;
    //     let pubkey = Pubkey::create_program_address(
    //         &[
    //             mpl_token_metadata::pda::PREFIX.as_ref(), 
    //             metadata_program_id.as_ref(), 
    //             metadata.mint.as_ref(), 
    //             &[metadata_bump],
    //         ], 
    //         metadata_program_id
    //     )?;
    //     Self::pubkey(&pubkey, metadata_info.key)?;
    //     // Check creator.
    //     let creators = &metadata.data.creators.unwrap();
    //     let creator = creators.get(0).unwrap();
    //     Check::pubkey(&creator.address, creator_key)?;
    //     Check::assert(creator.verified, "Unverified creator.")?;
    //     // Check collection.
    //     if let Some(collection_key) = collection_key {
    //         let collection = metadata.collection.unwrap();
    //         Check::assert(collection.verified, "Unverified collection.")?;
    //         Check::pubkey(collection_key, &collection.key)
    //     } else {
    //         Ok(())
    //     }
    // }

    // pub fn nftMetadata(
    //     address: &Pubkey,
    //     mint: &Pubkey,
    //     bump: u8,
    // ) -> Result<(), ProgramError> {
    //     let metadataProgramId = &mpl_token_metadata::ID;
    //     let pubkey = Pubkey::create_program_address(
    //         &[
    //             mpl_token_metadata::pda::PREFIX.as_ref(), 
    //             metadataProgramId.as_ref(), 
    //             mint.as_ref(), 
    //             &[bump],
    //         ], 
    //         metadataProgramId
    //     )?;
    //     Self::pubkey(&pubkey, address)
    // }

    pub fn assert(
        condition: bool,
        message: &str,
    ) -> Result<(), ProgramError> {
        if !condition {
            msg!("Assertion failed: {}", message);
            Err(ProgramError::InvalidArgument)
        } else {
            Ok(())
        }
    }
    
    /// Check that `account_info` is a signer account.
    pub fn signer(
        account_info: &AccountInfo,
    ) -> Result<(), ProgramError> {
        if !account_info.is_signer {
            msg!("Missing signature for account {}", account_info.key);
            Err(ProgramError::MissingRequiredSignature)
        } else {
            Ok(())
        }
    }

    /// Check that `account_info` is a writable account.
    pub fn writable(
        account_info: &AccountInfo,
    ) -> Result<(), ProgramError> {
        if !account_info.is_writable {
            msg!("Writable account required for {}", account_info.key);
            Err(ProgramError::InvalidAccountData)
        } else {
            Ok(())
        }
    }

    /// Check that `account_info` is a readonly account.
    pub fn readonly(
        account_info: &AccountInfo,
    ) -> Result<(), ProgramError> {
        if !account_info.is_writable {
            msg!("Readonly account required for {}", account_info.key);
            Err(ProgramError::InvalidAccountData)
        } else {
            Ok(())
        }
    }

    /// Check that `account_info` is a signer and writable account.
    pub fn signer_and_writable(
        account_info: &AccountInfo,
    ) -> Result<(), ProgramError> {
        Self::signer(account_info)?;
        Self::writable(account_info)
    }

    /// Check that `account_info` is a signer and readonly account.
    pub fn signer_and_readonly(
        account_info: &AccountInfo,
    ) -> Result<(), ProgramError> {
        Self::signer(account_info)?;
        Self::readonly(account_info)
    }

    /// Check that `account_info` is owned by `owner_id`.
    pub fn owner(
        account_info: &AccountInfo,
        owner_id: &Pubkey,
    ) -> Result<(), ProgramError> {
        if account_info.owner.ne(owner_id) {
            msg!("Invalid Owner: expected {}, received {}", owner_id, account_info.owner);
            Err(ProgramError::IncorrectProgramId)
        } else {
            Ok(())
        }
    }

    /// Check that `pubkey` is equal to `account_key`.
    pub fn pubkey(
        pubkey: &Pubkey,
        account_key: &Pubkey,
    ) -> Result<(), ProgramError> {
        if pubkey.ne(account_key) {
            msg!("Invalid Pubkey: expected {}, received {}", pubkey, account_key);
            Err(ProgramError::IncorrectProgramId)
        } else {
            Ok(())
        }
    }

    /// Check that `account_info` is `account_key`.
    pub fn account(
        account_info: &AccountInfo,
        account_key: &Pubkey
    ) -> Result<(), ProgramError> {
        if account_info.key.ne(account_key) {
            msg!("Invalid Account: expected {}, received {}", account_info.key, account_key);
            Err(ProgramError::InvalidAccountData)
        } else {
            Ok(())
        }
    }

    /// Check that `pda_info` is an account derived from `seeds`.
    pub fn pda(
        program_id: &Pubkey,
        pda_info: &AccountInfo,
        seeds: &[&[u8]],
    ) -> Result<(), ProgramError> {
        let pda = Pubkey::create_program_address(
            seeds,
            program_id,
        )?;
        Self::account(
            pda_info, 
            &pda,
        )
    }

    /// Check that `ata_info` is an associated token account derived from `pda_info` and 
    /// `token_mint`.
    pub fn ata(
        ata_info: &AccountInfo,
        token_mint: &Pubkey,
        wallet: &Pubkey,
    ) -> Result<(), ProgramError> {
        let ata = spl_associated_token_account::get_associated_token_address(
            &wallet, 
            token_mint,
        );
        Self::account(
            ata_info, 
            &ata,
        )
    }

    /// Check that `account` has been initialized.
    pub fn initialized(
        account: &impl BOQAccount,
        account_info: &AccountInfo,
    ) -> Result<(), ProgramError> {
        if !account.is_initialized() {
            msg!("Uninitialized account {}", account_info.key);
            Err(ProgramError::UninitializedAccount)
        } else {
            Ok(())
        }
    }

    /// Check that `account` is uninitialized.
    pub fn uninitialized(
        account: &impl BOQAccount,
        account_info: &AccountInfo,
    ) -> Result<(), ProgramError> {
        if account.is_initialized() {
            msg!("Account already initialized {}", account_info.key);
            Err(ProgramError::AccountAlreadyInitialized)
        } else {
            Ok(())
        }
    }
}