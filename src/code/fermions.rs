//! Second quantization

// Describe canonical ordering of indices in Hamiltonian

use std::{
    fmt::Display,
    ops::Range,
};

use crate::Error;

/// Spin one-half
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Spin {
    #[default]
    Down,
    Up,
}

impl Spin {
    /// True if spin up.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use f2q::code::fermions::Spin;
    ///
    /// let spin = Spin::Up;
    /// assert!(spin.is_up());
    ///
    /// let spin = Spin::Down;
    /// assert!(!spin.is_up());
    /// ```
    #[must_use]
    pub fn is_up(&self) -> bool {
        match self {
            Self::Down => false,
            Self::Up => true,
        }
    }

    /// Flip the spin to its opposite.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::fermions::Spin;
    ///
    /// let spin = Spin::Down;
    /// assert_eq!(spin.flip(), Spin::Up);
    /// ```
    #[must_use]
    pub fn flip(&self) -> Self {
        match self {
            Self::Down => Self::Up,
            Self::Up => Self::Down,
        }
    }

    /// Iterate over both configurations: down and then up.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use f2q::code::fermions::Spin;
    ///
    /// let spins: Vec<_> = Spin::both().collect();
    ///
    /// assert_eq!(spins, &[Spin::Down, Spin::Up]);
    /// ```
    pub fn both() -> impl Iterator<Item = Self> {
        [Spin::Down, Spin::Up].into_iter()
    }
}

impl From<bool> for Spin {
    fn from(value: bool) -> Self {
        if value {
            Spin::Up
        } else {
            Spin::Down
        }
    }
}

macro_rules! impl_spin_int {
    ($($Typ:ty)* ) => {
        $(
            impl From<Spin> for $Typ {
                fn from(value: Spin) -> Self {
                    match value {
                        Spin::Down => 0,
                        Spin::Up => 1,
                    }
                }
            }
        )*
    };
}

impl_spin_int!(u8 u16 u32 u64 u128 usize);
impl_spin_int!(i8 i16 i32 i64 i128 isize);

/// Electronic orbital consisting of a principal quantum number and a spin 1/2.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
pub struct Orbital {
    pub n: u32,
    pub s: Spin,
}

impl Orbital {
    /// Create a new orbital.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::fermions::{Orbital, Spin};
    ///
    /// let orb = Orbital::new(0, Spin::Down);
    /// assert_eq!(orb.index(), 0);
    /// ```
    #[must_use]
    pub fn new(
        n: u32,
        s: Spin,
    ) -> Self {
        Self {
            n,
            s,
        }
    }

    /// Compute orbital index.
    ///
    /// Orbitals can be enumerated according to the formula: `2*n + s`,
    /// where `n` is the principal quantum number and `s` is the spin number,
    /// i.e. `s = 0` for `Spin::Down` and `s = 1` for `Spin::Up`.
    ///
    /// # Panics
    ///
    /// Panics is the orbitals index cannot fit into `usize`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::fermions::{Orbital, Spin};
    ///
    /// let orb = Orbital::new(0, Spin::Down);
    /// assert_eq!(orb.index(), 0);
    ///
    /// let orb = Orbital::new(3, Spin::Up);
    /// assert_eq!(orb.index(), 7);
    /// ```
    #[must_use]
    pub fn index(&self) -> u32 {
        assert!(
            self.n <= u32::MAX / 2 - u32::from(self.s),
            "orbital index out of bound"
        );
        self.n * 2 + u32::from(self.s)
    }

    /// Return orbital corresponding to the given index.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::fermions::{Orbital, Spin};
    ///
    /// let orbital = Orbital::with_index(3);
    ///
    /// assert_eq!(orbital, Orbital::new(1, Spin::Up));
    /// ```
    #[must_use]
    pub fn with_index(index: u32) -> Self {
        Self::new(index / 2, Spin::from(index & 1 != 0))
    }

    /// Generate orbitals with indeces in the given range.
    ///
    /// If the start bound is unbounded, the iterator starts from zero.
    /// If the end bound is unbounded, it is taken to be `usize::MAX` (incl.)
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::fermions::{Orbital, Spin};
    ///
    /// let orbitals: Vec<_> = Orbital::gen_range((1..4)).collect();
    ///
    /// assert_eq!(
    ///     orbitals,
    ///     &[
    ///         Orbital::new(0, Spin::Up),
    ///         Orbital::new(1, Spin::Down),
    ///         Orbital::new(1, Spin::Up)
    ///     ]
    /// )
    /// ```
    pub fn gen_range(range: Range<u32>) -> impl Iterator<Item = Orbital> {
        // OrbitalRange::new(range)
        range.into_iter().map(Orbital::with_index)
    }
}

/// Creation operator
///
/// A newtype struct representing a creation operator.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cr(pub Orbital);

impl Cr {
    /// Orbital index.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use f2q::code::fermions::{Orbital, Cr};
    /// let cr = Cr(Orbital::with_index(1));
    ///
    /// assert_eq!(cr.index(), 1);
    /// ```
    #[must_use]
    pub fn index(&self) -> u32 {
        self.0.index()
    }
}

/// Annihilation operator
///
/// A newtype struct representing an annihilation operator.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct An(pub Orbital);

