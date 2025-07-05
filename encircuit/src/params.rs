/*!
Parameter configuration for FHE operations.

This module provides opinionated presets that wrap TFHE configuration to make
it easier to configure FHE parameters for common use cases.
*/

use anyhow::Result;

/// FHE parameter configuration builder.
/// 
/// Provides opinionated presets for common FHE configurations.
#[derive(Debug, Clone)]
pub struct Builder {
    security_level: SecurityLevel,
    operation_types: Vec<OperationType>,
}

/// Compiled FHE parameters ready for key generation.
#[derive(Debug, Clone)]
pub struct Params {
    security_level: SecurityLevel,
    operation_types: Vec<OperationType>,
}

/// Security level for FHE operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    /// 128-bit security level (recommended)
    Security128,
    /// 256-bit security level (high security)
    Security256,
}

/// Types of operations supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationType {
    /// Boolean operations (AND, OR, XOR, NOT)
    Boolean,
    /// 8-bit integer operations
    Integer8,
    /// 32-bit integer operations
    Integer32,
}

impl Builder {
    /// Create a new parameter builder with default settings.
    pub fn new() -> Self {
        Self {
            security_level: SecurityLevel::Security128,
            operation_types: vec![OperationType::Boolean],
        }
    }

    /// Configure for 128-bit security level.
    /// 
    /// This is the recommended security level for most applications.
    pub fn security_128(mut self) -> Self {
        self.security_level = SecurityLevel::Security128;
        self
    }

    /// Configure for 256-bit security level.
    /// 
    /// This provides higher security but with increased computation cost.
    pub fn security_256(mut self) -> Self {
        self.security_level = SecurityLevel::Security256;
        self
    }

    /// Configure for Boolean operations only.
    /// 
    /// This enables efficient Boolean circuit evaluation.
    pub fn boolean_only(mut self) -> Self {
        self.operation_types = vec![OperationType::Boolean];
        self
    }

    /// Enable 8-bit integer operations.
    pub fn with_integer8(mut self) -> Self {
        if !self.operation_types.contains(&OperationType::Integer8) {
            self.operation_types.push(OperationType::Integer8);
        }
        self
    }

    /// Build the final parameters.
    /// 
    /// # Errors
    /// 
    /// Returns an error if the parameter configuration is invalid.
    pub fn build(self) -> Result<Params> {
        Ok(Params {
            security_level: self.security_level,
            operation_types: self.operation_types,
        })
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Params {
    /// Create a new parameter builder.
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Get the security level.
    pub fn security_level(&self) -> SecurityLevel {
        self.security_level
    }

    /// Get the supported operation types.
    pub fn operation_types(&self) -> &[OperationType] {
        &self.operation_types
    }

    /// Check if Boolean operations are supported.
    pub fn supports_boolean(&self) -> bool {
        self.operation_types.contains(&OperationType::Boolean)
    }

    /// Check if 8-bit integer operations are supported.
    pub fn supports_integer8(&self) -> bool {
        self.operation_types.contains(&OperationType::Integer8)
    }
}
