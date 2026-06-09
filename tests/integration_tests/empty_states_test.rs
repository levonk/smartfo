// Integration tests for empty states in agent mode
// Tests empty states in the context of actual CLI operations

use smartfo::output::empty::{EmptyState, EmptyContext, check_empty};
use smartfo::output::toon::to_string;
use std::collections::HashMap;
use serde_json::json;

#[test]
fn test_empty_state_integration_with_list_operations() {
    // Test empty state in list operations context
    let items: Vec<String> = vec![];
    let context = EmptyContext::new("all operations");
    
    let state = check_empty(&items, context);
    
    assert!(state.is_some());
    let empty_state = state.unwrap();
    assert_eq!(empty_state.count, 0);
    assert!(empty_state.message.contains("0 results found in all operations"));
}

#[test]
fn test_empty_state_integration_with_filtered_list() {
    // Test empty state with filtered list operations
    let items: Vec<String> = vec![];
    let context = EmptyContext::new("operations")
        .with_filter("status", "completed")
        .with_filter("type", "move");
    
    let state = check_empty(&items, context);
    
    assert!(state.is_some());
    let empty_state = state.unwrap();
    assert!(empty_state.message.contains("0 results found in operations"));
    assert!(empty_state.message.contains("status=completed"));
    assert!(empty_state.message.contains("type=move"));
}

#[test]
fn test_empty_state_integration_with_total_scope() {
    // Test empty state when total scope is known
    let items: Vec<String> = vec![];
    let context = EmptyContext::new("operations")
        .with_total_scope_count(100);
    
    let state = check_empty(&items, context);
    
    assert!(state.is_some());
    let empty_state = state.unwrap();
    assert!(empty_state.message.contains("0 results found in operations"));
    assert!(empty_state.message.contains("total scope: 100 items"));
}

#[test]
fn test_empty_state_integration_with_filters_and_scope() {
    // Test empty state with both filters and total scope
    // Note: filters take precedence in message generation
    let items: Vec<String> = vec![];
    let context = EmptyContext::new("operations")
        .with_filter("status", "pending")
        .with_total_scope_count(50);
    
    let state = check_empty(&items, context);
    
    assert!(state.is_some());
    let empty_state = state.unwrap();
    assert!(empty_state.message.contains("0 results found in operations"));
    assert!(empty_state.message.contains("status=pending"));
    // Total scope is not shown when filters are present (implementation detail)
    assert!(!empty_state.message.contains("total scope"));
}

#[test]
fn test_empty_state_integration_non_empty_list() {
    // Test that empty state is not returned for non-empty lists
    let items = vec!["file1.txt", "file2.txt"];
    let context = EmptyContext::new("operations");
    
    let state = check_empty(&items, context);
    
    assert!(state.is_none());
}

#[test]
fn test_empty_state_for_list_all_integration() {
    // Test empty state for list all operations
    let state = EmptyState::for_list(true, None);
    
    assert_eq!(state.count, 0);
    assert!(state.message.contains("all operations"));
    assert!(!state.message.contains("limit"));
}

#[test]
fn test_empty_state_for_list_with_limit_integration() {
    // Test empty state for list with limit
    let state = EmptyState::for_list(false, Some(10));
    
    assert_eq!(state.count, 0);
    assert!(state.message.contains("limit: 10"));
}

#[test]
fn test_empty_state_for_status_detailed_integration() {
    // Test empty state for detailed status
    let state = EmptyState::for_status(true);
    
    assert_eq!(state.count, 0);
    assert!(state.message.contains("detailed status"));
}

#[test]
fn test_empty_state_for_status_summary_integration() {
    // Test empty state for status summary
    let state = EmptyState::for_status(false);
    
    assert_eq!(state.count, 0);
    assert!(state.message.contains("status summary"));
}

#[test]
fn test_empty_state_with_custom_filters_integration() {
    // Test empty state with custom filter map
    let mut filters = HashMap::new();
    filters.insert("status".to_string(), "pending".to_string());
    filters.insert("type".to_string(), "remove".to_string());
    filters.insert("source".to_string(), "/tmp".to_string());
    
    let state = EmptyState::with_filters("custom scope", filters);
    
    assert_eq!(state.count, 0);
    assert!(state.message.contains("custom scope"));
    assert!(state.message.contains("status=pending"));
    assert!(state.message.contains("type=remove"));
    assert!(state.message.contains("source=/tmp"));
}

#[test]
fn test_empty_state_serialization_integration() {
    // Test empty state serialization for JSON output
    let context = EmptyContext::new("test scope")
        .with_filter("status", "completed")
        .with_total_scope_count(100);
    let state = EmptyState::new(context);
    
    let json = serde_json::to_string(&state).unwrap();
    
    assert!(json.contains("\"count\":0"));
    assert!(json.contains("\"message\""));
    assert!(json.contains("\"context\""));
    assert!(json.contains("\"scope\""));
    assert!(json.contains("\"filters\""));
    assert!(json.contains("\"total_scope_count\":100"));
}

#[test]
fn test_empty_state_serialization_without_filters_integration() {
    // Test empty state serialization without filters
    let context = EmptyContext::new("test scope");
    let state = EmptyState::new(context);
    
    let json = serde_json::to_string(&state).unwrap();
    
    assert!(json.contains("\"count\":0"));
    assert!(json.contains("\"message\""));
    assert!(json.contains("\"context\""));
    assert!(json.contains("\"scope\""));
    // filters should not be in JSON when None
    assert!(!json.contains("\"filters\""));
}

