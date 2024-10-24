pub(crate) const GAMMA: &str = "gamma";
pub(crate) const BETA: &str = "beta";
pub(crate) const ALPHA: &str = "alpha";
pub(crate) const ZETA: &str = "zeta";

mod converter;
mod hash_to_field;
mod kzg;
mod proof;
mod transcript;
mod verify;

pub(crate) mod error;

use bn::Fr;
pub(crate) use converter::{load_plonk_proof_from_bytes, load_plonk_verifying_key_from_bytes};
pub(crate) use proof::PlonkProof;
use sha2::{Digest, Sha256};
pub(crate) use verify::verify_plonk;

use error::PlonkError;

use crate::{bn254_public_values, decode_sp1_vkey_hash};
/// A verifier for Plonk zero-knowledge proofs.
#[derive(Debug)]
pub struct PlonkVerifier;

impl PlonkVerifier {
    /// Verifies a Plonk proof.
    ///
    /// # Arguments
    ///
    /// * `proof` - The proof bytes.
    /// * `vk` - The verification key bytes.
    /// * `public_inputs` - The public inputs.
    ///
    /// # Returns
    ///
    /// A `Result` containing a boolean indicating whether the proof is valid,
    /// or a `PlonkError` if verification fails.
    ///
    pub fn verify(
        proof: &[u8],
        sp1_public_inputs: &[u8],
        sp1_vkey_hash: &str,
        plonk_vk: &[u8],
    ) -> Result<bool, PlonkError> {
        // Hash the vk and get the first 4 bytes.
        let plonk_vk_hash: [u8; 4] = Sha256::digest(plonk_vk)[..4].try_into().unwrap();

        // Check to make sure that this proof was generated by the groth16 proving key corresponding to
        // the given groth16_vk.
        //
        // SP1 prepends the raw Groth16 proof with the first 4 bytes of the groth16 vkey to
        // faciliate this check.
        if plonk_vk_hash != proof[..4] {
            return Err(PlonkError::PlonkVkeyHashMismatch);
        }

        let sp1_vkey_hash = decode_sp1_vkey_hash(sp1_vkey_hash)?;
        let public_inputs = bn254_public_values(&sp1_vkey_hash, sp1_public_inputs);

        let proof = load_plonk_proof_from_bytes(&proof[4..]).unwrap();
        let plonk_vk = load_plonk_verifying_key_from_bytes(plonk_vk).unwrap();

        verify_plonk(&plonk_vk, &proof, &public_inputs)
    }

    /// DEPRECATED: Will delete this in a future commit.
    pub fn verify_old(proof: &[u8], vk: &[u8], public_inputs: &[Fr]) -> Result<bool, PlonkError> {
        let proof = load_plonk_proof_from_bytes(proof).unwrap();
        let vk = load_plonk_verifying_key_from_bytes(vk).unwrap();

        verify_plonk(&vk, &proof, public_inputs)
    }
}
