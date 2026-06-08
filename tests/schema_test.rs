// Schema and field selection tests

use smartfo::output::{Field, Schema, SchemaRegistry, FieldSelector};

#[test]
fn test_field_parsing() {
    assert_eq!(Field::parse("id"), Some(Field::Id));
    assert_eq!(Field::parse("type"), Some(Field::Type));
    assert_eq!(Field::parse("status"), Some(Field::Status));
    assert_eq!(Field::parse("source"), Some(Field::Source));
    assert_eq!(Field::parse("operation"), Some(Field::Operation));
    assert_eq!(Field::parse("timestamp"), Some(Field::Timestamp));
    assert_eq!(Field::parse("path"), Some(Field::Path));
    assert_eq!(Field::parse("destination"), Some(Field::Destination));
    assert_eq!(Field::parse("reason"), Some(Field::Reason));
    assert_eq!(Field::parse("queue_size"), Some(Field::QueueSize));
    assert_eq!(Field::parse("active_jobs"), Some(Field::ActiveJobs));
    assert_eq!(Field::parse("daemon_status"), Some(Field::DaemonStatus));
    assert_eq!(Field::parse("pid"), Some(Field::Pid));
    assert_eq!(Field::parse("full_path"), Some(Field::FullPath));
    assert_eq!(Field::parse("vcs_message"), Some(Field::VcsMessage));
    assert_eq!(Field::parse("metadata"), Some(Field::Metadata));
    assert_eq!(Field::parse("error"), Some(Field::Error));
    assert_eq!(Field::parse("invalid_field"), None);
}

