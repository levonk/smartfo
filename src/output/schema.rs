// Schema definition and field selection for output
// Provides minimal default schemas to reduce token consumption

use serde::Serialize;
use std::collections::{HashSet, HashMap};
use anyhow::{Result, anyhow};

/// Available fields for different output types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Field {
    // Common fields
    Id,
    Type,
    Status,
    Source,
    
    // Operation-specific fields
    Operation,
    Timestamp,
    Path,
    Destination,
    Reason,
    
    // Queue-specific fields
    QueueSize,
    ActiveJobs,
    
    // Daemon-specific fields
    DaemonStatus,
    Pid,
    
    // Extended fields (for detail views)
    FullPath,
    VcsMessage,
    Metadata,
    Error,
    
    // Aggregate fields
    Count,
    Total,
    CountDisplay,
    Completed,
    Failed,
    Pending,
    StatusDisplay,
    QueueDisplay,
    DaemonDisplay,
}

impl Field {
    /// Parse field from string
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "id" => Some(Field::Id),
            "type" => Some(Field::Type),
            "status" => Some(Field::Status),
            "source" => Some(Field::Source),
            "operation" => Some(Field::Operation),
            "timestamp" => Some(Field::Timestamp),
            "path" => Some(Field::Path),
            "destination" => Some(Field::Destination),
            "reason" => Some(Field::Reason),
            "queue_size" => Some(Field::QueueSize),
            "active_jobs" => Some(Field::ActiveJobs),
            "daemon_status" => Some(Field::DaemonStatus),
            "pid" => Some(Field::Pid),
            "full_path" => Some(Field::FullPath),
            "vcs_message" => Some(Field::VcsMessage),
            "metadata" => Some(Field::Metadata),
            "error" => Some(Field::Error),
            "count" => Some(Field::Count),
            "total" => Some(Field::Total),
            "count_display" => Some(Field::CountDisplay),
            "completed" => Some(Field::Completed),
            "failed" => Some(Field::Failed),
            "pending" => Some(Field::Pending),
            "status_display" => Some(Field::StatusDisplay),
            "queue_display" => Some(Field::QueueDisplay),
            "daemon_display" => Some(Field::DaemonDisplay),
            _ => None,
        }
    }
    
    /// Convert field to string
    pub fn as_str(&self) -> &'static str {
        match self {
            Field::Id => "id",
            Field::Type => "type",
            Field::Status => "status",
            Field::Source => "source",
            Field::Operation => "operation",
            Field::Timestamp => "timestamp",
            Field::Path => "path",
            Field::Destination => "destination",
            Field::Reason => "reason",
            Field::QueueSize => "queue_size",
            Field::ActiveJobs => "active_jobs",
            Field::DaemonStatus => "daemon_status",
            Field::Pid => "pid",
            Field::FullPath => "full_path",
            Field::VcsMessage => "vcs_message",
            Field::Metadata => "metadata",
            Field::Error => "error",
            Field::Count => "count",
            Field::Total => "total",
            Field::CountDisplay => "count_display",
            Field::Completed => "completed",
            Field::Failed => "failed",
            Field::Pending => "pending",
            Field::StatusDisplay => "status_display",
            Field::QueueDisplay => "queue_display",
            Field::DaemonDisplay => "daemon_display",
        }
    }
}

/// Output schema definition
#[derive(Debug, Clone)]
pub struct Schema {
    /// Schema name
    pub name: String,
    /// Default fields for this schema
    pub default_fields: Vec<Field>,
    /// All available fields for this schema
    pub available_fields: Vec<Field>,
}

impl Schema {
    /// Create a new schema
    pub fn new(name: &str, default_fields: Vec<Field>, available_fields: Vec<Field>) -> Self {
        Self {
            name: name.to_string(),
            default_fields,
            available_fields,
        }
    }
    
    /// Get default fields
    pub fn get_default_fields(&self) -> &[Field] {
        &self.default_fields
    }
    
    /// Get all available fields
    pub fn get_available_fields(&self) -> &[Field] {
        &self.available_fields
    }
    
    /// Validate field selection against available fields
    pub fn validate_fields(&self, fields: &[Field]) -> Result<()> {
        let available: HashSet<_> = self.available_fields.iter().collect();
        for field in fields {
            if !available.contains(field) {
                anyhow::bail!("Field '{}' is not available in schema '{}'", field.as_str(), self.name);
            }
        }
        Ok(())
    }
}

