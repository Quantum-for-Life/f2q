//! Qubit representation

use std::{
    fmt::Display,
    ops::Mul,
};

use self::pauli_group::PauliGroup;
use crate::{
    math::Root4,
    Error,
};

const PAULI_MASK: u64 = 0b11;

/// Pauli operator
///
/// # Examples
///
/// ```rust
/// # use f2q::code::qubits::Sigma;
/// use f2q::Error::QubitIndex;
///
/// let paulis: Vec<_> = (0..=4).map(|i| Sigma::try_from(i)).collect();
///
/// assert_eq!(
///     paulis[0..=3],
///     [Ok(Sigma::I), Ok(Sigma::X), Ok(Sigma::Y), Ok(Sigma::Z),]
/// );
/// matches!(paulis[4], Err(QubitIndex { .. }));
/// ```
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum Sigma {
    #[default]
    I,
    X,
    Y,
    Z,
}

macro_rules! impl_pauli_int {
    ( $($Typ:ty)* ) => {
        $(

            impl TryFrom<$Typ> for Sigma {
                type Error = Error;

                fn try_from(value: $Typ) -> Result<Self, Self::Error> {
                    use Sigma::*;
                    match value {
                        0 => Ok(I),
                        1 => Ok(X),
                        2 => Ok(Y),
                        3 => Ok(Z),
                        _ => Err(Self::Error::QubitIndex{ msg: "Pauli index should be within 0..=3".to_string()}),
                    }
                }
            }

            impl From<Sigma> for $Typ {
                fn from(value: Sigma) -> Self {
                    match value {
                        Sigma::I => 0,
                        Sigma::X => 1,
                        Sigma::Y => 2,
                        Sigma::Z => 3,
                    }
                }
            }

        )*
    };
}

impl_pauli_int!(u8 u16 u32 u64 u128 usize);
impl_pauli_int!(i8 i16 i32 i64 i128 isize);

impl Display for Sigma {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let str_repr = match self {
            Sigma::I => "I",
            Sigma::X => "X",
            Sigma::Y => "Y",
            Sigma::Z => "Z",
        };
        write!(f, "{str_repr}")
    }
}

impl From<Sigma> for String {
    fn from(value: Sigma) -> Self {
        value.to_string()
    }
}

/// Pauli string of up to 64 qubits.
///
/// # Examples
///
/// ```rust
/// # use f2q::code::qubits::Pauli;
/// let code = Pauli::default();
///
/// assert_eq!(code.index(), 0);
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pauli {
    pack: (u64, u64),
}

impl Default for Pauli {
    fn default() -> Self {
        Self::new((0, 0))
    }
}

impl Pauli {
    /// Create new code.
    ///
    /// The pauli product is specified by providing a tuple `(u64, u64)` with 2
    /// bits for each Pauli operator in the tensor product:
    ///
    /// ```text
    /// Sigma::I = 0b00
    /// Sigma::X = 0b01
    /// Sigma::Y = 0b10
    /// Sigma::Z = 0b11
    /// ```
    ///
    /// The first integer in the tuple represents qubits 0 to 31 (incl.),
    /// and the second integer represents qubits 32 to 63 (incl.).
    /// The pairs of bits in each integer follow the little-endian convention.
    /// For example, `Pauli::new((0b1001, 0))` represents the following
    /// Pauli product of 64 Pauli operators:
    ///
    /// ```text
    /// [X, Y, I, I, ... , I]
    /// ```
    ///
    /// whereas `Pauli::new((0, 0b0111))` represents:
    ///
    /// ```text
    /// [I, I, .. I, Z, X, I, ... , I],
    /// ```
    ///
    /// with `Z`'s 0-based index 32.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::{Sigma, Pauli};
    ///
    /// let code = Pauli::new((0b0100, 0b1110));
    ///
    /// assert_eq!(code.pauli(0), Some(Sigma::I));
    /// assert_eq!(code.pauli(1), Some(Sigma::X));
    /// assert_eq!(code.pauli(32), Some(Sigma::Y));
    /// assert_eq!(code.pauli(33), Some(Sigma::Z));
    /// ```
    #[must_use]
    pub fn new(pack: (u64, u64)) -> Self {
        Self {
            pack,
        }
    }

