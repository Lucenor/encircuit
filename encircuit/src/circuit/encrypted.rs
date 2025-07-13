/*!
Encrypted circuit evaluation.

This module provides the `EncryptedCircuit` type which represents a circuit
with encrypted inputs, ready for homomorphic evaluation.

The `EncryptedCircuit` pre-encrypts Boolean constants (`true` and `false`) 
during construction to ensure they are available during server-side evaluation,
where the client key is not accessible.
*/

use crate::{
    ciphertext::BoolCt,
    circuit::{Circuit, Gate},
    keys::ServerKeyBytes,
};
use anyhow::Result;

use tfhe::boolean::prelude::*;

/// A circuit with encrypted inputs, ready for homomorphic evaluation.
///
/// The `EncryptedCircuit` contains the circuit structure along with encrypted
/// inputs and pre-encrypted Boolean constants for evaluation.
#[derive(Debug, Clone)]
pub struct EncryptedCircuit {
    circuit: Circuit,
    encrypted_inputs: Vec<BoolCt>,
    encrypted_false: BoolCt,
    encrypted_true: BoolCt,
}

impl EncryptedCircuit {
    /// Create a new encrypted circuit.
    ///
    /// This is typically called by `Circuit::encrypt_inputs()`.
    pub(super) fn new(circuit: Circuit, encrypted_inputs: Vec<BoolCt>, encrypted_false: BoolCt, encrypted_true: BoolCt) -> Self {
        Self {
            circuit,
            encrypted_inputs,
            encrypted_false,
            encrypted_true,
        }
    }

    /// Evaluate the circuit homomorphically using the server key.
    ///
    /// This performs the computation on encrypted data without ever decrypting
    /// the inputs or intermediate values.
    ///
    /// Returns a vector containing the encrypted outputs of the circuit.
    pub fn evaluate(&self, server_key: &ServerKeyBytes) -> Vec<BoolCt> {
        let tfhe_server_key = server_key.tfhe_key().expect("Failed to get TFHE server key");
        self.evaluate_with_tfhe_key(tfhe_server_key).expect("Evaluation failed")
    }

    /// Evaluate the circuit homomorphically with error recovery.
    ///
    /// This is a safer version of `evaluate` that returns detailed error information
    /// instead of panicking on evaluation failures.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The server key is invalid
    /// - Circuit evaluation fails
    /// - Memory allocation fails during evaluation
    pub fn try_evaluate(&self, server_key: &ServerKeyBytes) -> Result<Vec<BoolCt>> {
        let tfhe_server_key = server_key.tfhe_key()
            .map_err(|e| anyhow::anyhow!("Invalid server key: {}", e))?;
        
        self.evaluate_with_tfhe_key(tfhe_server_key)
            .map_err(|e| anyhow::anyhow!("Circuit evaluation failed: {}", e))
    }

    /// Evaluate with timeout protection.
    ///
    /// Attempts to evaluate the circuit with a timeout to prevent indefinite blocking.
    /// 
    /// # Arguments
    /// 
    /// * `server_key` - The server key for homomorphic operations
    /// * `timeout_ms` - Maximum time to wait for evaluation in milliseconds
    ///
    /// # Errors
    ///
    /// Returns an error if evaluation times out or fails.
    #[cfg(feature = "parallel")]
    pub fn try_evaluate_with_timeout(&self, server_key: &ServerKeyBytes, timeout_ms: u64) -> Result<Vec<BoolCt>> {
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;

        let (tx, rx) = mpsc::channel();
        let server_key_clone = server_key.clone();
        let self_clone = self.clone();

        // Spawn evaluation in a separate thread
        thread::spawn(move || {
            let result = self_clone.try_evaluate(&server_key_clone);
            let _ = tx.send(result);
        });

        // Wait for result with timeout
        match rx.recv_timeout(Duration::from_millis(timeout_ms)) {
            Ok(result) => result,
            Err(_) => anyhow::bail!("Circuit evaluation timed out after {}ms", timeout_ms),
        }
    }

