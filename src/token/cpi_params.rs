use pinocchio::account_info::AccountInfo;

/// TokenBurnParams
#[derive(Clone, Copy)]
pub struct TokenBurnParams<'a: 'b, 'b> {
    /// mint
    pub mint: &'a AccountInfo,
    /// source
    pub source: &'a AccountInfo,
    /// amount
    pub amount: u64,
    /// authority
    pub authority: &'a AccountInfo,
    /// authority_signer_seeds
    pub authority_signer_seeds: Option<&'b [&'b [u8]]>,
    /// token_program
    pub token_program: &'a AccountInfo,
}

/// TokenMintToParams
#[derive(Clone, Copy)]
pub struct TokenMintToParams<'a: 'b, 'b> {
    /// mint
    pub mint: &'a AccountInfo,
    /// destination
    pub destination: &'a AccountInfo,
    /// amount
    pub amount: u64,
    /// authority
    pub authority: &'a AccountInfo,
    /// authority_signer_seeds
    pub authority_signer_seeds: Option<&'b [&'b [u8]]>,
    /// token_program
    pub token_program: &'a AccountInfo,
}

/// TokenCloseParams
#[derive(Clone, Copy)]
pub struct TokenCloseParams<'a: 'b, 'b> {
    /// Token account
    pub account: &'a AccountInfo,
    /// Destination for redeemed SOL.
    pub destination: &'a AccountInfo,
    /// Owner of the token account.
    pub owner: &'a AccountInfo,
    /// authority_signer_seeds
    pub authority_signer_seeds: Option<&'b [&'b [u8]]>,
    /// token_program
    pub token_program: &'a AccountInfo,
}
