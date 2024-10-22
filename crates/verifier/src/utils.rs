use bn::Fr;
use sha2::{Digest, Sha256};

use crate::error::Error;

/// Hashes the public inputs in the same format as the BN254 verifier.
pub fn hash_public_inputs(public_inputs: &[u8]) -> [u8; 32] {
    let mut result = Sha256::digest(public_inputs);

    // The BN254 verifier operate over a 254 bit field, so we need to zero
    // out the first 3 bits. The same logic happens in the SP1 Ethereum verifier contract.
    result[0] &= 0x1F;

    result.into()
}

/// Formats the sp1 vkey hash and public inputs for use in the BN254 verifier.
pub fn bn254_public_values(sp1_vkey_hash: &[u8; 32], sp1_public_inputs: &[u8]) -> [Fr; 2] {
    let committed_values_digest = hash_public_inputs(sp1_public_inputs);
    let vkey_hash = Fr::from_slice(&sp1_vkey_hash[1..]).unwrap();
    let committed_values_digest = Fr::from_slice(&committed_values_digest).unwrap();
    [vkey_hash, committed_values_digest]
}

/// Decodes the sp1 vkey hash from the string from bytes32.
pub fn decode_sp1_vkey_hash(sp1_vkey_hash: &str) -> Result<[u8; 32], Error> {
    let bytes = hex::decode(&sp1_vkey_hash[2..]).map_err(|_| Error::InvalidProgramVkeyHash)?;
    bytes.try_into().map_err(|_| Error::InvalidProgramVkeyHash)
}
