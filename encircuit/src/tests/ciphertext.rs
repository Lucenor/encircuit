/*!
Tests for ciphertext encryption and decryption functionality.
*/

use crate::prelude::*;
use super::fixtures::{TestFixture, utils};

#[test]
fn test_simple_encryption_decryption() {
    let fixture = TestFixture::fast_demo();

    // Test simple encryption/decryption
    let value = true;
    let encrypted = value.encrypt(&fixture.client_key).expect("Failed to encrypt");
    let decrypted = encrypted.decrypt(&fixture.client_key).expect("Failed to decrypt");

    assert_eq!(value, decrypted);
}

#[test]
fn test_circuit_encryption_and_evaluation() {
    let fixture = TestFixture::fast_demo();

    // Build a simple AND circuit
    // Create AND circuit using helper
    let circuit = utils::and_circuit();

    // Test with inputs [true, false] -> should be false
    let encrypted = circuit
        .encrypt_inputs(&[true, false], &fixture.client_key)
        .expect("Failed to encrypt inputs");

    let result = encrypted.evaluate(&fixture.server_key);
    let decrypted = result[0]
        .decrypt(&fixture.client_key)
        .expect("Failed to decrypt result");

    assert!(!decrypted);
}
