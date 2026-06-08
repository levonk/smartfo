// Pre-computed aggregate counts and derived status fields
// Provides efficient aggregate computation for list outputs

use serde::{Serialize, Deserialize};

/// Aggregate counts for list operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAggregate {
    /// Current page size
    pub count: usize,
    /// Total number of items
    pub total: usize,
    /// Formatted count string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_display: Option<String>,
}

impl ListAggregate {
    /// Create a new list aggregate
    pub fn new(count: usize, total: usize) -> Self {
        let count_display = if total > 0 {
            Some(format!("{} of {} total", count, total))
        } else {
            None
        };
        
        Self {
            count,
            total,
            count_display,
        }
    }
    
    /// Create from a vector of items
    pub fn from_items<T>(items: &[T], total: usize) -> Self {
        Self::new(items.len(), total)
    }
    
    /// Check if there are more items available
    pub fn has_more(&self) -> bool {
        self.count < self.total
    }
}

/// Operation status aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationAggregate {
    /// Total number of operations
    pub total: usize,
    /// Number of completed operations
    pub completed: usize,
    /// Number of failed operations
    pub failed: usize,
    /// Number of pending operations
    pub pending: usize,
    /// Formatted status string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_display: Option<String>,
}

impl OperationAggregate {
    /// Create a new operation aggregate
    pub fn new(total: usize, completed: usize, failed: usize, pending: usize) -> Self {
        let status_display = if total > 0 {
            Some(format!("operations: {}/{} completed", completed, total))
        } else {
            None
        };
        
        Self {
            total,
            completed,
            failed,
            pending,
            status_display,
        }
    }
    
    /// Create from operation counts
    pub fn from_counts(total: usize, completed: usize, failed: usize) -> Self {
        let pending = total.saturating_sub(completed + failed);
        Self::new(total, completed, failed, pending)
    }
    
    /// Calculate completion percentage
    pub fn completion_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.completed as f64 / self.total as f64) * 100.0
        }
    }
}

/// Queue status aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueAggregate {
    /// Current queue size
    pub size: usize,
    /// Number of active jobs
    pub active_jobs: usize,
    /// Number of pending jobs
    pub pending_jobs: usize,
    /// Formatted queue status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_display: Option<String>,
}

impl QueueAggregate {
    /// Create a new queue aggregate
    pub fn new(size: usize, active_jobs: usize, pending_jobs: usize) -> Self {
        let queue_display = if size > 0 {
            Some(format!("queue: {} pending", pending_jobs))
        } else {
            None
        };
        
        Self {
            size,
            active_jobs,
            pending_jobs,
            queue_display,
        }
    }
    
    /// Create from queue size and active jobs
    pub fn from_queue(size: usize, active_jobs: usize) -> Self {
        let pending_jobs = size.saturating_sub(active_jobs);
        Self::new(size, active_jobs, pending_jobs)
    }
    
    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

/// Daemon status aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonAggregate {
    /// Daemon status (running, stopped, etc.)
    pub status: String,
    /// Process ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u32>,
    /// Uptime in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime_secs: Option<u64>,
    /// Formatted daemon status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_display: Option<String>,
}

impl DaemonAggregate {
    /// Create a new daemon aggregate
    pub fn new(status: String, pid: Option<u32>, uptime_secs: Option<u64>) -> Self {
        let daemon_display = Some(format!("daemon: {}", status));
        
        Self {
            status,
            pid,
            uptime_secs,
            daemon_display,
        }
    }
    
    /// Create from status string
    pub fn from_status(status: &str) -> Self {
        Self::new(status.to_string(), None, None)
    }
    
    /// Check if daemon is running
    pub fn is_running(&self) -> bool {
        self.status == "running"
    }
}

/// Combined status aggregate for status command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusAggregate {
    /// Operation aggregate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<OperationAggregate>,
    /// Queue aggregate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<QueueAggregate>,
    /// Daemon aggregate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon: Option<DaemonAggregate>,
}

impl StatusAggregate {
    /// Create a new status aggregate
    pub fn new(
        operations: Option<OperationAggregate>,
        queue: Option<QueueAggregate>,
        daemon: Option<DaemonAggregate>,
    ) -> Self {
        Self {
            operations,
            queue,
            daemon,
        }
    }
    
