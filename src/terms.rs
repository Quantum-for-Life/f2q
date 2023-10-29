//! Represent sum of terms.

use crate::{
    code::{
        fermions::Fermions,
        qubits::Pauli,
        Code,
    },
    Error,
};

pub type FermiSum = SumRepr<f64, Fermions>;
pub type PauliSum = SumRepr<f64, Pauli>;

#[doc(inline)]
pub use sumrepr::SumRepr;

/// Convert and serialize sum of terms in various encodings
pub trait Terms<T> {
    type Error;

    /// Add terms to the supplied representation.
    ///
    /// # Errors
    ///
    /// Return error on failure.
    fn add_to(
        self,
        repr: &mut impl Extend<T>,
    ) -> Result<(), Error>;
}

pub mod sumrepr {
    use std::{
        collections::HashMap,
        ops::Add,
    };

    use super::Terms;
    use crate::{
        prelude::Code,
        Error,
    };

    /// Weighted sum of codes
    #[derive(Debug)]
    pub struct SumRepr<T, K> {
        terms: HashMap<K, T>,
    }

    impl<T, K> Default for SumRepr<T, K>
    where
        K: Code,
    {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T, K> SumRepr<T, K>
    where
        K: Code,
    {
        /// Create new, empty sum
        ///
        /// # Examples
        ///
        /// ```rust
        /// # use f2q::{terms::SumRepr, code::qubits::Pauli};
        /// let repr = SumRepr::<f64, Pauli>::new();
        ///
        /// assert!(repr.is_empty());
        /// ```
        #[must_use]
        pub fn new() -> Self {
            Self {
                terms: HashMap::new(),
            }
        }

        /// Creates an empty `SumRepr` with at least the specified capacity.
        ///
        /// The struct will be able to hold at least `capacity` elements without
        /// reallocating.
        ///
        /// # Examples
        ///
        /// ```rust
        /// # use f2q::{terms::SumRepr, code::qubits::Pauli};
        /// let repr = SumRepr::<f64, Pauli>::with_capacity(8);
        ///
        /// assert!(repr.capacity() >= 8);
        /// ```
        #[must_use]
        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                terms: HashMap::with_capacity(capacity),
            }
        }

        /// Returns the number of elements the map can hold without
        /// reallocating.
        ///
        /// This number is a lower bound; the struct might be able to hold more,
        /// but is guaranteed to be able to hold at least this many.
        ///
        ///  /// # Examples
        ///
        /// ```rust
        /// # use f2q::{terms::SumRepr, code::qubits::Pauli};
        /// let repr = SumRepr::<f64, Pauli>::with_capacity(8);
        ///
        /// assert!(repr.capacity() >= 8);
        /// ```
        #[must_use]
        pub fn capacity(&self) -> usize {
            self.terms.capacity()
        }

        /// Number of terms in the sum.
        #[must_use]
        pub fn len(&self) -> usize {
            self.terms.len()
        }

        #[must_use]
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// Iterate over terms in the sum.
        ///
        /// The returned iterator runs over tuples of shared references of type:
        /// `(&T, &K)`.
        ///
        /// # Examples
        ///
        /// ```rust
        /// # use f2q::{code::qubits::Pauli, terms::SumRepr};
        ///
        /// let mut repr = SumRepr::new();
        ///
        /// repr.update(Pauli::default(), 0.5);
        /// repr.update(Pauli::new((1, 0)), 0.5);
        ///
        /// let sum = repr.iter().fold(0.0, |acc, (&coeff, _)| acc + coeff);
        ///
        /// assert_eq!(sum, 1.0);
        /// ```
        pub fn iter(&self) -> impl Iterator<Item = (&T, &K)> {
            self.terms.iter().map(|(code, coeff)| (coeff, code))
        }

