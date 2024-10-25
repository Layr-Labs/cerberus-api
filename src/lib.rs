mod generated;

pub use generated::{
    GenerateKeyPairRequest,
    GenerateKeyPairResponse,
    ImportKeyRequest,
    ImportKeyResponse,
    ListKeysRequest,
    ListKeysResponse,
    SignGenericRequest,
    SignGenericResponse,
};

pub mod client {
    pub use crate::generated::key_manager::key_manager_client::KeyManagerClient;
    pub use crate::generated::signer::signer_client::SignerClient;
}

pub mod server {
    pub use crate::generated::key_manager::key_manager_server::KeyManagerServer;
    pub use crate::generated::signer::signer_server::SignerServer;
}