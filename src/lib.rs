//! Reproducibility status dashboard backend
//!
//! SigmaOS component inspired by Arch Linux's `arch-repro-status` repository.
//! Designed for component isolation, loose coupling, and secure defaults.

use std::fmt;

/// Errors produced by this component. Fail-fast: errors are returned, not panicked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchReproStatusError {
    /// The requested operation is not valid in the current state.
    InvalidState,
    /// Input failed validation.
    InvalidInput(String),
}

impl fmt::Display for ArchReproStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidState => write!(f, "invalid state"),
            Self::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
        }
    }
}

impl std::error::Error for ArchReproStatusError {}

/// Core component type. Holds configuration; performs no I/O by default.
#[derive(Debug, Clone, Default)]
pub struct ArchReproStatus {
    name: String,
}

impl ArchReproStatus {
    /// Create a new component instance. Fails fast on empty names.
    pub fn new(name: impl Into<String>) -> Result<Self, ArchReproStatusError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(ArchReproStatusError::InvalidInput("name must not be empty".into()));
        }
        Ok(Self { name })
    }

    /// Component identifier.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Perform the component's primary action. Returns a deterministic result.
    pub fn run(&self) -> Result<String, ArchReproStatusError> {
        if self.name.is_empty() {
            return Err(ArchReproStatusError::InvalidState);
        }
        Ok(format!("{}: ok", self.name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty_name() {
        // Arrange / Act / Assert
        assert!(matches!(ArchReproStatus::new("  "), Err(ArchReproStatusError::InvalidInput(_))));
    }

    #[test]
    fn run_returns_deterministic_result() {
        let c = ArchReproStatus::new("test").unwrap();
        assert_eq!(c.run().unwrap(), "test: ok");
    }

    #[test]
    fn name_is_accessible() {
        let c = ArchReproStatus::new("abc").unwrap();
        assert_eq!(c.name(), "abc");
    }
}
