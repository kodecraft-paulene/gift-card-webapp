use serde::{Deserialize, Serialize};

/// `PermissionsResponse` is a struct that represents the response for permissions.
///
/// # Fields
/// * `data`: A vector of `Permission` structs that holds the permissions.
///
/// # Methods
/// * `isallowed`: Returns a boolean indicating whether a specific action on a specific field in a specific collection is allowed. It checks each `Permission` in `data` to see if the collection, action, and field match the given parameters. If a match is found and the fields in the `Permission` contain "*" or the given field, it returns true. Otherwise, it returns false.
///
/// # Example
/// ```rust
/// let data = vec![Permission { collection: "users".to_string(), action: "read".to_string(), fields: vec!["*".to_string()] }];
/// let response = PermissionsResponse { data };
/// let is_allowed = response.isallowed("users", "read", "email");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PermissionsResponse {
    pub data: Vec<Permission>,
}

/// `Permission` is a struct that represents a permission.
///
/// # Fields
/// * `collection`: A string that represents the collection that the permission applies to.
/// * `action`: A string that represents the action that the permission allows.
/// * `fields`: A vector of strings that represents the fields that the permission applies to.
///
/// # Example
/// ```rust
/// let permission = Permission { collection: "users".to_string(), action: "read".to_string(), fields: vec!["*".to_string()] };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Permission {
    pub collection: String,
    pub action: String,
    pub fields: Vec<String>,
}

impl PermissionsResponse {
    pub fn isallowed(&self, collection: &str, action: &str, field: &str) -> bool {
        for permission in &self.data {
            if permission.collection == collection && permission.action == action {
                if permission.fields.contains(&String::from("*")) ||permission.fields.contains(&field.to_string()) {
                    return true;
                }
            }
        }
        false
    }
}