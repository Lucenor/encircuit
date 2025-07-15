/*!
Tests for circuit building, validation, complexity analysis, and evaluation.
*/

use crate::prelude::*;
use super::fixtures::{TestFixture, utils};

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
fn test_circuit_evaluation_with_constants() {
    let fixture = TestFixture::fast_demo();

    // Build a circuit with constants: input OR true = true (regardless of input)
    let mut builder = CircuitBuilder::default();
    let input = builder.input();
    let const_true = builder.constant(true);
    let output = builder.or(input, const_true);
    let circuit = builder.finish(output);

    // Test with input false -> should be true (false OR true = true)
    let encrypted = circuit
        .encrypt_inputs(&[false], &fixture.client_key)
        .expect("Failed to encrypt inputs");

    let result = encrypted.evaluate(&fixture.server_key);
    let decrypted = result[0]
        .decrypt(&fixture.client_key)
        .expect("Failed to decrypt result");

    assert!(decrypted); // Should be true due to OR with constant true

    // Test with input true -> should be true (true OR true = true)
    let encrypted = circuit
        .encrypt_inputs(&[true], &fixture.client_key)
        .expect("Failed to encrypt inputs");

    let result = encrypted.evaluate(&fixture.server_key);
    let decrypted = result[0]
        .decrypt(&fixture.client_key)
        .expect("Failed to decrypt result");

    assert!(decrypted); // Should be true due to OR with constant true
}

#[test]
fn test_circuit_validation() {
    // Test valid circuit using helper
    let circuit = utils::and_circuit();

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
    let fixture = TestFixture::fast_demo();

    let mut builder = CircuitBuilder::default();
    let x = builder.input();
    let output = builder.not(x);
    let circuit = builder.finish(output);

    let encrypted = circuit.encrypt_inputs(&[true], &fixture.client_key)
        .expect("Failed to encrypt inputs");

    // Test try_evaluate
    let result = encrypted.try_evaluate(&fixture.server_key);
    assert!(result.is_ok());

    // Test timeout evaluation (if parallel feature is enabled)
    #[cfg(feature = "parallel")]
    {
        let result_with_timeout = encrypted.try_evaluate_with_timeout(&fixture.server_key, 5000);
        assert!(result_with_timeout.is_ok());
    }
}

#[test]
fn test_scenario_based_circuit_evaluation() {
    // Test that circuits work correctly with scenario-based parameters
    let fixture = TestFixture::fast_demo();

    // Use the utility function to test XOR truth table
    utils::test_xor_truth_table(&fixture.client_key, &fixture.server_key)
        .expect("XOR truth table test should pass");
}
