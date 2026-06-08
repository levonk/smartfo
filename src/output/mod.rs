// Output format abstraction for smartfo
// Supports multiple output formats: TOON, JSON, and human-readable

pub mod toon;

use serde::Serialize;
use std::io::Write;

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// TOON format (token-efficient for agents)
    Toon,
    /// JSON format (machine-readable)
    Json,
    /// Human-readable format
    Human,
}

impl OutputFormat {
    /// Parse from string
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "toon" => Some(OutputFormat::Toon),
            "json" => Some(OutputFormat::Json),
            "human" => Some(OutputFormat::Human),
            _ => None,
        }
    }

    /// Convert to string
    pub fn as_str(&self) -> &'static str {
        match self {
            OutputFormat::Toon => "toon",
            OutputFormat::Json => "json",
            OutputFormat::Human => "human",
        }
    }
}

/// Output writer that handles different formats
pub struct OutputWriter<W: Write> {
    writer: W,
    format: OutputFormat,
}

impl<W: Write> OutputWriter<W> {
    /// Create a new output writer
    pub fn new(writer: W, format: OutputFormat) -> Self {
        Self { writer, format }
    }

    /// Write data in the configured format
    pub fn write<T: Serialize>(&mut self, data: &T) -> anyhow::Result<()> {
        match self.format {
            OutputFormat::Toon => {
                let toon_output = toon::to_string(data)?;
                writeln!(self.writer, "{}", toon_output)?;
            }
            OutputFormat::Json => {
                let json_output = serde_json::to_string_pretty(data)?;
                writeln!(self.writer, "{}", json_output)?;
            }
            OutputFormat::Human => {
                // Human-readable format - use JSON for now, can be enhanced later
                let json_output = serde_json::to_string_pretty(data)?;
                writeln!(self.writer, "{}", json_output)?;
            }
        }
        Ok(())
    }

    /// Get the underlying writer
    pub fn into_inner(self) -> W {
        self.writer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_from_str() {
        assert_eq!(OutputFormat::parse("toon"), Some(OutputFormat::Toon));
        assert_eq!(OutputFormat::parse("TOON"), Some(OutputFormat::Toon));
        assert_eq!(OutputFormat::parse("json"), Some(OutputFormat::Json));
        assert_eq!(OutputFormat::parse("human"), Some(OutputFormat::Human));
        assert_eq!(OutputFormat::parse("invalid"), None);
    }

    #[test]
    fn test_output_format_as_str() {
        assert_eq!(OutputFormat::Toon.as_str(), "toon");
        assert_eq!(OutputFormat::Json.as_str(), "json");
        assert_eq!(OutputFormat::Human.as_str(), "human");
    }
}