    /// Create empty status aggregate
    pub fn empty() -> Self {
        Self {
            operations: None,
            queue: None,
            daemon: None,
        }
    }
}

/// Aggregate computation utilities
pub struct AggregateComputer;

impl AggregateComputer {
    /// Compute list aggregate from items
    pub fn compute_list_aggregate<T>(items: &[T], total: usize) -> ListAggregate {
        ListAggregate::from_items(items, total)
    }
    
    /// Compute operation aggregate from counts
    pub fn compute_operation_aggregate(
        total: usize,
        completed: usize,
        failed: usize,
    ) -> OperationAggregate {
        OperationAggregate::from_counts(total, completed, failed)
    }
    
    /// Compute queue aggregate from size and active jobs
    pub fn compute_queue_aggregate(size: usize, active_jobs: usize) -> QueueAggregate {
        QueueAggregate::from_queue(size, active_jobs)
    }
    
    /// Compute daemon aggregate from status
    pub fn compute_daemon_aggregate(status: &str, pid: Option<u32>) -> DaemonAggregate {
        DaemonAggregate::new(status.to_string(), pid, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_aggregate() {
        let agg = ListAggregate::new(5, 10);
        assert_eq!(agg.count, 5);
        assert_eq!(agg.total, 10);
        assert!(agg.has_more());
        assert_eq!(agg.count_display, Some("5 of 10 total".to_string()));
    }

    #[test]
    fn test_list_aggregate_complete() {
        let agg = ListAggregate::new(10, 10);
        assert!(!agg.has_more());
    }

    #[test]
    fn test_operation_aggregate() {
        let agg = OperationAggregate::from_counts(10, 7, 2);
        assert_eq!(agg.total, 10);
        assert_eq!(agg.completed, 7);
        assert_eq!(agg.failed, 2);
        assert_eq!(agg.pending, 1);
        assert_eq!(agg.status_display, Some("operations: 7/10 completed".to_string()));
    }

    #[test]
    fn test_operation_completion_rate() {
        let agg = OperationAggregate::from_counts(10, 5, 0);
        assert_eq!(agg.completion_rate(), 50.0);
    }

    #[test]
    fn test_queue_aggregate() {
        let agg = QueueAggregate::from_queue(10, 3);
        assert_eq!(agg.size, 10);
        assert_eq!(agg.active_jobs, 3);
        assert_eq!(agg.pending_jobs, 7);
        assert_eq!(agg.queue_display, Some("queue: 7 pending".to_string()));
    }

    #[test]
    fn test_queue_empty() {
        let agg = QueueAggregate::from_queue(0, 0);
        assert!(agg.is_empty());
    }

    #[test]
    fn test_daemon_aggregate() {
        let agg = DaemonAggregate::from_status("running");
        assert_eq!(agg.status, "running");
        assert!(agg.is_running());
        assert_eq!(agg.daemon_display, Some("daemon: running".to_string()));
    }

    #[test]
    fn test_status_aggregate() {
        let ops = OperationAggregate::from_counts(10, 8, 1);
        let queue = QueueAggregate::from_queue(5, 2);
        let daemon = DaemonAggregate::from_status("running");
        
        let status = StatusAggregate::new(Some(ops), Some(queue), Some(daemon));
        assert!(status.operations.is_some());
        assert!(status.queue.is_some());
        assert!(status.daemon.is_some());
    }

    #[test]
    fn test_aggregate_computer() {
        let items = vec![1, 2, 3, 4, 5];
        let list_agg = AggregateComputer::compute_list_aggregate(&items, 10);
        assert_eq!(list_agg.count, 5);
        assert_eq!(list_agg.total, 10);
        
        let op_agg = AggregateComputer::compute_operation_aggregate(10, 7, 2);
        assert_eq!(op_agg.completed, 7);
        
        let queue_agg = AggregateComputer::compute_queue_aggregate(10, 3);
        assert_eq!(queue_agg.pending_jobs, 7);
        
        let daemon_agg = AggregateComputer::compute_daemon_aggregate("running", Some(1234));
        assert_eq!(daemon_agg.status, "running");
        assert_eq!(daemon_agg.pid, Some(1234));
    }
}