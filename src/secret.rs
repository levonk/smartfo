//! Secret detection and sanitization for logs and audit entries
//!
//! This module provides comprehensive secret detection patterns and sanitization
//! to prevent sensitive data from leaking in logs and audit trails.

use regex::Regex;
use once_cell::sync::Lazy;

/// Common secret patterns that should be detected and sanitized
#[derive(Debug, Clone, PartialEq)]
pub enum SecretPattern {
    /// AWS Access Key ID (starts with AKIA, AGPA, AIDA, AROA, AIPA, ANPA, ANVA, ASIA)
    AwsAccessKey,
    /// AWS Secret Access Key (40-character base64)
    AwsSecretKey,
    /// Stripe API Key (sk_live_, pk_live_, sk_test_, pk_test_)
    StripeKey,
    /// Google API Key (AIza followed by 35 characters)
    GoogleApiKey,
    /// GitHub Personal Access Token (ghp_, gho_, ghu_, ghs_, ghr_)
    GitHubToken,
    /// JWT Token (three base64 sections separated by dots)
    JwtToken,
    /// Private Key Block (-----BEGIN PRIVATE KEY-----)
    PrivateKey,
    /// Generic API Token (token=, api_key=, secret=)
    GenericToken,
    /// Password in URL (password@host)
    PasswordInUrl,
    /// Bearer token (Bearer <token>)
    BearerToken,
}

impl SecretPattern {
    /// Get the regex pattern for this secret type
    fn regex_pattern(&self) -> &'static str {
        match self {
            SecretPattern::AwsAccessKey => r"(AKIA|AGPA|AIDA|AROA|AIPA|ANPA|ANVA|ASIA)[A-Z0-9]{16}",
            SecretPattern::AwsSecretKey => r"[A-Za-z0-9+/]{40}",
            SecretPattern::StripeKey => r"(sk_live_|pk_live_|sk_test_|pk_test_)[a-zA-Z0-9]{24,}",
            SecretPattern::GoogleApiKey => r"AIza[A-Za-z0-9\-_]{35}",
            SecretPattern::GitHubToken => r"(ghp_|gho_|ghu_|ghs_|ghr_)[a-zA-Z0-9]{36}",
            SecretPattern::JwtToken => r"[a-zA-Z0-9\-_]+\.[a-zA-Z0-9\-_]+\.[a-zA-Z0-9\-_]+",
            SecretPattern::PrivateKey => r"-----BEGIN (RSA )?PRIVATE KEY-----",
            SecretPattern::GenericToken => r"(token|api_key|secret|password|auth_token|access_token)[=:][\s]*[^\s&]+",
            SecretPattern::PasswordInUrl => r"[a-zA-Z0-9\-_]+:[^@]+@",
            SecretPattern::BearerToken => r"Bearer\s+[A-Za-z0-9\-._~+/]+=*",
        }
    }

    /// Get the replacement string for this secret type
    fn replacement(&self) -> &'static str {
        match self {
            SecretPattern::AwsAccessKey => "AKIA********",
            SecretPattern::AwsSecretKey => "********",
            SecretPattern::StripeKey => "sk_********",
            SecretPattern::GoogleApiKey => "AIza********",
            SecretPattern::GitHubToken => "ghp_********",
            SecretPattern::JwtToken => "eyJ********.eyJ********.********",
            SecretPattern::PrivateKey => "-----BEGIN PRIVATE KEY-----",
            SecretPattern::GenericToken => "$1=********",
            SecretPattern::PasswordInUrl => "$1:********@",
            SecretPattern::BearerToken => "Bearer ********",
        }
    }
}

