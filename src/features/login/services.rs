use crate::commons::models::{directus::DirectusErrorResponse, login::{DirectusLoginRequest, DirectusLoginResponse}};
use leptos_reqwest::send_and_parse;
use uuid::Uuid;
use crate::utilities::local_storage::{set_local_storage, JwtWithPermissions};


pub async fn directus_login(userid: String, password: String) -> Result<bool, DirectusErrorResponse> {
    let url = option_env!("DIRECTUSURL");
    // let url = if let Ok(var) = std::env::var("DIRECTUSURL") {
    //     var
    // } else {
    //     "".to_string()
    // };
    let path = format!("{}/login", url.unwrap_or_default());
    let email = userid.clone();
    let login_request = DirectusLoginRequest::new(userid.into(), password.into());
    let response = send_and_parse::<DirectusLoginRequest, DirectusLoginResponse, DirectusErrorResponse>(
        Some(login_request),
        path,
        reqwest::header::HeaderMap::new(),
        leptos_reqwest::HttpMethod::Post,
    )
    .await;

    match response {
        Ok(res) => {
            // Calculate expiration time in millis, subract 2 minute to be safe
            // Why 10 minutes? There are other api resource that are automatically when users navigate to a certain page
            // Only those API calls in action will have the refresh token
            // Which means during the manual submit, the refresh token is used
            // 10 minutes will act as a buffer for those action

            let expiration_time =
                chrono::Utc::now().timestamp_millis() + res.data.expires - 600_000;
                let uuid = Uuid::new_v4();

            let jwt_with_permissions = JwtWithPermissions::new(email, res.data.accessToken, res.data.refreshToken, expiration_time, res.data.permissions,uuid.to_string().replace("-", "_"));
            // let jabra_cookie = JabraCookie::new(
            //     email,
            //     res.data.accessToken,
            //     res.data.refreshToken,
            //     expiration_time,
            //     res.data.permissions,
            // );
            // set_jabra_cookie(jabra_cookie, "admin_portal_csr".to_string());
            set_local_storage("admin_portal_csr", &jwt_with_permissions.encrypt());
            Ok(true)
        }
        Err(e) => {
            Err(e)
        }
    }
}