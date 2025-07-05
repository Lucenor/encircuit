/*!
Circuit builder for constructing Boolean circuits.

This module provides the `CircuitBuilder` which allows users to construct
Boolean circuits by adding gates and maintaining topological order.
*/

use std::collections::HashMap;

/// Unique identifier for a node in the circuit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub(crate) usize);

/// Types of gates supported in Boolean circuits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Gate {
    /// Input gate - represents a circuit input
    Input,
    /// Constant gate - represents a constant Boolean value
    Constant(bool),
    /// AND gate with two inputs
    And(NodeId, NodeId),
    /// OR gate with two inputs
    Or(NodeId, NodeId),
    /// XOR gate with two inputs
    Xor(NodeId, NodeId),
    /// NOT gate with one input
    Not(NodeId),
}

/// Builder for constructing Boolean circuits.
///
/// The `CircuitBuilder` maintains gates in topological order to ensure
/// efficient evaluation during FHE computation.
#[derive(Debug, Default)]
pub struct CircuitBuilder {
    gates: Vec<Gate>,
    node_map: HashMap<NodeId, usize>,
    next_id: usize,
}

impl CircuitBuilder {
    /// Create a new circuit builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an input gate to the circuit.
    ///
    /// Returns the `NodeId` for this input.
    pub fn input(&mut self) -> NodeId {
        self.add_gate(Gate::Input)
    }

    /// Add a constant gate to the circuit.
    ///
    /// Returns the `NodeId` for this constant.
    pub fn constant(&mut self, value: bool) -> NodeId {
        self.add_gate(Gate::Constant(value))
    }

    /// Add an AND gate to the circuit.
    ///
    /// Returns the `NodeId` for the result of the AND operation.
    pub fn and(&mut self, left: NodeId, right: NodeId) -> NodeId {
        self.add_gate(Gate::And(left, right))
    }

    /// Add an OR gate to the circuit.
    ///
    /// Returns the `NodeId` for the result of the OR operation.
    pub fn or(&mut self, left: NodeId, right: NodeId) -> NodeId {
        self.add_gate(Gate::Or(left, right))
    }

    /// Add an XOR gate to the circuit.
    ///
    /// Returns the `NodeId` for the result of the XOR operation.
    pub fn xor(&mut self, left: NodeId, right: NodeId) -> NodeId {
        self.add_gate(Gate::Xor(left, right))
    }

    /// Add a NOT gate to the circuit.
    ///
    /// Returns the `NodeId` for the result of the NOT operation.
    pub fn not(&mut self, input: NodeId) -> NodeId {
        self.add_gate(Gate::Not(input))
    }

    /// Finish building the circuit and return an immutable `Circuit`.
    ///
    /// The `output` parameter specifies which node represents the circuit's output.
    pub fn finish(self, output: NodeId) -> super::Circuit {
        super::Circuit::new(self.gates, output)
    }

    /// Add a gate to the circuit and return its NodeId.
    fn add_gate(&mut self, gate: Gate) -> NodeId {
        let node_id = NodeId(self.next_id);
        self.next_id += 1;

        let gate_index = self.gates.len();
        self.gates.push(gate);
        self.node_map.insert(node_id, gate_index);

        node_id
    }

    /// Get the number of inputs in the circuit.
    pub fn input_count(&self) -> usize {
        self.gates
            .iter()
            .filter(|gate| matches!(gate, Gate::Input))
            .count()
    }

    /// Get the total number of gates in the circuit.
    pub fn gate_count(&self) -> usize {
        self.gates.len()
    }
}

impl NodeId {
    /// Get the raw ID value.
    pub fn id(self) -> usize {
        self.0
    }
}
