use std::{
    fmt::Display,
    hash::Hash,
};

use terms::SumRepr;

pub mod maps;
pub mod math;
pub mod qubit;
pub mod secq;
pub mod terms;

/// Basic flattened API  
pub mod prelude {
    pub use crate::{
        maps::JordanWigner,
        qubit::{
            Pauli,
            PauliCode,
        },
        secq::{
            An,
            Cr,
            Fermions,
            Orbital,
            Spin,
        },
        terms::{
            Hamil,
            SumRepr,
        },
        Code,
        Terms,
    };
}

/// Representation of Hermitian operators
pub trait Code: Copy + Clone + Eq + Hash + Default {}

impl Code for usize {}

/// Convert and serialize sum of terms in various encodings
pub trait Terms<T, K>
where
    K: Code,
{
    type Error;

    /// Add terms to the supplied [`SumRepr`].
    ///
    /// # Errors
    ///
    /// Return error on failure.
    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, K>,
    ) -> Result<(), Error>;
}

#[derive(Debug, PartialEq)]
pub enum Error {
    /// Invalid index of a Pauli operator
    PauliIndex { msg: String },
}

impl Display for Error {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::PauliIndex {
                msg,
            } => write!(f, "PauliIndex: {msg}"),
        }
    }
}

impl std::error::Error for Error {}
