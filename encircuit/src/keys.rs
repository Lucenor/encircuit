/*!
Key management for FHE operations.

This module provides a unified interface for managing client and server keys,
including generation, serialization, and splitting functionality.
*/

use crate::params::Params;
use anyhow::Result;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A complete set of FHE keys containing both client and server keys.
/// 
/// The `Keyset` provides a convenient way to manage both the client key
/// (used for encryption/decryption) and server key (used for computation).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Keyset {
    client_key: ClientKeyBytes,
    server_key: ServerKeyBytes,
}

/// Serialized client key bytes.
/// 
/// This type wraps the client key in a form that can be easily serialized
/// and transmitted between different parts of an application.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClientKeyBytes {
    data: Vec<u8>,
}

/// Serialized server key bytes.
/// 
/// This type wraps the server key in a form that can be easily serialized
/// and transmitted between different parts of an application.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ServerKeyBytes {
    data: Vec<u8>,
}

impl Keyset {
    /// Generate a new keyset from the given parameters.
    /// 
    /// # Errors
    /// 
    /// Returns an error if key generation fails.
    pub fn generate(_params: &Params) -> Result<Self> {
        // TODO: Implement actual key generation using TFHE
        todo!("Key generation not yet implemented")
    }

    /// Split the keyset into separate client and server keys.
    /// 
    /// Returns a tuple of (client_key, server_key).
    pub fn split(self) -> (ClientKeyBytes, ServerKeyBytes) {
        (self.client_key, self.server_key)
    }

    /// Get a reference to the client key.
    pub fn client_key(&self) -> &ClientKeyBytes {
        &self.client_key
    }

    /// Get a reference to the server key.
    pub fn server_key(&self) -> &ServerKeyBytes {
        &self.server_key
    }
}

impl ClientKeyBytes {
    /// Create a new client key from raw bytes.
    pub fn from_bytes(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Get the raw bytes of the client key.
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

impl ServerKeyBytes {
    /// Create a new server key from raw bytes.
    pub fn from_bytes(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Get the raw bytes of the server key.
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}
