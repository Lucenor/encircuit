/*!
Tests for the encircuit crate.

This module contains comprehensive tests for all the core functionality
of the encircuit FHE toolkit.
*/

#[cfg(test)]
mod unit_tests {
    use super::super::prelude::*;

    #[test]
    fn test_params_builder() {
        let params = Params::for_scenario(Scenario::SafeAndBalanced)
            .expect("Failed to create params");

        assert!(params.supports_boolean());
        assert_eq!(params.scenario(), Scenario::SafeAndBalanced);
    }

    #[test]
    fn test_circuit_builder() {
        let mut builder = CircuitBuilder::default();
        let x = builder.input();
        let y = builder.input();
        let not_y = builder.not(y);
        let and1 = builder.and(not_y, x);
        let and2 = builder.and(x, y);
        let output = builder.or(and1, and2);

        let circuit = builder.finish(output);

        assert_eq!(circuit.input_count(), 2);
        assert_eq!(circuit.gate_count(), 6); // 2 inputs + 1 not + 2 and + 1 or
    }

    #[test]
    fn test_circuit_stats() {
        let mut builder = CircuitBuilder::default();
        let x = builder.input();
        let y = builder.input();
        let const_true = builder.constant(true);
        let not_y = builder.not(y);
        let and_gate = builder.and(x, not_y);
        let or_gate = builder.or(and_gate, const_true);
        let xor_gate = builder.xor(or_gate, y);

        let circuit = builder.finish(xor_gate);
        let stats = circuit.stats();

        assert_eq!(stats.inputs, 2);
        assert_eq!(stats.constants, 1);
        assert_eq!(stats.not_gates, 1);
        assert_eq!(stats.and_gates, 1);
        assert_eq!(stats.or_gates, 1);
        assert_eq!(stats.xor_gates, 1);
        assert_eq!(stats.total_gates, 7);
    }

    #[test]
    fn test_keyset_generation() {
        let params = Params::for_scenario(Scenario::SafeAndBalanced)
            .expect("Failed to create params");

        let _keyset = Keyset::generate(&params).expect("Failed to generate keyset");
    }

    #[test]
    fn test_circuit_encryption_and_evaluation() {
        let params = Params::for_scenario(Scenario::SafeAndBalanced)
            .expect("Failed to create params");

        let keyset = Keyset::generate(&params).expect("Failed to generate keyset");
        let (client_key, server_key) = keyset.split();

        // Build a simple AND circuit
        let mut builder = CircuitBuilder::default();
        let x = builder.input();
        let y = builder.input();
        let output = builder.and(x, y);
        let circuit = builder.finish(output);

        // Test with inputs [true, false] -> should be false
        let encrypted = circuit
            .encrypt_inputs(&[true, false], &client_key)
            .expect("Failed to encrypt inputs");

        let result = encrypted.evaluate(&server_key);
        let decrypted = result[0]
            .decrypt(&client_key)
            .expect("Failed to decrypt result");

        assert!(!decrypted);
    }

    #[test]
    fn test_simple_encryption_decryption() {
        let params = Params::for_scenario(Scenario::SafeAndBalanced)
            .expect("Failed to create params");

        let keyset = Keyset::generate(&params).expect("Failed to generate keyset");
        let (client_key, _server_key) = keyset.split();

        // Test simple encryption/decryption
        let value = true;
        let encrypted = value.encrypt(&client_key).expect("Failed to encrypt");
        let decrypted = encrypted.decrypt(&client_key).expect("Failed to decrypt");

        assert_eq!(value, decrypted);
    }

