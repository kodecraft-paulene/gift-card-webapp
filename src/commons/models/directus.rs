use leptos::SerializationError;
use leptos_reqwest::LeptosReqwestError;
use reqwest::Error;
use serde::{Deserialize, Serialize};

/// `CollectionCountResponse` is a struct that represents the response from the Directus API for collection count.
///
/// # Fields
/// * `data`: A vector of `CollectionCount` structs that holds the data of the response.
///
/// # Methods
/// * `get_count`: Returns the count from the first `CollectionCount` in `data` as a u32. If the count cannot be parsed as a u32, it returns 0.
///
/// # Example
/// ```rust
/// let data = vec![CollectionCount { count: "10".to_string() }];
/// let response = CollectionCountResponse { data };
/// let count = response.get_count();
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CollectionCountResponse {
    data: Vec<CollectionCount>,
}

/// `CollectionCount` is a struct that represents the count of a collection in the Directus API.
///
/// # Fields
/// * `count`: A string that represents the count of the collection.
///
/// # Example
/// ```rust
/// let collection_count = CollectionCount { count: "10".to_string() };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CollectionCount {
    pub count: String,
}

impl CollectionCountResponse {
    pub fn get_count(&self) -> u32 {
        self.data.first().unwrap().count.parse::<u32>().unwrap_or_default()
    }
}

/// `DirectusErrorResponse` is a struct that represents the error response from the Directus API.
///
/// # Fields
/// * `errors`: A vector of `DirectusError` structs that represents the errors.
///
/// # Example
/// ```rust
/// let errors = vec![DirectusError { /* fields */ }];
/// let error_response = DirectusErrorResponse { errors };
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DirectusErrorResponse {
    pub errors: Vec<DirectusError>,
}

impl DirectusErrorResponse {
    pub fn get_error_message(&self) -> String {
        self.errors.first().unwrap().message.clone()
    }
}
impl LeptosReqwestError for DirectusErrorResponse {
    fn deserialization_error(e: SerializationError) -> Self {
        DirectusErrorResponse {
            errors: vec![DirectusError {message: e.to_string(), extensions: ErrorExtension { code: String::from("INVALID_RESPONSE_FORMATTING"), reason: Some(String::from("Response from service has invalid format.")) }}],
        }
    }

    fn read_error(e: Error) -> Self {
        DirectusErrorResponse {
            errors: vec![DirectusError {message: e.to_string(), extensions: ErrorExtension { code: String::from("INVALID_RESPONSE"), reason: Some(String::from("Cannot read response from service.")) }}],
        }
    }
    fn request_error(e: String, status_code: reqwest::StatusCode) -> Self {
        DirectusErrorResponse {
            errors: vec![DirectusError {message: e, extensions: ErrorExtension { code: status_code.to_string(), reason: Some(status_code.canonical_reason().unwrap_or_default().to_string()) }}],
        }
    }
}
impl Default for DirectusErrorResponse {
    fn default() -> Self {
        DirectusErrorResponse {
            errors: vec![DirectusError {message: String::from("System Error"), extensions: ErrorExtension { code: String::from("SYSTEM_ERROR"), reason: None }}],
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DirectusError {
    pub message: String,
    pub extensions: ErrorExtension,
}
#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct ErrorExtension {
    pub code: String,
    pub reason: Option<String>
}