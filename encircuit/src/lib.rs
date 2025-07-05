/*!
Zero-overhead Rust toolkit for building, encrypting & evaluating fully homomorphic (FHE) circuits.

# Quick Start

```rust,no_run
use encircuit::prelude::*;

// Generate parameters and keys
let params = Params::builder().security_128().boolean_only().build()?;
let keyset = Keyset::generate(&params)?;
let (client_key, server_key) = keyset.split();

// Build a circuit
let mut builder = CircuitBuilder::default();
let x = builder.input();
let y = builder.input();
let not_y = builder.not(y);
let and1 = builder.and(not_y, x);
let and2 = builder.and(x, y);
let output = builder.or(and1, and2);
let circuit = builder.finish(output);

// Encrypt inputs and evaluate
let encrypted = circuit.encrypt_inputs(&[true, false], &client_key)?;
let result = encrypted.evaluate(&server_key);
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
#[cfg(feature = "integer8")]
pub use ciphertext::Uint8Ct;
pub use circuit::{Circuit, CircuitBuilder, EncryptedCircuit};
pub use keys::{ClientKeyBytes, Keyset, ServerKeyBytes};
pub use params::Params;

// Conditional compile for macros
#[cfg(feature = "macros")]
pub use encircuit_macros::*;
