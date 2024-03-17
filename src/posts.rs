//! Types that are used by the various frontend implementations for posts / casts and their replies

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// The combined user's profile, holding values from all user update types
#[derive(Debug, Serialize, Deserialize)]
pub struct Cast {
    pub from_fid: u64,
    pub mentions: Vec<u64>,
    pub text: String,
    pub mentions_positions: Vec<u32>,
    pub embeds: Vec<CastEmbed>,
    pub parent: Option<Parent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CastEmbed {
    Url(String),
    CastId(CastId)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Parent {
    Url(String),
    CastId(CastId)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CastId {
    pub fid: u64,
    pub hash: Vec<u8>
}

/// A service to get merged user info form a hubble grpc service
pub trait PostService {
    /// Combine all the casts and their metadata into a 
    async fn get_posts(&mut self, fid: u64, parent: Parent, page_token: Option<Vec<u8>>, page_size: Option<u32>) -> Result<Vec<Cast>>;
}
