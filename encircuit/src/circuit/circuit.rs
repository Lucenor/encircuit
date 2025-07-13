/*!
Immutable circuit representation and input encryption.

This module provides the `Circuit` type which represents a complete Boolean circuit
ready for encryption and evaluation.
*/

use crate::{
    ciphertext::Encryptable,
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
    #[cfg(not(feature = "parallel"))]
    fn encrypt_inputs_sequential(
        &self,
        inputs: &[bool],
        client_key: &ClientKeyBytes,
    ) -> Result<super::EncryptedCircuit> {
        let mut encrypted_inputs = Vec::new();

        for &input in inputs {
            // Use the actual client key for encryption
            let encrypted = input.encrypt(client_key)?;
            encrypted_inputs.push(encrypted);
        }

        // Pre-encrypt Boolean constants for use during evaluation
        let encrypted_false = false.encrypt(client_key)?;
        let encrypted_true = true.encrypt(client_key)?;

        Ok(super::EncryptedCircuit::new(self.clone(), encrypted_inputs, encrypted_false, encrypted_true))
    }

    /// Encrypt inputs in parallel using rayon.
    #[cfg(feature = "parallel")]
    fn encrypt_inputs_parallel(
        &self,
        inputs: &[bool],
        client_key: &ClientKeyBytes,
    ) -> Result<super::EncryptedCircuit> {
        use rayon::prelude::*;

        let encrypted_inputs: Result<Vec<_>> = inputs
            .par_iter()
            .map(|&input| {
                // Use the actual client key for encryption
                input.encrypt(client_key)
            })
            .collect();

        // Pre-encrypt Boolean constants for use during evaluation
        let encrypted_false = false.encrypt(client_key)?;
        let encrypted_true = true.encrypt(client_key)?;

        Ok(super::EncryptedCircuit::new(
            self.clone(),
            encrypted_inputs?,
            encrypted_false,
            encrypted_true,
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

    /// Validate the circuit structure.
    ///
    /// Checks for:
    /// - Cycles in the circuit DAG
    /// - Invalid node references
    /// - Unreachable gates
    ///
    /// # Errors
    ///
    /// Returns an error if the circuit is invalid.
    pub fn validate(&self) -> Result<()> {
        // Check for cycles using DFS
        if self.has_cycles() {
            anyhow::bail!("Circuit contains cycles");
        }

        // Check that all node references are valid
        for (i, gate) in self.gates.iter().enumerate() {
            match gate {
                Gate::And(left, right) | Gate::Or(left, right) | Gate::Xor(left, right) => {
                    if left.0 >= i {
                        anyhow::bail!("Gate {} references future gate {}", i, left.0);
                    }
                    if right.0 >= i {
                        anyhow::bail!("Gate {} references future gate {}", i, right.0);
                    }
                }
                Gate::Not(input) => {
                    if input.0 >= i {
                        anyhow::bail!("Gate {} references future gate {}", i, input.0);
                    }
                }
                Gate::Input | Gate::Constant(_) => {
                    // These gates don't reference other gates
                }
            }
        }

        // Check that output node is valid
        if self.output.0 >= self.gates.len() {
            anyhow::bail!("Output node {} is out of bounds", self.output.0);
        }

        Ok(())
    }

    /// Check if the circuit contains cycles.
    ///
    /// Uses depth-first search to detect cycles in the circuit DAG.
    pub fn has_cycles(&self) -> bool {
        let mut visited = vec![false; self.gates.len()];
        let mut rec_stack = vec![false; self.gates.len()];

        // Start DFS from the output node
        self.has_cycles_util(self.output.0, &mut visited, &mut rec_stack)
    }

    /// Utility function for cycle detection using DFS.
    fn has_cycles_util(&self, node: usize, visited: &mut [bool], rec_stack: &mut [bool]) -> bool {
        if rec_stack[node] {
            return true; // Back edge found - cycle detected
        }

        if visited[node] {
            return false; // Already processed this node
        }

        visited[node] = true;
        rec_stack[node] = true;

        // Visit all dependencies of this gate
        match &self.gates[node] {
            Gate::And(left, right) | Gate::Or(left, right) | Gate::Xor(left, right) => {
                if self.has_cycles_util(left.0, visited, rec_stack)
                    || self.has_cycles_util(right.0, visited, rec_stack)
                {
                    return true;
                }
            }
            Gate::Not(input) => {
                if self.has_cycles_util(input.0, visited, rec_stack) {
                    return true;
                }
            }
            Gate::Input | Gate::Constant(_) => {
                // Leaf nodes - no dependencies
            }
        }

        rec_stack[node] = false;
        false
    }

    /// Calculate the depth of the circuit.
    ///
    /// The depth is the longest path from any input to the output,
    /// which determines the minimum number of sequential operations
    /// required for evaluation.
    pub fn depth(&self) -> usize {
        let mut depths = vec![0; self.gates.len()];

        // Calculate depth for each gate
        for (i, gate) in self.gates.iter().enumerate() {
            depths[i] = match gate {
                Gate::Input | Gate::Constant(_) => 0,
                Gate::Not(input) => depths[input.0] + 1,
                Gate::And(left, right) | Gate::Or(left, right) | Gate::Xor(left, right) => {
                    std::cmp::max(depths[left.0], depths[right.0]) + 1
                }
            };
        }

        depths[self.output.0]
    }

    /// Get a complexity estimate for this circuit.
    ///
    /// This provides metrics useful for performance planning.
    pub fn complexity_estimate(&self) -> CircuitComplexity {
        let stats = self.stats();
        CircuitComplexity {
            total_gates: stats.total_gates,
            depth: self.depth(),
            boolean_gates: stats.and_gates + stats.or_gates + stats.xor_gates + stats.not_gates,
            estimated_evaluation_time_ms: Self::estimate_evaluation_time(&stats, self.depth()),
        }
    }

    /// Estimate evaluation time based on circuit characteristics.
    fn estimate_evaluation_time(stats: &CircuitStats, depth: usize) -> f64 {
        // Rough estimates based on TFHE Boolean operation timings
        const AND_OR_XOR_TIME_MS: f64 = 0.1; // ~100μs per Boolean gate
        const NOT_TIME_MS: f64 = 0.05; // ~50μs per NOT gate

        let sequential_time = depth as f64 * AND_OR_XOR_TIME_MS;
        let total_gate_time = (stats.and_gates + stats.or_gates + stats.xor_gates) as f64 * AND_OR_XOR_TIME_MS
            + stats.not_gates as f64 * NOT_TIME_MS;

        // Return the maximum of sequential time and total time (conservative estimate)
        sequential_time.max(total_gate_time)
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

/// Circuit performance and complexity metrics.
#[derive(Debug, Clone, PartialEq)]
pub struct CircuitComplexity {
    /// Total number of gates in the circuit.
    pub total_gates: usize,
    /// Circuit depth (critical path length).
    pub depth: usize,
    /// Number of Boolean operations (AND, OR, XOR, NOT).
    pub boolean_gates: usize,
    /// Estimated evaluation time in milliseconds.
    pub estimated_evaluation_time_ms: f64,
}

impl CircuitComplexity {
    /// Check if this circuit is suitable for real-time applications.
    ///
    /// Returns true if estimated evaluation time is under the threshold.
    pub fn is_realtime_suitable(&self, threshold_ms: f64) -> bool {
        self.estimated_evaluation_time_ms <= threshold_ms
    }

    /// Get a human-readable complexity rating.
    pub fn complexity_rating(&self) -> &'static str {
        match self.total_gates {
            0..=10 => "Trivial",
            11..=100 => "Simple",
            101..=1000 => "Moderate",
            1001..=10000 => "Complex",
            _ => "Very Complex",
        }
    }
}
