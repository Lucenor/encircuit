# encircuit_macros

[![Crates.io](https://img.shields.io/crates/v/encircuit_macros.svg)](https://crates.io/crates/encircuit_macros)
[![Documentation](https://docs.rs/encircuit_macros/badge.svg)](https://docs.rs/encircuit_macros)

Procedural macros for the [encircuit](https://crates.io/crates/encircuit) FHE toolkit.

This crate provides compile-time macros for building fully homomorphic encryption (FHE) circuits using natural Rust syntax.

## Features

- **Natural Syntax**: Write Boolean circuits using familiar Rust operators
- **Compile-time Validation**: Catch errors early with descriptive messages
- **Zero Runtime Overhead**: Pure compile-time macro expansion
- **Comprehensive Testing**: 23+ integration tests covering all features

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
encircuit = { version = "0.0.1-alpha.0", features = ["macros"] }
```

Basic usage:

```rust
use encircuit::prelude::*;
use encircuit_macros::circuit;

// Build circuits with natural Boolean syntax
let my_circuit = circuit! { |a, b, c| (a & b) | (!c) };

// Use with the encircuit API
let params = Params::for_scenario(Scenario::SafeAndBalanced).unwrap();
let keyset = Keyset::generate(&params).unwrap();
let (client_key, server_key) = keyset.split();

// The circuit is ready to use for FHE operations
assert_eq!(my_circuit.input_count(), 3);
```

## Supported Operations

The `circuit!` macro supports all essential Boolean operations:

| Operation | Syntax | Description |
|-----------|--------|-------------|
| AND | `a & b` | Logical AND |
| OR | `a \| b` | Logical OR |
| XOR | `a ^ b` | Logical XOR |
| NOT | `!a` | Logical NOT |
| Constants | `true`, `false` | Boolean literals |
| Parentheses | `(a & b) \| c` | Grouping for precedence |

## Examples

### Basic Logic Gates

```rust
use encircuit_macros::circuit;

// Simple gates
let and_gate = circuit! { |a, b| a & b };
let or_gate = circuit! { |a, b| a | b };
let xor_gate = circuit! { |a, b| a ^ b };
let not_gate = circuit! { |a| !a };
```

### Arithmetic Circuits

```rust
// Half adder
let sum = circuit! { |a, b| a ^ b };
let carry = circuit! { |a, b| a & b };

// Full adder
let full_sum = circuit! { |a, b, cin| a ^ b ^ cin };
let full_carry = circuit! { |a, b, cin| (a & b) | (a & cin) | (b & cin) };
```

### Universal Gates

```rust
// NAND gate (universal)
let nand = circuit! { |a, b| !(a & b) };

// NOR gate (universal)  
let nor = circuit! { |a, b| !(a | b) };
```

### Control Circuits

```rust
// 2-to-1 Multiplexer: if select then b else a
let mux = circuit! { |a, b, select| (a & !select) | (b & select) };

// 3-input majority function
let majority = circuit! { |a, b, c| (a & b) | (a & c) | (b & c) };
```

### Complex Expressions

```rust
// Deeply nested Boolean logic
let complex = circuit! { |w, x, y, z| 
    ((w & x) | (y ^ z)) & (!w | !z)
};

// Circuits with constants
let with_const = circuit! { |a| a | true };

// No-input circuits
let always_true = circuit! { || true };
```

## Manual vs Macro Comparison

The macro generates the same efficient code as manual circuit building:

```rust
// Manual approach
let manual = {
    let mut builder = CircuitBuilder::default();
    let a = builder.input();
    let b = builder.input();
    let not_b = builder.not(b);
    let and1 = builder.and(not_b, a);
    let and2 = builder.and(a, b);
    let result = builder.or(and1, and2);
    builder.finish(result)
};

// Macro approach - identical result
let macro_circuit = circuit! { |a, b| (!b & a) | (a & b) };

assert_eq!(manual.input_count(), macro_circuit.input_count());
```

## Error Handling

The macro provides helpful compile-time error messages:

```rust
// ❌ This won't compile
// circuit! { |a| a + b };  // Error: operator + not supported
// circuit! { |a| unknown_var };  // Error: unknown variable 'unknown_var'

// ✅ This will compile  
circuit! { |a, b| a & b };
```

## Integration with Encircuit

Circuits created with the macro integrate seamlessly with the encircuit API:

```rust
use encircuit::prelude::*;
use encircuit_macros::circuit;

let circuit = circuit! { |x, y| (!y & x) | (x & y) };

// Circuit analysis
println!("Inputs: {}", circuit.input_count());
let stats = circuit.stats();
println!("Gates: AND={}, OR={}, NOT={}", 
    stats.and_gates, stats.or_gates, stats.not_gates);

// Ready for FHE operations (when backend is implemented)
// let encrypted = circuit.encrypt_inputs(&[true, false], &client_key);
// let result = encrypted.evaluate(&server_key);
```

## Performance

- **Zero Runtime Cost**: All macro expansion happens at compile time
- **Optimal Code Generation**: Produces the same efficient code as manual building
- **Fast Compilation**: Lightweight macro implementation with minimal dependencies

## Architecture

The macro implementation consists of:

1. **Parser**: Converts closure syntax to AST (`CircuitClosure`)
2. **Builder**: Tracks variables and generates code (`CircuitBuilder`)
3. **Error Handling**: Provides descriptive compile-time errors
4. **Code Generation**: Outputs efficient `CircuitBuilder` calls

## Testing

The crate includes comprehensive testing:

- 23 integration tests covering all operations
- Edge case testing (no inputs, constants only, deeply nested)
- Error message validation
- Performance regression tests

Run tests with:

```bash
cargo test -p encircuit_macros
```

## Compatibility

- **Rust**: 1.70+ (2021 edition)
- **Encircuit**: 0.0.1-alpha.0+
- **Platforms**: All platforms supported by encircuit

## Contributing

1. Follow the [encircuit contribution guidelines](../CONTRIBUTING.md)
2. Run `cargo fmt` and `cargo clippy` before submitting
3. Add tests for new features
4. Update documentation for public APIs

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](../LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Roadmap

- **Current**: Boolean operations with natural syntax
- **Future**: Circuit optimization hints, advanced syntax
- **Long-term**: Integration with circuit visualization and analysis tools

---

Part of the [encircuit](https://github.com/encircuit/encircuit) FHE toolkit. 🦀🔒
