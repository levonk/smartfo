// TOON vs JSON token usage benchmark

use smartfo::output::toon::to_string;
use serde_json::json;

#[test]
fn benchmark_toon_vs_json_simple_object() {
    let data = json!({
        "name": "test",
        "value": 42
    });
    
    let json_output = serde_json::to_string_pretty(&data).unwrap();
    let toon_output = to_string(&data).unwrap();
    
    println!("JSON length: {} chars", json_output.len());
    println!("TOON length: {} chars", toon_output.len());
    println!("TOON savings: {:.1}%", (1.0 - (toon_output.len() as f64 / json_output.len() as f64)) * 100.0);
    
    // TOON should be more compact
    assert!(toon_output.len() < json_output.len());
}

#[test]
fn benchmark_toon_vs_json_complex_object() {
    let data = json!({
        "operation": "move",
        "source": "/path/to/source",
        "destination": "/path/to/destination",
        "status": "success",
        "timestamp": "2024-01-01T00:00:00Z",
        "metadata": {
            "vcs": "git",
            "tracked": true,
            "async": false
        }
    });
    
    let json_output = serde_json::to_string_pretty(&data).unwrap();
    let toon_output = to_string(&data).unwrap();
    
    println!("JSON length: {} chars", json_output.len());
    println!("TOON length: {} chars", toon_output.len());
    println!("TOON savings: {:.1}%", (1.0 - (toon_output.len() as f64 / json_output.len() as f64)) * 100.0);
    
    // TOON should be more compact
    assert!(toon_output.len() < json_output.len());
}

#[test]
fn benchmark_toon_vs_json_array() {
    let data = json!([
        {"name": "Alice", "age": 30},
        {"name": "Bob", "age": 25},
        {"name": "Charlie", "age": 35}
    ]);
    
    let json_output = serde_json::to_string_pretty(&data).unwrap();
    let toon_output = to_string(&data).unwrap();
    
    println!("JSON length: {} chars", json_output.len());
    println!("TOON length: {} chars", toon_output.len());
    println!("TOON savings: {:.1}%", (1.0 - (toon_output.len() as f64 / json_output.len() as f64)) * 100.0);
    
    // TOON should be more compact
    assert!(toon_output.len() < json_output.len());
}

#[test]
fn benchmark_toon_vs_json_large_structure() {
    let data = json!({
        "users": [
            {"name": "Alice", "age": 30, "email": "alice@example.com"},
            {"name": "Bob", "age": 25, "email": "bob@example.com"},
            {"name": "Charlie", "age": 35, "email": "charlie@example.com"},
            {"name": "Diana", "age": 28, "email": "diana@example.com"},
            {"name": "Eve", "age": 32, "email": "eve@example.com"}
        ],
        "metadata": {
            "total": 5,
            "page": 1,
            "per_page": 5
        }
    });
    
    let json_output = serde_json::to_string_pretty(&data).unwrap();
    let toon_output = to_string(&data).unwrap();
    
    println!("JSON length: {} chars", json_output.len());
    println!("TOON length: {} chars", toon_output.len());
    println!("TOON savings: {:.1}%", (1.0 - (toon_output.len() as f64 / json_output.len() as f64)) * 100.0);
    
    // TOON should be more compact
    assert!(toon_output.len() < json_output.len());
}
