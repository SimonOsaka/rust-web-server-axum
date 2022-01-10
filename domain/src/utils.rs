use ring::{digest, pbkdf2};
use std::num::NonZeroU32;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
const HASH_ROUNDS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(1_000) };
pub type Credential = [u8; CREDENTIAL_LEN];

/// verify two str same
/// - input
///     - `crypt_str`: crypt string
///     - `other_str`: no crypt string
/// - output
///     - return `true` as same, `false` as not same
pub fn verify(crypt_str: String, other_str: String) -> bool {
    let str = base64::decode(&crypt_str).unwrap();

    pbkdf2::verify(
        PBKDF2_ALG,
        HASH_ROUNDS,
        "asdf".as_bytes(),
        other_str.as_bytes(),
        str.as_slice(),
    )
    .is_ok()
}

/// encrypt string
/// - input
///     - `to_encrypt`: string to encrypt
/// - output
///     - return `encrypt result`
pub fn hash(to_encrypt: String) -> String {
    let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        PBKDF2_ALG,
        HASH_ROUNDS,
        "asdf".as_bytes(),
        to_encrypt.as_bytes(),
        &mut to_store,
    );
    base64::encode(&to_store)
}
