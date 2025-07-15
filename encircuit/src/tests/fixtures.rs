/*!
Shared test fixtures and utilities for optimized test performance.

This module provides reusable test fixtures to avoid redundant key generation
and parameter setup across multiple tests.
*/

use crate::prelude::*;
use crate::keys::{ClientKeyBytes, ServerKeyBytes};
use std::sync::OnceLock;

/// Global test fixture for FastDemo scenario keys - generated once and reused
static FAST_DEMO_FIXTURE: OnceLock<TestFixture> = OnceLock::new();

/// Global test fixture for SafeAndBalanced scenario keys
static SAFE_BALANCED_FIXTURE: OnceLock<TestFixture> = OnceLock::new();

/// Global test fixture for DeepCircuitUltraLowError scenario keys
static DEEP_CIRCUIT_FIXTURE: OnceLock<TestFixture> = OnceLock::new();

/// Global test fixture for MaxSecurityMinimalOps scenario keys
static MAX_SECURITY_FIXTURE: OnceLock<TestFixture> = OnceLock::new();

/// Test fixture containing commonly used test objects
#[derive(Clone)]
pub struct TestFixture {
    #[allow(dead_code)] // Available for parameter-specific testing in future tests
    pub params: Params,
    pub keyset: Keyset,
    pub client_key: ClientKeyBytes,
    pub server_key: ServerKeyBytes,
}

impl TestFixture {
    /// Create a new test fixture for the given scenario
    fn new(scenario: Scenario) -> Self {
        let params = Params::for_scenario(scenario)
            .expect("Failed to create params for test fixture");
        let keyset = Keyset::generate(&params)
            .expect("Failed to generate keyset for test fixture");
        let (client_key, server_key) = keyset.clone().split();
        
        Self {
            params,
            keyset,
            client_key,
            server_key,
        }
    }
    
    /// Get a reference to the FastDemo test fixture (created once, reused)
    pub fn fast_demo() -> &'static TestFixture {
        FAST_DEMO_FIXTURE.get_or_init(|| Self::new(Scenario::FastDemo))
    }
    
    /// Get a reference to the SafeAndBalanced test fixture (created once, reused)
    pub fn safe_balanced() -> &'static TestFixture {
        SAFE_BALANCED_FIXTURE.get_or_init(|| Self::new(Scenario::SafeAndBalanced))
    }
    
    /// Get a reference to the DeepCircuitUltraLowError test fixture (created once, reused)
    pub fn deep_circuit() -> &'static TestFixture {
        DEEP_CIRCUIT_FIXTURE.get_or_init(|| Self::new(Scenario::DeepCircuitUltraLowError))
    }
    
    /// Get a reference to the MaxSecurityMinimalOps test fixture (created once, reused)
    pub fn max_security() -> &'static TestFixture {
        MAX_SECURITY_FIXTURE.get_or_init(|| Self::new(Scenario::MaxSecurityMinimalOps))
    }
    
    /// Create a fresh keyset for tests that need isolation
    #[allow(dead_code)] // Used in #[cfg(not(feature = "serde"))] tests
    pub fn fresh_keyset(scenario: Scenario) -> (ClientKeyBytes, ServerKeyBytes) {
        let params = Params::for_scenario(scenario)
            .expect("Failed to create params");
        let keyset = Keyset::generate(&params)
            .expect("Failed to generate keyset");
        keyset.split()
    }
}

/// Test utility functions
pub mod utils {
    use super::*;
    
    /// Create a simple identity circuit for testing
    pub fn identity_circuit() -> crate::circuit::Circuit {
        let mut builder = crate::circuit::CircuitBuilder::default();
        let input = builder.input();
        builder.finish(input)
    }
    
    /// Create a simple AND circuit for testing
    pub fn and_circuit() -> crate::circuit::Circuit {
        let mut builder = crate::circuit::CircuitBuilder::default();
        let x = builder.input();
        let y = builder.input();
        let output = builder.and(x, y);
        builder.finish(output)
    }
    
    /// Create a simple XOR circuit for testing
    pub fn xor_circuit() -> crate::circuit::Circuit {
        let mut builder = crate::circuit::CircuitBuilder::default();
        let x = builder.input();
        let y = builder.input();
        let output = builder.xor(x, y);
        builder.finish(output)
    }
    
    /// Test XOR truth table with given client/server keys
    pub fn test_xor_truth_table(
        client_key: &ClientKeyBytes,
        server_key: &ServerKeyBytes
    ) -> anyhow::Result<()> {
        let circuit = xor_circuit();
        
        let test_cases = [
            ([false, false], false),
            ([false, true], true),
            ([true, false], true),
            ([true, true], false),
        ];

        for (inputs, expected) in test_cases {
            let encrypted = circuit
                .encrypt_inputs(&inputs, client_key)?;
            let result = encrypted.evaluate(server_key);
            let decrypted = result[0].decrypt(client_key)?;
            
            assert_eq!(decrypted, expected, "XOR({:?}) should be {}", inputs, expected);
        }
        
        Ok(())
    }
}
