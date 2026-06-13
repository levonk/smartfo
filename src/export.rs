//! Export and import functionality for job data
//!
//! This module separates collection (daemon job processing) from processing (CLI analysis).
//! It allows exporting job data from the queue for analysis in different environments.
//!
//! Export Format (JSON):
//! ```json
//! {
//!   "version": "1.0",
//!   "exported_at": "2026-06-13T12:00:00Z",
//!   "jobs": [...]
//! }
//! ```
//!
//! Export Format (TOON):
//! Token-Oriented Object Notation for agent-friendly output
//!
//! This implements ADR #28: Collection vs Processing Separation

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use crate::queue::{Job, JobStatus, OperationType};

/// Export format version for compatibility
pub const EXPORT_FORMAT_VERSION: &str = "1.0";

/// Export format types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Json,
    Toon,
}

impl ExportFormat {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "json" => Ok(ExportFormat::Json),
            "toon" => Ok(ExportFormat::Toon),
            _ => anyhow::bail!("Unsupported export format: {}", s),
        }
    }
}

/// Export filters for job data
#[derive(Debug, Clone, Default)]
pub struct ExportFilters {
    /// Filter by date range (start, end)
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// Filter by job status
    pub status: Option<JobStatus>,
    /// Filter by operation type
    pub op_type: Option<OperationType>,
}

/// Export metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetadata {
    /// Export format version
    pub version: String,
    /// Export timestamp
    pub exported_at: String,
    /// Number of jobs in export
    pub job_count: usize,
    /// Filters applied during export
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,
}

/// Complete export structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobExport {
    /// Export metadata
    pub metadata: ExportMetadata,
    /// Exported jobs
    pub jobs: Vec<ExportedJob>,
}

/// Job data for export (serializable version of queue::Job)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedJob {
    /// Job UUID
    pub uuid: String,
    /// Source path
    pub source: String,
    /// Destination path (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest: Option<String>,
    /// Job status
    pub status: String,
    /// Retry count
    pub retry_count: i32,
    /// Operation type
    pub op_type: String,
    /// Creation timestamp
    pub created_at: String,
    /// Last update timestamp
    pub updated_at: String,
}

impl From<&Job> for ExportedJob {
    fn from(job: &Job) -> Self {
        ExportedJob {
            uuid: job.uuid.to_string(),
            source: job.source.to_string_lossy().to_string(),
            dest: job.dest.as_ref().map(|p| p.to_string_lossy().to_string()),
            status: format!("{:?}", job.status),
            retry_count: job.retry_count,
            op_type: format!("{:?}", job.op_type),
            created_at: job.created_at.to_rfc3339(),
            updated_at: job.updated_at.to_rfc3339(),
        }
    }
}

/// Export manager for job data
pub struct ExportManager {
    /// Queue database path
    queue_db_path: PathBuf,
}

impl ExportManager {
    /// Create a new export manager
    pub fn new(queue_db_path: PathBuf) -> Self {
        Self { queue_db_path }
    }

    /// Export jobs to a file
    ///
    /// # Arguments
    /// * `output_path` - Path to write export file
    /// * `format` - Export format (Json or Toon)
    /// * `filters` - Optional filters to apply
    pub fn export_jobs(
        &self,
        output_path: &Path,
        format: ExportFormat,
        filters: Option<ExportFilters>,
    ) -> Result<()> {
        use crate::queue::JobQueue;

        // Open queue database
        let queue = JobQueue::new(&self.queue_db_path)
            .context("Failed to open job queue database")?;

        // Get all jobs
        let jobs = queue.list_jobs(None)
            .context("Failed to list jobs from queue")?;

        // Apply filters
        let filtered_jobs = self.apply_filters(&jobs, filters.clone());

        // Convert to export format
        let exported_jobs: Vec<ExportedJob> = filtered_jobs.iter().map(ExportedJob::from).collect();

        // Create export metadata
        let metadata = ExportMetadata {
            version: EXPORT_FORMAT_VERSION.to_string(),
            exported_at: Utc::now().to_rfc3339(),
            job_count: exported_jobs.len(),
            filters: filters.map(|f| format!("{:?}", f)),
        };

        let export = JobExport {
            metadata,
            jobs: exported_jobs,
        };

        // Write to file based on format
        match format {
            ExportFormat::Json => {
                let json = serde_json::to_string_pretty(&export)
                    .context("Failed to serialize export to JSON")?;
                std::fs::write(output_path, json)
                    .context("Failed to write export file")?;
            }
            ExportFormat::Toon => {
                let toon = self.to_toon_format(&export)
                    .context("Failed to convert to TOON format")?;
                std::fs::write(output_path, toon)
                    .context("Failed to write TOON export file")?;
            }
        }

        tracing::info!("Exported {} jobs to {:?}", export.metadata.job_count, output_path);
        Ok(())
    }