impl An {
    /// Orbital index.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use f2q::code::fermions::{Orbital, An};
    /// let an = An(Orbital::with_index(1));
    ///
    /// assert_eq!(an.index(), 1);
    /// ```
    #[must_use]
    pub fn index(&self) -> u32 {
        self.0.index()
    }
}

/// Electronic integral with creation (cr) and annihilation (an)
/// operators indexed by orbitals in canonical order:
///
/// - If Integral is a one-electron integral and `p = cr.index()`, `q =
///   an.index()`, then `p <= q`
/// - If Integral is a two-electron integral and
///
///   ```text
///   p = cr.0.index()
///   q = cr.1.index()
///   r = an.0.index()
///   s = an.0.index()
///   ```
///
///   then `p < q`, `r > s` and `p <= s`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Fermions {
    #[default]
    Offset,
    One {
        cr: Cr,
        an: An,
    },
    Two {
        cr: (Cr, Cr),
        an: (An, An),
    },
}

impl Fermions {
    /// Create Integral as constant offset.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::fermions::Fermions;
    ///
    /// let integral = Fermions::new();
    ///
    /// assert_eq!(integral, Fermions::Offset);
    /// ```
    #[must_use]
    pub fn new() -> Fermions {
        Self::default()
    }

    /// Create a one-electron integral.
    ///
    /// Orbitals must be in canonical order: `cr.index() <= an.index()`,
    /// otherwise return None.
    ///
    /// ```rust
    /// # use f2q::code::fermions::{Fermions, Orbital, Spin, Cr, An};
    ///
    /// let integral = Fermions::one_electron(
    ///     Cr(Orbital::new(0, Spin::Down)),
    ///     An(Orbital::new(1, Spin::Down)),
    /// );
    /// assert!(integral.is_some());
    ///
    /// let integral = Fermions::one_electron(
    ///     Cr(Orbital::new(1, Spin::Down)),
    ///     An(Orbital::new(0, Spin::Down)),
    /// );
    /// assert!(integral.is_none());
    /// ```
    #[must_use]
    pub fn one_electron(
        cr: Cr,
        an: An,
    ) -> Option<Self> {
        (cr.index() <= an.index()).then_some(Self::One {
            cr,
            an,
        })
    }

    /// Create a two-electron integral.
    ///
    /// Orbitals must be in canonical order:
    ///
    /// ```text
    /// cr.0.index() < cr.1.index()
    /// an.0.index() > an.1.index()
    /// cr.0.index() <= an.1.index()
    /// ```
    /// otherwise return None.
    ///
    /// ```rust
    /// # use f2q::code::fermions::{Fermions, Orbital, Spin, Cr, An};
    ///
    /// let integral = Fermions::two_electron(
    ///     (
    ///         Cr(Orbital::new(0, Spin::Down)),
    ///         Cr(Orbital::new(0, Spin::Up)),
    ///     ),
    ///     (
    ///         An(Orbital::new(1, Spin::Down)),
    ///         An(Orbital::new(0, Spin::Down)),
    ///     ),
    /// );
    /// assert!(integral.is_some());
    ///
    /// let integral = Fermions::two_electron(
    ///     (
    ///         Cr(Orbital::new(0, Spin::Down)),
    ///         Cr(Orbital::new(0, Spin::Down)),
    ///     ),
    ///     (
    ///         An(Orbital::new(1, Spin::Down)),
    ///         An(Orbital::new(0, Spin::Down)),
    ///     ),
    /// );
    /// assert!(integral.is_none());
    /// ```
    #[must_use]
    pub fn two_electron(
        cr: (Cr, Cr),
        an: (An, An),
    ) -> Option<Self> {
        (cr.0.index() < cr.1.index()
            && an.0.index() > an.1.index()
            && cr.0.index() <= an.1.index())
        .then_some(Self::Two {
            cr,
            an,
        })
    }
}

impl From<()> for Fermions {
    fn from((): ()) -> Self {
        Fermions::Offset
    }
}

impl TryFrom<(u32, u32)> for Fermions {
    type Error = Error;

    fn try_from(value: (u32, u32)) -> Result<Self, Self::Error> {
        Fermions::one_electron(
            Cr(Orbital::with_index(value.0)),
            An(Orbital::with_index(value.1)),
        )
        .ok_or(Self::Error::QubitIndex {
            msg: "one-electron term orbital ordering".to_string(),
        })
    }
}

impl TryFrom<(u32, u32, u32, u32)> for Fermions {
    type Error = Error;

    fn try_from(value: (u32, u32, u32, u32)) -> Result<Self, Self::Error> {
        Fermions::two_electron(
            (
                Cr(Orbital::with_index(value.0)),
                Cr(Orbital::with_index(value.1)),
            ),
            (
                An(Orbital::with_index(value.2)),
                An(Orbital::with_index(value.3)),
            ),
        )
        .ok_or(Self::Error::QubitIndex {
            msg: "two-electron term orbital ordering".to_string(),
        })
    }
}

impl Display for Fermions {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Fermions::Offset => write!(f, "[]"),
            Fermions::One {
                cr,
                an,
            } => {
                write!(f, "[{}, {}]", cr.index(), an.index())
            }
            Fermions::Two {
                cr,
                an,
            } => write!(
                f,
                "[{}, {}, {}, {}]",
                cr.0.index(),
                cr.1.index(),
                an.0.index(),
                an.1.index()
            ),
        }
    }
}
