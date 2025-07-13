/*!
Procedural macros for the encircuit FHE toolkit.

This crate provides compile-time macros for building FHE circuits with natural Rust syntax.

# Example

```rust
use encircuit_macros::circuit;

// Build a circuit with Boolean logic
let my_circuit = circuit! { |a, b, c| (a & b) | (!c) };

// The circuit can be used with the encircuit API
assert_eq!(my_circuit.input_count(), 3);
```
*/

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::collections::HashMap;
use syn::{
    BinOp, Expr, ExprBinary, ExprClosure, ExprParen, ExprUnary, Ident, Pat, PatIdent, PatType,
    Result, UnOp,
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::{And, Not, Or},
};

/// Error messages for better diagnostics
mod error_messages {
    pub const UNSUPPORTED_PATTERN: &str = "Only simple identifiers are supported as circuit inputs. Try using patterns like |a, b| instead of |a: bool, b: bool| or destructuring patterns.";
    pub const UNKNOWN_VARIABLE: &str =
        "Unknown variable. Make sure all variables are declared as circuit inputs.";
    pub const UNSUPPORTED_OPERATOR: &str = "Only &, |, ^, and ! operators are supported in circuits. Use & for AND, | for OR, ^ for XOR, and ! for NOT.";
    pub const UNSUPPORTED_UNARY: &str =
        "Only ! (NOT) operator is supported for unary operations in circuits.";
    pub const UNSUPPORTED_LITERAL: &str =
        "Only boolean literals (true, false) are supported in circuits.";
    pub const UNSUPPORTED_EXPRESSION: &str = "Unsupported expression type. Circuits support: Boolean operators (&, |, ^, !), parentheses, boolean literals (true/false), and input variables.";
}

/// Parsed circuit closure: |a, b| expr
struct CircuitClosure {
    inputs: Vec<Ident>,
    body: Expr,
}

impl Parse for CircuitClosure {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse the closure: |a, b| expr
        let closure: ExprClosure = input.parse()?;

        // Extract input parameter names
        let mut inputs = Vec::new();
        for input_pat in &closure.inputs {
            match input_pat {
                Pat::Ident(PatIdent { ident, .. }) => {
                    inputs.push(ident.clone());
                }
                Pat::Type(PatType { pat, .. }) => {
                    if let Pat::Ident(PatIdent { ident, .. }) = pat.as_ref() {
                        inputs.push(ident.clone());
                    } else {
                        return Err(syn::Error::new_spanned(
                            input_pat,
                            error_messages::UNSUPPORTED_PATTERN,
                        ));
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        input_pat,
                        error_messages::UNSUPPORTED_PATTERN,
                    ));
                }
            }
        }

        Ok(CircuitClosure {
            inputs,
            body: *closure.body,
        })
    }
}

/// Context for building the circuit, tracking variable assignments
struct CircuitBuilder {
    statements: Vec<TokenStream2>,
    next_var_id: usize,
}

impl CircuitBuilder {
    fn new() -> Self {
        Self {
            statements: Vec::new(),
            next_var_id: 0,
        }
    }

    /// Generate a unique variable name
    fn next_var(&mut self) -> Ident {
        let var_name = format!("__var_{}", self.next_var_id);
        self.next_var_id += 1;
        Ident::new(&var_name, proc_macro2::Span::call_site())
    }

    /// Add an input variable
    fn add_input(&mut self, _input_name: &Ident) -> Ident {
        let var = self.next_var();
        let stmt = quote! {
            let #var = __builder.input();
        };
        self.statements.push(stmt);
        var
    }

    /// Convert an expression to circuit building code
    fn build_expr(&mut self, expr: &Expr, input_map: &HashMap<String, Ident>) -> Result<Ident> {
        match expr {
            // Simple identifier (input variable)
            Expr::Path(path) if path.path.segments.len() == 1 => {
                let ident = &path.path.segments[0].ident;
                if let Some(var) = input_map.get(&ident.to_string()) {
                    Ok(var.clone())
                } else {
                    Err(syn::Error::new_spanned(
                        ident,
                        error_messages::UNKNOWN_VARIABLE,
                    ))
                }
            }

            // Binary operations: &, |, ^
            Expr::Binary(ExprBinary {
                left, op, right, ..
            }) => {
                let left_var = self.build_expr(left, input_map)?;
                let right_var = self.build_expr(right, input_map)?;
                let result_var = self.next_var();

                let method = match op {
                    BinOp::BitAnd(And { .. }) => quote! { and },
                    BinOp::BitOr(Or { .. }) => quote! { or },
                    BinOp::BitXor(_) => quote! { xor },
                    _ => {
                        return Err(syn::Error::new_spanned(
                            op,
                            error_messages::UNSUPPORTED_OPERATOR,
                        ));
                    }
                };

                let stmt = quote! {
                    let #result_var = __builder.#method(#left_var, #right_var);
                };
                self.statements.push(stmt);
                Ok(result_var)
            }

            // Unary operations: !
            Expr::Unary(ExprUnary { op, expr, .. }) => {
                let expr_var = self.build_expr(expr, input_map)?;
                let result_var = self.next_var();

                match op {
                    UnOp::Not(Not { .. }) => {
                        let stmt = quote! {
                            let #result_var = __builder.not(#expr_var);
                        };
                        self.statements.push(stmt);
                        Ok(result_var)
                    }
                    _ => Err(syn::Error::new_spanned(
                        op,
                        error_messages::UNSUPPORTED_UNARY,
                    )),
                }
            }

            // Parenthesized expressions
            Expr::Paren(ExprParen { expr, .. }) => self.build_expr(expr, input_map),

            // Boolean literals
            Expr::Lit(lit) => {
                if let syn::Lit::Bool(bool_lit) = &lit.lit {
                    let result_var = self.next_var();
                    let value = bool_lit.value;
                    let stmt = quote! {
                        let #result_var = __builder.constant(#value);
                    };
                    self.statements.push(stmt);
                    Ok(result_var)
                } else {
                    Err(syn::Error::new_spanned(
                        lit,
                        error_messages::UNSUPPORTED_LITERAL,
                    ))
                }
            }

            _ => Err(syn::Error::new_spanned(
                expr,
                error_messages::UNSUPPORTED_EXPRESSION,
            )),
        }
    }

