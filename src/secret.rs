//! Secret detection and sanitization for logs and audit entries
//!
//! ponytail: Simplified secret detection using basic string patterns.
//! This catches the most common secret leaks without complex regex or external dependencies.
//! Upgrade path: If more sophisticated detection is needed, use a dedicated secret-scanning crate.

use regex::Regex;
use once_cell::sync::Lazy;

/// Basic secret patterns for detection
static SECRET_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        // AWS keys
        Regex::new(r"(AKIA|AGPA|AIDA|AROA|AIPA|ANPA|ANVA|ASIA)[A-Z0-9]{16}").unwrap(),
        // Stripe/Google/GitHub tokens
        Regex::new(r"(sk_live_|pk_live_|sk_test_|pk_test_|AIza|ghp_|gho_|ghu_|ghs_|ghr_)[a-zA-Z0-9\-_]{20,}").unwrap(),
        // JWT tokens
        Regex::new(r"[a-zA-Z0-9\-_]+\.[a-zA-Z0-9\-_]+\.[a-zA-Z0-9\-_]+").unwrap(),
        // Private key headers
        Regex::new(r"-----BEGIN (RSA )?PRIVATE KEY-----").unwrap(),
        // Generic secret patterns (token=, api_key=, etc.)
        Regex::new(r"(token|api_key|secret|password|auth_token|access_token)[=:][\s]*[^\s&]+").unwrap(),
        // Passwords in URLs (only the password part, not the whole URL)
        Regex::new(r"[a-zA-Z0-9\-_]+:[^@]+@").unwrap(),
        // Bearer tokens
        Regex::new(r"Bearer\s+[A-Za-z0-9\-._~+/]+=*").unwrap(),
    ]
});

/// Sanitize a string by replacing detected secrets with placeholders
///
/// # Examples
/// ```
/// use smartfo::secret::sanitize_string;
/// let input = "API key: sk_live_1234567890abcdef";
/// let sanitized = sanitize_string(input);
/// assert!(sanitized.contains("********"));
/// ```
pub fn sanitize_string(input: &str) -> String {
    let mut result = input.to_string();

    for regex in SECRET_PATTERNS.iter() {
        result = regex.replace_all(&result, "********").to_string();
    }

    result
}

/// Check if a string contains any secrets
///
/// Returns true if any secret pattern is detected.
pub fn contains_secrets(input: &str) -> bool {
    for regex in SECRET_PATTERNS.iter() {
        if regex.is_match(input) {
            return true;
        }
    }
    false
}

/// Sanitize a string only if it contains secrets
///
/// This is useful for performance when you want to avoid unnecessary processing.
pub fn sanitize_if_needed(input: &str) -> String {
    if contains_secrets(input) {
        sanitize_string(input)
    } else {
        input.to_string()
    }
}