    /// Import jobs from an export file
    ///
    /// # Arguments
    /// * `input_path` - Path to read export file
    ///
    /// # Returns
    /// Vector of jobs that can be re-enqueued
    pub fn import_jobs(&self, input_path: &Path) -> Result<Vec<Job>> {
        let content = std::fs::read_to_string(input_path)
            .context("Failed to read export file")?;

        // Detect format from file extension or content
        let export: JobExport = if input_path.extension().and_then(|s| s.to_str()) == Some("toon") {
            // Parse TOON format (simplified for now - assumes JSON-compatible structure)
            serde_json::from_str(&content)
                .context("Failed to parse TOON export")?
        } else {
            // Parse JSON format
            serde_json::from_str(&content)
                .context("Failed to parse JSON export")?
        };

        // Validate version
        if export.metadata.version != EXPORT_FORMAT_VERSION {
            tracing::warn!(
                "Export version {} differs from current version {}",
                export.metadata.version,
                EXPORT_FORMAT_VERSION
            );
        }

        // Convert exported jobs back to Job structs
        let jobs: Result<Vec<Job>> = export.jobs.iter().map(|ej| self.exported_to_job(ej)).collect();
        let jobs = jobs.context("Failed to convert exported jobs")?;

        tracing::info!("Imported {} jobs from {:?}", jobs.len(), input_path);
        Ok(jobs)
    }

    /// Analyze exported job data without requiring daemon
    ///
    /// # Arguments
    /// * `input_path` - Path to export file
    ///
    /// # Returns
    /// Analysis results as a string
    pub fn analyze_export(&self, input_path: &Path) -> Result<String> {
        let jobs = self.import_jobs(input_path)?;

        let mut analysis = String::new();
        analysis.push_str("# Job Data Analysis\n\n");

        // Total jobs
        analysis.push_str(&format!("Total jobs: {}\n\n", jobs.len()));

        // Status breakdown
        let mut status_counts = std::collections::HashMap::new();
        for job in &jobs {
            *status_counts.entry(format!("{:?}", job.status)).or_insert(0) += 1;
        }
        analysis.push_str("Status breakdown:\n");
        for (status, count) in status_counts {
            analysis.push_str(&format!("  {}: {}\n", status, count));
        }
        analysis.push_str("\n");

        // Operation type breakdown
        let mut op_counts = std::collections::HashMap::new();
        for job in &jobs {
            *op_counts.entry(format!("{:?}", job.op_type)).or_insert(0) += 1;
        }
        analysis.push_str("Operation type breakdown:\n");
        for (op_type, count) in op_counts {
            analysis.push_str(&format!("  {}: {}\n", op_type, count));
        }
        analysis.push_str("\n");

        // Date range
        if let Some((earliest, latest)) = self.get_date_range(&jobs) {
            analysis.push_str(&format!("Date range: {} to {}\n", earliest, latest));
        }

        Ok(analysis)
    }

