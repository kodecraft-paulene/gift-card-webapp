use leptos::{use_context, ServerFnError};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use crate::commons::models::permissions::PermissionsResponse;
use leptos::*;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct JwtWithPermissions {
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub permissions: PermissionsResponse,
    pub uuid: String,
}

impl JwtWithPermissions {
    pub fn new(
        user_id: String,
        access_token: String,
        refresh_token: String,
        expires_in: i64,
        permissions: PermissionsResponse,
        uuid: String,
    ) -> Self {
        Self {
            user_id,
            access_token,
            refresh_token,
            expires_in,
            permissions,
            uuid,
        }
    }

    pub fn encrypt(&self) -> String {
        let cookie_string = serde_json::to_string(self).unwrap();
        super::encryption::enc(cookie_string)
    }

    pub fn decrypt(encrypted_text: String) -> Result<Self, Error> {
        let decrypted_text = super::encryption::dec(encrypted_text);
        match serde_json::from_str(&decrypted_text) {
            Ok(cookie) => Ok(cookie),
            Err(e) => {
                log::error!("Error: {:?}", e);
                Err(e)
            },
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now().timestamp_millis();
        now > self.expires_in
    }
}


pub fn set_local_storage(key: &str, value: &str) -> Option<()> {
    let window = leptos::window();
    let local_storage = window.local_storage().ok().flatten()?;
    let _result = local_storage.set_item(key, value);
    Some(())
}

pub fn get_local_storage(key: &str) -> Option<String> {
    let window = leptos::window();
    let local_storage = window.local_storage().ok().flatten()?;
    let result = local_storage.get_item(key);
    match result {
        Ok(value) => value,
        Err(_e) => None,
    }
}

pub fn remove_local_storage(key: &str) -> Option<()> {
    let window = leptos::window();
    let local_storage = window.local_storage().ok().flatten()?;
    let _result = local_storage.remove_item(key);
    Some(())
}
pub async fn check_local_session(local_session_name: &str) -> Result<bool, ServerFnError> {
    let local_storage_value = get_local_storage(local_session_name);
    match local_storage_value {
        Some(e) => {
            let jwt_with_permission = JwtWithPermissions::decrypt(e);
            match jwt_with_permission {
                Ok(jwt) => {
                    Ok(!jwt.is_expired())
                },
                Err(_e) => Ok(false)
            }
        },
        _ => Ok(false)
    }
}

pub fn logout_expired_token_with_redirect(_cookie_name: String) {
    let navigate = leptos_router::use_navigate();
    // let (_cookie, set_cookie) = use_cookie_with_options::<String, FromToStringCodec>(&cookie_name, UseCookieOptions::default().path("/"));
    remove_local_storage("admin_portal_csr");
    use_context::<crate::Refetcher>().unwrap().0.update(|s| *s = !*s);
    navigate("/login", Default::default());
}

pub fn logout_expired_token(_cookie_name: String) {
    // let (_cookie, set_cookie) = use_cookie_with_options::<String, FromToStringCodec>(&cookie_name, UseCookieOptions::default().path("/"));
    // set_cookie(Some("".to_string()));
    remove_local_storage("admin_portal_csr");
    use_context::<crate::Refetcher>().unwrap().0.update(|s| *s = !*s);
}
