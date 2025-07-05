# Contributing to Encircuit

Thank you for your interest in contributing to Encircuit! This document provides guidelines and information for contributors to help maintain code quality and project consistency.

## 🚀 Quick Start

1. **Fork** the repository on GitHub
2. **Clone** your fork locally
3. **Create** a feature branch from `main`
4. **Make** your changes
5. **Test** your changes thoroughly
6. **Submit** a pull request

## 📋 Contribution Checklist

Before submitting any pull request, ensure you've completed all items in this checklist:

### 🔧 Code Quality

- [ ] Run **`cargo fmt --all`** to format all code
- [ ] Run **`cargo clippy --all-targets`** and fix all warnings
- [ ] Ensure no `unsafe` code in `encircuit` crate unless tightly audited
- [ ] Core crate **must not** depend on `syn/quote` (proc-macro deps go in `encircuit_macros`)
- [ ] All public APIs have comprehensive rustdoc comments
- [ ] Code follows Rust naming conventions and best practices

### 🧪 Testing

- [ ] Add unit tests for new functionality
- [ ] Add integration tests where appropriate
- [ ] Mark unimplemented tests with `#[ignore]` and TODO comments
- [ ] Ensure all tests pass: `cargo test --workspace --all-features`
- [ ] Add doctests for public API examples
- [ ] Consider adding property-based tests with `proptest` for complex logic

### 📚 Documentation

- [ ] Update README.md if adding new features
- [ ] Add rustdoc comments for all public items
- [ ] Include code examples in documentation
- [ ] Update CHANGELOG.md (when applicable)

### 🔒 Security & FHE Requirements

- [ ] New ciphertext types implement required traits:
  - `Encryptable` - for encryption from plaintext
  - `Decryptable` - for decryption to plaintext  
  - `GateOps` - if the type supports Boolean operations
- [ ] Add compile-fail tests ensuring no `ClientKey` leaks into server builds
- [ ] Verify that server-side code cannot access client keys
- [ ] Ensure proper separation of client/server responsibilities

## 🏗️ Development Setup

### Prerequisites

- Rust 1.70+ (check with `rustc --version`)
- Git

### Local Development

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/encircuit.git
cd encircuit

# Install development dependencies
cargo build --workspace --all-features

# Run tests
cargo test --workspace --all-features

# Run lints
cargo clippy --all-targets
cargo fmt --all -- --check
```

### Workspace Structure

```
encircuit/
├── Cargo.toml              # Workspace root
├── encircuit/               # Core library (no proc-macro deps!)
│   ├── src/
│   │   ├── lib.rs          # Public API
│   │   ├── prelude.rs      # Re-exports
│   │   ├── params.rs       # FHE parameter configuration
│   │   ├── keys.rs         # Key management
│   │   ├── ciphertext.rs   # Ciphertext types & traits
│   │   ├── circuit/        # Circuit building & evaluation
│   │   └── tests/          # Integration tests
│   └── Cargo.toml
└── encircuit_macros/        # Procedural macros only
    ├── src/lib.rs
    └── Cargo.toml
```

## 📝 Code Style Guidelines

### Rust Conventions

- Use `snake_case` for functions, variables, and modules
- Use `PascalCase` for types, traits, and enum variants
- Use `SCREAMING_SNAKE_CASE` for constants and statics
- Prefer explicit types over `auto` when it improves clarity
- Use `?` operator for error propagation

### Documentation

```rust
/// Brief one-line description.
///
/// Longer description with more details about the function's behavior,
/// edge cases, and usage patterns.
///
/// # Arguments
///
/// * `param1` - Description of the first parameter
/// * `param2` - Description of the second parameter
///
/// # Returns
///
/// Description of the return value and its meaning.
///
/// # Errors
///
/// Describe when and why this function might return an error.
///
/// # Examples
///
/// ```rust
/// use encircuit::prelude::*;
///
/// let result = example_function(42, "test")?;
/// assert_eq!(result, expected_value);
/// ```
pub fn example_function(param1: i32, param2: &str) -> Result<String> {
    // Implementation
}
```

### Error Handling

- Use `anyhow::Result<T>` for functions that can fail
- Use `thiserror` for custom error types when appropriate
- Provide meaningful error messages with context

### Testing Guidelines

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_basic_functionality() {
        // Arrange
        let input = create_test_input();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result.unwrap(), expected_output());
    }

    #[test]
    #[ignore] // Remove when implementation is complete
    fn test_unimplemented_feature() {
        // TODO: Implement when FHE integration is ready
        todo!("Test for future FHE functionality");
    }
}
```

