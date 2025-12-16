use bytemuck::{Pod, Zeroable};
use pinocchio::program_error::ProgramError;
#[allow(unused_imports)]
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_mpl_utils::{
    assert_initialized, assert_owned_by, assert_owner_in, assert_rent_exempt, assert_signer,
    cmp_pubkeys,
};
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::state::TokenAccount;

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug, PartialEq)]
pub struct GlobalState {
    pub owner: [u8; 32],
    pub token_mint: [u8; 32],
}

#[allow(unused)]
impl GlobalState {
    const LEN: usize = size_of::<Self>();
    pub fn to_bytes(&self) -> Vec<u8> {
        bytemuck::bytes_of(self).to_vec()
    }
}

pub fn process_intialize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // fetch the accounts

    let [signer, state_account, token_account, token_mint, system_program, associated_token_program, token_program, rent_sysvar] =
        accounts
    else {
        return Err(pinocchio::program_error::ProgramError::NotEnoughAccountKeys);
    };

    let seeds = &[b"global_state", signer.key().as_ref()];
    // derive state account onchain
    let (derived_pda, bump) = pubkey::find_program_address(seeds, &crate::ID);

    // check that keys are equal using cmp_key 🔴🔴🔴🔴🔴🔴🔴
    cmp_pubkeys(state_account.key(), &derived_pda);

    // check that signer is signer 🔴🔴🔴🔴🔴🔴🔴
    assert_signer(signer).expect("Signer account should be a signer");

    // assert that the token account is initialized
    assert_initialized::<TokenAccount>(token_account, ProgramError::InvalidAccountData)?;

    let rent_state = Rent::from_account_info(rent_sysvar)?;
    // let rent_state = Rent::get()?;

    let mininum_balance = rent_state.minimum_balance(GlobalState::LEN);

    let initial_bump = bump.to_le();
    let bump = [initial_bump];
    let seeds = [
        Seed::from(b"global_state"),
        Seed::from(signer.key().as_ref()),
        Seed::from(&bump),
    ];

    let signers = Signer::from(&seeds);

    // create account
    CreateAccount {
        from: signer,
        lamports: mininum_balance,
        owner: &crate::ID,
        space: GlobalState::LEN as u64,
        to: state_account,
    }
    .invoke_signed(&[signers])?; // to change this to the create_account in pinocchio_mpl_utils

    // assert who owns the account 🔴🔴🔴🔴🔴🔴🔴
    assert_owned_by(state_account, &crate::ID, ProgramError::IllegalOwner)?;

    // assert the accunt is rent-exempt 🔴🔴🔴🔴🔴🔴🔴
    assert_rent_exempt(
        &*rent_state,
        state_account,
        ProgramError::AccountNotRentExempt,
    )?;

    // assert owner is present in list of accounts 🔴🔴🔴🔴🔴🔴🔴
    assert_owner_in(
        state_account,
        &[crate::ID, *signer.key(), *token_mint.key()],
        ProgramError::IllegalOwner,
    )?;

    Ok(())
}