    /// Apply filters to job list
    fn apply_filters(&self, jobs: &[Job], filters: Option<ExportFilters>) -> Vec<Job> {
        let filters = filters.unwrap_or_default();

        jobs.iter()
            .filter(|job| {
                // Filter by date range
                if let Some((start, end)) = &filters.date_range {
                    if job.created_at < *start || job.created_at > *end {
                        return false;
                    }
                }

                // Filter by status
                if let Some(status) = filters.status {
                    if job.status != status {
                        return false;
                    }
                }

                // Filter by operation type
                if let Some(op_type) = filters.op_type {
                    if job.op_type != op_type {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    /// Convert export format to TOON (Token-Oriented Object Notation)
    fn to_toon_format(&self, export: &JobExport) -> Result<String> {
        let mut toon = String::new();

        // Metadata
        toon.push_str(&format!("version: {}\n", export.metadata.version));
        toon.push_str(&format!("exported_at: {}\n", export.metadata.exported_at));
        toon.push_str(&format!("job_count: {}\n\n", export.metadata.job_count));

        // Jobs in TOON format
        toon.push_str("jobs[");
        for (i, job) in export.jobs.iter().enumerate() {
            if i > 0 {
                toon.push_str(" ");
            }
            toon.push_str(&format!(
                "{}{{uuid,status,op_type}}: \"{}\",\"{}\",\"{}\"",
                i, job.uuid, job.status, job.op_type
            ));
        }
        toon.push_str("]\n");

        Ok(toon)
    }

    /// Convert ExportedJob back to Job
    fn exported_to_job(&self, exported: &ExportedJob) -> Result<Job> {
        let uuid = uuid::Uuid::parse_str(&exported.uuid)
            .context("Failed to parse job UUID")?;

        let status = match exported.status.as_str() {
            "Queued" => JobStatus::Queued,
            "Running" => JobStatus::Running,
            "Done" => JobStatus::Done,
            "Failed" => JobStatus::Failed,
            _ => anyhow::bail!("Unknown job status: {}", exported.status),
        };

        let op_type = match exported.op_type.as_str() {
            "Move" => OperationType::Move,
            "Copy" => OperationType::Copy,
            "Delete" => OperationType::Delete,
            _ => anyhow::bail!("Unknown operation type: {}", exported.op_type),
        };

        let created_at = DateTime::parse_from_rfc3339(&exported.created_at)
            .context("Failed to parse created_at")?
            .with_timezone(&Utc);

        let updated_at = DateTime::parse_from_rfc3339(&exported.updated_at)
            .context("Failed to parse updated_at")?
            .with_timezone(&Utc);

        Ok(Job {
            uuid,
            source: PathBuf::from(&exported.source),
            dest: exported.dest.as_ref().map(PathBuf::from),
            status,
            retry_count: exported.retry_count,
            op_type,
            created_at,
            updated_at,
        })
    }

    /// Get date range of jobs
    fn get_date_range(&self, jobs: &[Job]) -> Option<(String, String)> {
        if jobs.is_empty() {
            return None;
        }

        let earliest = jobs.iter().map(|j| j.created_at).min()?;
        let latest = jobs.iter().map(|j| j.created_at).max()?;

        Some((earliest.to_rfc3339(), latest.to_rfc3339()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::queue::{Job, JobStatus, OperationType};
    use tempfile::NamedTempFile;
    use uuid::Uuid;

    fn create_test_job() -> Job {
        Job {
            uuid: Uuid::new_v4(),
            source: PathBuf::from("/tmp/test.txt"),
            dest: Some(PathBuf::from("/tmp/test_moved.txt")),
            status: JobStatus::Queued,
            retry_count: 0,
            op_type: OperationType::Move,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_export_format_version() {
        assert_eq!(EXPORT_FORMAT_VERSION, "1.0");
    }

    #[test]
    fn test_export_format_from_str() {
        assert!(matches!(ExportFormat::from_str("json").unwrap(), ExportFormat::Json));
        assert!(matches!(ExportFormat::from_str("JSON").unwrap(), ExportFormat::Json));
        assert!(matches!(ExportFormat::from_str("toon").unwrap(), ExportFormat::Toon));
        assert!(ExportFormat::from_str("invalid").is_err());
    }

    #[test]
    fn test_job_to_exported_job() {
        let job = create_test_job();
        let exported = ExportedJob::from(&job);

        assert_eq!(exported.uuid, job.uuid.to_string());
        assert_eq!(exported.source, job.source.to_string_lossy().to_string());
        assert_eq!(exported.status, "Queued");
        assert_eq!(exported.op_type, "Move");
    }

    #[test]
    fn test_apply_filters_by_status() {
        let job1 = Job {
            uuid: Uuid::new_v4(),
            source: PathBuf::from("/tmp/test1.txt"),
            dest: None,
            status: JobStatus::Queued,
            retry_count: 0,
            op_type: OperationType::Delete,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let job2 = Job {
            uuid: Uuid::new_v4(),
            source: PathBuf::from("/tmp/test2.txt"),
            dest: None,
            status: JobStatus::Done,
            retry_count: 0,
            op_type: OperationType::Delete,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let manager = ExportManager::new(PathBuf::from("/tmp/queue.db"));
        let filters = ExportFilters {
            status: Some(JobStatus::Queued),
            ..Default::default()
        };

        let filtered = manager.apply_filters(&[job1, job2], Some(filters));
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].status, JobStatus::Queued);
    }

    #[test]
    fn test_apply_filters_by_op_type() {
        let job1 = Job {
            uuid: Uuid::new_v4(),
            source: PathBuf::from("/tmp/test1.txt"),
            dest: None,
            status: JobStatus::Queued,
            retry_count: 0,
            op_type: OperationType::Move,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let job2 = Job {
            uuid: Uuid::new_v4(),
            source: PathBuf::from("/tmp/test2.txt"),
            dest: None,
            status: JobStatus::Queued,
            retry_count: 0,
            op_type: OperationType::Delete,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let manager = ExportManager::new(PathBuf::from("/tmp/queue.db"));
        let filters = ExportFilters {
            op_type: Some(OperationType::Move),
            ..Default::default()
        };

        let filtered = manager.apply_filters(&[job1, job2], Some(filters));
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].op_type, OperationType::Move);
    }

    #[test]
    fn test_to_toon_format() {
        let export = JobExport {
            metadata: ExportMetadata {
                version: "1.0".to_string(),
                exported_at: Utc::now().to_rfc3339(),
                job_count: 1,
                filters: None,
            },
            jobs: vec![ExportedJob {
                uuid: Uuid::new_v4().to_string(),
                source: "/tmp/test.txt".to_string(),
                dest: None,
                status: "Queued".to_string(),
                retry_count: 0,
                op_type: "Delete".to_string(),
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            }],
        };

        let manager = ExportManager::new(PathBuf::from("/tmp/queue.db"));
        let toon = manager.to_toon_format(&export).unwrap();

        assert!(toon.contains("version: 1.0"));
        assert!(toon.contains("job_count: 1"));
        assert!(toon.contains("jobs["));
    }

    #[test]
    fn test_exported_to_job() {
        let exported = ExportedJob {
            uuid: Uuid::new_v4().to_string(),
            source: "/tmp/test.txt".to_string(),
            dest: Some("/tmp/dest.txt".to_string()),
            status: "Queued".to_string(),
            retry_count: 0,
            op_type: "Move".to_string(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        let manager = ExportManager::new(PathBuf::from("/tmp/queue.db"));
        let job = manager.exported_to_job(&exported).unwrap();

        assert_eq!(job.uuid.to_string(), exported.uuid);
        assert_eq!(job.source, PathBuf::from(exported.source));
        assert_eq!(job.status, JobStatus::Queued);
        assert_eq!(job.op_type, OperationType::Move);
    }

    #[test]
    fn test_exported_to_job_invalid_status() {
        let exported = ExportedJob {
            uuid: Uuid::new_v4().to_string(),
            source: "/tmp/test.txt".to_string(),
            dest: None,
            status: "InvalidStatus".to_string(),
            retry_count: 0,
            op_type: "Move".to_string(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        let manager = ExportManager::new(PathBuf::from("/tmp/queue.db"));
        assert!(manager.exported_to_job(&exported).is_err());
    }

    #[test]
    fn test_get_date_range() {
        let job1 = Job {
            uuid: Uuid::new_v4(),
            source: PathBuf::from("/tmp/test1.txt"),
            dest: None,
            status: JobStatus::Queued,
            retry_count: 0,
            op_type: OperationType::Delete,
            created_at: Utc::now() - chrono::Duration::days(1),
            updated_at: Utc::now(),
        };

        let job2 = Job {
            uuid: Uuid::new_v4(),
            source: PathBuf::from("/tmp/test2.txt"),
            dest: None,
            status: JobStatus::Queued,
            retry_count: 0,
            op_type: OperationType::Delete,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let manager = ExportManager::new(PathBuf::from("/tmp/queue.db"));
        let (earliest, latest) = manager.get_date_range(&[job1, job2]).unwrap();

        assert!(earliest < latest);
    }

    #[test]
    fn test_get_date_range_empty() {
        let manager = ExportManager::new(PathBuf::from("/tmp/queue.db"));
        assert!(manager.get_date_range(&[]).is_none());
    }
}
