pub mod key_manager {
  include!(concat!(".", "/keymanager.v1.rs"));
}

// If you have multiple services, you can organize them here
pub mod signer {
  include!(concat!(".", "/signer.v1.rs"));
}

pub use self::key_manager::{GenerateKeyPairRequest, GenerateKeyPairResponse, ImportKeyRequest, ImportKeyResponse, ListKeysRequest, ListKeysResponse};
pub use self::signer::{SignGenericRequest, SignGenericResponse};