use serde::{Deserialize, Serialize};

use crate::utilities::encryption::enc;

use super::permissions::PermissionsResponse;

/// Struct for the Directus Login Request.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DirectusLoginRequest {
    pub email: String,
    pub password: String,
}

impl DirectusLoginRequest {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DirectusLoginResponse {
    pub data: DirectusLoginResponseData,
}

impl DirectusLoginResponse {
    pub fn _get_response(&self) -> DirectusLoginResponseData {
        self.data.clone()
    }

    pub fn _update_expiry(&mut self, expiry: i64) {
        self.data.expires += expiry;
    }

    pub  fn _serialize(&self) -> String {
        serde_json::to_string(&self._get_response()).unwrap()
    }

    pub fn _encrypt(&self) -> String {
        enc(self._serialize())
    }
}

/// Struct for the Directus Login Response Data.
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DirectusLoginResponseData {
    pub accessToken: String,
    pub expires: i64,
    pub refreshToken: String,
    pub permissions: PermissionsResponse,
}