/// Field selector for output
#[derive(Debug, Clone)]
pub struct FieldSelector {
    /// Selected fields
    pub fields: Vec<Field>,
}

impl FieldSelector {
    /// Create a field selector from schema defaults
    pub fn from_schema(schema: &Schema) -> Self {
        Self {
            fields: schema.get_default_fields().to_vec(),
        }
    }
    
    /// Create a field selector from explicit field list
    pub fn from_fields(fields: Vec<Field>, schema: &Schema) -> Result<Self> {
        schema.validate_fields(&fields)?;
        Ok(Self { fields })
    }
    
    /// Create a field selector from comma-separated string
    pub fn from_string(s: &str, schema: &Schema) -> Result<Self> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Ok(Self::from_schema(schema));
        }
        
        let fields: Vec<Field> = trimmed
            .split(',')
            .map(|f| f.trim())
            .map(|f| Field::parse(f).ok_or_else(|| anyhow::anyhow!("Invalid field: {}", f)))
            .collect::<Result<Vec<_>>>()?;
        
        if fields.is_empty() {
            return Ok(Self::from_schema(schema));
        }
        
        Self::from_fields(fields, schema)
    }
    
    /// Get selected fields
    pub fn get_fields(&self) -> &[Field] {
        &self.fields
    }
}

/// Schema registry
pub struct SchemaRegistry {
    schemas: HashMap<String, Schema>,
}

impl SchemaRegistry {
    /// Create a new schema registry with default schemas
    pub fn new() -> Self {
        let mut schemas = HashMap::new();
        
        // List command schema (operations, queue, daemon status)
        schemas.insert(
            "list".to_string(),
            Schema::new(
                "list",
                vec![Field::Id, Field::Type, Field::Status, Field::Source],
                vec![
                    Field::Id,
                    Field::Type,
                    Field::Status,
                    Field::Source,
                    Field::Operation,
                    Field::Timestamp,
                    Field::Path,
                    Field::Destination,
                    Field::Reason,
                    Field::FullPath,
                    Field::VcsMessage,
                    Field::Metadata,
                    Field::Count,
                    Field::Total,
                    Field::CountDisplay,
                ],
            ),
        );
        
        // Status command schema (operations, queue, daemon)
        schemas.insert(
            "status".to_string(),
            Schema::new(
                "status",
                vec![Field::Operation, Field::QueueSize, Field::DaemonStatus],
                vec![
                    Field::Operation,
                    Field::QueueSize,
                    Field::ActiveJobs,
                    Field::DaemonStatus,
                    Field::Pid,
                    Field::Timestamp,
                    Field::Error,
                    Field::Metadata,
                    Field::Completed,
                    Field::Failed,
                    Field::Pending,
                    Field::StatusDisplay,
                    Field::QueueDisplay,
                    Field::DaemonDisplay,
                ],
            ),
        );
        
        // Install command schema
        schemas.insert(
            "install".to_string(),
            Schema::new(
                "install",
                vec![Field::Status, Field::Source],
                vec![
                    Field::Status,
                    Field::Source,
                    Field::Path,
                    Field::Error,
                    Field::Metadata,
                ],
            ),
        );
        
        // Move operation schema
        schemas.insert(
            "move".to_string(),
            Schema::new(
                "move",
                vec![Field::Id, Field::Status, Field::Source, Field::Destination],
                vec![
                    Field::Id,
                    Field::Status,
                    Field::Source,
                    Field::Destination,
                    Field::Operation,
                    Field::Timestamp,
                    Field::Reason,
                    Field::FullPath,
                    Field::VcsMessage,
                    Field::Error,
                ],
            ),
        );
        
        // Remove operation schema
        schemas.insert(
            "remove".to_string(),
            Schema::new(
                "remove",
                vec![Field::Id, Field::Status, Field::Source],
                vec![
                    Field::Id,
                    Field::Status,
                    Field::Source,
                    Field::Operation,
                    Field::Timestamp,
                    Field::Reason,
                    Field::FullPath,
                    Field::VcsMessage,
                    Field::Error,
                ],
            ),
        );
        
        Self { schemas }
    }
    