#[test]
fn test_empty_state_roundtrip_integration() {
    // Test empty state serialization roundtrip
    let context = EmptyContext::new("test scope")
        .with_filter("status", "completed")
        .with_total_scope_count(50);
    let original = EmptyState::new(context);
    
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: EmptyState = serde_json::from_str(&json).unwrap();
    
    assert_eq!(original.count, deserialized.count);
    assert_eq!(original.message, deserialized.message);
    assert_eq!(original.context.scope, deserialized.context.scope);
    assert_eq!(original.context.total_scope_count, deserialized.context.total_scope_count);
}

#[test]
fn test_empty_state_with_toon_format() {
    // Test that empty states work with TOON format for agent mode
    let context = EmptyContext::new("operations")
        .with_filter("status", "completed");
    let state = EmptyState::new(context);
    
    // Convert to JSON first (as TOON wraps JSON)
    let json_value = json!({
        "count": state.count,
        "message": state.message,
        "scope": state.context.scope
    });
    
    let toon_str = to_string(&json_value).unwrap();
    
    // Verify TOON encoding preserves empty state data
    assert!(toon_str.contains("0"));
    assert!(toon_str.contains("operations"));
}

#[test]
fn test_empty_state_message_consistency_integration() {
    // Test that messages are consistent across different creation methods
    let context1 = EmptyContext::new("all operations");
    let state1 = EmptyState::new(context1);
    
    let state2 = EmptyState::for_list(true, None);
    
    assert_eq!(state1.message, state2.message);
}

#[test]
fn test_empty_state_filter_ordering_integration() {
    // Test that filter order doesn't affect message content
    let context1 = EmptyContext::new("test")
        .with_filter("status", "completed")
        .with_filter("type", "move");
    
    let context2 = EmptyContext::new("test")
        .with_filter("type", "move")
        .with_filter("status", "completed");
    
    let state1 = EmptyState::new(context1);
    let state2 = EmptyState::new(context2);
    
    // Both should contain the same filter information
    assert!(state1.message.contains("status=completed"));
    assert!(state1.message.contains("type=move"));
    assert!(state2.message.contains("status=completed"));
    assert!(state2.message.contains("type=move"));
}

#[test]
fn test_empty_state_edge_cases_integration() {
    // Test edge cases in integration context
    
    // Empty context with no filters
    let context = EmptyContext::new("test");
    assert!(context.filters.is_none());
    assert!(context.total_scope_count.is_none());
    
    // Context with single filter
    let context = EmptyContext::new("test").with_filter("key", "value");
    let filters = context.filters.unwrap();
    assert_eq!(filters.len(), 1);
    assert_eq!(filters.get("key"), Some(&"value".to_string()));
    
    // Context with zero total scope
    let context = EmptyContext::new("test").with_total_scope_count(0);
    assert_eq!(context.total_scope_count, Some(0));
    
    // Empty state with zero count
    let state = EmptyState::new(context);
    assert_eq!(state.count, 0);
}

#[test]
fn test_empty_state_with_field_selection() {
    // Test that empty states work with field selection for agent mode
    let context = EmptyContext::new("operations")
        .with_filter("status", "completed")
        .with_total_scope_count(100);
    let state = EmptyState::new(context);
    
    // Simulate field selection by manually selecting fields
    let selected = json!({
        "count": state.count,
        "message": state.message,
        "scope": state.context.scope
    });
    
    assert_eq!(selected["count"], 0);
    assert!(selected["message"].as_str().unwrap().contains("operations"));
    assert_eq!(selected["scope"], "operations");
}

#[test]
fn test_empty_state_multiple_filters_integration() {
    // Test empty state with multiple complex filters
    let context = EmptyContext::new("operations")
        .with_filter("status", "pending")
        .with_filter("type", "move")
        .with_filter("source", "/home/user")
        .with_filter("destination", "/tmp")
        .with_total_scope_count(1000);
    
    let state = EmptyState::new(context);
    
    assert_eq!(state.count, 0);
    assert!(state.message.contains("0 results found in operations"));
    assert!(state.message.contains("status=pending"));
    assert!(state.message.contains("type=move"));
    assert!(state.message.contains("source=/home/user"));
    assert!(state.message.contains("destination=/tmp"));
    // Total scope is not shown when filters are present (implementation detail)
    assert!(!state.message.contains("total scope"));
}

#[test]
fn test_empty_state_realistic_scenarios() {
    // Test realistic empty state scenarios
    
    // Scenario 1: No operations found
    let items: Vec<String> = vec![];
    let context = EmptyContext::new("operations");
    let state = check_empty(&items, context).unwrap();
    assert!(state.message.contains("0 results found in operations"));
    
    // Scenario 2: No completed operations
    let context = EmptyContext::new("operations")
        .with_filter("status", "completed")
        .with_total_scope_count(50);
    let state = check_empty(&items, context).unwrap();
    assert!(state.message.contains("status=completed"));
    // Total scope is not shown when filters are present (implementation detail)
    assert!(!state.message.contains("total scope"));
    
    // Scenario 3: No operations in specific directory
    let context = EmptyContext::new("operations")
        .with_filter("source", "/nonexistent/path");
    let state = check_empty(&items, context).unwrap();
    assert!(state.message.contains("source=/nonexistent/path"));
}
