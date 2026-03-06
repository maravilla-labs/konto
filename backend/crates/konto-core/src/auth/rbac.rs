use konto_common::error::AppError;
use serde_json::Value;

/// Check if a role's permissions JSON grants a specific permission.
pub fn has_permission(permissions_json: &str, permission: &str) -> Result<bool, AppError> {
    let perms: Value = serde_json::from_str(permissions_json)
        .map_err(|e| AppError::Internal(format!("Invalid permissions JSON: {e}")))?;

    // Admin has all permissions
    if perms.get("all").and_then(|v| v.as_bool()).unwrap_or(false) {
        return Ok(true);
    }

    // Read-only access
    if permission.starts_with("read") && perms.get("read_all").and_then(|v| v.as_bool()).unwrap_or(false) {
        return Ok(true);
    }

    // Specific permission check
    Ok(perms.get(permission).and_then(|v| v.as_bool()).unwrap_or(false))
}

/// Standard permission names used throughout the application.
pub mod permissions {
    pub const ACCOUNTING: &str = "accounting";
    pub const CONTACTS: &str = "contacts";
    pub const PROJECTS: &str = "projects";
    pub const TIME_ENTRIES: &str = "time_entries";
    pub const IMPORT: &str = "import";
    pub const READ_ALL: &str = "read_all";
    pub const OWN_DATA: &str = "own_data";
}
