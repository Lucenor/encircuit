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
        let params = Params::builder()
            .security_128()
            .boolean_only()
            .build()
            .expect("Failed to build params");

        assert!(params.supports_boolean());
        assert!(!params.supports_integer8());
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
    #[ignore] // TODO: Remove when key generation is implemented
    fn test_keyset_generation() {
        let params = Params::builder()
            .security_128()
            .boolean_only()
            .build()
            .expect("Failed to build params");

        let _keyset = Keyset::generate(&params).expect("Failed to generate keyset");
    }

    #[test]
    #[ignore] // TODO: Remove when encryption is implemented
    fn test_circuit_encryption_and_evaluation() {
        let params = Params::builder()
            .security_128()
            .boolean_only()
            .build()
            .expect("Failed to build params");

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
}
