/*!
Demo showing the `circuit!` macro in action.

This example demonstrates how to use the procedural macro to build circuits
and integrate them with the encircuit API.

Run with: `cargo run --example macro_demo --features macros`
*/

use encircuit::prelude::*;

// Enable macros for this demo
#[cfg(feature = "macros")]
use encircuit_macros::circuit;

fn main() -> anyhow::Result<()> {
    println!("🔧 Encircuit Macro Demo");
    println!("========================");

    // Show manual circuit building vs macro
    demo_manual_vs_macro();

    // Show various circuit patterns
    demo_circuit_patterns();

    // Show circuit analysis
    demo_circuit_analysis();

    println!("\n✅ Macro demo completed successfully!");
    Ok(())
}

fn demo_manual_vs_macro() {
    println!("\n📋 Manual vs Macro Circuit Building");
    println!("-----------------------------------");

    // Manual circuit building
    let manual_circuit = {
        let mut builder = CircuitBuilder::default();
        let a = builder.input();
        let b = builder.input();
        let not_b = builder.not(b);
        let and1 = builder.and(not_b, a);
        let and2 = builder.and(a, b);
        let result = builder.or(and1, and2);
        builder.finish(result)
    };

    // Same circuit using macro
    #[cfg(feature = "macros")]
    let macro_circuit = circuit! { |a, b| (!b & a) | (a & b) };

    #[cfg(not(feature = "macros"))]
    let macro_circuit = manual_circuit.clone();

    println!("Manual circuit: {} inputs, {} gates", 
        manual_circuit.input_count(), 
        manual_circuit.stats().total_gates
    );
    
    println!("Macro circuit:  {} inputs, {} gates", 
        macro_circuit.input_count(), 
        macro_circuit.stats().total_gates
    );

    // Both should be equivalent
    assert_eq!(manual_circuit.input_count(), macro_circuit.input_count());
}

fn demo_circuit_patterns() {
    println!("\n🔗 Common Circuit Patterns");
    println!("---------------------------");

    #[cfg(feature = "macros")]
    {
        // Basic gates
        let and_gate = circuit! { |a, b| a & b };
        let or_gate = circuit! { |a, b| a | b };
        let xor_gate = circuit! { |a, b| a ^ b };
        let not_gate = circuit! { |a| !a };

        println!("AND gate: {} inputs -> {}", and_gate.input_count(), and_gate.stats().and_gates);
        println!("OR gate:  {} inputs -> {}", or_gate.input_count(), or_gate.stats().or_gates);
        println!("XOR gate: {} inputs -> {}", xor_gate.input_count(), xor_gate.stats().xor_gates);
        println!("NOT gate: {} inputs -> {}", not_gate.input_count(), not_gate.stats().not_gates);

        // Arithmetic circuits
        let half_adder_sum = circuit! { |a, b| a ^ b };
        let half_adder_carry = circuit! { |a, b| a & b };
        let full_adder_sum = circuit! { |a, b, cin| a ^ b ^ cin };
        let full_adder_carry = circuit! { |a, b, cin| (a & b) | (a & cin) | (b & cin) };

        println!("\nArithmetic Circuits:");
        println!("Half adder sum:   {} gates", half_adder_sum.stats().total_gates);
        println!("Half adder carry: {} gates", half_adder_carry.stats().total_gates);
        println!("Full adder sum:   {} gates", full_adder_sum.stats().total_gates);
        println!("Full adder carry: {} gates", full_adder_carry.stats().total_gates);

        // Universal gates
        let nand = circuit! { |a, b| !(a & b) };
        let nor = circuit! { |a, b| !(a | b) };

        println!("\nUniversal Gates:");
        println!("NAND: {} gates", nand.stats().total_gates);
        println!("NOR:  {} gates", nor.stats().total_gates);

        // Control circuits
        let mux_2to1 = circuit! { |a, b, select| (a & !select) | (b & select) };
        let majority = circuit! { |a, b, c| (a & b) | (a & c) | (b & c) };

        println!("\nControl Circuits:");
        println!("2-to-1 MUX: {} gates", mux_2to1.stats().total_gates);
        println!("3-bit majority: {} gates", majority.stats().total_gates);
    }

    #[cfg(not(feature = "macros"))]
    println!("Enable the 'macros' feature to see circuit patterns demo");
}

fn demo_circuit_analysis() {
    println!("\n📊 Circuit Analysis");
    println!("--------------------");

    #[cfg(feature = "macros")]
    {
        // Complex circuit for analysis
        let complex = circuit! { |w, x, y, z| 
            ((w & x) | (y ^ z)) & (!w | !z)
        };

        let stats = complex.stats();
        println!("Complex circuit analysis:");
        println!("  Inputs: {}", complex.input_count());
        println!("  AND gates: {}", stats.and_gates);
        println!("  OR gates: {}", stats.or_gates);
        println!("  XOR gates: {}", stats.xor_gates);
        println!("  NOT gates: {}", stats.not_gates);
        println!("  Constants: {}", stats.constants);
        println!("  Total gates: {}", stats.total_gates);

        // Test with different input patterns
        println!("\nCircuit complexity comparison:");
        
        let simple = circuit! { |a, b| a & b };
        let medium = circuit! { |a, b, c| (a & b) | c };
        let complex2 = circuit! { |a, b, c, d| (a ^ b) & (c | d) };
        
        println!("Simple:  {} gates", simple.stats().total_gates);
        println!("Medium:  {} gates", medium.stats().total_gates);
        println!("Complex: {} gates", complex2.stats().total_gates);
    }

    #[cfg(not(feature = "macros"))]
    println!("Enable the 'macros' feature to see circuit analysis demo");
}