    /// Generate the final circuit building code
    fn finish(self, output_var: Ident, _input_names: &[Ident]) -> TokenStream2 {
        let statements = self.statements;

        quote! {
            {
                let mut __builder = encircuit::CircuitBuilder::default();
                #(#statements)*
                __builder.finish(#output_var)
            }
        }
    }
}

/// Build a Boolean circuit in pure Rust syntax.
///
/// The macro transforms Boolean expressions into a `Circuit` that can be encrypted and evaluated
/// using fully homomorphic encryption.
///
/// # Supported Operations
///
/// - **AND**: `a & b`
/// - **OR**: `a | b`
/// - **XOR**: `a ^ b`
/// - **NOT**: `!a`
/// - **Constants**: `true`, `false`
/// - **Parentheses**: `(a & b) | c`
///
/// # Example
///
/// ```rust
/// use encircuit_macros::circuit;
///
/// // Simple AND gate
/// let and_gate = circuit! { |a, b| a & b };
///
/// // More complex expression
/// let complex = circuit! { |x, y, z| (!x & y) | (x & !z) };
///
/// // With constants
/// let with_const = circuit! { |a| a | true };
/// ```
///
/// # Generated Code
///
/// The macro generates code equivalent to manually building the circuit:
///
/// ```rust,ignore
/// {
///     let mut builder = CircuitBuilder::default();
///     let input_a = builder.input();
///     let input_b = builder.input();
///     let result = builder.and(input_a, input_b);
///     builder.finish(result)
/// }
/// ```
#[proc_macro]
pub fn circuit(input: TokenStream) -> TokenStream {
    let circuit_closure = parse_macro_input!(input as CircuitClosure);

    match generate_circuit_code(&circuit_closure) {
        Ok(code) => code.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Generate the circuit building code from the parsed closure
fn generate_circuit_code(closure: &CircuitClosure) -> Result<TokenStream2> {
    let mut builder = CircuitBuilder::new();
    let mut input_map = HashMap::new();

    // Add input variables
    for input_name in &closure.inputs {
        let var = builder.add_input(input_name);
        input_map.insert(input_name.to_string(), var);
    }

    // Build the expression
    let output_var = builder.build_expr(&closure.body, &input_map)?;

    // Generate the final code
    Ok(builder.finish(output_var, &closure.inputs))
}

/// Additional examples and usage patterns for the `circuit!` macro.
///
/// # Common Patterns
///
/// ## Arithmetic Circuits
///
/// ```rust
/// use encircuit_macros::circuit;
///
/// // Half adder
/// let sum = circuit! { |a, b| a ^ b };
/// let carry = circuit! { |a, b| a & b };
///
/// // Full adder  
/// let full_sum = circuit! { |a, b, cin| a ^ b ^ cin };
/// let full_carry = circuit! { |a, b, cin| (a & b) | (a & cin) | (b & cin) };
/// ```
///
/// ## Logic Gates
///
/// ```rust
/// use encircuit_macros::circuit;
///
/// // Universal gates
/// let nand = circuit! { |a, b| !(a & b) };
/// let nor = circuit! { |a, b| !(a | b) };
///
/// // Other common gates
/// let xnor = circuit! { |a, b| !(a ^ b) };  // Equivalence
/// let implies = circuit! { |a, b| !a | b }; // Implication
/// ```
///
/// ## Control Circuits
///
/// ```rust
/// use encircuit_macros::circuit;
///
/// // Multiplexer: select ? b : a
/// let mux2to1 = circuit! { |a, b, select| (a & !select) | (b & select) };
///
/// // Majority function (3-input)
/// let majority = circuit! { |a, b, c| (a & b) | (a & c) | (b & c) };
/// ```
///
/// # Error Handling
///
/// The macro provides descriptive error messages for common mistakes:
///
/// - Using unsupported operators
/// - Referencing undefined variables  
/// - Invalid closure syntax
///
/// These errors are caught at compile time, preventing runtime issues.
///
/// # Performance Notes
///
/// - The macro generates efficient circuit-building code
/// - No runtime overhead compared to manual circuit construction
/// - Circuit structure is determined entirely at compile time
///
/// # Future Enhancements
///
/// Planned improvements include:
/// - Circuit optimization hints
/// - Compile-time circuit analysis and warnings
/// - Integration with circuit visualization tools
const _DOCUMENTATION: () = ();
