// Performance benchmarks for CLI AXI agent mode features
// Measures: TOON encoding, truncation, and basic operations

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use smartfo::output::toon::{to_string};
use smartfo::output::truncation::{truncate};
use serde_json::json;

fn bench_toon_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("toon_encoding");
    
    // Small object
    let small = json!({
        "id": "123",
        "name": "test",
        "status": "pending"
    });
    group.bench_function("small_object", |b| {
        b.iter(|| to_string(black_box(&small)).unwrap())
    });
    
    // Medium object
    let medium = json!({
        "id": "123",
        "name": "test",
        "status": "pending",
        "source": "/path/to/source",
        "destination": "/path/to/destination",
        "timestamp": "2025-01-15T10:30:00Z",
        "metadata": {"key": "value"}
    });
    group.bench_function("medium_object", |b| {
        b.iter(|| to_string(black_box(&medium)).unwrap())
    });
    
    // Large object (nested)
    let large = json!({
        "id": "123",
        "items": (0..100).map(|i| json!({
            "index": i,
            "name": format!("item_{}", i),
            "values": vec![1, 2, 3, 4, 5]
        })).collect::<Vec<_>>(),
        "metadata": {
            "created": "2025-01-15T10:30:00Z",
            "updated": "2025-01-15T10:30:00Z",
            "tags": ["tag1", "tag2", "tag3"]
        }
    });
    group.bench_function("large_object", |b| {
        b.iter(|| to_string(black_box(&large)).unwrap())
    });
    
    group.finish();
}

fn bench_toon_decoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("toon_decoding");
    
    // Note: Full TOON decoding is not yet implemented, so we benchmark JSON decoding
    // as a proxy for the expected TOON decoding performance
    
    let json_small = r#"{"id":"123","name":"test","status":"pending"}"#;
    group.bench_function("small_object_json", |b| {
        b.iter(|| serde_json::from_str::<serde_json::Value>(json_small).unwrap())
    });
    
    let json_medium = r#"{"id":"123","name":"test","status":"pending","source":"/path/to/source","destination":"/path/to/destination","timestamp":"2025-01-15T10:30:00Z","metadata":{"key":"value"}}"#;
    group.bench_function("medium_object_json", |b| {
        b.iter(|| serde_json::from_str::<serde_json::Value>(json_medium).unwrap())
    });
    
    let large_json = json!({
        "id": "123",
        "items": (0..100).map(|i| json!({
            "index": i,
            "name": format!("item_{}", i),
            "values": vec![1, 2, 3, 4, 5]
        })).collect::<Vec<_>>()
    });
    let json_large_str = serde_json::to_string(&large_json).unwrap();
    group.bench_function("large_object_json", |b| {
        b.iter(|| serde_json::from_str::<serde_json::Value>(&json_large_str).unwrap())
    });
    
    group.finish();
}

fn bench_schema_filtering(c: &mut Criterion) {
    let mut group = c.benchmark_group("schema_filtering");
    
    // Benchmark JSON field selection (manual filtering)
    let full_data = json!({
        "id": "123",
        "type": "move",
        "status": "pending",
        "source": "/path/to/source",
        "destination": "/path/to/destination",
        "timestamp": "2025-01-15T10:30:00Z",
        "metadata": {"key": "value"},
        "queue_position": 5,
        "retry_count": 2
    });
    
    // Filter to 4 fields
    group.bench_function("filter_4_fields", |b| {
        b.iter(|| {
            let mut filtered = serde_json::Map::new();
            if let Some(obj) = full_data.as_object() {
                for key in &["id", "type", "status", "source"] {
                    if let Some(value) = obj.get(*key) {
                        filtered.insert(key.to_string(), value.clone());
                    }
                }
            }
            filtered
        })
    });
    
    // Filter to 2 fields
    group.bench_function("filter_2_fields", |b| {
        b.iter(|| {
            let mut filtered = serde_json::Map::new();
            if let Some(obj) = full_data.as_object() {
                for key in &["id", "status"] {
                    if let Some(value) = obj.get(*key) {
                        filtered.insert(key.to_string(), value.clone());
                    }
                }
            }
            filtered
        })
    });
    
    group.finish();
}

