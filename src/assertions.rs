use pinocchio::{
    account_info::{AccountInfo, Ref},
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    sysvars::rent::Rent,
    ProgramResult,
};

// #[cfg(feature = "token-2022")]
// use pinocchio_token::state::{AccountState, TokenAccount};
// use pinocchio_token_2022::state::{AccountState, TokenAccount};

use crate::cmp_pubkeys;

/// Trait for accounts that can be initialized
pub trait IsInitialized {
    fn is_initialized(&self) -> bool;
}

/// Trait for accounts that can be unpacked from account data  ...more like read from bytes than unpack
pub trait Pack: Sized {
    const LEN: usize;
    unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self;
}

// // Implement for pinocchio_token::TokenAccount
// impl IsInitialized for pinocchio_token::state::TokenAccount {
//     fn is_initialized(&self) -> bool {
//         self.is_initialized()
//     }
// }

// impl Pack for pinocchio_token::state::TokenAccount {
//     const LEN: usize = Self::LEN;

//     unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
//         Self::from_bytes_unchecked(bytes)
//     }
// }

pub fn assert_signer(account_info: &AccountInfo) -> ProgramResult {
    if !account_info.is_signer() {
        Err(ProgramError::MissingRequiredSignature)
    } else {
        Ok(())
    }
}

pub fn assert_initialized<T: Pack + IsInitialized>(
    account_info: &AccountInfo,
    error: impl Into<ProgramError>,
) -> Result<Ref<'_, T>, ProgramError> {
    let data = account_info.try_borrow_data()?;
    if data.len() < T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }

    let account = Ref::map(data, |bytes| unsafe { T::from_bytes_unchecked(bytes) });

    if !account.is_initialized() {
        Err(error.into())
    } else {
        Ok(account)
    }
}

pub fn assert_owned_by(
    account: &AccountInfo,
    owner: &Pubkey,
    error: impl Into<ProgramError>,
) -> ProgramResult {
    if account.owner() != owner {
        Err(error.into())
    } else {
        Ok(())
    }
}

pub fn assert_owner_in(
    account: &AccountInfo,
    owners: &[Pubkey],
    error: impl Into<ProgramError>,
) -> ProgramResult {
    if owners
        .iter()
        .any(|owner| cmp_pubkeys(owner, account.owner()))
    {
        Ok(())
    } else {
        Err(error.into())
    }
}

pub fn assert_derivation(
    program_id: &Pubkey,
    account: &AccountInfo,
    path: &[&[u8]],
    error: impl Into<ProgramError>,
) -> Result<u8, ProgramError> {
    let (key, bump) = pubkey::find_program_address(path, program_id);
    if key != *account.key() {
        return Err(error.into());
    }
    Ok(bump)
}

pub fn assert_derivation_with_bump(
    program_id: &Pubkey,
    account: &AccountInfo,
    path: &[&[u8]],
    error: impl Into<ProgramError>,
) -> Result<(), ProgramError> {
    let key = pubkey::create_program_address(path, program_id)?;
    if key != *account.key() {
        return Err(error.into());
    }
    Ok(())
}

pub fn assert_rent_exempt(
    rent: &Rent,
    account_info: &AccountInfo,
    error: impl Into<ProgramError>,
) -> ProgramResult {
    if !rent.is_exempt(account_info.lamports(), account_info.data_len()) {
        Err(error.into())
    } else {
        Ok(())
    }
}
