//! Integration tests for export/import functionality
//!
//! Tests the full export/import cycle with actual queue database operations

use smartfo::export::{ExportManager, ExportFormat, ExportFilters};
use smartfo::queue::{Job, JobQueue, JobStatus, OperationType};
use tempfile::TempDir;
use uuid::Uuid;
use std::path::PathBuf;

#[test]
fn test_export_import_cycle() {
    let temp_dir = TempDir::new().unwrap();
    let queue_path = temp_dir.path().join("queue.db");
    let export_path = temp_dir.path().join("export.json");

    // Create queue and add test jobs
    let queue = JobQueue::new(&queue_path).unwrap();

    let job1 = Job {
        uuid: Uuid::new_v4(),
        source: PathBuf::from("/tmp/test1.txt"),
        dest: Some(PathBuf::from("/tmp/test1_moved.txt")),
        status: JobStatus::Queued,
        retry_count: 0,
        op_type: OperationType::Move,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let job2 = Job {
        uuid: Uuid::new_v4(),
        source: PathBuf::from("/tmp/test2.txt"),
        dest: None,
        status: JobStatus::Done,
        retry_count: 0,
        op_type: OperationType::Delete,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    queue.enqueue(&job1).unwrap();
    queue.enqueue(&job2).unwrap();

    // Export jobs
    let export_manager = ExportManager::new(queue_path.clone());
    export_manager
        .export_jobs(&export_path, ExportFormat::Json, None)
        .unwrap();

    assert!(export_path.exists());

    // Import jobs
    let imported_jobs = export_manager.import_jobs(&export_path).unwrap();
    assert_eq!(imported_jobs.len(), 2);

    // Verify imported jobs match original
    let imported_uuids: Vec<_> = imported_jobs.iter().map(|j| j.uuid).collect();
    assert!(imported_uuids.contains(&job1.uuid));
    assert!(imported_uuids.contains(&job2.uuid));
}

#[test]
fn test_export_with_filters() {
    let temp_dir = TempDir::new().unwrap();
    let queue_path = temp_dir.path().join("queue.db");
    let export_path = temp_dir.path().join("export.json");

    // Create queue and add test jobs with different statuses
    let queue = JobQueue::new(&queue_path).unwrap();

    let job_queued = Job {
        uuid: Uuid::new_v4(),
        source: PathBuf::from("/tmp/queued.txt"),
        dest: None,
        status: JobStatus::Queued,
        retry_count: 0,
        op_type: OperationType::Delete,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let job_done = Job {
        uuid: Uuid::new_v4(),
        source: PathBuf::from("/tmp/done.txt"),
        dest: None,
        status: JobStatus::Done,
        retry_count: 0,
        op_type: OperationType::Delete,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    queue.enqueue(&job_queued).unwrap();
    queue.enqueue(&job_done).unwrap();

    // Export with status filter (only queued jobs)
    let export_manager = ExportManager::new(queue_path.clone());
    let filters = ExportFilters {
        status: Some(JobStatus::Queued),
        ..Default::default()
    };

    export_manager
        .export_jobs(&export_path, ExportFormat::Json, Some(filters))
        .unwrap();

    // Import and verify only queued job was exported
    let imported_jobs = export_manager.import_jobs(&export_path).unwrap();
    assert_eq!(imported_jobs.len(), 1);
    assert_eq!(imported_jobs[0].status, JobStatus::Queued);
}

#[test]
fn test_export_toon_format() {
    let temp_dir = TempDir::new().unwrap();
    let queue_path = temp_dir.path().join("queue.db");
    let export_path = temp_dir.path().join("export.toon");

    // Create queue and add test job
    let queue = JobQueue::new(&queue_path).unwrap();

    let job = Job {
        uuid: Uuid::new_v4(),
        source: PathBuf::from("/tmp/test.txt"),
        dest: None,
        status: JobStatus::Queued,
        retry_count: 0,
        op_type: OperationType::Delete,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    queue.enqueue(&job).unwrap();

    // Export to TOON format
    let export_manager = ExportManager::new(queue_path.clone());
    export_manager
        .export_jobs(&export_path, ExportFormat::Toon, None)
        .unwrap();

    assert!(export_path.exists());

    // Verify TOON format content
    let content = std::fs::read_to_string(&export_path).unwrap();
    assert!(content.contains("version:"));
    assert!(content.contains("job_count:"));
    assert!(content.contains("jobs["));
}

#[test]
fn test_analyze_export() {
    let temp_dir = TempDir::new().unwrap();
    let queue_path = temp_dir.path().join("queue.db");
    let export_path = temp_dir.path().join("export.json");

    // Create queue and add test jobs
    let queue = JobQueue::new(&queue_path).unwrap();

    let job1 = Job {
        uuid: Uuid::new_v4(),
        source: PathBuf::from("/tmp/test1.txt"),
        dest: None,
        status: JobStatus::Queued,
        retry_count: 0,
        op_type: OperationType::Delete,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let job2 = Job {
        uuid: Uuid::new_v4(),
        source: PathBuf::from("/tmp/test2.txt"),
        dest: None,
        status: JobStatus::Done,
        retry_count: 0,
        op_type: OperationType::Move,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    queue.enqueue(&job1).unwrap();
    queue.enqueue(&job2).unwrap();

    // Export jobs
    let export_manager = ExportManager::new(queue_path.clone());
    export_manager
        .export_jobs(&export_path, ExportFormat::Json, None)
        .unwrap();

    // Analyze export
    let analysis = export_manager.analyze_export(&export_path).unwrap();

    assert!(analysis.contains("Total jobs: 2"));
    assert!(analysis.contains("Status breakdown:"));
    assert!(analysis.contains("Operation type breakdown:"));
}

#[test]
fn test_export_empty_queue() {
    let temp_dir = TempDir::new().unwrap();
    let queue_path = temp_dir.path().join("queue.db");
    let export_path = temp_dir.path().join("export.json");

    // Create empty queue
    let queue = JobQueue::new(&queue_path).unwrap();

    // Export empty queue
    let export_manager = ExportManager::new(queue_path.clone());
    export_manager
        .export_jobs(&export_path, ExportFormat::Json, None)
        .unwrap();

    assert!(export_path.exists());

    // Import and verify empty
    let imported_jobs = export_manager.import_jobs(&export_path).unwrap();
    assert_eq!(imported_jobs.len(), 0);
}

#[test]
fn test_export_with_date_range_filter() {
    let temp_dir = TempDir::new().unwrap();
    let queue_path = temp_dir.path().join("queue.db");
    let export_path = temp_dir.path().join("export.json");

    // Create queue and add test jobs with different timestamps
    let queue = JobQueue::new(&queue_path).unwrap();

    let now = chrono::Utc::now();
    let yesterday = now - chrono::Duration::days(1);

    let job_old = Job {
        uuid: Uuid::new_v4(),
        source: PathBuf::from("/tmp/old.txt"),
        dest: None,
        status: JobStatus::Queued,
        retry_count: 0,
        op_type: OperationType::Delete,
        created_at: yesterday,
        updated_at: yesterday,
    };

    let job_new = Job {
        uuid: Uuid::new_v4(),
        source: PathBuf::from("/tmp/new.txt"),
        dest: None,
        status: JobStatus::Queued,
        retry_count: 0,
        op_type: OperationType::Delete,
        created_at: now,
        updated_at: now,
    };

    queue.enqueue(&job_old).unwrap();
    queue.enqueue(&job_new).unwrap();

    // Export with date range filter (only today)
    let export_manager = ExportManager::new(queue_path.clone());
    let filters = ExportFilters {
        date_range: Some((now - chrono::Duration::hours(1), now + chrono::Duration::hours(1))),
        ..Default::default()
    };

    export_manager
        .export_jobs(&export_path, ExportFormat::Json, Some(filters))
        .unwrap();

    // Import and verify only new job was exported
    let imported_jobs = export_manager.import_jobs(&export_path).unwrap();
    assert_eq!(imported_jobs.len(), 1);
    assert_eq!(imported_jobs[0].uuid, job_new.uuid);
}
