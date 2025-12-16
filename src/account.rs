use core::array;

use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{Pubkey, MAX_SEEDS},
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

#[allow(unused_imports)]
use pinocchio_log::log;

#[allow(unused_imports)]
use pinocchio_system::ID as SYSTEM_PROGRAM_ID;

#[allow(unused_imports)]
use pinocchio_system::instructions::{Allocate, Assign, CreateAccount, Transfer};

/// Create account almost from scratch, lifted from
/// <https://github.com/solana-labs/solana-program-library/tree/master/associated-token-account/program/src/processor.rs#L51-L98>
#[allow(unused_variables)]
pub fn create_or_allocate_account_raw<'a>(
    program_id: Pubkey,
    new_account_info: &'a AccountInfo,
    system_program_info: &'a AccountInfo,
    payer_info: &'a AccountInfo,
    size: usize,
    signer_seeds: &[&[u8]],
) -> ProgramResult {
    let rent = Rent::get()?;
    let required_lamports = rent
        .minimum_balance(size)
        .max(1)
        .saturating_sub(new_account_info.lamports());

    if signer_seeds.len() >= MAX_SEEDS {
        return Err(ProgramError::MaxSeedLengthExceeded);
    } // safety check to prevent seeds longer than 16

    let (_pda, bump) = pinocchio::pubkey::find_program_address(signer_seeds, &program_id);
    let bump_binding = [bump];

    // PRE-ALLOCATE STACK BUFFER - you see, you can't just pass a 2D array &[&[u8]] in pinocchio as seeds,
    // so here's my approach since we don't know what seeds format the dev is passing to this helper function

    // Solana limits PDAs to 16 seeds max. We make a buffer of 16 empty seeds.
    // Seed::from(&[]) creates a lightweight empty seed.
    let mut seed_buffer: [Seed; MAX_SEEDS] = array::from_fn(|_| Seed::from(&[]));

    // COPY SEEDS IN A LOOP
    // This is very cheap. It just copies pointers (references), not data.
    for (i, raw_seed) in signer_seeds.iter().enumerate() {
        seed_buffer[i] = Seed::from(*raw_seed);
    }

    let total_seeds = signer_seeds.len();

    // adding the bump to the end of the seeds
    seed_buffer[total_seeds] = Seed::from(&bump_binding);

    let active_seeds = &seed_buffer[0..=total_seeds];

    // [ if lamports exists, do the transfer, allocate, then assign ] else just do the create account

    if new_account_info.lamports() == 0 && new_account_info.data_is_empty() {
        let signer = Signer::from(active_seeds);
        CreateAccount {
            from: payer_info,
            lamports: required_lamports,
            owner: &program_id,
            space: size as u64,
            to: new_account_info,
        }
        .invoke_signed(&[signer])?;
    } else {
        let signer = Signer::from(active_seeds);
        // 1. Transfer lamports
        if required_lamports > 0 {
            log!("Transfer {} lamports to the new account", required_lamports);

            Transfer {
                from: payer_info,
                lamports: required_lamports,
                to: new_account_info,
            }
            .invoke()?;
        }

        // 2. Allocate / Resize
        log!("Allocate space for the account");
        Allocate {
            account: new_account_info,
            space: size as u64,
        }
        .invoke_signed(&[signer])?;

        // 3. Assign (Set Owner)
        log!("Assign the account to the owning program");
        Assign {
            account: new_account_info,
            owner: &program_id,
        }
        .invoke_signed(&[Signer::from(active_seeds)])?;
    }
    Ok(())
}

/// Resize an account using resize
pub fn resize_or_reallocate_account_raw<'a>(
    target_account: &'a AccountInfo,
    funding_account: &'a AccountInfo,
    system_program: &'a AccountInfo,
    new_size: usize,
) -> ProgramResult {
    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(new_size);

    let current_lamports = target_account.lamports();

    if new_size == target_account.data_len() {
        return Ok(());
    }

    if new_size > target_account.data_len() {
        let lamports_needed = new_minimum_balance.saturating_sub(current_lamports);

        if lamports_needed > 0 {
            Transfer {
                from: funding_account,
                lamports: lamports_needed,
                to: target_account,
            }
            .invoke()?;
        }
    } else if target_account.owner() == system_program.key() {
        let lamports_needed = new_minimum_balance.saturating_sub(current_lamports);

        Transfer {
            from: target_account,
            to: funding_account,
            lamports: lamports_needed,
        }
        .invoke()?;
    } else {
        // Calculate excess. If current < new_min (underfunded), this returns 0.
        let lamports_excess = current_lamports.saturating_sub(new_minimum_balance);

        if lamports_excess > 0 {
            // Debit Target account
            *target_account.try_borrow_mut_lamports()? = current_lamports
                .checked_sub(lamports_excess)
                .ok_or(ProgramError::InvalidRealloc)?;

            // Credit Funding account
            let funding_current = funding_account.lamports();
            *funding_account.try_borrow_mut_lamports()? = funding_current
                .checked_add(lamports_excess)
                .ok_or(ProgramError::InvalidRealloc)?;
        }
    }

    target_account.resize(new_size)
}

/// Close src_account and transfer lamports to dst_account
pub fn close_account_raw<'a>(
    dest_account_info: &'a AccountInfo,
    src_account_info: &'a AccountInfo,
) -> ProgramResult {
    // rewrote the logic in a straightforward way
    // 1. Transfer all lamports from source to dest
    let src_lamports = src_account_info.lamports();

    // Borrow mutably to add lamports to destination
    let mut dest_lamports = dest_account_info.try_borrow_mut_lamports()?;
    *dest_lamports = dest_lamports
        .checked_add(src_lamports)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // 2. Zero out source lamports
    let mut src_lamports_ref = src_account_info.try_borrow_mut_lamports()?;
    *src_lamports_ref = 0;

    drop(src_lamports_ref); // without dropping this, calling `.close()` on src_account_info would have panicked

    src_account_info.close()?;
    Ok(())

    // // ====================================================
    // // just modified the initial implementation  here
    // let dest_starting_lamports = dest_account_info.lamports();
    // let mut dest_lamports_mut = dest_account_info
    //     .try_borrow_mut_lamports()
    //     .map_err(|_| ProgramError::AccountBorrowFailed)?;
    // *dest_lamports_mut = dest_starting_lamports
    //     .checked_add(src_account_info.lamports())
    //     .ok_or(ProgramError::InvalidRealloc)?;

    // let mut src_lamports_mut = src_account_info
    //     .try_borrow_mut_lamports()
    //     .map_err(|_| ProgramError::AccountBorrowFailed)?;
    // *src_lamports_mut = 0;

    // unsafe {
    //     src_account_info.assign(&SYSTEM_PROGRAM_ID);
    // }
    // src_account_info.resize(0).unwrap();
}