    /// Tensor product of identity operators.
    ///
    /// This is the same as `Pauli::default()` or `Pauli::new((0,0))`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use f2q::code::qubits::{Sigma, Pauli};
    ///
    /// let code = Pauli::identity();
    ///
    /// assert_eq!(code, Pauli::new((0, 0)));
    /// assert_eq!(code, Pauli::default());
    /// assert_eq!(code, Pauli::with_ops([Sigma::I]));
    /// ```
    #[must_use]
    pub fn identity() -> Self {
        Self::default()
    }

    /// Enumerate Pauli code.
    ///
    /// This convert the code to a 128-wide integer.
    /// The code consisting of only `Pauli:I` has index zero.
    ///
    /// You can also use implementation of [`From<Pauli>`] for `u128`
    /// (and vice versa).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::Pauli;
    /// let code = Pauli::new((3, 4));
    ///
    /// assert_eq!(code.index(), 3 + (4 << 64));
    /// assert_eq!(u128::from(code), 3 + (4 << 64));
    /// ```
    #[must_use]
    pub fn index(&self) -> u128 {
        u128::from(self.pack.0) + (u128::from(self.pack.1) << 64)
    }

    /// Read out the Pauli operator at site `i`.
    ///
    ///
    /// # Safety
    ///
    /// The user must ensure that that `i` is within 0..64 (excl.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::{Pauli, Sigma};
    /// let code = Pauli::new((0b1000, 0));
    /// let pauli = unsafe { code.pauli_unchecked(1) };
    ///
    /// assert_eq!(pauli, Sigma::Y);
    /// ```
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub unsafe fn pauli_unchecked(
        &self,
        index: u16,
    ) -> Sigma {
        let pauli_int = if index < 32 {
            (self.pack.0 >> (index * 2)) & PAULI_MASK
        } else {
            (self.pack.1 >> ((index - 32) * 2)) & PAULI_MASK
        };
        Sigma::try_from(pauli_int).expect("incorrect encoding. This is a bug")
    }

    /// Read out the Pauli operator at site `i`.
    ///
    /// # Returns
    ///
    /// Returns None if index `i` is larger or equal `64`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::{Pauli, Sigma};
    /// let code = Pauli::new((0b1000, 0));
    ///
    /// let pauli = code.pauli(1);
    /// assert_eq!(pauli, Some(Sigma::Y));
    ///
    /// let pauli = code.pauli(64);
    /// assert_eq!(pauli, None);
    /// ```
    #[must_use]
    pub fn pauli(
        &self,
        index: u16,
    ) -> Option<Sigma> {
        if index >= 64 {
            None
        } else {
            // SAFETY: We just checked if index is within bounds
            Some(unsafe { self.pauli_unchecked(index) })
        }
    }

    /// Modify the Pauli operator in the code at site `i`.
    ///
    /// The supplied closure will receive a mutable reference to the relevant
    /// Pauli.
    ///
    /// # Safety
    ///
    /// The user must ensure that that `i` is within `0..64` (excl.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::{Sigma, Pauli};
    /// let mut code = Pauli::new((0, 0b01));
    /// assert_eq!(code.pauli(32), Some(Sigma::X));
    ///
    /// unsafe {
    ///     code.pauli_mut_unchecked(32, |pauli| *pauli = Sigma::Y);
    /// }
    ///
    /// assert_eq!(code.pauli(32), Some(Sigma::Y));
    /// ```
    pub unsafe fn pauli_mut_unchecked<OP>(
        &mut self,
        index: u16,
        f: OP,
    ) where
        OP: FnOnce(&mut Sigma),
    {
        let mut pauli = self.pauli_unchecked(index);
        f(&mut pauli);
        if index < 32 {
            self.pack.0 &= !(PAULI_MASK << (index * 2));
            self.pack.0 |= u64::from(pauli) << (index * 2);
        } else {
            self.pack.1 &= !(PAULI_MASK << ((index - 32) * 2));
            self.pack.1 |= u64::from(pauli) << ((index - 32) * 2);
        }
    }

