/*!
Integration tests that verify the complete FHE workflow from end to end.
*/

use crate::prelude::*;
use super::fixtures::{TestFixture, utils};

#[test]
fn test_complete_fhe_workflow() {
    // This test demonstrates that all the main components work:
    // 1. Parameter configuration
    // 2. Key generation
    // 3. Circuit building
    // 4. Encryption
    // 5. Decryption

    let fixture = TestFixture::fast_demo();

    // Build a simple circuit
    let circuit = utils::identity_circuit();

    // Test the complete workflow
    let inputs = [true];
    let encrypted = circuit
        .encrypt_inputs(&inputs, &fixture.client_key)
        .expect("Failed to encrypt inputs");

    // Test evaluation
    let result = encrypted.evaluate(&fixture.server_key);
    assert_eq!(result.len(), 1, "Expected one output from evaluation");

    // Test decryption
    let decrypted = result[0]
        .decrypt(&fixture.client_key)
        .expect("Failed to decrypt result");
    assert_eq!(decrypted, inputs[0]);
}
