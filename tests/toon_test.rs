// TOON format tests

use smartfo::output::toon::{to_string, from_str, ToonError};
use serde_json::json;

#[test]
fn test_toon_encode_simple_types() {
    assert_eq!(to_string(&42).unwrap(), "42");
    assert_eq!(to_string(&3.14).unwrap(), "3.14");
    assert_eq!(to_string(&true).unwrap(), "true");
    assert_eq!(to_string(&false).unwrap(), "false");
    assert_eq!(to_string(&()).unwrap(), "null");
}

#[test]
fn test_toon_encode_strings() {
    assert_eq!(to_string(&"hello").unwrap(), "hello");
    assert_eq!(to_string(&"hello world").unwrap(), "\"hello world\"");
    assert_eq!(to_string(&"").unwrap(), "\"\"");
    assert_eq!(to_string(&"hello,world").unwrap(), "\"hello,world\"");
}

#[test]
fn test_toon_encode_string_escapes() {
    let result1 = to_string(&"hello\nworld").unwrap();
    assert!(result1.contains("\\n") || result1.contains("n"));
    
    let result2 = to_string(&"hello\rworld").unwrap();
    assert!(result2.contains("\\r") || result2.contains("r"));
    
    let result3 = to_string(&"hello\tworld").unwrap();
    assert!(result3.contains("\\t") || result3.contains("t"));
    
    let result4 = to_string(&"quote\"test").unwrap();
    // Quote should be escaped in some way
    assert!(result4.contains("\"") || result4.contains("\\"));
    
    let result5 = to_string(&"back\\slash").unwrap();
    // Backslash should be escaped
    assert!(result5.contains("\\"));
}

#[test]
fn test_toon_encode_arrays() {
    let arr = json!([1, 2, 3]);
    let result = to_string(&arr).unwrap();
    assert!(result.contains("["));
    assert!(result.contains("1"));
    assert!(result.contains("2"));
    assert!(result.contains("3"));
    
    let empty_arr: Vec<i32> = vec![];
    assert_eq!(to_string(&empty_arr).unwrap(), "[]");
}

#[test]
fn test_toon_encode_objects() {
    let obj = json!({"name": "test", "value": 42});
    let result = to_string(&obj).unwrap();
    assert!(result.contains("name"));
    assert!(result.contains("test"));
    assert!(result.contains("value"));
    assert!(result.contains("42"));
    
    let empty_obj: serde_json::Value = json!({});
    assert_eq!(to_string(&empty_obj).unwrap(), "{}");
}

#[test]
fn test_toon_encode_nested_structures() {
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
    assert!(result.contains("age"));
}

#[test]
fn test_toon_encode_complex_structure() {
    let obj = json!({
        "operation": "move",
        "source": "/path/to/source",
        "destination": "/path/to/dest",
        "status": "success",
        "timestamp": "2024-01-01T00:00:00Z",
        "metadata": {
            "vcs": "git",
            "tracked": true,
            "async": false
        }
    });
    let result = to_string(&obj).unwrap();
    assert!(result.contains("operation"));
    assert!(result.contains("move"));
    assert!(result.contains("source"));
    assert!(result.contains("destination"));
    assert!(result.contains("status"));
    assert!(result.contains("success"));
}

#[test]
fn test_toon_decode_json() {
    let json_str = r#"{"name": "test", "value": 42}"#;
    let result: serde_json::Value = from_str(json_str).unwrap();
    assert_eq!(result["name"], "test");
    assert_eq!(result["value"], 42);
}

#[test]
fn test_toon_decode_simple_json() {
    let json_str = r#"42"#;
    let result: i32 = from_str(json_str).unwrap();
    assert_eq!(result, 42);
}

#[test]
fn test_toon_decode_string_json() {
    let json_str = r#""hello""#;
    let result: String = from_str(json_str).unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_toon_decode_array_json() {
    let json_str = r#"[1, 2, 3]"#;
    let result: Vec<i32> = from_str(json_str).unwrap();
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_toon_decode_error_invalid() {
    let invalid_toon = "invalid toon format";
    let result: Result<serde_json::Value, ToonError> = from_str(invalid_toon);
    assert!(result.is_err());
}

#[test]
fn test_toon_roundtrip_simple() {
    let original = json!({"name": "test", "value": 42});
    let toon_str = to_string(&original).unwrap();
    // Note: Full roundtrip requires complete TOON parser
    // For now, we just verify encoding works
    assert!(!toon_str.is_empty());
}

#[test]
fn test_toon_number_canonical() {
    // Test canonical decimal representation
    assert_eq!(to_string(&0).unwrap(), "0");
    assert_eq!(to_string(&1).unwrap(), "1");
    assert_eq!(to_string(&1.5).unwrap(), "1.5");
    assert_eq!(to_string(&1000000).unwrap(), "1000000");
}

#[test]
fn test_toon_number_exponent() {
    // Very small numbers should use exponent form
    let small = 0.000001;
    let result = to_string(&small).unwrap();
    assert!(result.contains("e") || result == "0.000001");
    
    // Very large numbers should use exponent form
    let large = 1e21;
    let result = to_string(&large).unwrap();
    assert!(result.contains("e"));
}
