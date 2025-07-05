# Encircuit

> Zero-overhead Rust toolkit for building, encrypting & evaluating fully homomorphic (FHE) circuits

[![Crates.io](https://img.shields.io/crates/v/encircuit.svg)](https://crates.io/crates/encircuit)
[![Documentation](https://docs.rs/encircuit/badge.svg)](https://docs.rs/encircuit)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

## Overview

Encircuit is a high-level Rust library for building and evaluating Boolean circuits using fully homomorphic encryption (FHE). It provides a clean, ergonomic API for constructing circuits, encrypting inputs, and performing computations on encrypted data without ever decrypting intermediate values.

## Features

- **🛠️ Circuit Builder**: Intuitive API for constructing Boolean circuits
- **🔒 FHE Integration**: Built on top of the TFHE library for secure computation
- **⚡ Zero-Overhead**: Efficient circuit representation and evaluation
- **🔧 Flexible Parameters**: Configurable security levels and operation types
- **📦 Modular Design**: Optional features for different use cases
- **🎯 Type Safety**: Rust's type system ensures correctness at compile time

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
encircuit = "0.0.1-alpha.0"
```

### Basic Usage

```rust,no_run
use encircuit::prelude::*;

// Generate parameters and keys
let params = Params::builder().security_128().boolean_only().build()?;
let keyset = Keyset::generate(&params)?;
let (client_key, server_key) = keyset.split();

// Build a circuit: (¬y ∧ x) ∨ (x ∧ y)
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
assert_eq!(decrypted, true); // (¬false ∧ true) ∨ (true ∧ false) = true
```

## Workspace Structure

```
encircuit/
├── Cargo.toml              # Workspace root
├── encircuit/               # Core library
│   ├── src/
│   │   ├── lib.rs          # Public API
│   │   ├── prelude.rs      # Convenient imports
│   │   ├── params.rs       # Parameter configuration
│   │   ├── keys.rs         # Key management
│   │   ├── ciphertext.rs   # Ciphertext types
│   │   └── circuit/        # Circuit building & evaluation
│   └── Cargo.toml
└── encircuit_macros/        # Procedural macros (optional)
    ├── src/lib.rs
    └── Cargo.toml
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `boolean` | ✅ | Boolean FHE operations |
| `integer8` | ❌ | 8-bit integer operations |
| `parallel` | ✅ | Parallel encryption using Rayon |
| `serde` | ✅ | Serialization support |
| `macros` | ❌ | Procedural macro support |

## Current Status

🚧 **Alpha Version** - This library is currently in early development.

**Implemented:**

- ✅ Circuit builder API
- ✅ Type-safe circuit representation
- ✅ Parameter configuration
- ✅ Key management structures
- ✅ Ciphertext abstractions
- ✅ Comprehensive test suite

**TODO:**

- 🔄 Actual TFHE integration (currently uses placeholders)
- 🔄 `circuit!` procedural macro implementation
- 🔄 Performance optimizations
- 🔄 Extended integer support
- 🔄 Documentation improvements

## Contributing

Contributions are welcome! Please see our [contribution guidelines](CONTRIBUTING.md) for details.

1. Run `rustfmt` and `clippy --all-targets`
2. Ensure no `unsafe` code without thorough review
3. Add tests for new functionality
4. Update documentation

## License

Licensed under the Apache License, Version 2.0 ([LICENSE](LICENSE) or <http://www.apache.org/licenses/LICENSE-2.0>)

## Roadmap

| Milestone | Focus | Goal |
|-----------|-------|------|
| **M1** | Core FHE | Boolean TFHE integration (< 100 µs circuits) |
| **M2** | Macros | `circuit!` procedural macro |
| **M3** | DSL | Text/JSON → Circuit parser |
| **M4** | Runtime | gRPC/HTTP circuit evaluation service |
| **M5** | Extended Types | 8-bit and 32-bit integer support |
| **M6** | Performance | GPU acceleration |
