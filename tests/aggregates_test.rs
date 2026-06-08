// Tests for aggregate computation

use smartfo::output::aggregates::*;

#[test]
fn test_list_aggregate_basic() {
    let agg = ListAggregate::new(5, 10);
    assert_eq!(agg.count, 5);
    assert_eq!(agg.total, 10);
    assert_eq!(agg.count_display, Some("5 of 10 total".to_string()));
    assert!(agg.has_more());
}

#[test]
fn test_list_aggregate_complete() {
    let agg = ListAggregate::new(10, 10);
    assert_eq!(agg.count, 10);
    assert_eq!(agg.total, 10);
    assert!(!agg.has_more());
}

#[test]
fn test_list_aggregate_empty() {
    let agg = ListAggregate::new(0, 0);
    assert_eq!(agg.count, 0);
    assert_eq!(agg.total, 0);
    assert!(agg.count_display.is_none());
    assert!(!agg.has_more());
}

#[test]
fn test_list_aggregate_from_items() {
    let items = vec![1, 2, 3, 4, 5];
    let agg = ListAggregate::from_items(&items, 10);
    assert_eq!(agg.count, 5);
    assert_eq!(agg.total, 10);
}

#[test]
fn test_operation_aggregate_basic() {
    let agg = OperationAggregate::from_counts(10, 7, 2);
    assert_eq!(agg.total, 10);
    assert_eq!(agg.completed, 7);
    assert_eq!(agg.failed, 2);
    assert_eq!(agg.pending, 1);
    assert_eq!(agg.status_display, Some("operations: 7/10 completed".to_string()));
}

#[test]
fn test_operation_aggregate_all_completed() {
    let agg = OperationAggregate::from_counts(10, 10, 0);
    assert_eq!(agg.completed, 10);
    assert_eq!(agg.failed, 0);
    assert_eq!(agg.pending, 0);
    assert_eq!(agg.completion_rate(), 100.0);
}

#[test]
fn test_operation_aggregate_all_failed() {
    let agg = OperationAggregate::from_counts(10, 0, 10);
    assert_eq!(agg.completed, 0);
    assert_eq!(agg.failed, 10);
    assert_eq!(agg.pending, 0);
    assert_eq!(agg.completion_rate(), 0.0);
}

#[test]
fn test_operation_aggregate_empty() {
    let agg = OperationAggregate::from_counts(0, 0, 0);
    assert_eq!(agg.total, 0);
    assert_eq!(agg.completed, 0);
    assert_eq!(agg.failed, 0);
    assert_eq!(agg.pending, 0);
    assert!(agg.status_display.is_none());
    assert_eq!(agg.completion_rate(), 0.0);
}

#[test]
fn test_operation_completion_rate() {
    let agg = OperationAggregate::from_counts(10, 5, 0);
    assert_eq!(agg.completion_rate(), 50.0);
    
    let agg2 = OperationAggregate::from_counts(100, 75, 10);
    assert_eq!(agg2.completion_rate(), 75.0);
}

#[test]
fn test_queue_aggregate_basic() {
    let agg = QueueAggregate::from_queue(10, 3);
    assert_eq!(agg.size, 10);
    assert_eq!(agg.active_jobs, 3);
    assert_eq!(agg.pending_jobs, 7);
    assert_eq!(agg.queue_display, Some("queue: 7 pending".to_string()));
}

#[test]
fn test_queue_aggregate_empty() {
    let agg = QueueAggregate::from_queue(0, 0);
    assert_eq!(agg.size, 0);
    assert_eq!(agg.active_jobs, 0);
    assert_eq!(agg.pending_jobs, 0);
    assert!(agg.queue_display.is_none());
    assert!(agg.is_empty());
}

#[test]
fn test_queue_aggregate_all_active() {
    let agg = QueueAggregate::from_queue(10, 10);
    assert_eq!(agg.size, 10);
    assert_eq!(agg.active_jobs, 10);
    assert_eq!(agg.pending_jobs, 0);
    assert!(!agg.is_empty());
}

#[test]
fn test_daemon_aggregate_basic() {
    let agg = DaemonAggregate::from_status("running");
    assert_eq!(agg.status, "running");
    assert!(agg.pid.is_none());
    assert!(agg.is_running());
    assert_eq!(agg.daemon_display, Some("daemon: running".to_string()));
}

#[test]
fn test_daemon_aggregate_with_pid() {
    let agg = DaemonAggregate::new("running".to_string(), Some(1234), Some(3600));
    assert_eq!(agg.status, "running");
    assert_eq!(agg.pid, Some(1234));
    assert_eq!(agg.uptime_secs, Some(3600));
    assert!(agg.is_running());
}