#[test]
fn test_field_to_string() {
    assert_eq!(Field::Id.as_str(), "id");
    assert_eq!(Field::Type.as_str(), "type");
    assert_eq!(Field::Status.as_str(), "status");
    assert_eq!(Field::Source.as_str(), "source");
    assert_eq!(Field::Operation.as_str(), "operation");
    assert_eq!(Field::Timestamp.as_str(), "timestamp");
    assert_eq!(Field::Path.as_str(), "path");
    assert_eq!(Field::Destination.as_str(), "destination");
    assert_eq!(Field::Reason.as_str(), "reason");
    assert_eq!(Field::QueueSize.as_str(), "queue_size");
    assert_eq!(Field::ActiveJobs.as_str(), "active_jobs");
    assert_eq!(Field::DaemonStatus.as_str(), "daemon_status");
    assert_eq!(Field::Pid.as_str(), "pid");
    assert_eq!(Field::FullPath.as_str(), "full_path");
    assert_eq!(Field::VcsMessage.as_str(), "vcs_message");
    assert_eq!(Field::Metadata.as_str(), "metadata");
    assert_eq!(Field::Error.as_str(), "error");
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
fn test_schema_field_validation() {
    let schema = Schema::new(
        "test",
        vec![Field::Id, Field::Type],
        vec![Field::Id, Field::Type, Field::Status],
    );
    
    // Valid fields should pass
    assert!(schema.validate_fields(&[Field::Id, Field::Type]).is_ok());
    assert!(schema.validate_fields(&[Field::Id, Field::Type, Field::Status]).is_ok());
    
    // Invalid field should fail
    assert!(schema.validate_fields(&[Field::Id, Field::Source]).is_err());
    assert!(schema.validate_fields(&[Field::Operation]).is_err());
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
    assert_eq!(selector.get_fields(), &[Field::Id, Field::Type]);
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
    assert_eq!(selector.get_fields(), &[Field::Id, Field::Status]);
}

#[test]
fn test_field_selector_from_fields_invalid() {
    let schema = Schema::new(
        "test",
        vec![Field::Id, Field::Type],
        vec![Field::Id, Field::Type, Field::Status],
    );
    
    // Invalid field should fail
    let result = FieldSelector::from_fields(vec![Field::Id, Field::Source], &schema);
    assert!(result.is_err());
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
    assert_eq!(selector.get_fields(), &[Field::Id, Field::Status]);
}

#[test]
fn test_field_selector_from_string_with_spaces() {
    let schema = Schema::new(
        "test",
        vec![Field::Id, Field::Type],
        vec![Field::Id, Field::Type, Field::Status],
    );
    
    let selector = FieldSelector::from_string("id, status , type", &schema).unwrap();
    assert_eq!(selector.get_fields().len(), 3);
    assert_eq!(selector.get_fields(), &[Field::Id, Field::Status, Field::Type]);
}

#[test]
fn test_field_selector_from_string_invalid() {
    let schema = Schema::new(
        "test",
        vec![Field::Id, Field::Type],
        vec![Field::Id, Field::Type, Field::Status],
    );
    
    // Invalid field should fail
    let result = FieldSelector::from_string("id,invalid_field", &schema);
    assert!(result.is_err());
}

#[test]
fn test_field_selector_from_string_empty() {
    let schema = Schema::new(
        "test",
        vec![Field::Id, Field::Type],
        vec![Field::Id, Field::Type, Field::Status],
    );
    
    // Empty string should use default schema
    let selector = FieldSelector::from_string("", &schema).unwrap();
    assert_eq!(selector.get_fields().len(), 2);
    assert_eq!(selector.get_fields(), &[Field::Id, Field::Type]);
}

#[test]
fn test_schema_registry() {
    let registry = SchemaRegistry::new();
    
    // Check that all expected schemas exist
    assert!(registry.get_schema("list").is_some());
    assert!(registry.get_schema("status").is_some());
    assert!(registry.get_schema("install").is_some());
    assert!(registry.get_schema("move").is_some());
    assert!(registry.get_schema("remove").is_some());
    
    // Check that non-existent schema returns None
    assert!(registry.get_schema("nonexistent").is_none());
}

#[test]
fn test_schema_registry_list_defaults() {
    let registry = SchemaRegistry::new();
    let list_schema = registry.get_schema("list").unwrap();
    
    assert_eq!(list_schema.name, "list");
    assert_eq!(list_schema.get_default_fields().len(), 4);
    assert_eq!(list_schema.get_default_fields(), &[
        Field::Id,
        Field::Type,
        Field::Status,
        Field::Source,
    ]);
}

#[test]
fn test_schema_registry_status_defaults() {
    let registry = SchemaRegistry::new();
    let status_schema = registry.get_schema("status").unwrap();
    
    assert_eq!(status_schema.name, "status");
    assert_eq!(status_schema.get_default_fields().len(), 3);
    assert_eq!(status_schema.get_default_fields(), &[
        Field::Operation,
        Field::QueueSize,
        Field::DaemonStatus,
    ]);
}

#[test]
fn test_schema_registry_install_defaults() {
    let registry = SchemaRegistry::new();
    let install_schema = registry.get_schema("install").unwrap();
    
    assert_eq!(install_schema.name, "install");
    assert_eq!(install_schema.get_default_fields().len(), 2);
    assert_eq!(install_schema.get_default_fields(), &[
        Field::Status,
        Field::Source,
    ]);
}

#[test]
fn test_schema_registry_move_defaults() {
    let registry = SchemaRegistry::new();
    let move_schema = registry.get_schema("move").unwrap();
    
    assert_eq!(move_schema.name, "move");
    assert_eq!(move_schema.get_default_fields().len(), 4);
    assert_eq!(move_schema.get_default_fields(), &[
        Field::Id,
        Field::Status,
        Field::Source,
        Field::Destination,
    ]);
}

#[test]
fn test_schema_registry_remove_defaults() {
    let registry = SchemaRegistry::new();
    let remove_schema = registry.get_schema("remove").unwrap();
    
    assert_eq!(remove_schema.name, "remove");
    assert_eq!(remove_schema.get_default_fields().len(), 3);
    assert_eq!(remove_schema.get_default_fields(), &[
        Field::Id,
        Field::Status,
        Field::Source,
    ]);
}

#[test]
fn test_schema_registry_get_or_default() {
    let registry = SchemaRegistry::new();
    
    // Existing schema should return the schema
    let list_schema = registry.get_or_default_schema("list");
    assert_eq!(list_schema.name, "list");
    
    // Non-existent schema should return default (list)
    let default_schema = registry.get_or_default_schema("nonexistent");
    assert_eq!(default_schema.name, "list");
}

#[test]
fn test_default_schemas_max_fields() {
    let registry = SchemaRegistry::new();
    
    // All default schemas should have 3-4 fields maximum
    let list_schema = registry.get_schema("list").unwrap();
    assert!(list_schema.get_default_fields().len() <= 4);
    
    let status_schema = registry.get_schema("status").unwrap();
    assert!(status_schema.get_default_fields().len() <= 4);
    
    let install_schema = registry.get_schema("install").unwrap();
    assert!(install_schema.get_default_fields().len() <= 4);
    
    let move_schema = registry.get_schema("move").unwrap();
    assert!(move_schema.get_default_fields().len() <= 4);
    
    let remove_schema = registry.get_schema("remove").unwrap();
    assert!(remove_schema.get_default_fields().len() <= 4);
}
