/*!
Ciphertext types and operations for FHE.

This module provides wrapper types around TFHE ciphertexts with additional
functionality for encryption, decryption, and circuit operations.

The architecture uses extensible traits (Encryptable/Decryptable) to support
future addition of integer types while maintaining a Boolean-focused core.
*/

use crate::keys::ClientKeyBytes;
use anyhow::Result;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A Boolean ciphertext that can be used in FHE computations.
///
/// `BoolCt` wraps TFHE's Boolean ciphertext type and provides a clean
/// interface for encryption, decryption, and Boolean operations.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BoolCt {
    ciphertext: tfhe::boolean::prelude::Ciphertext,
}

/// Trait for types that can be encrypted.
///
/// This trait provides the extensible foundation for supporting multiple
/// ciphertext types (Boolean, integers, etc.) while maintaining type safety.
pub trait Encryptable {
    /// The ciphertext type produced by encryption.
    type Ciphertext;

    /// Encrypt this value using the given client key.
    fn encrypt(&self, client_key: &ClientKeyBytes) -> Result<Self::Ciphertext>;
}

/// Trait for types that can be decrypted.
///
/// This trait provides the extensible foundation for supporting multiple
/// ciphertext types (Boolean, integers, etc.) while maintaining type safety.
pub trait Decryptable {
    /// The plaintext type produced by decryption.
    type Plaintext;

    /// Decrypt this ciphertext using the given client key.
    fn decrypt(&self, client_key: &ClientKeyBytes) -> Result<Self::Plaintext>;
}

impl BoolCt {
    /// Create a new Boolean ciphertext from raw bytes.
    /// 
    /// This deserializes a ciphertext that was previously serialized with [`to_bytes`].
    /// In production, use `bool::encrypt()` with a client key for new ciphertexts.
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        #[cfg(feature = "serde")]
        {
            // Use bincode to deserialize the TFHE ciphertext directly
            let ciphertext: tfhe::boolean::prelude::Ciphertext = bincode::deserialize(&data)
                .map_err(|e| anyhow::anyhow!("Ciphertext deserialization failed: {}", e))?;
            Ok(Self { ciphertext })
        }
        #[cfg(not(feature = "serde"))]
        {
            let _ = data; // Suppress unused warning
            anyhow::bail!("Deserialization requires 'serde' feature to be enabled")
        }
    }

    /// Serialize the ciphertext to bytes using TFHE's native serialization.
    ///
    /// This uses bincode for efficient binary serialization of TFHE ciphertexts.
    /// The returned bytes can be stored or transmitted and later 
    /// reconstructed using [`BoolCt::from_bytes`].
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        #[cfg(feature = "serde")]
        {
            // Use bincode to serialize the TFHE ciphertext directly
            // TFHE-rs implements Serialize for its ciphertext types
            bincode::serialize(&self.ciphertext)
                .map_err(|e| anyhow::anyhow!("Ciphertext serialization failed: {}", e))
        }
        #[cfg(not(feature = "serde"))]
        {
            anyhow::bail!("Serialization requires 'serde' feature to be enabled")
        }
    }

    /// Create a BoolCt from a TFHE ciphertext.
    pub fn from_tfhe_ciphertext(ciphertext: tfhe::boolean::prelude::Ciphertext) -> Self {
        Self { ciphertext }
    }

    /// Get the underlying TFHE ciphertext.
    pub fn tfhe_ciphertext(&self) -> &tfhe::boolean::prelude::Ciphertext {
        &self.ciphertext
    }

}

impl Encryptable for bool {
    type Ciphertext = BoolCt;

    fn encrypt(&self, client_key: &ClientKeyBytes) -> Result<Self::Ciphertext> {
        let tfhe_key = client_key.tfhe_key()?;
        let ciphertext = tfhe_key.encrypt(*self);
        Ok(BoolCt::from_tfhe_ciphertext(ciphertext))
    }
}

impl Decryptable for BoolCt {
    type Plaintext = bool;

    fn decrypt(&self, client_key: &ClientKeyBytes) -> Result<Self::Plaintext> {
        let tfhe_key = client_key.tfhe_key()?;
        Ok(tfhe_key.decrypt(&self.ciphertext))
    }
}

impl std::fmt::Debug for BoolCt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoolCt")
            .field("encrypted", &"<encrypted_data>")
            .finish()
    }
}
