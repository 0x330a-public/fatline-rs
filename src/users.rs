//! Types that are used by the various frontend implementations

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tonic::Response;
use tonic::transport::Channel;

use crate::HubServiceClient;
use crate::proto::{Message, UserDataRequest, UserDataType};
use crate::utils::{optional_get_user_data, optional_get_user_data_value};

/// The combined user's profile, holding values from all user update types
#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub fid: u64,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub profile_picture: Option<String>,
    pub bio: Option<String>,
    pub url: Option<String>
}

/// A service to get merged user info form a hubble grpc service
pub trait UserService {
    /// Combine all the user's data messages into a serializable Profile type
    async fn get_user_profile(&mut self, fid: u64) -> Result<Profile>;
}

fn get_user_data_request(fid: u64, data_type: UserDataType) -> UserDataRequest {
    UserDataRequest {
        fid,
        user_data_type: data_type as i32
    }
}

fn get_user_data(response: Response<Message>) -> Option<String> {
    optional_get_user_data(response).and_then(optional_get_user_data_value)
}

impl UserService for HubServiceClient<Channel> {
    async fn get_user_profile(&mut self, fid: u64) -> Result<Profile> {
        let username = self.get_user_data(
            get_user_data_request(fid, UserDataType::Username)
        ).await.map_or(None, get_user_data);

        let display_name = self.get_user_data(
            get_user_data_request(fid, UserDataType::Display)
        ).await.map_or(None, get_user_data);

        let profile_picture  = self.get_user_data(
            get_user_data_request(fid, UserDataType::Pfp)
        ).await.map_or(None, get_user_data);

        let bio = self.get_user_data(
            get_user_data_request(fid, UserDataType::Bio)
        ).await.map_or(None, get_user_data);

        let url = self.get_user_data(
            get_user_data_request(fid, UserDataType::Url)
        ).await.map_or(None, get_user_data);

        Ok(Profile {
            fid,
            username,
            display_name,
            profile_picture,
            bio,
            url,
        })
    }
}
