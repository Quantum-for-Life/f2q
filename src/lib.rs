//! Fermion-to-qubit mappings.
//!
//! High-octane representation of Pauli Hamiltonians
//! with up to 64 qubits. 🎇
//!
//! This is a software library to parse and convert quantum chemistry
//! Hamiltonians into a form suitable for quantum hardware based on qubit gates.

use std::fmt::Display;

pub mod code;
pub mod map;
pub mod terms;

pub(crate) mod math;

mod serialize;

#[derive(Debug, PartialEq)]
pub enum Error {
    /// Invalid qubit index in a Pauli string
    QubitIndex { msg: String },
}

impl Display for Error {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::QubitIndex {
                msg,
            } => write!(f, "PauliIndex: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests;