        /// Iterate over terms in the sum, allow mutable access to coefficients.
        ///
        /// The returned iterator runs over tuples of references of type:
        /// `(&mut T, &K)`.
        ///
        ///  /// # Examples
        ///
        /// ```rust
        /// # use f2q::{code::qubits::Pauli, terms::SumRepr};
        ///
        /// let mut repr = SumRepr::new();
        ///
        /// repr.update(Pauli::default(), 0.5);
        /// repr.update(Pauli::new((1, 0)), 0.5);
        /// for (coeff, _) in repr.iter_mut() {
        ///     *coeff += 0.1;
        /// }
        ///
        /// assert_eq!(repr.coeff(Pauli::default()), Some(&0.6));
        /// assert_eq!(repr.coeff(Pauli::new((1, 0))), Some(&0.6));
        /// ```
        pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut T, &K)> {
            self.terms.iter_mut().map(|(code, coeff)| (coeff, code))
        }
    }

    pub struct IntoIter<T, K>(std::collections::hash_map::IntoIter<K, T>);

    impl<T, K> Iterator for IntoIter<T, K> {
        type Item = (T, K);

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next().map(|(k, t)| (t, k))
        }
    }

    impl<T, K> IntoIterator for SumRepr<T, K> {
        type IntoIter = IntoIter<T, K>;
        type Item = (T, K);

        fn into_iter(self) -> Self::IntoIter {
            IntoIter(self.terms.into_iter())
        }
    }

    impl<T, K> SumRepr<T, K>
    where
        T: Add<Output = T>,
        K: Code,
    {
        /// Returns coefficient in the sum for a given code.
        ///
        /// # Examples
        ///
        /// ```rust
        /// # use f2q::terms::SumRepr;
        /// let mut repr = SumRepr::new();
        /// repr.update(1, 0.5);
        ///
        /// assert_eq!(repr.coeff(1), Some(&0.5));
        /// assert_eq!(repr.coeff(2), None);
        /// ```
        #[must_use]
        pub fn coeff(
            &self,
            code: K,
        ) -> Option<&T> {
            self.terms.get(&code)
        }

        /// Replace coefficient for the given code.
        ///
        /// Returns the previous coefficient, if present, or `None`.
        ///
        /// # Examples
        ///
        /// ```rust
        /// # use f2q::terms::SumRepr;
        /// let mut repr = SumRepr::new();
        /// let old_coeff = repr.update(1, 0.5);
        /// assert_eq!(old_coeff, None);
        ///
        /// let old_coeff = repr.update(1, 0.7);
        /// assert_eq!(old_coeff, Some(0.5));
        /// assert_eq!(repr.coeff(1), Some(&0.7));
        /// ```
        pub fn update(
            &mut self,
            code: K,
            coeff: T,
        ) -> Option<T> {
            self.terms.insert(code, coeff)
        }

        /// Add coefficient to the given code.
        ///
        /// # Examples
        ///
        /// ```rust
        /// # use f2q::terms::SumRepr;
        /// let mut repr = SumRepr::new();
        /// assert_eq!(repr.coeff(1), None);
        /// repr.add_term(1, 0.5);
        /// assert_eq!(repr.coeff(1), Some(&0.5));
        /// repr.add_term(1, 0.5);
        /// assert_eq!(repr.coeff(1), Some(&1.0));
        /// ```
        pub fn add_term(
            &mut self,
            code: K,
            coeff: T,
        ) {
            if let Some(prev_coeff) = self.terms.remove(&code) {
                let _ = self.update(code, prev_coeff + coeff);
            } else {
                let _ = self.update(code, coeff);
            }
        }
    }

    impl<T, K> FromIterator<(T, K)> for SumRepr<T, K>
    where
        T: Add<Output = T>,
        K: Code,
    {
        fn from_iter<I: IntoIterator<Item = (T, K)>>(iter: I) -> Self {
            let mut repr = SumRepr::new();
            repr.extend(iter);
            repr
        }
    }

    impl<T, K, const N: usize> From<[(T, K); N]> for SumRepr<T, K>
    where
        T: Add<Output = T>,
        K: Code,
    {
        fn from(value: [(T, K); N]) -> Self {
            Self::from_iter(value)
        }
    }

    impl<T, K> Terms<(T, K)> for SumRepr<T, K>
    where
        T: Add<Output = T>,
        K: Code,
    {
        type Error = Error;

        fn add_to(
            self,
            repr: &mut impl Extend<(T, K)>,
        ) -> Result<(), Error> {
            repr.extend(self);
            Ok(())
        }
    }

    impl<T, K> Extend<(T, K)> for SumRepr<T, K>
    where
        T: Add<Output = T>,
        K: Code,
    {
        fn extend<I>(
            &mut self,
            iter: I,
        ) where
            I: IntoIterator<Item = (T, K)>,
        {
            for (coeff, code) in iter {
                self.add_term(code, coeff);
            }
        }
    }
}

#[derive(Debug)]
pub struct StackRepr<T, K, OP>
where
    OP: FnMut() -> Option<(T, K)>,
{
    f: OP,
}

impl<T, K, OP> StackRepr<T, K, OP>
where
    OP: FnMut() -> Option<(T, K)>,
{
    pub fn new(f: OP) -> Self {
        Self {
            f,
        }
    }
}

impl<T, K, OP> Terms<(T, K)> for StackRepr<T, K, OP>
where
    K: Code,
    OP: FnMut() -> Option<(T, K)>,
{
    type Error = Error;

    fn add_to(
        mut self,
        repr: &mut impl Extend<(T, K)>,
    ) -> Result<(), Error> {
        while let Some((coeff, code)) = (self.f)() {
            repr.extend(Some((coeff, code)));
        }

        Ok(())
    }
}

pub struct HeapRepr<'a, T, K> {
    f: Box<dyn FnMut() -> Option<(T, K)> + 'a>,
}

impl<'a, T, K> HeapRepr<'a, T, K> {
    /// Allocate memory for the closure on the heap.
    pub fn new<OP>(f: OP) -> Self
    where
        OP: FnMut() -> Option<(T, K)> + 'a,
    {
        Self {
            f: Box::new(f)
        }
    }
}

impl<'a, T, K> Terms<(T, K)> for HeapRepr<'a, T, K>
where
    K: Code,
{
    type Error = Error;

    fn add_to(
        mut self,
        repr: &mut impl Extend<(T, K)>,
    ) -> Result<(), Error> {
        while let Some((coeff, code)) = (self.f)() {
            repr.extend(Some((coeff, code)));
        }

        Ok(())
    }
}