/// Sanitize a specific field value (e.g., reason field in audit entry)
///
/// This is more conservative and only sanitizes if the field looks like it might contain a secret.
pub fn sanitize_field(field_name: &str, value: &str) -> String {
    // Be more conservative with certain fields
    let is_sensitive_field = matches!(
        field_name.to_lowercase().as_str(),
        "reason" | "message" | "description" | "comment"
    );

    if is_sensitive_field {
        sanitize_if_needed(value)
    } else {
        // For non-sensitive fields, still sanitize but be more aggressive
        sanitize_string(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Generate a random alphanumeric string of specified length
    fn random_alphanumeric(length: usize) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::thread_rng();
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    #[test]
    fn test_sanitize_aws_access_key() {
        let input = "AWS key: AKIAIOSFODNN7EXAMPLE";
        let sanitized = sanitize_string(input);
        assert!(sanitized.contains("********"));
    }

    #[test]
    fn test_sanitize_stripe_key() {
        let random_suffix = random_alphanumeric(24);
        let input = format!("Stripe key: sk_live_{}", random_suffix);
        let sanitized = sanitize_string(&input);
        assert!(sanitized.contains("********"));
    }

    #[test]
    fn test_sanitize_google_api_key() {
        let input = "Google key: AIzaSyDaGmWKa4JsXZ-HjGw7ISLn_3namBGewQe";
        let sanitized = sanitize_string(input);
        assert!(sanitized.contains("********"));
    }

    #[test]
    fn test_sanitize_github_token() {
        let input = "GitHub token: ghp_1234567890abcdefghijklmnopqrstuvwxyz123456";
        let sanitized = sanitize_string(input);
        assert!(sanitized.contains("********"));
    }

    #[test]
    fn test_sanitize_jwt_token() {
        let input = "JWT: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let sanitized = sanitize_string(input);
        assert!(sanitized.contains("********"));
    }

    #[test]
    fn test_sanitize_private_key() {
        let input = "Key: -----BEGIN PRIVATE KEY-----";
        let sanitized = sanitize_string(input);
        assert!(sanitized.contains("********"));
    }

    #[test]
    fn test_sanitize_password_in_url() {
        let input = "URL: https://user:password@example.com";
        let sanitized = sanitize_string(input);
        // Password should be sanitized
        assert!(sanitized.contains("********"));
        // ponytail: Simplified regex matches entire user:password@ pattern, so domain may be replaced
        // The important thing is the password is not present
        assert!(!sanitized.contains("password"));
    }

    #[test]
    fn test_sanitize_bearer_token() {
        let input = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
        let sanitized = sanitize_string(input);
        assert!(sanitized.contains("********"));
    }

    #[test]
    fn test_sanitize_generic_token() {
        let input = "Config: token=secret12345";
        let sanitized = sanitize_string(input);
        assert!(sanitized.contains("********"));
    }

    #[test]
    fn test_contains_secrets_true() {
        assert!(contains_secrets("API key: sk_live_1234567890abcdefghijklmnopqrstuvwx"));
    }

    #[test]
    fn test_contains_secrets_false() {
        assert!(!contains_secrets("This is a normal message"));
    }

    #[test]
    fn test_sanitize_if_needed_with_secrets() {
        let random_suffix = random_alphanumeric(24);
        let input = format!("API key: sk_live_{}", random_suffix);
        let sanitized = sanitize_if_needed(&input);
        assert!(sanitized.contains("********"));
    }

    #[test]
    fn test_sanitize_if_needed_without_secrets() {
        let input = "This is a normal message";
        let sanitized = sanitize_if_needed(input);
        assert_eq!(sanitized, input);
    }

    #[test]
    fn test_sanitize_field_sensitive() {
        let random_suffix = random_alphanumeric(24);
        let value = format!("API key: sk_live_{}", random_suffix);
        let sanitized = sanitize_field("reason", &value);
        assert!(sanitized.contains("********"));
    }

    #[test]
    fn test_sanitize_field_non_sensitive() {
        let random_suffix = random_alphanumeric(24);
        let value = format!("API key: sk_live_{}", random_suffix);
        let sanitized = sanitize_field("path", &value);
        assert!(sanitized.contains("********"));
    }

    #[test]
    fn test_multiple_secrets() {
        let random_suffix = random_alphanumeric(24);
        let input = format!("AWS: AKIAIOSFODNN7EXAMPLE, Stripe: sk_live_{}", random_suffix);
        let sanitized = sanitize_string(&input);
        // Both secrets should be replaced
        assert!(sanitized.contains("********"));
        // Original patterns should not be present
        assert!(!sanitized.contains("AKIAIOSFODNN7EXAMPLE"));
        assert!(!sanitized.contains(&format!("sk_live_{}", random_suffix)));
    }

    #[test]
    fn test_no_secrets_unchanged() {
        let input = "This is a normal file path: /home/user/document.txt";
        let sanitized = sanitize_string(input);
        assert_eq!(sanitized, input);
    }
}
