// Library exports for testing
pub mod config;
pub mod output;

// Re-export aggregate types for testing
pub use output::aggregates::{ListAggregate, OperationAggregate, QueueAggregate, DaemonAggregate, StatusAggregate, AggregateComputer};
