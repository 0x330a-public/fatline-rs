use ed25519_dalek as ed25519;
pub use ed25519::{VerifyingKey, SigningKey};

pub const HASH_LENGTH: usize = 20;

pub mod proto {
    tonic::include_proto!("_"); // farcaster protos don't use package
}

#[cfg(feature = "client")]
pub use proto::hub_service_client::HubServiceClient;

pub mod utils {
    use ed25519_dalek::{SIGNATURE_LENGTH, Signer};
    use prost::Message;
    use rand::rngs::OsRng;

    use crate::HASH_LENGTH;
    use crate::proto::MessageData;

    use super::ed25519::SigningKey;

    pub fn generate_signing_key() -> SigningKey {
        SigningKey::generate(&mut OsRng)
    }

    pub fn message_hash(message_data: &MessageData) -> [u8; HASH_LENGTH] {
        let hash = blake3::hash(&message_data.encode_to_vec());
        let mut truncated: [u8; HASH_LENGTH] = [0u8; HASH_LENGTH];
        truncated.copy_from_slice(hash.as_bytes());
        truncated
    }

    pub fn sign_hash(signing_key: &SigningKey, hash: &[u8; HASH_LENGTH]) -> [u8; SIGNATURE_LENGTH] {
        signing_key.sign(hash).to_bytes()
    }
}

#[cfg(test)]
mod tests {}
