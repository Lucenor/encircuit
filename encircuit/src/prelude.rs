/*!
Prelude module that re-exports commonly used types and traits.

This module provides a convenient way to import all the essential types and traits
from the encircuit crate with a single `use encircuit::prelude::*;` statement.
*/

// Core types
pub use crate::{
    BoolCt, Circuit, CircuitBuilder, CircuitComplexity, ClientKeyBytes, EncryptedCircuit, Keyset, Params,
    ServerKeyBytes,
};

// Parameter configuration
pub use crate::params::Scenario;

// Core traits
pub use crate::ciphertext::{Decryptable, Encryptable};

// Re-export common error types
pub use anyhow::{Context, Error, Result};

// Conditional macro re-exports
#[cfg(feature = "macros")]
pub use encircuit_macros::*;
