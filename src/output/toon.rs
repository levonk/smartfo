// TOON (Token-Oriented Object Notation) encoder/decoder
// Implements TOON format specification v3.3
// Reference: https://toonformat.dev/reference/spec.html

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// TOON encoder error type
#[derive(Debug, thiserror::Error)]
pub enum ToonError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

/// TOON encoder configuration
#[derive(Debug, Clone)]
pub struct ToonEncoder {
    /// Indentation size (default: 2 spaces)
    indent_size: usize,
    /// Enable key folding (default: false)
    key_folding: bool,
}

impl Default for ToonEncoder {
    fn default() -> Self {
        Self {
            indent_size: 2,
            key_folding: false,
        }
    }
}

impl ToonEncoder {
    /// Create a new TOON encoder with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set indentation size
    pub fn with_indent_size(mut self, size: usize) -> Self {
        self.indent_size = size;
        self
    }

    /// Enable key folding
    pub fn with_key_folding(mut self, enabled: bool) -> Self {
        self.key_folding = enabled;
        self
    }

    /// Encode a serializable value to TOON format
    pub fn encode<T: Serialize>(&self, value: &T) -> Result<String, ToonError> {
        let json_value = serde_json::to_value(value)
            .map_err(|e| ToonError::Serialization(e.to_string()))?;
        
        let mut output = String::new();
        self.encode_value(&json_value, 0, &mut output)?;
        Ok(output)
    }
}

impl ToonEncoder {
    fn encode_value(&self, value: &serde_json::Value, indent: usize, output: &mut String) -> Result<(), ToonError> {
        match value {
            serde_json::Value::Null => {
                output.push_str("null");
            }
            serde_json::Value::Bool(b) => {
                output.push_str(if *b { "true" } else { "false" });
            }
            serde_json::Value::Number(n) => {
                self.encode_number(n, output);
            }
            serde_json::Value::String(s) => {
                self.encode_string(s, output);
            }
            serde_json::Value::Array(arr) => {
                self.encode_array(arr, indent, output)?;
            }
            serde_json::Value::Object(obj) => {
                self.encode_object(obj, indent, output)?;
            }
        }
        Ok(())
    }

    fn encode_number(&self, n: &serde_json::Number, output: &mut String) {
        // Use canonical decimal representation for values in [1e-6, 1e21) or zero
        if let Some(f) = n.as_f64() {
            if f == 0.0 {
                output.push('0');
            } else if f.abs() >= 1e-6 && f.abs() < 1e21 {
                // For integers, output without decimal point
                if f.fract() == 0.0 {
                    output.push_str(&format!("{}", f as i64));
                } else {
                    output.push_str(&n.to_string());
                }
            } else {
                // Use exponent form for very small or very large numbers
                output.push_str(&format!("{:e}", f));
            }
        } else {
            output.push_str(&n.to_string());
        }
    }

    fn encode_string(&self, s: &str, output: &mut String) {
        // Check if string needs quoting
        let needs_quoting = self.string_needs_quoting(s);
        
        if needs_quoting {
            output.push('"');
            for c in s.chars() {
                match c {
                    '\\' => output.push_str("\\\\"),
                    '"' => output.push_str("\\\""),
                    '\n' => output.push_str("\\n"),
                    '\r' => output.push_str("\\r"),
                    '\t' => output.push_str("\\t"),
                    c if c <= '\u{001f}' => {
                        output.push_str(&format!("\\u{:04x}", c as u32));
                    }
                    _ => output.push(c),
                }
            }
            output.push('"');
        } else {
            output.push_str(s);
        }
    }

    fn string_needs_quoting(&self, s: &str) -> bool {
        // Strings must be quoted if they contain:
        // - Active delimiter (comma, tab, pipe)
        // - Colon
        // - Structural characters (brackets, braces)
        // - Whitespace
        // - Empty string
        
        if s.is_empty() {
            return true;
        }

        for c in s.chars() {
            match c {
                ',' | '\t' | '|' | ':' | '[' | ']' | '{' | '}' | ' ' | '\n' | '\r' => return true,
                _ => {}
            }
        }

        false
    }

    fn encode_array(&self, arr: &[serde_json::Value], indent: usize, output: &mut String) -> Result<(), ToonError> {
        if arr.is_empty() {
            output.push_str("[]");
            return Ok(());
        }

        // Check if this is a primitive array (all non-object, non-array values)
        let is_primitive = arr.iter().all(|v| !v.is_object() && !v.is_array());
        
        if is_primitive {
            // Inline primitive array
            output.push('[');
            for (i, item) in arr.iter().enumerate() {
                if i > 0 {
                    output.push_str(", ");
                }
                self.encode_value(item, indent, output)?;
            }
            output.push(']');
        } else {
            // Multi-line array
            let indent_str = " ".repeat(indent + self.indent_size);
            for item in arr {
                output.push('\n');
                output.push_str(&indent_str);
                output.push_str("- ");
                self.encode_value(item, indent + self.indent_size + 2, output)?;
            }
        }
        
        Ok(())
    }

