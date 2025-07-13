/*!
Tests for key generation, key management, and key serialization.
*/

use crate::prelude::*;

#[test]
fn test_keyset_generation() {
    let params = Params::for_scenario(Scenario::FastDemo)
        .expect("Failed to create params");

    let _keyset = Keyset::generate(&params).expect("Failed to generate keyset");
}

#[test]
fn test_keyset_generation_with_different_scenarios() {
    // Test SafeAndBalanced scenario
    let params_safe = Params::for_scenario(Scenario::SafeAndBalanced)
        .expect("Failed to create SafeAndBalanced params");

    let keyset_safe = Keyset::generate(&params_safe).expect("Failed to generate SafeAndBalanced keyset");
    let (client_key_safe, _) = keyset_safe.split();

    // Test MaxSecurityMinimalOps scenario
    let params_max_security = Params::for_scenario(Scenario::MaxSecurityMinimalOps)
        .expect("Failed to create MaxSecurityMinimalOps params");

    let keyset_max_security = Keyset::generate(&params_max_security).expect("Failed to generate MaxSecurityMinimalOps keyset");
    let (client_key_max_security, _) = keyset_max_security.split();

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
    let scenarios = [
        Scenario::SafeAndBalanced,
        Scenario::DeepCircuitUltraLowError,
        Scenario::FastDemo,
        Scenario::MaxSecurityMinimalOps,
    ];

    for scenario in scenarios {
        let params = Params::for_scenario(scenario)
            .expect("Failed to create params for scenario");
        
        let _keyset = Keyset::generate(&params)
            .expect("Failed to generate keyset for scenario");
    }
}

#[cfg(feature = "serde")]
mod serde_tests {
    use super::*;
    use crate::keys::{ClientKeyBytes, ServerKeyBytes};

    #[test]
    fn test_client_key_serialization_round_trip() {
        let params = Params::for_scenario(Scenario::FastDemo)
            .expect("Failed to create params");

        let keyset = Keyset::generate(&params).expect("Failed to generate keyset");
        let (client_key, _) = keyset.split();

        // Test round-trip serialization
        let serialized = client_key.as_bytes().expect("Failed to serialize client key");
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
        let params = Params::for_scenario(Scenario::FastDemo)
            .expect("Failed to create params");

        let keyset = Keyset::generate(&params).expect("Failed to generate keyset");
        let (_, server_key) = keyset.split();

        // Test round-trip serialization
        let serialized = server_key.as_bytes().expect("Failed to serialize server key");
        let deserialized = ServerKeyBytes::from_bytes(serialized)
            .expect("Failed to deserialize server key");

        // Verify the deserialized key has the TFHE key accessible
        let _tfhe_key = deserialized.tfhe_key().expect("Failed to get TFHE key from deserialized server key");
    }

    #[test]
    fn test_keyset_serialization_round_trip() {
        let params = Params::for_scenario(Scenario::FastDemo)
            .expect("Failed to create params");

        let original_keyset = Keyset::generate(&params).expect("Failed to generate keyset");

        // Test serde serialization of the entire keyset
        let serialized = bincode::serialize(&original_keyset)
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
        let params = Params::for_scenario(Scenario::FastDemo)
            .expect("Failed to create params");

        let keyset = Keyset::generate(&params).expect("Failed to generate keyset");
        let (client_key, server_key) = keyset.split();

        // Check that serialization produces non-empty data
        let client_bytes = client_key.as_bytes().expect("Failed to serialize client key");
        let server_bytes = server_key.as_bytes().expect("Failed to serialize server key");

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
        let params = Params::for_scenario(Scenario::FastDemo)
            .expect("Failed to create params");

        let keyset = Keyset::generate(&params).expect("Failed to generate keyset");
        let (client_key, server_key) = keyset.split();

        // Without serde feature, serialization should fail
        let client_result = client_key.as_bytes();
        assert!(client_result.is_err());

        let server_result = server_key.as_bytes();
        assert!(server_result.is_err());
    }
}
