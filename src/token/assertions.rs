use pinocchio::{
    account_info::{AccountInfo, Ref},
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};
use pinocchio_pubkey::pubkey;
use pinocchio_token_2022::state::{AccountState as Account, TokenAccount};

use crate::assert_initialized;
use crate::assertions::{IsInitialized, Pack};

pub static SPL_TOKEN_PROGRAM_IDS: [Pubkey; 2] = [
    pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
    pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
];

impl Pack for TokenAccount {
    const LEN: usize = Self::BASE_LEN;

    unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
        Self::from_bytes_unchecked(bytes)
    }
}

impl IsInitialized for TokenAccount {
    fn is_initialized(&self) -> bool {
        Self::is_initialized(&self)
    }
}

pub trait ToTokenAccount<'a> {
    fn to_token_account(&'a self) -> Ref<'a, TokenAccount>;
}

impl<'a> ToTokenAccount<'a> for AccountInfo {
    fn to_token_account(&'a self) -> Ref<'a, TokenAccount> {
        assert_initialized::<TokenAccount>(&self, ProgramError::UninitializedAccount).unwrap()
    }
}

// impl<'a> ToTokenAccount<'a> for TokenAccount {
//     fn to_token_account(&'a self) -> Ref<'a, TokenAccount> {
//         self // we can't just return self or *self[moving out of scope] in a no-copy environment
//     }
// }

pub fn assert_token_program_matches_package(
    token_program_info: &AccountInfo,
    error: impl Into<ProgramError>,
) -> ProgramResult {
    if SPL_TOKEN_PROGRAM_IDS
        .iter()
        .any(|program_id| program_id == token_program_info.key())
    {
        Ok(())
    } else {
        Err(error.into())
    }
}

/// Asserts that
/// * the given token account is initialized
/// * it's owner matches the provided owner
/// * it's mint matches the provided mint
/// * it holds more than than 0 tokens of the given mint.
/// Accepts either an &AccountInfo or an Account for token_account parameter.
pub fn assert_holder(
    token_account: &AccountInfo, // we're using just AccountInfo here since we only want to work with already parsed AccountInfo and not AccountInfo | TOkenAccount
    owner_info: &AccountInfo,
    mint_info: &AccountInfo,
    error: impl Into<ProgramError> + Clone,
) -> ProgramResult {
    let token_account = token_account.to_token_account();

    if token_account.owner() != owner_info.key() {
        return Err(error.into());
    }

    if token_account.mint() != mint_info.key() {
        return Err(error.into());
    }

    if token_account.amount() == 0 {
        return Err(error.into());
    }

    Ok(())
}
