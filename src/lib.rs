pub use ed25519::{SigningKey, VerifyingKey};
use ed25519_dalek as ed25519;
pub use prost::Message as MessageTrait;

#[cfg(feature = "client")]
pub use proto::hub_service_client::HubServiceClient;

#[cfg(feature = "service_types")]
pub mod users;
#[cfg(feature = "service_types")]
mod posts;

// 160-bit blake3 hash length used by Farcaster rpc requests
pub const HASH_LENGTH: usize = 20;
// Farcaster epoch in seconds, used to calculate timestamps in rpc
pub const FARCASTER_EPOCH: u64 = 1609459200;

pub mod proto {
    tonic::include_proto!("_"); // farcaster protos don't use package
}

pub mod utils {
    use std::time::{SystemTime, UNIX_EPOCH};

    use ed25519_dalek::{SIGNATURE_LENGTH, Signer};
    use prost::Message;
    use rand::rngs::OsRng;
    use tonic::Response;

    use crate::{FARCASTER_EPOCH, HASH_LENGTH};
    use crate::proto::{MessageData, UserDataBody};
    use crate::proto::message_data::Body;

    use super::ed25519::SigningKey;

    pub fn generate_signing_key() -> SigningKey {
        SigningKey::generate(&mut OsRng)
    }

    pub fn optional_get_user_data(response: Response<crate::proto::Message>) -> Option<UserDataBody> {
        response.into_inner().data
            // map the inner data
            .and_then(|data| data.body)
            // map the inner UserDataBody
            .and_then(|body| match body {
                Body::UserDataBody(body) => Some(body),
                _ => None
            })
    }

    pub fn optional_get_user_data_value(user_data_body: UserDataBody) -> Option<String> {
        Some(user_data_body.value)
    }

    pub fn message_hash(message_data: &MessageData) -> [u8; HASH_LENGTH] {
        let hash = blake3::hash(&message_data.encode_to_vec());
        let mut truncated: [u8; HASH_LENGTH] = [0u8; HASH_LENGTH];
        truncated.copy_from_slice(&hash.as_bytes()[0..HASH_LENGTH]);
        truncated
    }

    pub fn sign_hash(signing_key: &SigningKey, hash: &[u8; HASH_LENGTH]) -> [u8; SIGNATURE_LENGTH] {
        signing_key.sign(hash).to_bytes()
    }

    pub fn now() -> u32 {
        (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - FARCASTER_EPOCH) as u32
    }
}

#[cfg(test)]
mod tests {
    use prost::Message;

    use crate::proto::MessageData;

    #[tokio::test]
    async fn test_ok() -> Result<(), Box<dyn std::error::Error>> {
        let data = MessageData {
            r#type: 0,
            fid: 0,
            timestamp: 0,
            network: 0,
            body: None,
        };

        data.encode_to_vec();

        Ok(())
    }
}
