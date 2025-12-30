use axum::http::{HeaderMap, StatusCode};

/// IAP user information extracted from headers
#[derive(Debug, Clone)]
pub struct IapUser {
    pub email: String,
    pub id: String,
}

/// Extract IAP user information from request headers
/// IAP adds these headers to authenticated requests:
/// - X-Goog-IAP-JWT-Assertion: JWT assertion (can be verified if needed)
/// - X-Goog-Authenticated-User-Email: User's email
/// - X-Goog-Authenticated-User-ID: User's ID
pub async fn verify_iap_auth(headers: &HeaderMap) -> Result<IapUser, StatusCode> {
    // Skip IAP check in development mode (for local testing)
    if std::env::var("SKIP_IAP_CHECK").is_ok() {
        return Ok(IapUser {
            email: "dev@example.com".to_string(),
            id: "dev-user".to_string(),
        });
    }

    // Check for IAP headers
    let email_header = headers
        .get("X-Goog-Authenticated-User-Email")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let id_header = headers
        .get("X-Goog-Authenticated-User-ID")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // IAP headers are in format: "accounts.google.com:email@example.com"
    // We need to extract just the email/ID part
    let email = email_header
        .split(':')
        .nth(1)
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_string();

    let id = id_header
        .split(':')
        .nth(1)
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_string();

    // Optional: Verify JWT assertion if present
    // For now, we trust IAP has already verified the user
    // If you want to verify the JWT, you can add that here

    Ok(IapUser { email, id })
}

/// Check if IAP is enabled (headers are present)
pub fn is_iap_enabled(headers: &HeaderMap) -> bool {
    headers.contains_key("X-Goog-Authenticated-User-Email")
        && headers.contains_key("X-Goog-Authenticated-User-ID")
}