    /// Set Pauli operator at `index`.
    ///
    /// # Safety
    ///
    /// The user must ensure that that `i` is within `0..64` (excl.)
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::{Sigma, Pauli};
    /// let mut code = Pauli::default();
    /// assert_eq!(code.pauli(17), Some(Sigma::I));
    ///
    /// unsafe {
    ///     code.set_unchecked(17, Sigma::Z);
    /// }
    ///
    /// assert_eq!(code.pauli(17), Some(Sigma::Z));
    /// ```
    pub unsafe fn set_unchecked(
        &mut self,
        index: u16,
        pauli: Sigma,
    ) {
        self.pauli_mut_unchecked(index, |p| {
            *p = pauli;
        });
    }

    /// Modify the Pauli operator in the code at site `i`.
    ///
    /// If index `i` is less then `64`, the supplied closure will receive a
    /// mutable reference to the relevant Pauli.  Otherwise it will receive
    /// None.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::{Sigma, Pauli};
    /// let mut code = Pauli::new((0, 0b01));
    /// assert_eq!(code.pauli(32), Some(Sigma::X));
    ///
    /// code.pauli_mut(32, |x| {
    ///     if let Some(pauli) = x {
    ///         *pauli = Sigma::Y
    ///     }
    /// });
    /// assert_eq!(code.pauli(32), Some(Sigma::Y));
    ///
    /// let mut was_it_none = false;
    /// code.pauli_mut(64, |x| {
    ///     if x.is_none() {
    ///         was_it_none = true
    ///     }
    /// });
    /// assert!(was_it_none);
    /// ```
    pub fn pauli_mut<OP>(
        &mut self,
        index: u16,
        f: OP,
    ) where
        OP: FnOnce(Option<&mut Sigma>),
    {
        if index >= 64 {
            f(None);
        } else {
            // SAFETY: We just checked if index is within bounds
            unsafe {
                self.pauli_mut_unchecked(index, |x: &mut Sigma| f(Some(x)));
            }
        }
    }

    /// Set Pauli operator at `index`.
    ///
    /// # Panics
    ///
    /// Panics if index outside of `0..64` (excl.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::{Sigma, Pauli};
    /// let mut code = Pauli::default();
    /// assert_eq!(code.pauli(17), Some(Sigma::I));
    ///
    /// code.set(17, Sigma::Z);
    ///
    /// assert_eq!(code.pauli(17), Some(Sigma::Z));
    /// ```
    pub fn set(
        &mut self,
        index: u16,
        pauli: Sigma,
    ) {
        self.pauli_mut(index, |x| {
            if let Some(p) = x {
                *p = pauli;
            } else {
                panic!("index should be within 0..64");
            }
        });
    }

    /// Build the code from an iterator over Paulis.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::{Sigma, Pauli};
    /// use f2q::code::qubits::Sigma::{
    ///     X,
    ///     Y,
    ///     Z,
    /// };
    ///
    /// let code = Pauli::with_ops([X, Y, Z]);
    ///
    /// assert_eq!(code.pauli(0), Some(X));
    /// assert_eq!(code.pauli(1), Some(Y));
    /// assert_eq!(code.pauli(2), Some(Z));
    /// ```
    #[allow(clippy::missing_panics_doc)]
    pub fn with_ops<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Sigma>,
    {
        let mut code = Self::default();
        for (i, pauli) in iter.into_iter().take(64).enumerate() {
            // SAFETY: we take only 64 elements, so the index must be within
            // bounds
            let i = u16::try_from(i)
                .expect("index out of bounds for type u16. This is a bug");
            unsafe {
                code.set_unchecked(i, pauli);
            }
        }
        code
    }

