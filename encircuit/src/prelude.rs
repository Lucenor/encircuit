/*!
Prelude module that re-exports commonly used types and traits.

This module provides a convenient way to import all the essential types and traits
from the encircuit crate with a single `use encircuit::prelude::*;` statement.
*/

// Core types
pub use crate::{
    BoolCt, Circuit, CircuitBuilder, ClientKeyBytes, EncryptedCircuit, Keyset, Params,
    ServerKeyBytes,
};

// Core traits
pub use crate::ciphertext::{Decryptable, Encryptable, GateOps};

#[cfg(feature = "integer8")]
pub use crate::Uint8Ct;

// Re-export common error types
pub use anyhow::{Context, Error, Result};

// Conditional macro re-exports
#[cfg(feature = "macros")]
pub use encircuit_macros::*;