## 🎯 Feature Development

### Adding New Ciphertext Types

When adding new ciphertext types (e.g., `Uint16Ct`, `FloatCt`):

1. **Define the type** in `ciphertext.rs`:

```rust
#[cfg(feature = "integer16")]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Uint16Ct {
    data: Vec<u8>, // Placeholder for TFHE ciphertext
}
```

2. **Implement required traits**:

```rust
impl Encryptable for u16 {
    type Ciphertext = Uint16Ct;
    // ...
}

impl Decryptable for Uint16Ct {
    type Plaintext = u16;
    // ...
}
```

3. **Add feature flag** in `Cargo.toml`:

```toml
[features]
integer16 = []
```

4. **Update exports** in `lib.rs` and `prelude.rs`

5. **Add comprehensive tests**

### Adding New Gate Types

1. **Extend the `Gate` enum** in `circuit/builder.rs`
2. **Add builder methods** for the new gate type
3. **Update evaluation logic** in `circuit/encrypted.rs`
4. **Add tests** for the new functionality

## 🐛 Bug Reports

When reporting bugs, please include:

- **Rust version**: Output of `rustc --version`
- **Encircuit version**: From `Cargo.toml`
- **Platform**: OS and architecture
- **Minimal reproduction case**: Complete code that demonstrates the issue
- **Expected behavior**: What should happen
- **Actual behavior**: What actually happens
- **Error messages**: Full error output with stack traces

## ✨ Feature Requests

For new features:

1. **Check existing issues** to avoid duplicates
2. **Describe the use case** - why is this feature needed?
3. **Propose an API** - how should it work?
4. **Consider alternatives** - are there other ways to solve this?
5. **Discuss implementation** - any technical considerations?

## 🔄 Pull Request Process

1. **Create an issue** first for significant changes
2. **Use descriptive commit messages**:

   ```
   feat: Add support for 16-bit integer ciphertexts
   
   - Implement Uint16Ct type with required traits
   - Add comprehensive test suite
   - Update documentation and examples
   
   Closes #123
   ```

3. **Keep PRs focused** - one feature/fix per PR
4. **Update documentation** as needed
5. **Respond to review feedback** promptly

### Commit Message Format

We follow [Conventional Commits](https://conventionalcommits.org/):

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Adding or updating tests
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `chore:` - Maintenance tasks

## 📚 Learning Resources

### FHE Background

- [TFHE Documentation](https://docs.zama.ai/tfhe-rs)
- [Fully Homomorphic Encryption Overview](https://blog.zama.ai/what-is-homomorphic-encryption/)
- [Boolean Circuit Evaluation](https://en.wikipedia.org/wiki/Boolean_circuit)

### Rust Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)

## 🤝 Community

- **Be respectful** and constructive in all interactions
- **Help others** learn and contribute
- **Ask questions** when you're unsure
- **Share knowledge** and best practices

## 📄 License

By contributing to Encircuit, you agree that your contributions will be licensed under the Apache License 2.0.

## 🙏 Recognition

All contributors will be recognized in our CONTRIBUTORS.md file. Thank you for helping make Encircuit better!

---

Happy hacking! 🦀🔒🛠️
