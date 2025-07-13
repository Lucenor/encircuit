/*!
Integration tests that verify the complete FHE workflow from end to end.
*/

use crate::prelude::*;

#[test]
fn test_complete_fhe_workflow() {
    // This test demonstrates that all the main components work:
    // 1. Parameter configuration
    // 2. Key generation
    // 3. Circuit building
    // 4. Encryption
    // 5. Decryption

    let params = Params::for_scenario(Scenario::FastDemo)
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