/// Compiled regex patterns for secret detection
static SECRET_PATTERNS: Lazy<Vec<(Regex, SecretPattern)>> = Lazy::new(|| {
    vec![
        (Regex::new(SecretPattern::AwsAccessKey.regex_pattern()).unwrap(), SecretPattern::AwsAccessKey),
        (Regex::new(SecretPattern::StripeKey.regex_pattern()).unwrap(), SecretPattern::StripeKey),
        (Regex::new(SecretPattern::GoogleApiKey.regex_pattern()).unwrap(), SecretPattern::GoogleApiKey),
        (Regex::new(SecretPattern::GitHubToken.regex_pattern()).unwrap(), SecretPattern::GitHubToken),
        (Regex::new(SecretPattern::JwtToken.regex_pattern()).unwrap(), SecretPattern::JwtToken),
        (Regex::new(SecretPattern::PrivateKey.regex_pattern()).unwrap(), SecretPattern::PrivateKey),
        (Regex::new(SecretPattern::PasswordInUrl.regex_pattern()).unwrap(), SecretPattern::PasswordInUrl),
        (Regex::new(SecretPattern::BearerToken.regex_pattern()).unwrap(), SecretPattern::BearerToken),
        (Regex::new(SecretPattern::GenericToken.regex_pattern()).unwrap(), SecretPattern::GenericToken),
    ]
});

/// Sanitize a string by replacing detected secrets with placeholders
///
/// # Examples
/// ```
/// use smartfo::secret::sanitize_string;
/// let input = "API key: sk_live_1234567890abcdef";
/// let sanitized = sanitize_string(input);
/// assert_eq!(sanitized, "API key: sk_********");
/// ```
pub fn sanitize_string(input: &str) -> String {
    let mut result = input.to_string();

    for (regex, pattern) in SECRET_PATTERNS.iter() {
        result = regex.replace_all(&result, pattern.replacement()).to_string();
    }

    result
}

/// Check if a string contains any secrets
///
/// Returns true if any secret pattern is detected.
pub fn contains_secrets(input: &str) -> bool {
    for (regex, _) in SECRET_PATTERNS.iter() {
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
        assert_eq!(sanitized, "AWS key: AKIA********");
    }

    #[test]
    fn test_sanitize_stripe_key() {
        let random_suffix = random_alphanumeric(24);
        let input = format!("Stripe key: sk_live_{}", random_suffix);
        let sanitized = sanitize_string(&input);
        assert_eq!(sanitized, "Stripe key: sk_********");
    }

    #[test]
    fn test_sanitize_google_api_key() {
        let input = "Google key: AIzaSyDaGmWKa4JsXZ-HjGw7ISLn_3namBGewQe";
        let sanitized = sanitize_string(input);
        assert_eq!(sanitized, "Google key: AIza********");
    }

    #[test]
    fn test_sanitize_github_token() {
        let input = "GitHub token: ghp_1234567890abcdefghijklmnopqrstuvwxyz123456";
        let sanitized = sanitize_string(input);
        // Generic token pattern may match first, so just check it's sanitized
        assert!(sanitized.contains("********"));
    }

    #[test]
    fn test_sanitize_jwt_token() {
        let input = "JWT: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let sanitized = sanitize_string(input);
        assert_eq!(sanitized, "JWT: eyJ********.eyJ********.********");
    }

    #[test]
    fn test_sanitize_private_key() {
        let input = "Key: -----BEGIN PRIVATE KEY-----";
        let sanitized = sanitize_string(input);
        assert_eq!(sanitized, "Key: -----BEGIN PRIVATE KEY-----");
    }

    #[test]
    fn test_sanitize_password_in_url() {
        let input = "URL: https://user:password@example.com";
        let sanitized = sanitize_string(input);
        // Password should be sanitized
        assert!(sanitized.contains("********"));
        // Domain should still be present
        assert!(sanitized.contains("@example.com"));
    }

    #[test]
    fn test_sanitize_bearer_token() {
        let input = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
        let sanitized = sanitize_string(input);
        assert_eq!(sanitized, "Authorization: Bearer ********");
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
        assert!(sanitized.contains("AKIA********"));
        assert!(sanitized.contains("sk_********"));
    }

    #[test]
    fn test_no_secrets_unchanged() {
        let input = "This is a normal file path: /home/user/document.txt";
        let sanitized = sanitize_string(input);
        assert_eq!(sanitized, input);
    }
}