    /// Get a schema by name
    pub fn get_schema(&self, name: &str) -> Option<&Schema> {
        self.schemas.get(name)
    }
    
    /// Get or create default schema
    pub fn get_or_default_schema(&self, name: &str) -> &Schema {
        self.get_schema(name).unwrap_or_else(|| {
            // Return a generic default schema if not found
            &self.schemas["list"]
        })
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Filter a serializable structure based on selected fields
pub trait FieldFilterable: Serialize {
    /// Filter output to include only selected fields
    fn filter_fields(&self, _fields: &[Field]) -> Result<serde_json::Value> {
        let value = serde_json::to_value(self)?;
        Ok(value)
    }
}

// Implement FieldFilterable for common types
impl<T: Serialize> FieldFilterable for T {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_field_parse() {
        assert_eq!(Field::parse("id"), Some(Field::Id));
        assert_eq!(Field::parse("type"), Some(Field::Type));
        assert_eq!(Field::parse("status"), Some(Field::Status));
        assert_eq!(Field::parse("invalid"), None);
    }
    
    #[test]
    fn test_field_as_str() {
        assert_eq!(Field::Id.as_str(), "id");
        assert_eq!(Field::Type.as_str(), "type");
        assert_eq!(Field::Status.as_str(), "status");
    }
    
    #[test]
    fn test_schema_creation() {
        let schema = Schema::new(
            "test",
            vec![Field::Id, Field::Type],
            vec![Field::Id, Field::Type, Field::Status],
        );
        assert_eq!(schema.name, "test");
        assert_eq!(schema.get_default_fields().len(), 2);
        assert_eq!(schema.get_available_fields().len(), 3);
    }
    
    #[test]
    fn test_schema_validate_fields() {
        let schema = Schema::new(
            "test",
            vec![Field::Id, Field::Type],
            vec![Field::Id, Field::Type, Field::Status],
        );
        
        // Valid fields
        assert!(schema.validate_fields(&[Field::Id, Field::Type]).is_ok());
        
        // Invalid field
        assert!(schema.validate_fields(&[Field::Id, Field::Source]).is_err());
    }
    
    #[test]
    fn test_field_selector_from_schema() {
        let schema = Schema::new(
            "test",
            vec![Field::Id, Field::Type],
            vec![Field::Id, Field::Type, Field::Status],
        );
        
        let selector = FieldSelector::from_schema(&schema);
        assert_eq!(selector.get_fields().len(), 2);
    }
    
    #[test]
    fn test_field_selector_from_fields() {
        let schema = Schema::new(
            "test",
            vec![Field::Id, Field::Type],
            vec![Field::Id, Field::Type, Field::Status],
        );
        
        let selector = FieldSelector::from_fields(vec![Field::Id, Field::Status], &schema).unwrap();
        assert_eq!(selector.get_fields().len(), 2);
    }
    
    #[test]
    fn test_field_selector_from_string() {
        let schema = Schema::new(
            "test",
            vec![Field::Id, Field::Type],
            vec![Field::Id, Field::Type, Field::Status],
        );
        
        let selector = FieldSelector::from_string("id,status", &schema).unwrap();
        assert_eq!(selector.get_fields().len(), 2);
    }
    
    #[test]
    fn test_field_selector_from_string_invalid() {
        let schema = Schema::new(
            "test",
            vec![Field::Id, Field::Type],
            vec![Field::Id, Field::Type, Field::Status],
        );
        
        // Invalid field should fail
        assert!(FieldSelector::from_string("id,invalid", &schema).is_err());
    }
    
    #[test]
    fn test_schema_registry() {
        let registry = SchemaRegistry::new();
        
        assert!(registry.get_schema("list").is_some());
        assert!(registry.get_schema("status").is_some());
        assert!(registry.get_schema("install").is_some());
        assert!(registry.get_schema("move").is_some());
        assert!(registry.get_schema("remove").is_some());
    }
    
    #[test]
    fn test_schema_registry_default_fields() {
        let registry = SchemaRegistry::new();
        
        let list_schema = registry.get_schema("list").unwrap();
        assert_eq!(list_schema.get_default_fields().len(), 4);
        
        let status_schema = registry.get_schema("status").unwrap();
        assert_eq!(status_schema.get_default_fields().len(), 3);
    }
}