#[test]
fn test_daemon_aggregate_stopped() {
    let agg = DaemonAggregate::from_status("stopped");
    assert_eq!(agg.status, "stopped");
    assert!(!agg.is_running());
}

#[test]
fn test_status_aggregate_full() {
    let ops = OperationAggregate::from_counts(10, 8, 1);
    let queue = QueueAggregate::from_queue(5, 2);
    let daemon = DaemonAggregate::from_status("running");
    
    let status = StatusAggregate::new(Some(ops), Some(queue), Some(daemon));
    assert!(status.operations.is_some());
    assert!(status.queue.is_some());
    assert!(status.daemon.is_some());
}

#[test]
fn test_status_aggregate_partial() {
    let ops = OperationAggregate::from_counts(10, 8, 1);
    let status = StatusAggregate::new(Some(ops), None, None);
    assert!(status.operations.is_some());
    assert!(status.queue.is_none());
    assert!(status.daemon.is_none());
}

#[test]
fn test_status_aggregate_empty() {
    let status = StatusAggregate::empty();
    assert!(status.operations.is_none());
    assert!(status.queue.is_none());
    assert!(status.daemon.is_none());
}

#[test]
fn test_aggregate_computer_list() {
    let items = vec![1, 2, 3, 4, 5];
    let agg = AggregateComputer::compute_list_aggregate(&items, 10);
    assert_eq!(agg.count, 5);
    assert_eq!(agg.total, 10);
}

#[test]
fn test_aggregate_computer_operation() {
    let agg = AggregateComputer::compute_operation_aggregate(10, 7, 2);
    assert_eq!(agg.total, 10);
    assert_eq!(agg.completed, 7);
    assert_eq!(agg.failed, 2);
    assert_eq!(agg.pending, 1);
}

#[test]
fn test_aggregate_computer_queue() {
    let agg = AggregateComputer::compute_queue_aggregate(10, 3);
    assert_eq!(agg.size, 10);
    assert_eq!(agg.active_jobs, 3);
    assert_eq!(agg.pending_jobs, 7);
}

#[test]
fn test_aggregate_computer_daemon() {
    let agg = AggregateComputer::compute_daemon_aggregate("running", Some(1234));
    assert_eq!(agg.status, "running");
    assert_eq!(agg.pid, Some(1234));
}

#[test]
fn test_aggregate_serialization() {
    let agg = ListAggregate::new(5, 10);
    let json = serde_json::to_string(&agg).unwrap();
    assert!(json.contains("\"count\":5"));
    assert!(json.contains("\"total\":10"));
    
    let deserialized: ListAggregate = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.count, 5);
    assert_eq!(deserialized.total, 10);
}

#[test]
fn test_operation_aggregate_serialization() {
    let agg = OperationAggregate::from_counts(10, 7, 2);
    let json = serde_json::to_string(&agg).unwrap();
    assert!(json.contains("\"total\":10"));
    assert!(json.contains("\"completed\":7"));
    assert!(json.contains("\"failed\":2"));
    
    let deserialized: OperationAggregate = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.total, 10);
    assert_eq!(deserialized.completed, 7);
    assert_eq!(deserialized.failed, 2);
}

#[test]
fn test_queue_aggregate_serialization() {
    let agg = QueueAggregate::from_queue(10, 3);
    let json = serde_json::to_string(&agg).unwrap();
    assert!(json.contains("\"size\":10"));
    assert!(json.contains("\"active_jobs\":3"));
    
    let deserialized: QueueAggregate = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.size, 10);
    assert_eq!(deserialized.active_jobs, 3);
}

#[test]
fn test_daemon_aggregate_serialization() {
    let agg = DaemonAggregate::new("running".to_string(), Some(1234), Some(3600));
    let json = serde_json::to_string(&agg).unwrap();
    assert!(json.contains("\"status\":\"running\""));
    assert!(json.contains("\"pid\":1234"));
    
    let deserialized: DaemonAggregate = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.status, "running");
    assert_eq!(deserialized.pid, Some(1234));
}

#[test]
fn test_status_aggregate_serialization() {
    let ops = OperationAggregate::from_counts(10, 8, 1);
    let queue = QueueAggregate::from_queue(5, 2);
    let daemon = DaemonAggregate::from_status("running");
    
    let status = StatusAggregate::new(Some(ops), Some(queue), Some(daemon));
    let json = serde_json::to_string(&status).unwrap();
    assert!(json.contains("\"operations\""));
    assert!(json.contains("\"queue\""));
    assert!(json.contains("\"daemon\""));
    
    let deserialized: StatusAggregate = serde_json::from_str(&json).unwrap();
    assert!(deserialized.operations.is_some());
    assert!(deserialized.queue.is_some());
    assert!(deserialized.daemon.is_some());
}