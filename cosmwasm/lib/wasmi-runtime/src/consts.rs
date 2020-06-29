#![cfg_attr(not(feature = "SGX_MODE_HW"), allow(unused))]

pub use enclave_ffi_types::ENCRYPTED_SEED_SIZE;

#[cfg(feature = "production")]
pub static SPID_FILE: &str = "spid_production.txt";
#[cfg(feature = "production")]
pub static API_KEY_FILE: &str = "key_production.txt";

#[cfg(not(feature = "production"))]
pub static SPID_FILE: &str = "spid.txt";
#[cfg(not(feature = "production"))]
pub static API_KEY_FILE: &str = "api_key.txt";

pub const CERTEXPIRYDAYS: i64 = 90i64;

pub const BECH32_PREFIX_ACC_ADDR: &str = "secret";

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum SigningMethod {
    MRSIGNER,
    MRENCLAVE,
    NONE,
}

pub const ATTESTATION_CERTIFICATE_SAVE_PATH: &str = "attestation_cert.der";

pub const SEED_EXCH_CERTIFICATE_SAVE_PATH: &str = "node-master-cert.der";
pub const IO_CERTIFICATE_SAVE_PATH: &str = "io-master-cert.der";

//todo: set this to the real value
#[cfg(feature = "production")]
pub const MRSIGNER: [u8; 32] = [
    131, 215, 25, 231, 125, 234, 202, 20, 112, 246, 186, 246, 42, 77, 119, 67, 3, 200, 153, 219,
    105, 2, 15, 156, 112, 238, 29, 252, 8, 199, 206, 158,
];

#[cfg(not(feature = "production"))]
pub const MRSIGNER: [u8; 32] = [
    131, 215, 25, 231, 125, 234, 202, 20, 112, 246, 186, 246, 42, 77, 119, 67, 3, 200, 153, 219,
    105, 2, 15, 156, 112, 238, 29, 252, 8, 199, 206, 158,
];

#[cfg(feature = "production")]
pub const SIGNING_METHOD: SigningMethod = SigningMethod::MRENCLAVE;

#[cfg(not(feature = "production"))]
pub const SIGNING_METHOD: SigningMethod = SigningMethod::MRENCLAVE;

pub const CONSENSUS_SEED_SEALING_PATH: &str = "./.sgx_secrets/consensus_seed.sealed";
pub const REGISTRATION_KEY_SEALING_PATH: &str =
    "./.sgx_secrets/new_node_seed_exchange_keypair.sealed";

pub const CONSENSUS_SEED_EXCHANGE_KEYPAIR_DERIVE_ORDER: u32 = 1;
pub const CONSENSUS_IO_EXCHANGE_KEYPAIR_DERIVE_ORDER: u32 = 2;
pub const CONSENSUS_STATE_IKM_DERIVE_ORDER: u32 = 3;
