/*!
Tests for ciphertext encryption and decryption functionality.
*/

use crate::prelude::*;

#[test]
fn test_simple_encryption_decryption() {
    let params = Params::for_scenario(Scenario::FastDemo)
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
fn test_circuit_encryption_and_evaluation() {
    let params = Params::for_scenario(Scenario::FastDemo)
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
