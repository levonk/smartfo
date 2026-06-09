// Integration tests for pre-computed aggregates in agent mode
// Tests aggregates in the context of actual CLI operations

use smartfo::output::aggregates::*;
use smartfo::output::toon::to_string;
use serde_json::json;

#[test]
fn test_list_aggregate_integration_with_cli_output() {
    // Test that list aggregates work in CLI output context
    let items = vec!["file1.txt", "file2.txt", "file3.txt"];
    let total_items = 10;
    
    let agg = ListAggregate::from_items(&items, total_items);
    
    // Verify aggregate computation
    assert_eq!(agg.count, 3);
    assert_eq!(agg.total, 10);
    assert!(agg.has_more());
    
    // Verify display string is formatted correctly
    assert_eq!(agg.count_display, Some("3 of 10 total".to_string()));
    
    // Verify serialization for JSON output
    let json = serde_json::to_string(&agg).unwrap();
    assert!(json.contains("\"count\":3"));
    assert!(json.contains("\"total\":10"));
}

#[test]
fn test_operation_aggregate_integration_with_status() {
    // Test operation aggregates in status command context
    let total_ops = 15;
    let completed = 12;
    let failed = 2;
    
    let agg = OperationAggregate::from_counts(total_ops, completed, failed);
    
    // Verify aggregate computation
    assert_eq!(agg.total, 15);
    assert_eq!(agg.completed, 12);
    assert_eq!(agg.failed, 2);
    assert_eq!(agg.pending, 1);
    
    // Verify completion rate
    assert_eq!(agg.completion_rate(), 80.0);
    
    // Verify display string
    assert_eq!(agg.status_display, Some("operations: 12/15 completed".to_string()));
    
    // Verify serialization
    let json = serde_json::to_string(&agg).unwrap();
    assert!(json.contains("\"total\":15"));
    assert!(json.contains("\"completed\":12"));
    assert!(json.contains("\"failed\":2"));
}

#[test]
fn test_queue_aggregate_integration_with_daemon() {
    // Test queue aggregates in daemon status context
    let queue_size = 20;
    let active_jobs = 5;
    
    let agg = QueueAggregate::from_queue(queue_size, active_jobs);
    
    // Verify aggregate computation
    assert_eq!(agg.size, 20);
    assert_eq!(agg.active_jobs, 5);
    assert_eq!(agg.pending_jobs, 15);
    
    // Verify display string
    assert_eq!(agg.queue_display, Some("queue: 15 pending".to_string()));
    
    // Verify empty check
    assert!(!agg.is_empty());
    
    // Verify serialization
    let json = serde_json::to_string(&agg).unwrap();
    assert!(json.contains("\"size\":20"));
    assert!(json.contains("\"active_jobs\":5"));
}

#[test]
fn test_daemon_aggregate_integration_with_process() {
    // Test daemon aggregates with actual process information
    let status = "running";
    let pid = Some(12345);
    let uptime = Some(3600);
    
    let agg = DaemonAggregate::new(status.to_string(), pid, uptime);
    
    // Verify aggregate computation
    assert_eq!(agg.status, "running");
    assert_eq!(agg.pid, Some(12345));
    assert_eq!(agg.uptime_secs, Some(3600));
    
    // Verify running state
    assert!(agg.is_running());
    
    // Verify display string
    assert_eq!(agg.daemon_display, Some("daemon: running".to_string()));
    
    // Verify serialization
    let json = serde_json::to_string(&agg).unwrap();
    assert!(json.contains("\"status\":\"running\""));
    assert!(json.contains("\"pid\":12345"));
}

#[test]
fn test_status_aggregate_integration_full_status() {
    // Test combined status aggregate for full status command
    let ops = OperationAggregate::from_counts(10, 8, 1);
    let queue = QueueAggregate::from_queue(5, 2);
    let daemon = DaemonAggregate::from_status("running");
    
    let status = StatusAggregate::new(Some(ops), Some(queue), Some(daemon));
    
    // Verify all components are present
    assert!(status.operations.is_some());
    assert!(status.queue.is_some());
    assert!(status.daemon.is_some());
    
    // Verify operations component
    let ops = status.operations.as_ref().unwrap();
    assert_eq!(ops.total, 10);
    assert_eq!(ops.completed, 8);
    
    // Verify queue component
    let queue = status.queue.as_ref().unwrap();
    assert_eq!(queue.size, 5);
    assert_eq!(queue.active_jobs, 2);
    
    // Verify daemon component
    let daemon = status.daemon.as_ref().unwrap();
    assert_eq!(daemon.status, "running");
    
    // Verify serialization
    let json = serde_json::to_string(&status).unwrap();
    assert!(json.contains("\"operations\""));
    assert!(json.contains("\"queue\""));
    assert!(json.contains("\"daemon\""));
}

#[test]
fn test_status_aggregate_integration_partial_status() {
    // Test partial status aggregate (e.g., when daemon is not running)
    let ops = OperationAggregate::from_counts(10, 8, 1);
    let queue = QueueAggregate::from_queue(5, 2);
    
    let status = StatusAggregate::new(Some(ops), Some(queue), None);
    
    // Verify operations and queue are present
    assert!(status.operations.is_some());
    assert!(status.queue.is_some());
    assert!(status.daemon.is_none());
    
    // Verify serialization skips None values
    let json = serde_json::to_string(&status).unwrap();
    assert!(json.contains("\"operations\""));
    assert!(json.contains("\"queue\""));
    assert!(!json.contains("\"daemon\""));
}

