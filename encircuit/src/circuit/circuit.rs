/*!
Immutable circuit representation and input encryption.

This module provides the `Circuit` type which represents a complete Boolean circuit
ready for encryption and evaluation.
*/

use crate::{
    ciphertext::BoolCt,
    circuit::{Gate, NodeId},
    keys::ClientKeyBytes,
};
use anyhow::Result;

/// An immutable Boolean circuit represented as a directed acyclic graph (DAG).
///
/// The `Circuit` contains all gates in topological order and provides methods
/// for encrypting inputs and preparing for evaluation.
#[derive(Debug, Clone)]
pub struct Circuit {
    gates: Vec<Gate>,
    output: NodeId,
}

impl Circuit {
    /// Create a new circuit from gates and output node.
    ///
    /// This is typically called by `CircuitBuilder::finish()`.
    pub(super) fn new(gates: Vec<Gate>, output: NodeId) -> Self {
        Self { gates, output }
    }

    /// Encrypt the circuit inputs using the given client key.
    ///
    /// The `inputs` slice must contain exactly the same number of values
    /// as there are input gates in the circuit.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The number of inputs doesn't match the number of input gates
    /// - Encryption fails
    pub fn encrypt_inputs(
        &self,
        inputs: &[bool],
        client_key: &ClientKeyBytes,
    ) -> Result<super::EncryptedCircuit> {
        let input_count = self.count_inputs();
        if inputs.len() != input_count {
            anyhow::bail!(
                "Input count mismatch: expected {}, got {}",
                input_count,
                inputs.len()
            );
        }

        #[cfg(feature = "parallel")]
        {
            self.encrypt_inputs_parallel(inputs, client_key)
        }
        #[cfg(not(feature = "parallel"))]
        {
            self.encrypt_inputs_sequential(inputs, client_key)
        }
    }

    /// Encrypt inputs sequentially.
    fn encrypt_inputs_sequential(
        &self,
        inputs: &[bool],
        _client_key: &ClientKeyBytes,
    ) -> Result<super::EncryptedCircuit> {
        let mut encrypted_inputs = Vec::new();

        for &input in inputs {
            // TODO: Implement actual encryption
            let encrypted = BoolCt::from_bytes(vec![if input { 1 } else { 0 }]);
            encrypted_inputs.push(encrypted);
        }

        Ok(super::EncryptedCircuit::new(self.clone(), encrypted_inputs))
    }

    /// Encrypt inputs in parallel using rayon.
    #[cfg(feature = "parallel")]
    fn encrypt_inputs_parallel(
        &self,
        inputs: &[bool],
        _client_key: &ClientKeyBytes,
    ) -> Result<super::EncryptedCircuit> {
        use rayon::prelude::*;

        let encrypted_inputs: Result<Vec<_>> = inputs
            .par_iter()
            .map(|&input| {
                // TODO: Implement actual encryption
                Ok(BoolCt::from_bytes(vec![if input { 1 } else { 0 }]))
            })
            .collect();

        Ok(super::EncryptedCircuit::new(
            self.clone(),
            encrypted_inputs?,
        ))
    }

    /// Count the number of input gates in the circuit.
    fn count_inputs(&self) -> usize {
        self.gates
            .iter()
            .filter(|gate| matches!(gate, Gate::Input))
            .count()
    }

    /// Get the gates in this circuit.
    pub(super) fn gates(&self) -> &[Gate] {
        &self.gates
    }

    /// Get the output node of this circuit.
    pub(super) fn output(&self) -> NodeId {
        self.output
    }

    /// Get the total number of gates in the circuit.
    pub fn gate_count(&self) -> usize {
        self.gates.len()
    }

    /// Get the number of input gates in the circuit.
    pub fn input_count(&self) -> usize {
        self.count_inputs()
    }

    /// Get statistics about the circuit.
    pub fn stats(&self) -> CircuitStats {
        let mut stats = CircuitStats::default();

        for gate in &self.gates {
            match gate {
                Gate::Input => stats.inputs += 1,
                Gate::Constant(_) => stats.constants += 1,
                Gate::And(_, _) => stats.and_gates += 1,
                Gate::Or(_, _) => stats.or_gates += 1,
                Gate::Xor(_, _) => stats.xor_gates += 1,
                Gate::Not(_) => stats.not_gates += 1,
            }
        }

        stats.total_gates = self.gates.len();
        stats
    }
}

/// Statistics about a circuit's composition.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CircuitStats {
    /// Number of input gates
    pub inputs: usize,
    /// Number of constant gates
    pub constants: usize,
    /// Number of AND gates
    pub and_gates: usize,
    /// Number of OR gates
    pub or_gates: usize,
    /// Number of XOR gates
    pub xor_gates: usize,
    /// Number of NOT gates
    pub not_gates: usize,
    /// Total number of gates
    pub total_gates: usize,
}
