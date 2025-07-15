/*!
Tests for key generation, key management, and key serialization.
*/

use crate::prelude::*;
use super::fixtures::TestFixture;

#[test]
fn test_keyset_generation() {
    // This test verifies basic keyset generation works
    let fixture = TestFixture::fast_demo();
    
    // Test that we can use the keys for basic operations
    let value = true;
    let encrypted = value.encrypt(&fixture.client_key).expect("Failed to encrypt");
    let decrypted = encrypted.decrypt(&fixture.client_key).expect("Failed to decrypt");
    assert_eq!(value, decrypted);
}

#[test]
fn test_keyset_generation_with_different_scenarios() {
    // Test SafeAndBalanced scenario using shared fixture
    let fixture_safe = TestFixture::safe_balanced();
    let (client_key_safe, _) = fixture_safe.keyset.clone().split();

    // Test MaxSecurityMinimalOps scenario using shared fixture
    let fixture_max_security = TestFixture::max_security();
    let (client_key_max_security, _) = fixture_max_security.keyset.clone().split();

    // Verify both keysets can encrypt/decrypt (basic functionality test)
    let value = true;

    let encrypted_safe = value.encrypt(&client_key_safe).expect("Failed to encrypt with SafeAndBalanced key");
    let decrypted_safe = encrypted_safe.decrypt(&client_key_safe).expect("Failed to decrypt with SafeAndBalanced key");
    assert_eq!(value, decrypted_safe);

    let encrypted_max_security = value.encrypt(&client_key_max_security).expect("Failed to encrypt with MaxSecurityMinimalOps key");
    let decrypted_max_security = encrypted_max_security.decrypt(&client_key_max_security).expect("Failed to decrypt with MaxSecurityMinimalOps key");
    assert_eq!(value, decrypted_max_security);

    println!("✅ Successfully generated and tested keysets with different scenarios");
    println!("   - SafeAndBalanced: good balance of security and performance");
    println!("   - MaxSecurityMinimalOps: maximum security for simple operations");
}

#[test]
fn test_keyset_generation_with_scenarios() {
    // Test all scenarios using shared fixtures for optimal performance
    let _fixture_fast = TestFixture::fast_demo();
    let _fixture_safe = TestFixture::safe_balanced();
    let _fixture_deep = TestFixture::deep_circuit();
    let _fixture_max_security = TestFixture::max_security();
    
    // All scenarios are now covered by fixtures - no manual generation needed!
}

#[cfg(feature = "serde")]
mod serde_tests {
    use super::*;
    use crate::keys::{ClientKeyBytes, ServerKeyBytes};

    #[test]
    fn test_client_key_serialization_round_trip() {
        let fixture = TestFixture::fast_demo();

        // Test round-trip serialization
        let serialized = fixture.client_key.as_bytes().expect("Failed to serialize client key");
        let deserialized = ClientKeyBytes::from_bytes(serialized)
            .expect("Failed to deserialize client key");

        // Test that the deserialized key can still encrypt/decrypt
        let value = true;
        let encrypted = value.encrypt(&deserialized).expect("Failed to encrypt with deserialized key");
        let decrypted = encrypted.decrypt(&deserialized).expect("Failed to decrypt with deserialized key");

        assert_eq!(value, decrypted);
    }

    #[test]
    fn test_server_key_serialization_round_trip() {
        let fixture = TestFixture::fast_demo();

        // Test round-trip serialization
        let serialized = fixture.server_key.as_bytes().expect("Failed to serialize server key");
        let deserialized = ServerKeyBytes::from_bytes(serialized)
            .expect("Failed to deserialize server key");

        // Verify the deserialized key has the TFHE key accessible
        let _tfhe_key = deserialized.tfhe_key().expect("Failed to get TFHE key from deserialized server key");
    }

    #[test]
    fn test_keyset_serialization_round_trip() {
        let fixture = TestFixture::fast_demo();

        // Test serde serialization of the entire keyset
        let serialized = bincode::serialize(&fixture.keyset)
            .expect("Failed to serialize keyset");
        let deserialized: Keyset = bincode::deserialize(&serialized)
            .expect("Failed to deserialize keyset");

        // Test that both keys in the deserialized keyset work
        let value = false;
        let encrypted = value.encrypt(deserialized.client_key()).expect("Failed to encrypt");
        let decrypted = encrypted.decrypt(deserialized.client_key()).expect("Failed to decrypt");

        assert_eq!(value, decrypted);
    }

    #[test]
    fn test_key_serialization_error_cases() {
        // Test invalid UTF-8 data
        let invalid_utf8 = vec![0xFF, 0xFE, 0xFD];
        let client_result = ClientKeyBytes::from_bytes(invalid_utf8.clone());
        assert!(client_result.is_err());

        let server_result = ServerKeyBytes::from_bytes(invalid_utf8);
        assert!(server_result.is_err());

        // Test invalid binary data
        let invalid_data = b"not valid bincode data".to_vec();
        let client_result = ClientKeyBytes::from_bytes(invalid_data.clone());
        assert!(client_result.is_err());

        let server_result = ServerKeyBytes::from_bytes(invalid_data);
        assert!(server_result.is_err());

        // Test empty data
        let empty_data = vec![];
        let client_result = ClientKeyBytes::from_bytes(empty_data.clone());
        assert!(client_result.is_err());

        let server_result = ServerKeyBytes::from_bytes(empty_data);
        assert!(server_result.is_err());
    }

    #[test]
    fn test_key_serialization_size_check() {
        let fixture = TestFixture::fast_demo();

        // Check that serialization produces non-empty data
        let client_bytes = fixture.client_key.as_bytes().expect("Failed to serialize client key");
        let server_bytes = fixture.server_key.as_bytes().expect("Failed to serialize server key");

        assert!(!client_bytes.is_empty(), "Client key serialization should not be empty");
        assert!(!server_bytes.is_empty(), "Server key serialization should not be empty");

        // Log the sizes for informational purposes
        println!("Client key serialized size: {} bytes", client_bytes.len());
        println!("Server key serialized size: {} bytes", server_bytes.len());
    }
}

#[cfg(not(feature = "serde"))]
mod no_serde_tests {
    use super::*;

    #[test]
    fn test_key_serialization_without_serde_feature() {
        // For this test we still need fresh keys since we're testing error conditions
        let (client_key, server_key) = TestFixture::fresh_keyset(Scenario::FastDemo);

        // Without serde feature, serialization should fail
        let client_result = client_key.as_bytes();
        assert!(client_result.is_err());

        let server_result = server_key.as_bytes();
        assert!(server_result.is_err());
    }
}