    /// Parity operator.
    ///
    /// Returns code that consists of a consecutive string of `num_qubits`
    /// [`Sigma::Z`].
    ///
    /// # Panics
    ///
    /// Panics if `num_qubits > 64`
    ///
    /// # Examples
    ///
    ///
    /// ```rust
    /// # use f2q::code::qubits::{Sigma, Pauli};
    ///
    /// let par_op = Pauli::parity_op(2);
    ///
    /// assert_eq!(par_op.pauli(0), Some(Sigma::Z));
    /// assert_eq!(par_op.pauli(1), Some(Sigma::Z));
    /// assert_eq!(par_op.pauli(2), Some(Sigma::I));
    ///
    /// assert_eq!(Pauli::parity_op(0), Pauli::identity());
    /// ```
    #[must_use]
    pub fn parity_op(num_qubits: u16) -> Self {
        assert!(num_qubits <= 64, "number of qubits must be within 1..=64");

        Pauli::with_ops((0..num_qubits).map(|_| Sigma::Z))
    }

    /// Return the number of non-trivial Pauli operators.
    ///
    /// The value returned is equal to the number of [`Sigma`]s in self
    /// that are different than `Sigma::I`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use f2q::code::qubits::{Sigma, Pauli};
    /// let code = Pauli::parity_op(3);
    ///
    /// assert_eq!(code.num_nontrivial(), 3);
    /// ```
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn num_nontrivial(&self) -> u8 {
        u8::try_from(self.into_iter().filter(|&x| x != Sigma::I).count())
            .expect("pauli iterator has no more than 64 elements")
    }

    /// Return the minimal size of a qubit register that would fit the code.
    ///
    /// The value returned is simply the number of all [`Sigma`]'s that remain
    /// after removing the trailing `Sigma::I`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::{Sigma, Pauli};
    /// use Sigma::{
    ///     I,
    ///     X,
    ///     Y,
    ///     Z,
    /// };
    ///
    /// assert_eq!(Pauli::identity().min_register_size(), 0);
    ///
    /// let code = Pauli::with_ops([X, I, Y, I, Z, I, I]);
    /// assert_eq!(code.min_register_size(), 5);
    /// ```
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn min_register_size(&self) -> u8 {
        u8::try_from(if self.pack.1 == 0 {
            (0..32).filter(|i| self.pack.0 >> (2 * i) != 0).count()
        } else {
            (0..32).filter(|i| self.pack.1 >> (2 * i) != 0).count() + 32
        })
        .expect("pauli iterator has no more than 64 elements")
    }
}

/// Iterate over `Sigma`s in `Pauli`
#[derive(Debug)]
pub struct IntoIter {
    code:  Pauli,
    index: u16,
}

impl IntoIter {
    fn new(code: Pauli) -> Self {
        Self {
            code,
            index: 0,
        }
    }
}

impl Iterator for IntoIter {
    type Item = Sigma;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 64 {
            return None;
        }

        let pauli = self.code.pauli(self.index);
        self.index += 1;
        pauli
    }
}

impl IntoIterator for Pauli {
    type IntoIter = IntoIter;
    type Item = Sigma;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl Display for Pauli {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if self.index() == 0 {
            write!(f, "I")
        } else {
            let mut pauli_str = String::with_capacity(64);
            for pauli in self.into_iter() {
                let ch = match pauli {
                    Sigma::I => 'I',
                    Sigma::X => 'X',
                    Sigma::Y => 'Y',
                    Sigma::Z => 'Z',
                };
                pauli_str.push(ch);
            }

            write!(f, "{}", pauli_str.trim_end_matches('I'))
        }
    }
}

impl From<Pauli> for u128 {
    fn from(value: Pauli) -> Self {
        value.index()
    }
}