fn bench_aggregate_computation(c: &mut Criterion) {
    let mut group = c.benchmark_group("aggregate_computation");
    
    // Small dataset (10 items)
    let small_items: Vec<serde_json::Value> = (0..10).map(|i| json!({
        "id": format!("id_{}", i),
        "status": if i % 3 == 0 { "completed" } else { "pending" },
        "type": "move"
    })).collect();
    group.bench_function("small_dataset_10_items", |b| {
        b.iter(|| {
            let total = small_items.len();
            let completed = small_items.iter().filter(|item| {
                item.get("status").and_then(|s| s.as_str()) == Some("completed")
            }).count();
            (total, completed)
        })
    });
    
    // Medium dataset (100 items)
    let medium_items: Vec<serde_json::Value> = (0..100).map(|i| json!({
        "id": format!("id_{}", i),
        "status": match i % 4 {
            0 => "completed",
            1 => "pending",
            2 => "failed",
            _ => "running"
        },
        "type": if i % 2 == 0 { "move" } else { "remove" }
    })).collect();
    group.bench_function("medium_dataset_100_items", |b| {
        b.iter(|| {
            let total = medium_items.len();
            let completed = medium_items.iter().filter(|item| {
                item.get("status").and_then(|s| s.as_str()) == Some("completed")
            }).count();
            (total, completed)
        })
    });
    
    // Large dataset (1000 items)
    let large_items: Vec<serde_json::Value> = (0..1000).map(|i| json!({
        "id": format!("id_{}", i),
        "status": match i % 5 {
            0 => "completed",
            1 => "pending",
            2 => "failed",
            3 => "running",
            _ => "cancelled"
        },
        "type": match i % 3 {
            0 => "move",
            1 => "remove",
            _ => "install"
        }
    })).collect();
    group.bench_function("large_dataset_1000_items", |b| {
        b.iter(|| {
            let total = large_items.len();
            let completed = large_items.iter().filter(|item| {
                item.get("status").and_then(|s| s.as_str()) == Some("completed")
            }).count();
            (total, completed)
        })
    });
    
    group.finish();
}

fn bench_content_truncation(c: &mut Criterion) {
    let mut group = c.benchmark_group("content_truncation");
    
    // Small content (100 chars)
    let small_content = "a".repeat(100);
    group.bench_function("small_content_100_chars", |b| {
        b.iter(|| truncate(black_box(&small_content), 1000))
    });
    
    // Medium content (1000 chars)
    let medium_content = "a".repeat(1000);
    group.bench_function("medium_content_1000_chars", |b| {
        b.iter(|| truncate(black_box(&medium_content), 1000))
    });
    
    // Large content (10000 chars)
    let large_content = "a".repeat(10000);
    group.bench_function("large_content_10000_chars", |b| {
        b.iter(|| truncate(black_box(&large_content), 1000))
    });
    
    // Very large content (100000 chars)
    let very_large_content = "a".repeat(100000);
    group.bench_function("very_large_content_100000_chars", |b| {
        b.iter(|| truncate(black_box(&very_large_content), 1000))
    });
    
    // Unicode content
    let unicode_content = "文件内容测试".repeat(200);
    group.bench_function("unicode_content", |b| {
        b.iter(|| truncate(black_box(&unicode_content), 1000))
    });
    
    group.finish();
}

fn bench_suggestion_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("suggestion_generation");
    
    // Benchmark simple string operations for suggestion generation
    let commands = vec!["list", "status", "install", "move", "remove"];
    let statuses = vec!["pending", "completed", "failed", "running"];
    
    group.bench_function("generate_suggestions", |b| {
        b.iter(|| {
            let mut suggestions = Vec::new();
            for cmd in &commands {
                for status in &statuses {
                    suggestions.push(format!("smartfo {} --status {}", cmd, status));
                }
            }
            suggestions
        })
    });
    
    group.finish();
}

fn bench_toon_vs_json(c: &mut Criterion) {
    let mut group = c.benchmark_group("toon_vs_json");
    
    let test_data = json!({
        "id": "123",
        "name": "test_operation",
        "status": "pending",
        "source": "/path/to/source",
        "destination": "/path/to/destination",
        "timestamp": "2025-01-15T10:30:00Z",
        "metadata": {
            "created_by": "user",
            "priority": "high",
            "tags": ["important", "urgent"]
        }
    });
    
    // TOON encoding
    group.bench_function("toon_encode", |b| {
        b.iter(|| to_string(black_box(&test_data)).unwrap())
    });
    
    // JSON encoding
    group.bench_function("json_encode", |b| {
        b.iter(|| serde_json::to_string(black_box(&test_data)).unwrap())
    });
    
    // JSON decoding (TOON decoding not yet implemented)
    let json_string = serde_json::to_string(&test_data).unwrap();
    group.bench_function("json_decode", |b| {
        b.iter(|| serde_json::from_str::<serde_json::Value>(black_box(&json_string)).unwrap())
    });
    
    group.finish();
}

criterion_group!(
    agent_mode_benches,
    bench_toon_encoding,
    bench_toon_decoding,
    bench_schema_filtering,
    bench_aggregate_computation,
    bench_content_truncation,
    bench_suggestion_generation,
    bench_toon_vs_json
);
criterion_main!(agent_mode_benches);
