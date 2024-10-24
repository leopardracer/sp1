mod converter;
pub(crate) mod error;
mod verify;

pub(crate) use converter::{load_groth16_proof_from_bytes, load_groth16_verifying_key_from_bytes};
use sha2::{Digest, Sha256};
pub(crate) use verify::*;

use error::Groth16Error;

use crate::{bn254_public_values, decode_sp1_vkey_hash};

/// A verifier for Groth16 zero-knowledge proofs.
#[derive(Debug)]
pub struct Groth16Verifier;

impl Groth16Verifier {
    /// Verifies a Groth16 proof.
    ///
    /// # Arguments
    ///
    /// * `proof` - The proof bytes.
    /// * `public_inputs` - The SP1 public inputs.
    /// * `sp1_vkey_hash` - The SP1 vkey hash.
    ///   This is generated in the following manner:
    ///
    /// ```ignore
    /// use sp1_sdk::ProverClient;
    /// let client = ProverClient::new();
    /// let (pk, vk) = client.setup(ELF);
    /// let sp1_vkey_hash = vk.bytes32();
    /// ```
    /// * `groth16_vk` - The Groth16 verifying key bytes.
    ///   Usually this will be the [`crate::GROTH16_VK_BYTES`] constant.
    ///
    /// # Returns
    ///
    /// A `Result` containing a boolean indicating whether the proof is valid,
    /// or a [`Groth16Error`] if verification fails.
    pub fn verify(
        proof: &[u8],
        sp1_public_inputs: &[u8],
        sp1_vkey_hash: &str,
        groth16_vk: &[u8],
    ) -> Result<bool, Groth16Error> {
        // Hash the vk and get the first 4 bytes.
        let groth16_vk_hash: [u8; 4] = Sha256::digest(groth16_vk)[..4].try_into().unwrap();

        // Check to make sure that this proof was generated by the groth16 proving key corresponding to
        // the given groth16_vk.
        //
        // SP1 prepends the raw Groth16 proof with the first 4 bytes of the groth16 vkey to
        // facilitate this check.
        if groth16_vk_hash != proof[..4] {
            return Err(Groth16Error::Groth16VkeyHashMismatch);
        }

        let sp1_vkey_hash = decode_sp1_vkey_hash(sp1_vkey_hash)?;
        let public_inputs = bn254_public_values(&sp1_vkey_hash, sp1_public_inputs);

        let proof = load_groth16_proof_from_bytes(&proof[4..]).unwrap();
        let groth16_vk = load_groth16_verifying_key_from_bytes(groth16_vk).unwrap();

        verify_groth16_raw(&groth16_vk, &proof, &public_inputs)
    }
}
