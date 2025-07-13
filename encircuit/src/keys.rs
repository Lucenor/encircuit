/*!
Key management for FHE operations.

This module provides a unified interface for managing client and server keys,
including generation, serialization, and splitting functionality.

## Scenario-Based Key Generation

The `Keyset::generate` function uses the scenario-based parameter selection from
the `Params` module. This provides an easy way to generate keys optimized for
specific use cases:

```rust,no_run
use encircuit::prelude::*;
# fn main() -> anyhow::Result<()> {

// Generate keys for a balanced general-purpose scenario
let params = Params::for_scenario(Scenario::SafeAndBalanced)?;
let keyset = Keyset::generate(&params)?;

// Or for deep circuits requiring ultra-low error probability
let params = Params::for_scenario(Scenario::DeepCircuitUltraLowError)?; 
let keyset = Keyset::generate(&params)?;
# Ok(())
# }
```

## Parameter Selection Details

The key generation automatically selects appropriate TFHE parameters based on:

1. **Scenario** (if specified): Uses predefined parameter sets optimized for common use cases
2. **Security Level** (fallback): Uses general parameter sets based on security requirements

Available scenarios:
- **SafeAndBalanced**: DEFAULT_PARAMETERS - good security/performance balance
- **DeepCircuitUltraLowError**: PARAMETERS_ERROR_PROB_2_POW_MINUS_165 - ultra-low error probability
- **FastDemo**: DEFAULT_PARAMETERS_KS_PBS - optimized for speed
- **MaxSecurityMinimalOps**: PARAMETERS_ERROR_PROB_2_POW_MINUS_165_KS_PBS - maximum security

## Serialization

Key serialization is supported when the `serde` feature is enabled. The implementation
uses binary serialization via `bincode` with TFHE's built-in serde support.

**Important Notes:**
- TFHE keys provide full serde support that works with bincode
- Serialized keys are stored in compact binary format for efficiency
- Round-trip serialization (serialize -> deserialize) is guaranteed to preserve key functionality
- Keys serialized without the `serde` feature will fail at runtime with appropriate error messages

## Security Considerations

- Client keys should be kept secure and never transmitted over untrusted channels
- Server keys can be safely transmitted to computation servers
- Serialized keys contain the same cryptographic material as the original keys
*/

use crate::params::Params;
use anyhow::Result;

use tfhe::boolean::prelude;

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
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClientKeyBytes {
    key: prelude::ClientKey
}

/// Serialized server key bytes.
///
/// This type wraps the server key in a form that can be easily serialized
/// and transmitted between different parts of an application.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ServerKeyBytes {
    key: prelude::ServerKey
}

impl Keyset {
    /// Generate a new keyset from the given parameters.
    ///
    /// This method uses the scenario-based parameter selection from the `Params`
    /// to choose appropriate TFHE parameters for key generation.
    ///
    /// # Parameters
    ///
    /// * `params` - FHE parameters specifying scenario and supported operations
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Boolean operations are not supported by the provided parameters
    /// - Key generation fails
    pub fn generate(params: &Params) -> Result<Self> {
        // Only proceed if Boolean operations are supported
        if !params.supports_boolean() {
            anyhow::bail!("Boolean operations not supported by the provided parameters");
        }

        // Get TFHE parameters using the scenario-based selection
        let tfhe_params = params.tfhe_boolean_params();
        
        // Generate client key
        let client_key = prelude::ClientKey::new(tfhe_params);
        
        // Derive server key from client key
        let server_key = prelude::ServerKey::new(&client_key);

        Ok(Self {
            client_key: ClientKeyBytes { key: client_key },
            server_key: ServerKeyBytes { key: server_key },
        })
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
    ///
    /// # Security
    ///
    /// This method deserializes a client key from bytes. The input should only
    /// come from trusted sources, as malformed data could cause deserialization
    /// to fail.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The `serde` feature is not enabled
    /// - The input data cannot be deserialized as a valid TFHE ClientKey
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        #[cfg(feature = "serde")]
        {
            // Deserialize using bincode for TFHE keys
            let key: prelude::ClientKey = bincode::deserialize(&data)
                .map_err(|e| anyhow::anyhow!("Failed to deserialize client key: {}", e))?;
            Ok(Self { key })
        }
        #[cfg(not(feature = "serde"))]
        {
            let _ = data; // Suppress unused warning
            anyhow::bail!("Serde feature required for key deserialization")
        }
    }

    /// Get the raw bytes of the client key.
    ///
    /// The returned bytes contain a binary representation of the TFHE ClientKey
    /// that can be stored or transmitted and later reconstructed using
    /// [`ClientKeyBytes::from_bytes`].
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The `serde` feature is not enabled
    /// - Binary serialization fails (very unlikely for valid keys)
    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        #[cfg(feature = "serde")]
        {
            // Serialize using bincode for TFHE keys
            bincode::serialize(&self.key)
                .map_err(|e| anyhow::anyhow!("Failed to serialize client key: {}", e))
        }
        #[cfg(not(feature = "serde"))]
        {
            anyhow::bail!("Serde feature required for key serialization")
        }
    }

    /// Get the TFHE client key for encryption/decryption operations.
    pub fn tfhe_key(&self) -> Result<&prelude::ClientKey> {
        Ok(&self.key)
    }
}

impl ServerKeyBytes {
    /// Create a new server key from raw bytes.
    ///
    /// # Security
    ///
    /// Server keys are safe to transmit over untrusted channels and store on
    /// computation servers, as they do not allow decryption of ciphertexts.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The `serde` feature is not enabled
    /// - The input data cannot be deserialized as a valid TFHE ServerKey
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        #[cfg(feature = "serde")]
        {
            // Deserialize using bincode for TFHE keys
            let key: prelude::ServerKey = bincode::deserialize(&data)
                .map_err(|e| anyhow::anyhow!("Failed to deserialize server key: {}", e))?;
            Ok(Self { key })
        }
        #[cfg(not(feature = "serde"))]
        {
            let _ = data; // Suppress unused warning
            anyhow::bail!("Serde feature required for key deserialization")
        }
    }

    /// Get the raw bytes of the server key.
    ///
    /// The returned bytes contain a binary representation of the TFHE ServerKey
    /// that can be stored or transmitted and later reconstructed using
    /// [`ServerKeyBytes::from_bytes`].
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The `serde` feature is not enabled
    /// - Binary serialization fails (very unlikely for valid keys)
    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        #[cfg(feature = "serde")]
        {
            // Serialize using bincode for TFHE keys
            bincode::serialize(&self.key)
                .map_err(|e| anyhow::anyhow!("Failed to serialize server key: {}", e))
        }
        #[cfg(not(feature = "serde"))]
        {
            anyhow::bail!("Serde feature required for key serialization")
        }
    }

    /// Get the TFHE server key for homomorphic operations.
    pub fn tfhe_key(&self) -> Result<&prelude::ServerKey> {
        Ok(&self.key)
    }
}

impl std::fmt::Debug for ClientKeyBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientKeyBytes")
            .field("encrypted", &"<tfhe_client_key>")
            .finish()
    }
}

impl std::fmt::Debug for ServerKeyBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServerKeyBytes")
            .field("encrypted", &"<tfhe_server_key>")
            .finish()
    }
}
