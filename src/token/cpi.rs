use core::array;

use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    pubkey::MAX_SEEDS,
    ProgramResult,
};

use pinocchio_token::instructions::CloseAccount;
#[cfg(not(feature = "token-2022"))]
use pinocchio_token::instructions::{Burn, MintTo};

#[cfg(feature = "token-2022")]
use pinocchio_token_2022::instructions::{Burn, MintTo};

use crate::token::cpi_params::*;

pub fn spl_token_burn(params: TokenBurnParams<'_, '_>) -> ProgramResult {
    let TokenBurnParams {
        mint,
        source,
        authority,
        token_program,
        amount,
        authority_signer_seeds,
    } = params;

    let mut seed_buffer: [Seed; MAX_SEEDS] = array::from_fn(|_| Seed::from(&[]));

    #[cfg(feature = "token-2022")]
    let sized_token_program = {
        let try_sized_token_program = token_program.try_borrow_data().expect("borrow failed");
        let sized: &[u8; 32] = try_sized_token_program
            .as_ref()
            .try_into()
            .expect("wrong account length");
        sized
    };
    // COPY SEEDS IN A LOOP
    // This is very cheap. It just copies pointers (references), not data.
    if let Some(authority_signer) = authority_signer_seeds {
        for (i, raw_seed) in authority_signer.iter().enumerate() {
            seed_buffer[i] = Seed::from(*raw_seed);
        }
        let total_seeds = authority_signer.len();

        let active_seeds = &seed_buffer[0..total_seeds];

        let signer = Signer::from(active_seeds);

        #[cfg(feature = "token-2022")]
        Burn {
            token_program: sized_token_program,
            account: source,
            amount,
            authority,
            mint,
        }
        .invoke_signed(&[signer])?;

        #[cfg(not(feature = "token-2022"))]
        Burn {
            account: source,
            amount,
            authority,
            mint,
        }
        .invoke_signed(&[signer])?;
    } else {
        #[cfg(feature = "token-2022")]
        Burn {
            token_program: sized_token_program,
            account: source,
            amount,
            authority,
            mint,
        }
        .invoke()?;

        #[cfg(not(feature = "token-2022"))]
        Burn {
            account: source,
            amount,
            authority,
            mint,
        }
        .invoke()?;
    }
    Ok(())
}

pub fn spl_token_mint_to(params: TokenMintToParams<'_, '_>) -> ProgramResult {
    let TokenMintToParams {
        mint,
        destination,
        authority,
        token_program,
        amount,
        authority_signer_seeds,
    } = params;

    let mut seed_buffer: [Seed; MAX_SEEDS] = array::from_fn(|_| Seed::from(&[]));
    #[cfg(feature = "token-2022")]
    let sized_token_program = {
        let try_sized_token_program = token_program.try_borrow_data().expect("borrow failed");
        let sized: &[u8; 32] = try_sized_token_program
            .as_ref()
            .try_into()
            .expect("wrong account length");
        sized
    };

    if let Some(authority_signer) = authority_signer_seeds {
        for (i, raw_seed) in authority_signer.iter().enumerate() {
            seed_buffer[i] = Seed::from(*raw_seed);
        }
        let total_seeds = authority_signer.len();

        let active_seeds = &seed_buffer[0..total_seeds];

        let signer = Signer::from(active_seeds);

        #[cfg(not(feature = "token-2022"))]
        MintTo {
            mint,
            account: destination,
            amount,
            mint_authority: authority,
        }
        .invoke_signed(&[signer])?;

        #[cfg(feature = "token-2022")]
        MintTo {
            account: destination,
            amount,
            mint,
            mint_authority: authority,
            token_program: sized_token_program,
        }
        .invoke_signed(&[signer])?;
    } else {
        #[cfg(not(feature = "token-2022"))]
        MintTo {
            mint,
            account: destination,
            amount,
            mint_authority: authority,
        }
        .invoke()?;

        #[cfg(feature = "token-2022")]
        MintTo {
            account: destination,
            amount,
            mint,
            mint_authority: authority,
            token_program: sized_token_program,
        }
        .invoke()?;
    }

    Ok(())
}

pub fn spl_token_close(params: TokenCloseParams<'_, '_>) -> ProgramResult {
    let TokenCloseParams {
        account,
        destination,
        owner,
        authority_signer_seeds,
        token_program,
    } = params;

    let mut seed_buffer: [Seed; MAX_SEEDS] = array::from_fn(|_| Seed::from(&[]));
    #[cfg(feature = "token-2022")]
    let sized_token_program = {
        let try_sized_token_program = token_program.try_borrow_data().expect("borrow failed");
        let sized: &[u8; 32] = try_sized_token_program
            .as_ref()
            .try_into()
            .expect("wrong account length");
        sized
    };

    if let Some(authority_signer) = authority_signer_seeds {
        for (i, raw_seed) in authority_signer.iter().enumerate() {
            seed_buffer[i] = Seed::from(*raw_seed);
        }
        let total_seeds = authority_signer.len();

        let active_seeds = &seed_buffer[0..total_seeds];

        let signer = Signer::from(active_seeds);

        #[cfg(feature = "token-2022")]
        CloseAccount {
            account,
            authority: owner,
            destination,
            token_program: sized_token_program,
        }
        .invoke_signed(&[signer])?;

        #[cfg(not(feature = "token-2022"))]
        CloseAccount {
            account,
            authority: owner,
            destination,
        }
        .invoke_signed(&[signer])?;
    } else {
        #[cfg(feature = "token-2022")]
        CloseAccount {
            account,
            authority: owner,
            destination,
            token_program: sized_token_program,
        }
        .invoke()?;

        #[cfg(not(feature = "token-2022"))]
        CloseAccount {
            account,
            authority: owner,
            destination,
        }
        .invoke()?;
    }

    Ok(())
}
