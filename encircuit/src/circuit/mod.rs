/*!
Circuit building and evaluation for FHE.

This module provides the core functionality for building, encrypting, and evaluating
Boolean circuits using fully homomorphic encryption.
*/

pub mod builder;
#[allow(clippy::module_inception)]
pub mod circuit;
pub mod encrypted;

// Re-exports for convenience
pub use builder::{CircuitBuilder, Gate, NodeId};
pub use circuit::{Circuit, CircuitComplexity};
pub use encrypted::EncryptedCircuit;
