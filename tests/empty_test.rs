// Tests for empty state formatting

use smartfo::output::empty::{EmptyState, EmptyContext, check_empty};
use std::collections::HashMap;

#[test]
fn test_empty_context_basic() {
    let context = EmptyContext::new("test scope");
    assert_eq!(context.scope, "test scope");
    assert!(context.filters.is_none());
    assert!(context.total_scope_count.is_none());
}

#[test]
fn test_empty_context_with_single_filter() {
    let context = EmptyContext::new("operations")
        .with_filter("status", "completed");
    
    assert_eq!(context.scope, "operations");
    let filters = context.filters.unwrap();
    assert_eq!(filters.len(), 1);
    assert_eq!(filters.get("status"), Some(&"completed".to_string()));
}

#[test]
fn test_empty_context_with_multiple_filters() {
    let context = EmptyContext::new("operations")
        .with_filter("status", "completed")
        .with_filter("type", "move")
        .with_filter("source", "/tmp");
    
    let filters = context.filters.unwrap();
    assert_eq!(filters.len(), 3);
    assert_eq!(filters.get("status"), Some(&"completed".to_string()));
    assert_eq!(filters.get("type"), Some(&"move".to_string()));
    assert_eq!(filters.get("source"), Some(&"/tmp".to_string()));
}

#[test]
fn test_empty_context_with_total_count() {
    let context = EmptyContext::new("operations")
        .with_total_scope_count(100);
    
    assert_eq!(context.total_scope_count, Some(100));
}

#[test]
fn test_empty_context_combined() {
    let context = EmptyContext::new("operations")
        .with_filter("status", "completed")
        .with_total_scope_count(100);
    
    assert_eq!(context.scope, "operations");
    assert!(context.filters.is_some());
    assert_eq!(context.total_scope_count, Some(100));
}

#[test]
fn test_empty_state_basic_message() {
    let context = EmptyContext::new("all operations");
    let state = EmptyState::new(context);
    
    assert_eq!(state.count, 0);
    assert_eq!(state.message, "0 results found in all operations");
}

#[test]
fn test_empty_state_message_with_single_filter() {
    let context = EmptyContext::new("operations")
        .with_filter("status", "completed");
    let state = EmptyState::new(context);
    
    assert!(state.message.contains("0 results found in operations"));
    assert!(state.message.contains("status=completed"));
}

#[test]
fn test_empty_state_message_with_multiple_filters() {
    let mut filters = HashMap::new();
    filters.insert("status".to_string(), "completed".to_string());
    filters.insert("type".to_string(), "move".to_string());
    
    let context = EmptyContext::new("operations").with_filters_map(filters);
    let state = EmptyState::new(context);
    
    assert!(state.message.contains("0 results found in operations"));
    assert!(state.message.contains("status=completed"));
    assert!(state.message.contains("type=move"));
}

#[test]
fn test_empty_state_message_with_total_count() {
    let context = EmptyContext::new("operations")
        .with_total_scope_count(100);
    let state = EmptyState::new(context);
    
    assert!(state.message.contains("0 results found in operations"));
    assert!(state.message.contains("total scope: 100 items"));
}

#[test]
fn test_empty_state_message_with_filters_and_count() {
    let context = EmptyContext::new("operations")
        .with_filter("status", "completed")
        .with_total_scope_count(100);
    let state = EmptyState::new(context);
    
    assert!(state.message.contains("0 results found in operations"));
    assert!(state.message.contains("status=completed"));
    assert!(state.message.contains("total scope: 100 items"));
}

#[test]
fn test_empty_state_for_list_all() {
    let state = EmptyState::for_list(true, None);
    
    assert_eq!(state.count, 0);
    assert!(state.message.contains("all operations"));
    assert!(!state.message.contains("limit"));
}

#[test]
fn test_empty_state_for_list_with_limit() {
    let state = EmptyState::for_list(false, Some(10));
    
    assert_eq!(state.count, 0);
    assert!(state.message.contains("limit: 10"));
}

#[test]
fn test_empty_state_for_list_default_limit() {
    let state = EmptyState::for_list(false, None);
    
    assert_eq!(state.count, 0);
    assert!(state.message.contains("limit: 0"));
}

#[test]
fn test_empty_state_for_status_detailed() {
    let state = EmptyState::for_status(true);
    
    assert_eq!(state.count, 0);
    assert!(state.message.contains("detailed status"));
}

#[test]
fn test_empty_state_for_status_summary() {
    let state = EmptyState::for_status(false);
    
    assert_eq!(state.count, 0);
    assert!(state.message.contains("status summary"));
}

#[test]
fn test_empty_state_with_custom_filters() {
    let mut filters = HashMap::new();
    filters.insert("status".to_string(), "pending".to_string());
    filters.insert("type".to_string(), "remove".to_string());
    
    let state = EmptyState::with_filters("custom scope", filters);
    
    assert_eq!(state.count, 0);
    assert!(state.message.contains("custom scope"));
    assert!(state.message.contains("status=pending"));
    assert!(state.message.contains("type=remove"));
}

#[test]
fn test_check_empty_with_items() {
    let items = vec![1, 2, 3];
    let context = EmptyContext::new("test");
    let result = check_empty(&items, context);
    
    assert!(result.is_none());
}

#[test]
fn test_check_empty_without_items() {
    let items: Vec<i32> = vec![];
    let context = EmptyContext::new("test");
    let result = check_empty(&items, context);
    
    assert!(result.is_some());
    let state = result.unwrap();
    assert_eq!(state.count, 0);
}

#[test]
fn test_check_empty_with_single_item() {
    let items = vec![1];
    let context = EmptyContext::new("test");
    let result = check_empty(&items, context);
    
    assert!(result.is_none());
}

#[test]
fn test_empty_state_serialization() {
    let context = EmptyContext::new("test scope")
        .with_filter("status", "completed");
    let state = EmptyState::new(context);
    
    let json = serde_json::to_string(&state).unwrap();
    
    assert!(json.contains("\"count\":0"));
    assert!(json.contains("\"message\""));
    assert!(json.contains("\"context\""));
    assert!(json.contains("\"scope\""));
    assert!(json.contains("\"filters\""));
}

#[test]
fn test_empty_state_serialization_without_filters() {
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
fn test_empty_state_serialization_with_total_count() {
    let context = EmptyContext::new("test scope")
        .with_total_scope_count(100);
    let state = EmptyState::new(context);
    
    let json = serde_json::to_string(&state).unwrap();
    
    assert!(json.contains("\"total_scope_count\":100"));
}

#[test]
fn test_empty_state_roundtrip() {
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
fn test_empty_state_message_consistency() {
    // Test that messages are consistent across different creation methods
    let context1 = EmptyContext::new("all operations");
    let state1 = EmptyState::new(context1);
    
    let state2 = EmptyState::for_list(true, None);
    
    assert_eq!(state1.message, state2.message);
}

#[test]
fn test_empty_state_filter_ordering() {
    // Filter order should not affect the message content
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
