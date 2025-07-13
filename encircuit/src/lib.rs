/*!
Zero-overhead Rust toolkit for building, encrypting & evaluating fully homomorphic (FHE) circuits.

# Quick Start

```rust,no_run
use encircuit::prelude::*;

// Generate parameters and keys using scenarios for easy configuration
let params = Params::for_scenario(Scenario::SafeAndBalanced)?;
let keyset = Keyset::generate(&params)?;
let (client_key, server_key) = keyset.split();

// Different scenarios available for different use cases
// let params = Params::for_scenario(Scenario::DeepCircuitUltraLowError)?;
// let params = Params::for_scenario(Scenario::MaxSecurityMinimalOps)?;

// Build a circuit using the builder pattern
let mut builder = CircuitBuilder::default();
let x = builder.input();
let y = builder.input();
let not_y = builder.not(y);
let and1 = builder.and(not_y, x);
let and2 = builder.and(x, y);
let output = builder.or(and1, and2);
let circuit = builder.finish(output);

// Validate circuit before use (recommended)
circuit.validate()?;

// Analyze circuit complexity
let complexity = circuit.complexity_estimate();
println!("Circuit depth: {}, estimated time: {:.2}ms", 
         complexity.depth, complexity.estimated_evaluation_time_ms);

// Encrypt inputs and evaluate
let encrypted = circuit.encrypt_inputs(&[true, false], &client_key)?;
let result = encrypted.try_evaluate(&server_key)?; // Safe evaluation with error handling
let decrypted = result[0].decrypt(&client_key)?;
# Ok::<(), anyhow::Error>(())
```

# Scenario-Based Configuration

The library provides predefined scenarios for common use cases:

- **SafeAndBalanced**: General-purpose circuits with good security/performance balance
- **DeepCircuitUltraLowError**: Deep circuits requiring ultra-low error probability
- **FastDemo**: Quick prototyping with minimal security requirements  
- **MaxSecurityMinimalOps**: Maximum security for minimal operations

```rust,no_run
use encircuit::prelude::*;

// Different scenarios for different needs
let demo_params = Params::for_scenario(Scenario::FastDemo)?;
let production_params = Params::for_scenario(Scenario::SafeAndBalanced)?;
let high_security_params = Params::for_scenario(Scenario::MaxSecurityMinimalOps)?;
# Ok::<(), anyhow::Error>(())
```

# Using the Macro (requires "macros" feature)

```rust,ignore
use encircuit::prelude::*;
use encircuit_macros::circuit;

// Build the same circuit with natural syntax
let circuit = circuit! { |x, y| (!y & x) | (x & y) };

// Validate and analyze
circuit.validate()?;
let complexity = circuit.complexity_estimate();

// Use it the same way with scenario-based parameters
let params = Params::for_scenario(Scenario::SafeAndBalanced)?;
let keyset = Keyset::generate(&params)?;
let (client_key, server_key) = keyset.split();

let encrypted = circuit.encrypt_inputs(&[true, false], &client_key)?;
let result = encrypted.try_evaluate(&server_key)?;
let decrypted = result[0].decrypt(&client_key)?;
# Ok::<(), anyhow::Error>(())
```
*/

pub mod prelude;

mod ciphertext;
mod circuit;
mod keys;
mod params;

#[cfg(test)]
mod tests;

// Public re-exports
pub use ciphertext::BoolCt;
pub use circuit::{Circuit, CircuitBuilder, CircuitComplexity, EncryptedCircuit};
pub use keys::{ClientKeyBytes, Keyset, ServerKeyBytes};
pub use params::{Params, Scenario};

// Conditional compile for macros
#[cfg(feature = "macros")]
pub use encircuit_macros::*;