impl From<u128> for Pauli {
    #[allow(clippy::cast_possible_truncation)]
    fn from(value: u128) -> Self {
        Self::new((value as u64, (value >> 64) as u64))
    }
}

impl PartialOrd for Pauli {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pauli {
    fn cmp(
        &self,
        other: &Self,
    ) -> std::cmp::Ordering {
        u128::from(*self).cmp(&u128::from(*other))
    }
}

impl Mul for Pauli {
    type Output = (Root4, Pauli);

    fn mul(
        self,
        rhs: Self,
    ) -> Self::Output {
        (PauliGroup::from(self) * PauliGroup::from(rhs)).into()
    }
}

pub(crate) mod pauli_group {
    use std::ops::Mul;

    use crate::{
        code::qubits::{
            Pauli,
            Sigma,
        },
        math::{
            Group,
            Root4,
        },
    };

    struct PGrp(Root4, Sigma);

    impl Mul for PGrp {
        type Output = Self;

        fn mul(
            self,
            rhs: Self,
        ) -> Self::Output {
            use Root4::{
                R0,
                R2,
                R3,
            };
            let (omega, pauli) = match self.1 {
                Sigma::I => (R0, rhs.1),
                Sigma::X => match rhs.1 {
                    Sigma::I => (R0, Sigma::X),
                    Sigma::X => (R0, Sigma::I),
                    Sigma::Y => (R2, Sigma::Z),
                    Sigma::Z => (R3, Sigma::Y),
                },
                Sigma::Y => match rhs.1 {
                    Sigma::I => (R0, Sigma::Y),
                    Sigma::X => (R3, Sigma::Z),
                    Sigma::Y => (R0, Sigma::I),
                    Sigma::Z => (R2, Sigma::X),
                },
                Sigma::Z => match rhs.1 {
                    Sigma::I => (R0, Sigma::Z),
                    Sigma::X => (R2, Sigma::Y),
                    Sigma::Y => (R3, Sigma::X),
                    Sigma::Z => (R0, Sigma::I),
                },
            };

            PGrp(self.0 * rhs.0 * omega, pauli)
        }
    }

    impl Group for PGrp {
        fn identity() -> Self {
            Self(Root4::R0, Sigma::I)
        }

        fn inverse(self) -> Self {
            Self(self.0.inverse(), self.1)
        }
    }

    /// Cross-product Root4 x `Pauli`
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) struct PauliGroup(Root4, Pauli);

    impl PauliGroup {
        #[must_use]
        pub fn new(
            omega: Root4,
            code: Pauli,
        ) -> Self {
            Self(omega, code)
        }
    }

    impl From<Pauli> for PauliGroup {
        fn from(value: Pauli) -> Self {
            Self::new(Root4::identity(), value)
        }
    }

    impl From<Root4> for PauliGroup {
        fn from(value: Root4) -> Self {
            Self::new(value, Pauli::identity())
        }
    }

    impl From<PauliGroup> for (Root4, Pauli) {
        fn from(value: PauliGroup) -> Self {
            (value.0, value.1)
        }
    }

    impl Mul for PauliGroup {
        type Output = Self;

        fn mul(
            self,
            rhs: Self,
        ) -> Self::Output {
            let phase = self.1.into_iter().zip(rhs.1).fold(
                self.0 * rhs.0,
                |acc, (pauli_l, pauli_r)| {
                    let PGrp(omega, _) =
                        PGrp(Root4::R0, pauli_l) * PGrp(Root4::R0, pauli_r);
                    acc * omega
                },
            );

            let code = Pauli::new((
                self.1.pack.0 ^ rhs.1.pack.0,
                self.1.pack.1 ^ rhs.1.pack.1,
            ));
            PauliGroup::new(phase, code)
        }
    }

    impl Group for PauliGroup {
        fn identity() -> Self {
            Self(Root4::identity(), Pauli::default())
        }

        fn inverse(self) -> Self {
            Self(self.0.inverse(), self.1)
        }
    }
}
