# fatline-rs
## Farcaster Rust client library

A library project to re-export some common Farcaster client RPC code and utilities,
so that someone could interface with their own Farcaster Hub.

## Usage

### Dependencies
Add the dependency to your project
```toml
# Cargo.toml

[dependencies.fatline-rs]
git = "https://github.com/0x330a-public/fatline-rs.git"
features = ["client"]
rev = "00aabbccdd..." # latest github commit while in early dev
```
(Maybe the protobuf dependencies are required for the build script I'm not sure):

nixpkgs: `protobuf`

### Usage in project
Firstly, create an ed25519 Signing key if you don't have the bytes already to create one:
```rust
use fatline_rs::utils::generate_signing_key;

fn main() {
    // *****INITIAL SETUP*****
    // 1. generate a signing key
    let key = generate_signing_key();
    // 2. print out the hex encoded bytes here to re-use the same key
    println!("keep this one secret: {}", hex::encode(key.as_bytes()));
    // 3. add the signing key to your farcaster account
    // open this in browser, just easier to highlight this way
    let verifying_key = VerifyingKey::from(&key);
    println!("publish this one in dashboard: {}", hex::encode(verifying_key.as_bytes()));
    let dashboard_url = "https://terminex.mempool.online";
    // connect registered farcaster wallet -> add key -> paste hex output of verifying key ^ -> submit tx
    // ************************
    
    // 4. disregard the above lines and re-build your key from saved output from first run
    let mut private_bytes: [u8; 32] = [0u8; 32];
    hex::decode_to_slice("aabbccddeeff...", &mut private_bytes).unwrap();
}
```

Making a post:
```rust
use fatline_rs::{HubServiceClient, MessageTrait, SigningKey, VerifyingKey};
use fatline_rs::proto::{CastAddBody, FarcasterNetwork, HashScheme, Message, MessageData, MessageType, SignatureScheme};
use fatline_rs::proto::message_data::Body::CastAddBody as CABody;
use fatline_rs::utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut private_bytes: [u8; 32] = [0u8; 32];
    hex::decode_to_slice("aabbccddeeff...", &mut private_bytes).unwrap();
    
    let my_fid = 123456;
    
    let key = SigningKey::from(private_bytes);
    let verifying_key = VerifyingKey::from(&key);
    
    let mut client = HubServiceClient::connect("grpc://[some url or IP address]:2283").await?;
    
    // Construct the body per 
    let body = CABody(CastAddBody {
        parent: None,
        text: "Some cool cast".to_string(),
        embeds_deprecated: vec![],
        mentions_positions: vec![],
        embeds: vec![],
        mentions: vec![]
    });

    let data = MessageData {
        r#type: MessageType::CastAdd as i32,
        fid: my_fid,
        timestamp: utils::now(),
        network: FarcasterNetwork::Mainnet as i32,
        body: Some(body),
    };

    let hash = utils::message_hash(&data);
    let signature = utils::sign_hash(&key, &hash);

    let message = Message {
        data_bytes: Some(data.encode_to_vec()),
        data: None,
        hash_scheme: HashScheme::Blake3 as i32,
        signature_scheme: SignatureScheme::Ed25519 as i32,
        signature: signature.to_vec(),
        signer: verifying_key.to_bytes().to_vec(),
        hash: hash.to_vec(),
    };

    let result = client.submit_message(message).await?;
    println!("{:?}", result);
    
    Ok(())
}

```