    fn evaluate_with_tfhe_key(&self, server_key: &tfhe::boolean::prelude::ServerKey) -> anyhow::Result<Vec<BoolCt>> {
        // Use NodeId directly as index into gates - no mapping needed
        let mut gate_results: Vec<Option<BoolCt>> = vec![None; self.circuit.gates().len()];
        let mut input_index = 0;

        // Walk through gates in topological order
        for (gate_index, gate) in self.circuit.gates().iter().enumerate() {
            let result = match gate {
                Gate::Input => {
                    // Use the encrypted input
                    let encrypted_input = self.encrypted_inputs[input_index].clone();
                    input_index += 1;
                    encrypted_input
                }
                Gate::Constant(value) => {
                    // Use the pre-encrypted constants
                    if *value {
                        self.encrypted_true.clone()
                    } else {
                        self.encrypted_false.clone()
                    }
                }
                Gate::And(left, right) => {
                    let left_val = gate_results[left.0]
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("Left input gate {} not yet computed", left.0))?;
                    let right_val = gate_results[right.0]
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("Right input gate {} not yet computed", right.0))?;
                    
                    self.homomorphic_and(left_val, right_val, server_key)
                }
                Gate::Or(left, right) => {
                    let left_val = gate_results[left.0]
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("Left input gate {} not yet computed", left.0))?;
                    let right_val = gate_results[right.0]
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("Right input gate {} not yet computed", right.0))?;
                    
                    self.homomorphic_or(left_val, right_val, server_key)
                }
                Gate::Xor(left, right) => {
                    let left_val = gate_results[left.0]
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("Left input gate {} not yet computed", left.0))?;
                    let right_val = gate_results[right.0]
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("Right input gate {} not yet computed", right.0))?;
                    
                    self.homomorphic_xor(left_val, right_val, server_key)
                }
                Gate::Not(input) => {
                    let input_val = gate_results[input.0]
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("Input gate {} not yet computed", input.0))?;
                    
                    self.homomorphic_not(input_val, server_key)
                }
            };

            gate_results[gate_index] = Some(result);
        }

        // Return the output value(s)
        let output_result = gate_results[self.circuit.output().0]
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Output gate {} not computed", self.circuit.output().0))?;
        
        Ok(vec![output_result.clone()])
    }

    /// Perform homomorphic AND operation.
    fn homomorphic_and(&self, left: &BoolCt, right: &BoolCt, server_key: &tfhe::boolean::prelude::ServerKey) -> BoolCt {
        let result = server_key.and(left.tfhe_ciphertext(), right.tfhe_ciphertext());
        BoolCt::from_tfhe_ciphertext(result)
    }

    /// Perform homomorphic OR operation.
    fn homomorphic_or(&self, left: &BoolCt, right: &BoolCt, server_key: &tfhe::boolean::prelude::ServerKey) -> BoolCt {
        let result = server_key.or(left.tfhe_ciphertext(), right.tfhe_ciphertext());
        BoolCt::from_tfhe_ciphertext(result)
    }

    /// Perform homomorphic XOR operation.
    fn homomorphic_xor(&self, left: &BoolCt, right: &BoolCt, server_key: &tfhe::boolean::prelude::ServerKey) -> BoolCt {
        let result = server_key.xor(left.tfhe_ciphertext(), right.tfhe_ciphertext());
        BoolCt::from_tfhe_ciphertext(result)
    }

    /// Perform homomorphic NOT operation.
    fn homomorphic_not(&self, input: &BoolCt, server_key: &tfhe::boolean::prelude::ServerKey) -> BoolCt {
        let result = server_key.not(input.tfhe_ciphertext());
        BoolCt::from_tfhe_ciphertext(result)
    }

    /// Get the underlying circuit.
    pub fn circuit(&self) -> &Circuit {
        &self.circuit
    }

    /// Get the encrypted inputs.
    pub fn encrypted_inputs(&self) -> &[BoolCt] {
        &self.encrypted_inputs
    }

    /// Get the number of encrypted inputs.
    pub fn input_count(&self) -> usize {
        self.encrypted_inputs.len()
    }

    /// Get a reference to the encrypted false constant.
    pub fn encrypted_false(&self) -> &BoolCt {
        &self.encrypted_false
    }

    /// Get a reference to the encrypted true constant.
    pub fn encrypted_true(&self) -> &BoolCt {
        &self.encrypted_true
    }

    /// Get references to both encrypted constants as a tuple (false, true).
    pub fn encrypted_constants(&self) -> (&BoolCt, &BoolCt) {
        (&self.encrypted_false, &self.encrypted_true)
    }
}
