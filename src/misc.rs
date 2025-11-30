#[allow(unused_imports)]
use pinocchio::{
    memory::sol_memcmp,
    pubkey::{pubkey_eq, Pubkey, PUBKEY_BYTES},
};

#[allow(unexpected_cfgs)]
pub fn cmp_pubkeys(a: &Pubkey, b: &Pubkey) -> bool {
    #[cfg(target_os = "solana")]
    unsafe {
        sol_memcmp(a.as_ref(), b.as_ref(), PUBKEY_BYTES) == 0
    }

    #[cfg(not(target_os = "solana"))]
    pubkey_eq(a, b)
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn check_keys_equal() {
        let key1 = Pubkey::from([1u8; 32]);
        assert!(cmp_pubkeys(&key1, &key1));
    }

    #[test]
    fn check_keys_not_equal() {
        let key1 = Pubkey::from([1u8; 32]);
        let key2 = Pubkey::from([2u8; 32]);
        assert!(!cmp_pubkeys(&key1, &key2));
    }
}