    #[test]
    fn test_complete_fhe_workflow() {
        // This test demonstrates that all the main components work:
        // 1. Parameter configuration
        // 2. Key generation
        // 3. Circuit building
        // 4. Encryption
        // 5. Decryption

        let params = Params::for_scenario(Scenario::SafeAndBalanced)
            .expect("Failed to create params");

        let keyset = Keyset::generate(&params).expect("Failed to generate keyset");
        let (client_key, server_key) = keyset.split();

        // Build a simple circuit
        let mut builder = CircuitBuilder::default();
        let x = builder.input();
        let circuit = builder.finish(x); // Identity circuit

        // Test the complete workflow
        let inputs = [true];
        let encrypted = circuit
            .encrypt_inputs(&inputs, &client_key)
            .expect("Failed to encrypt inputs");

        // Test evaluation
        let result = encrypted.evaluate(&server_key);
        assert_eq!(result.len(), 1, "Expected one output from evaluation");

        // Test decryption
        let decrypted = result[0]
            .decrypt(&client_key)
            .expect("Failed to decrypt result");
        assert_eq!(decrypted, inputs[0]);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_client_key_serialization_round_trip() {
        use crate::keys::{Keyset, ClientKeyBytes};

        let params = Params::for_scenario(Scenario::SafeAndBalanced)
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
    #[cfg(feature = "serde")]
    fn test_server_key_serialization_round_trip() {
        use crate::keys::{Keyset, ServerKeyBytes};

        let params = Params::for_scenario(Scenario::SafeAndBalanced)
            
            
            
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
    #[cfg(feature = "serde")]
    fn test_keyset_serialization_round_trip() {
        use crate::keys::Keyset;

        let params = Params::for_scenario(Scenario::SafeAndBalanced)
            
            
            
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
    #[cfg(feature = "serde")]
    fn test_key_serialization_error_cases() {
        use crate::keys::{ClientKeyBytes, ServerKeyBytes};

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
    #[cfg(feature = "serde")]
    fn test_key_serialization_size_check() {
        use crate::keys::Keyset;

        let params = Params::for_scenario(Scenario::SafeAndBalanced)
            
            
            
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

    #[test]
    #[cfg(not(feature = "serde"))]
    fn test_key_serialization_without_serde_feature() {
        use crate::keys::Keyset;

        let params = Params::for_scenario(Scenario::SafeAndBalanced)
            
            
            
            .expect("Failed to create params");

        let keyset = Keyset::generate(&params).expect("Failed to generate keyset");
        let (client_key, server_key) = keyset.split();

        // Without serde feature, serialization should fail
        let client_result = client_key.as_bytes();
        assert!(client_result.is_err());

        let server_result = server_key.as_bytes();
        assert!(server_result.is_err());
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
    fn test_circuit_evaluation_with_constants() {
        let params = Params::for_scenario(Scenario::SafeAndBalanced)
            
            
            
            .expect("Failed to create params");

        let keyset = Keyset::generate(&params).expect("Failed to generate keyset");
        let (client_key, server_key) = keyset.split();

        // Build a circuit with constants: input OR true = true (regardless of input)
        let mut builder = CircuitBuilder::default();
        let input = builder.input();
        let const_true = builder.constant(true);
        let output = builder.or(input, const_true);
        let circuit = builder.finish(output);

        // Test with input false -> should be true (false OR true = true)
        let encrypted = circuit
            .encrypt_inputs(&[false], &client_key)
            .expect("Failed to encrypt inputs");

        let result = encrypted.evaluate(&server_key);
        let decrypted = result[0]
            .decrypt(&client_key)
            .expect("Failed to decrypt result");

        assert!(decrypted); // Should be true due to OR with constant true

        // Test with input true -> should be true (true OR true = true)
        let encrypted = circuit
            .encrypt_inputs(&[true], &client_key)
            .expect("Failed to encrypt inputs");

        let result = encrypted.evaluate(&server_key);
        let decrypted = result[0]
            .decrypt(&client_key)
            .expect("Failed to decrypt result");

        assert!(decrypted); // Should be true due to OR with constant true
    }

    #[test]
    fn test_circuit_validation() {
        // Test valid circuit
        let mut builder = CircuitBuilder::default();
        let x = builder.input();
        let y = builder.input();
        let output = builder.and(x, y);
        let circuit = builder.finish(output);

        assert!(circuit.validate().is_ok());
        assert!(!circuit.has_cycles());
    }

    #[test]
    fn test_circuit_complexity() {
        let mut builder = CircuitBuilder::default();
        let x = builder.input();
        let y = builder.input();
        let z = builder.input();
        let and1 = builder.and(x, y);
        let and2 = builder.and(and1, z);
        let output = builder.or(and2, x);
        let circuit = builder.finish(output);

        let complexity = circuit.complexity_estimate();
        assert_eq!(complexity.total_gates, 6); // 3 inputs + 2 ands + 1 or
        assert_eq!(complexity.depth, 3); // longest path: input -> and -> and -> or
        assert_eq!(complexity.boolean_gates, 3); // 2 ands + 1 or
        assert!(complexity.estimated_evaluation_time_ms > 0.0);
        
        assert_eq!(complexity.complexity_rating(), "Trivial");
        assert!(complexity.is_realtime_suitable(1000.0)); // Should be fast enough
    }

    #[test]
    fn test_circuit_error_recovery() {
        let params = Params::for_scenario(Scenario::SafeAndBalanced)
            
            
            
            .expect("Failed to create params");

        let keyset = Keyset::generate(&params).expect("Failed to generate keyset");
        let (client_key, server_key) = keyset.split();

        let mut builder = CircuitBuilder::default();
        let x = builder.input();
        let output = builder.not(x);
        let circuit = builder.finish(output);

        let encrypted = circuit.encrypt_inputs(&[true], &client_key)
            .expect("Failed to encrypt inputs");

        // Test try_evaluate
        let result = encrypted.try_evaluate(&server_key);
        assert!(result.is_ok());

        // Test timeout evaluation (if parallel feature is enabled)
        #[cfg(feature = "parallel")]
        {
            let result_with_timeout = encrypted.try_evaluate_with_timeout(&server_key, 5000);
            assert!(result_with_timeout.is_ok());
        }
    }

    #[test]
    fn test_scenario_based_params() {
        // Test all scenarios can be created
        let scenarios = [
            Scenario::SafeAndBalanced,
            Scenario::DeepCircuitUltraLowError,
            Scenario::FastDemo,
            Scenario::MaxSecurityMinimalOps,
        ];

        for scenario in scenarios {
            let params = Params::for_scenario(scenario)
                .expect("Failed to create params for scenario");
            
            assert!(params.supports_boolean());
            assert_eq!(params.scenario(), scenario);
        }
    }

    #[test]
    fn test_scenario_to_tfhe_params_mapping() {
        use crate::params::{scenario_to_tfhe_params, Scenario};

        // Test that each scenario maps to the expected TFHE parameters
        // We can't compare pointers since they may differ, so we compare parameter properties
        let safe_params = scenario_to_tfhe_params(Scenario::SafeAndBalanced);
        let deep_params = scenario_to_tfhe_params(Scenario::DeepCircuitUltraLowError);
        let fast_params = scenario_to_tfhe_params(Scenario::FastDemo);
        let max_security_params = scenario_to_tfhe_params(Scenario::MaxSecurityMinimalOps);

        // Verify we get different parameter sets for different scenarios
        // These should have different LWE dimensions or polynomial sizes
        assert_ne!(
            (safe_params.lwe_dimension, safe_params.glwe_dimension),
            (deep_params.lwe_dimension, deep_params.glwe_dimension)
        );
        
        // Just verify we get valid parameter objects
        assert!(safe_params.lwe_dimension.0 > 0);
        assert!(deep_params.lwe_dimension.0 > 0);
        assert!(fast_params.lwe_dimension.0 > 0);
        assert!(max_security_params.lwe_dimension.0 > 0);
    }

    #[test]
    fn test_params_direct_creation() {
        let params = Params::for_scenario(Scenario::SafeAndBalanced)
            .expect("Failed to create params");

        assert!(params.supports_boolean());
        assert_eq!(params.scenario(), Scenario::SafeAndBalanced);
    }

    #[test]
    fn test_params_tfhe_boolean_params_with_scenario() {
        let params = Params::for_scenario(Scenario::DeepCircuitUltraLowError)
            .expect("Failed to create params");
        
        let tfhe_params = params.tfhe_boolean_params();
        
        // Verify it returns a valid parameter set
        assert!(tfhe_params.lwe_dimension.0 > 0);
        assert!(tfhe_params.glwe_dimension.0 > 0);
        
        // Verify it's different from the default parameters
        use tfhe::boolean::prelude::*;
        let default_params = &DEFAULT_PARAMETERS;
        
        // The deep circuit parameters should have different characteristics
        // (either different dimensions or different noise parameters)
        let is_different = tfhe_params.lwe_dimension != default_params.lwe_dimension
            || tfhe_params.glwe_dimension != default_params.glwe_dimension
            || tfhe_params.pbs_base_log != default_params.pbs_base_log;
        
        assert!(is_different, "Deep circuit parameters should differ from default");
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

    #[test]
    fn test_scenario_based_circuit_evaluation() {
        // Test that circuits work correctly with scenario-based parameters
        let params = Params::for_scenario(Scenario::SafeAndBalanced)
            .expect("Failed to create params");
        
        let keyset = Keyset::generate(&params)
            .expect("Failed to generate keyset");
        let (client_key, server_key) = keyset.split();

        // Build a simple XOR circuit
        let mut builder = CircuitBuilder::default();
        let x = builder.input();
        let y = builder.input();
        let output = builder.xor(x, y);
        let circuit = builder.finish(output);

        // Test XOR truth table
        let test_cases = [
            ([false, false], false),
            ([false, true], true),
            ([true, false], true),
            ([true, true], false),
        ];

        for (inputs, expected) in test_cases {
            let encrypted = circuit
                .encrypt_inputs(&inputs, &client_key)
                .expect("Failed to encrypt inputs");

            let result = encrypted.evaluate(&server_key);
            let decrypted = result[0]
                .decrypt(&client_key)
                .expect("Failed to decrypt result");

            assert_eq!(decrypted, expected, "XOR({:?}) should be {}", inputs, expected);
        }
    }
}
