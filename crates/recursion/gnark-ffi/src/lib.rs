mod babybear;

pub mod ffi;
pub mod groth16_bn254;
pub mod plonk_bn254;
pub mod proof;
pub mod witness;

pub use groth16_bn254::*;
pub use plonk_bn254::*;
pub use proof::*;
pub use witness::*;

const SP1_CIRCUIT_VERSION: &str = include_str!("../../../SP1_CIRCUIT_VERSION");