#[test]
fn test_aggregate_computer_integration_with_real_data() {
    // Test AggregateComputer with realistic data patterns
    
    // List aggregate from real file list
    let files = vec!["file1.txt", "file2.txt", "file3.txt", "file4.txt", "file5.txt"];
    let list_agg = AggregateComputer::compute_list_aggregate(&files, 20);
    assert_eq!(list_agg.count, 5);
    assert_eq!(list_agg.total, 20);
    assert!(list_agg.has_more());
    
    // Operation aggregate from real operation counts
    let op_agg = AggregateComputer::compute_operation_aggregate(100, 75, 15);
    assert_eq!(op_agg.total, 100);
    assert_eq!(op_agg.completed, 75);
    assert_eq!(op_agg.failed, 15);
    assert_eq!(op_agg.pending, 10);
    assert_eq!(op_agg.completion_rate(), 75.0);
    
    // Queue aggregate from real queue state
    let queue_agg = AggregateComputer::compute_queue_aggregate(50, 10);
    assert_eq!(queue_agg.size, 50);
    assert_eq!(queue_agg.active_jobs, 10);
    assert_eq!(queue_agg.pending_jobs, 40);
    
    // Daemon aggregate from real process state
    let daemon_agg = AggregateComputer::compute_daemon_aggregate("running", Some(54321));
    assert_eq!(daemon_agg.status, "running");
    assert_eq!(daemon_agg.pid, Some(54321));
}

#[test]
fn test_aggregate_with_toon_format() {
    // Test that aggregates work with TOON format for agent mode
    let agg = ListAggregate::new(5, 10);
    
    // Convert to JSON first (as TOON wraps JSON)
    let json_value = json!({
        "count": agg.count,
        "total": agg.total,
        "count_display": agg.count_display
    });
    
    let toon_str = to_string(&json_value).unwrap();
    
    // Verify TOON encoding preserves aggregate data
    assert!(toon_str.contains("5"));
    assert!(toon_str.contains("10"));
}

#[test]
fn test_aggregate_edge_cases_integration() {
    // Test edge cases in integration context
    
    // Empty list
    let empty_items: Vec<String> = vec![];
    let empty_agg = ListAggregate::from_items(&empty_items, 0);
    assert_eq!(empty_agg.count, 0);
    assert_eq!(empty_agg.total, 0);
    assert!(empty_agg.count_display.is_none());
    assert!(!empty_agg.has_more());
    
    // Single item
    let single_item = vec!["file.txt"];
    let single_agg = ListAggregate::from_items(&single_item, 1);
    assert_eq!(single_agg.count, 1);
    assert_eq!(single_agg.total, 1);
    assert!(!single_agg.has_more());
    
    // All operations completed
    let all_done = OperationAggregate::from_counts(10, 10, 0);
    assert_eq!(all_done.completion_rate(), 100.0);
    assert_eq!(all_done.pending, 0);
    
    // All operations failed
    let all_failed = OperationAggregate::from_counts(10, 0, 10);
    assert_eq!(all_failed.completion_rate(), 0.0);
    assert_eq!(all_failed.pending, 0);
    
    // Empty queue
    let empty_queue = QueueAggregate::from_queue(0, 0);
    assert!(empty_queue.is_empty());
    assert!(empty_queue.queue_display.is_none());
}

#[test]
fn test_aggregate_serialization_roundtrip() {
    // Test that aggregates can be serialized and deserialized correctly
    let ops = OperationAggregate::from_counts(10, 7, 2);
    let queue = QueueAggregate::from_queue(5, 2);
    let daemon = DaemonAggregate::new("running".to_string(), Some(1234), Some(3600));
    
    let status = StatusAggregate::new(Some(ops), Some(queue), Some(daemon));
    
    // Serialize
    let json = serde_json::to_string(&status).unwrap();
    
    // Deserialize
    let deserialized: StatusAggregate = serde_json::from_str(&json).unwrap();
    
    // Verify roundtrip preservation
    assert!(deserialized.operations.is_some());
    assert!(deserialized.queue.is_some());
    assert!(deserialized.daemon.is_some());
    
    let des_ops = deserialized.operations.unwrap();
    assert_eq!(des_ops.total, 10);
    assert_eq!(des_ops.completed, 7);
    assert_eq!(des_ops.failed, 2);
    
    let des_queue = deserialized.queue.unwrap();
    assert_eq!(des_queue.size, 5);
    assert_eq!(des_queue.active_jobs, 2);
    
    let des_daemon = deserialized.daemon.unwrap();
    assert_eq!(des_daemon.status, "running");
    assert_eq!(des_daemon.pid, Some(1234));
    assert_eq!(des_daemon.uptime_secs, Some(3600));
}

#[test]
fn test_aggregate_performance_large_dataset() {
    // Test aggregate computation performance with large datasets
    let large_list: Vec<i32> = (0..10000).collect();
    
    let agg = ListAggregate::from_items(&large_list, 20000);
    
    assert_eq!(agg.count, 10000);
    assert_eq!(agg.total, 20000);
    assert!(agg.has_more());
    
    // Verify display string is still correctly formatted
    assert_eq!(agg.count_display, Some("10000 of 20000 total".to_string()));
}

#[test]
fn test_aggregate_with_field_selection() {
    // Test that aggregates work with field selection for agent mode
    let agg = OperationAggregate::from_counts(10, 7, 2);
    
    // Simulate field selection by manually selecting fields
    let selected = json!({
        "total": agg.total,
        "completed": agg.completed,
        "completion_rate": agg.completion_rate()
    });
    
    assert_eq!(selected["total"], 10);
    assert_eq!(selected["completed"], 7);
    assert_eq!(selected["completion_rate"], 70.0);
}
