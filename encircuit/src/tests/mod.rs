/*!
Tests for the encircuit crate.

This module organizes comprehensive tests for all the core functionality
of the encircuit FHE toolkit into focused test modules.
*/

// Test modules organized by functionality
mod params;       // Parameter configuration and scenario tests
mod keys;         // Key generation and serialization tests  
mod circuit;      // Circuit building, validation, and evaluation tests
mod ciphertext;   // Ciphertext encryption and decryption tests
mod integration;  // End-to-end integration tests