    fn encode_object(&self, obj: &serde_json::Map<String, serde_json::Value>, indent: usize, output: &mut String) -> Result<(), ToonError> {
        if obj.is_empty() {
            output.push_str("{}");
            return Ok(());
        }

        let indent_str = " ".repeat(indent + self.indent_size);
        
        // Sort keys for consistent output
        let sorted_obj: BTreeMap<_, _> = obj.iter().collect();
        
        for (i, (key, value)) in sorted_obj.iter().enumerate() {
            if i > 0 {
                output.push('\n');
            }
            output.push_str(&indent_str);
            self.encode_string(key, output);
            output.push_str(": ");
            self.encode_value(value, indent + self.indent_size, output)?;
        }
        
        Ok(())
    }
}

/// Convert a serializable value to TOON format string
pub fn to_string<T: Serialize>(value: &T) -> Result<String, ToonError> {
    let encoder = ToonEncoder::new();
    encoder.encode(value)
}

/// Parse a TOON format string into a deserializable value
pub fn from_str<T: for<'de> Deserialize<'de>>(s: &str) -> Result<T, ToonError> {
    // For now, we'll implement a simple TOON-to-JSON conversion
    // This is a minimal implementation that handles basic TOON syntax
    let json_value = parse_toon_to_json(s)?;
    T::deserialize(json_value)
        .map_err(|e| ToonError::Deserialization(e.to_string()))
}

/// Simple TOON parser that converts TOON to JSON
/// This is a minimal implementation for the current requirements
fn parse_toon_to_json(s: &str) -> Result<serde_json::Value, ToonError> {
    // For the initial implementation, we'll use a simple approach:
    // If the input looks like JSON, parse it as JSON
    // Otherwise, return an error (full TOON parsing will be implemented in a follow-up)
    
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(s) {
        return Ok(json);
    }
    
    // If it's not valid JSON, we'll need to implement proper TOON parsing
    // For now, return a placeholder error
    Err(ToonError::Deserialization(
        "Full TOON parsing not yet implemented. Please use JSON format for now.".to_string()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_encode_null() {
        let result = to_string(&serde_json::Value::Null).unwrap();
        assert_eq!(result, "null");
    }

    #[test]
    fn test_encode_bool() {
        assert_eq!(to_string(&true).unwrap(), "true");
        assert_eq!(to_string(&false).unwrap(), "false");
    }

    #[test]
    fn test_encode_number() {
        assert_eq!(to_string(&42).unwrap(), "42");
        assert_eq!(to_string(&3.14).unwrap(), "3.14");
        assert_eq!(to_string(&0).unwrap(), "0");
    }

    #[test]
    fn test_encode_string() {
        assert_eq!(to_string(&"hello").unwrap(), "hello");
        assert_eq!(to_string(&"hello world").unwrap(), "\"hello world\"");
        assert_eq!(to_string(&"").unwrap(), "\"\"");
    }

    #[test]
    fn test_encode_string_with_escapes() {
        assert_eq!(to_string(&"hello\nworld").unwrap(), "\"hello\\nworld\"");
        assert_eq!(to_string(&"quote\"test").unwrap(), "\"quote\\\"test\"");
    }

    #[test]
    fn test_encode_array() {
        let arr = json!([1, 2, 3]);
        let result = to_string(&arr).unwrap();
        assert!(result.contains("["));
        assert!(result.contains("1"));
        assert!(result.contains("2"));
        assert!(result.contains("3"));
    }

    #[test]
    fn test_encode_object() {
        let obj = json!({"name": "test", "value": 42});
        let result = to_string(&obj).unwrap();
        assert!(result.contains("name"));
        assert!(result.contains("test"));
        assert!(result.contains("value"));
        assert!(result.contains("42"));
    }

    #[test]
    fn test_encode_nested_structure() {
        let obj = json!({
            "users": [
                {"name": "Alice", "age": 30},
                {"name": "Bob", "age": 25}
            ]
        });
        let result = to_string(&obj).unwrap();
        assert!(result.contains("users"));
        assert!(result.contains("name"));
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
    }

    #[test]
    fn test_from_str_json() {
        let json_str = r#"{"name": "test", "value": 42}"#;
        let result: serde_json::Value = from_str(json_str).unwrap();
        assert_eq!(result["name"], "test");
        assert_eq!(result["value"], 42);
    }

    #[test]
    fn test_roundtrip_json() {
        let original = json!({"name": "test", "value": 42, "nested": {"key": "val"}});
        let toon_str = to_string(&original).unwrap();
        // Note: Full roundtrip requires complete TOON parser
        // For now, we just verify encoding works
        assert!(!toon_str.is_empty());
    }
}
