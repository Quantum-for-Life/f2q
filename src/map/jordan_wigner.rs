use num::Float;

use crate::{
    code::{
        fermions::{
            An,
            Cr,
            Fermions,
            Orbital,
        },
        qubits::{
            Pauli,
            PauliOp,
        },
    },
    math::ReIm,
    terms::{
        SumRepr,
        Terms,
    },
    Error,
};

enum Map {
    An(Orbital),
    Cr(Orbital),
}

macro_rules! impl_tryfrom_map {
    ($($Typ:tt)* ) => {
        $(
            impl TryFrom<$Typ> for Map {
                type Error = Error;

                fn try_from(value: $Typ) -> Result<Self, Self::Error> {
                    (value.index() < 64)
                        .then_some(Self::$Typ(value.0))
                        .ok_or_else(|| Error::QubitIndex {
                            msg: "orbital index must be within 0..=63".to_string(),
                        })
                }
            }
        )*
    };
}

impl_tryfrom_map!(An Cr);

fn pauli_codes_from_index(index: u16) -> (Pauli, Pauli) {
    let code = Pauli::parity_op(index);

    let x = {
        let mut code = code;
        code.set(index, PauliOp::X);
        code
    };
    let y = {
        let mut code = code;
        code.set(index, PauliOp::Y);
        code
    };

    (x, y)
}

impl Map {
    fn index(&self) -> u16 {
        u16::try_from(match self {
            Self::An(an) => an.index(),
            Self::Cr(cr) => cr.index(),
        })
        .expect("index within 0..=63")
    }

    fn mul_iter<'a, T, I>(
        &'a self,
        rhs: I,
    ) -> impl Iterator<Item = (ReIm<T>, Pauli)> + 'a
    where
        T: Float + 'a,
        I: IntoIterator<Item = (ReIm<T>, Pauli)> + 'a,
    {
        let one_half =
            T::from(0.5_f64).expect("floating point conversion from 0.5");
        let (x, y) = pauli_codes_from_index(self.index());
        let term_x = ReIm::Re(one_half);
        let term_y = match self {
            Self::An(_) => ReIm::Im(one_half),
            Self::Cr(_) => ReIm::Im(-one_half),
        };

        rhs.into_iter().flat_map(move |(rhs_coeff, rhs_pauli)| {
            [(term_x, x), (term_y, y)].into_iter().map(
                move |(lhs_coeff, lhs_pauli)| {
                    let (root, prod) = lhs_pauli * rhs_pauli;

                    (lhs_coeff * rhs_coeff * ReIm::from(root), prod)
                },
            )
        })
    }
}

fn iter_hermitian<'a, T, I>(iter: I) -> impl Iterator<Item = (T, Pauli)> + 'a
where
    T: Float + 'a,
    I: IntoIterator<Item = (ReIm<T>, Pauli)> + 'a,
{
    let two = T::from(2.0_f64).expect("floating point conversion from 2.0");
    iter.into_iter().filter_map(move |(x, p)| {
        if let ReIm::Re(xre) = x {
            Some((xre * two, p))
        } else {
            None
        }
    })
}

#[inline]
fn jw_map_two<'a, T: Float + 'a>(
    op1: &'a Map,
    op2: &'a Map,
    coeff: T,
) -> impl Iterator<Item = (T, Pauli)> + 'a {
    iter_hermitian(
        op1.mul_iter(op2.mul_iter([(ReIm::Re(coeff), Pauli::identity())])),
    )
}

#[inline]
fn jw_map_four<'a, T: Float + 'a>(
    op1: &'a Map,
    op2: &'a Map,
    op3: &'a Map,
    op4: &'a Map,
    coeff: T,
) -> impl Iterator<Item = (T, Pauli)> + 'a {
    iter_hermitian(op1.mul_iter(op2.mul_iter(
        op3.mul_iter(op4.mul_iter([(ReIm::Re(coeff), Pauli::identity())])),
    )))
}
/// Jordan-Wigner mapping.
///
/// This mapping is initialized with [`SumRepr<T,Fermions>`],
/// but implements [`Terms<T, Pauli>`].  The standard way
/// of using it is presented in the following example.
///
/// # Examples
///
/// ```rust
/// # use f2q::{
/// #     code::{
/// #         fermions::{
/// #             An,
/// #             Cr,
/// #             Fermions,
/// #             Orbital,
/// #         },
/// #         qubits::{
/// #             Pauli,
/// #             PauliOp,
/// #         },
/// #     },
/// #     map::JordanWigner,
/// #     terms::{
/// #         PauliSum,
/// #         SumRepr,
/// #         Terms,
/// #     },
/// # };
/// # fn main() -> Result<(), f2q::Error> {
/// let idx = 11;
/// let mut fermi_repr = SumRepr::new();
///
/// // Create orbital with qubit index 11
/// let p = Orbital::with_index(idx);
///
/// // Add it as one-electron interaction term to the sum with coefficient: 1.0
/// fermi_repr.add_term(Fermions::one_electron(Cr(p), An(p)).unwrap(), 1.0);
///
/// // Map fermionic hamiltonian to a sum of Pauli strings
/// let mut pauli_repr = PauliSum::new();
/// JordanWigner::new(&fermi_repr).add_to(&mut pauli_repr)?;
///
/// // We should obtain the following two Pauli strings with weights 1.0
/// let code_i0 = Pauli::default();
/// let code_z0 = {
///     let mut code = Pauli::default();
///     code.set(idx.try_into().unwrap(), PauliOp::Z);
///     code
/// };
///
/// assert_eq!(pauli_repr.coeff(code_i0), Some(&1.0));
/// assert_eq!(pauli_repr.coeff(code_z0), Some(&-1.0));
/// #   Ok(())
/// # }
/// ```
pub struct JordanWigner<'a, T> {
    repr: &'a SumRepr<T, Fermions>,
}

impl<'a, T> JordanWigner<'a, T> {
    #[must_use]
    pub fn new(repr: &'a SumRepr<T, Fermions>) -> Self {
        Self {
            repr,
        }
    }
}

impl<'a, T> Terms<(T, Pauli)> for JordanWigner<'a, T>
where
    T: Float,
{
    type Error = Error;

    fn add_to(
        self,
        repr: &mut impl Extend<(T, Pauli)>,
    ) -> Result<(), Error> {
        for (&coeff, &code) in self.repr.iter() {
            match code {
                Fermions::Offset => {
                    repr.extend(Some((coeff, Pauli::identity())));
                }
                Fermions::One {
                    cr,
                    an,
                } => {
                    let jw_cr = Map::try_from(cr)?;
                    let jw_an = Map::try_from(an)?;
                    repr.extend(jw_map_two(&jw_cr, &jw_an, coeff));
                }
                Fermions::Two {
                    cr,
                    an,
                } => {
                    let jw_cr = (Map::try_from(cr.0)?, Map::try_from(cr.1)?);
                    let jw_an = (Map::try_from(an.0)?, Map::try_from(an.1)?);
                    repr.extend(jw_map_four(
                        &jw_cr.0, &jw_cr.1, &jw_an.0, &jw_an.1, coeff,
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use PauliOp::*;
    use ReIm::*;

    use super::*;

    #[test]
    fn mul_iter_01() {
        let jw_an = Map::try_from(An(Orbital::with_index(0))).unwrap();

        let result: Vec<_> =
            jw_an.mul_iter([(Re(2.0), Pauli::identity())]).collect();

        assert_eq!(
            result,
            &[
                (Re(1.0), Pauli::with_ops([X])),
                (Im(1.0), Pauli::with_ops([Y])),
            ]
        );
    }

    #[test]
    fn mul_iter_02() {
        let jw_cr = Map::try_from(Cr(Orbital::with_index(0))).unwrap();

        let result: Vec<_> =
            jw_cr.mul_iter([(Re(2.0), Pauli::identity())]).collect();

        assert_eq!(
            result,
            &[
                (Re(1.0), Pauli::with_ops([X])),
                (Im(-1.0), Pauli::with_ops([Y])),
            ]
        );
    }

    #[test]
    fn mul_iter_03() {
        let jw_an = Map::try_from(An(Orbital::with_index(3))).unwrap();

        let result: Vec<_> =
            jw_an.mul_iter([(Re(2.0), Pauli::identity())]).collect();

        assert_eq!(
            result,
            &[
                (Re(1.0), Pauli::with_ops([Z, Z, Z, X])),
                (Im(1.0), Pauli::with_ops([Z, Z, Z, Y])),
            ]
        );
    }

    #[test]
    fn mul_iter_04() {
        let jw_cr = Map::try_from(Cr(Orbital::with_index(3))).unwrap();

        let result: Vec<_> =
            jw_cr.mul_iter([(Re(2.0), Pauli::identity())]).collect();

        assert_eq!(
            result,
            &[
                (Re(1.0), Pauli::with_ops([Z, Z, Z, X])),
                (Im(-1.0), Pauli::with_ops([Z, Z, Z, Y])),
            ]
        );
    }

    #[test]
    fn mul_iter_05() {
        let jw_an_1 = Map::try_from(An(Orbital::with_index(0))).unwrap();
        let jw_an_2 = Map::try_from(An(Orbital::with_index(0))).unwrap();

        let result: Vec<_> = jw_an_1
            .mul_iter(jw_an_2.mul_iter([(Re(4.0), Pauli::identity())]))
            .collect();

        assert_eq!(
            result,
            &[
                (Re(1.0), Pauli::with_ops([I])),
                (Re(1.0), Pauli::with_ops([Z])),
                (Re(-1.0), Pauli::with_ops([Z])),
                (Re(-1.0), Pauli::with_ops([I])),
            ]
        );
    }

    #[test]
    fn mul_iter_06() {
        let jw_cr_1 = Map::try_from(Cr(Orbital::with_index(0))).unwrap();
        let jw_cr_2 = Map::try_from(Cr(Orbital::with_index(0))).unwrap();

        let result: Vec<_> = jw_cr_1
            .mul_iter(jw_cr_2.mul_iter([(Re(4.0), Pauli::identity())]))
            .collect();

        assert_eq!(
            result,
            &[
                (Re(1.0), Pauli::with_ops([I])),
                (Re(-1.0), Pauli::with_ops([Z])),
                (Re(1.0), Pauli::with_ops([Z])),
                (Re(-1.0), Pauli::with_ops([I])),
            ]
        );
    }

    #[test]
    fn mul_iter_07() {
        let jw_an = Map::try_from(An(Orbital::with_index(0))).unwrap();
        let jw_cr = Map::try_from(Cr(Orbital::with_index(0))).unwrap();

        let result: Vec<_> = jw_cr
            .mul_iter(jw_an.mul_iter([(Re(4.0), Pauli::identity())]))
            .collect();

        assert_eq!(
            result,
            &[
                (Re(1.0), Pauli::with_ops([I])),
                (Re(-1.0), Pauli::with_ops([Z])),
                (Re(-1.0), Pauli::with_ops([Z])),
                (Re(1.0), Pauli::with_ops([I])),
            ]
        );
    }

    #[test]
    fn mul_iter_08() {
        let jw_an = Map::try_from(An(Orbital::with_index(2))).unwrap();
        let jw_cr = Map::try_from(Cr(Orbital::with_index(2))).unwrap();

        let result: Vec<_> = jw_cr
            .mul_iter(jw_an.mul_iter([(Re(4.0), Pauli::identity())]))
            .collect();

        assert_eq!(
            result,
            &[
                (Re(1.0), Pauli::with_ops([I])),
                (Re(-1.0), Pauli::with_ops([I, I, Z])),
                (Re(-1.0), Pauli::with_ops([I, I, Z])),
                (Re(1.0), Pauli::with_ops([I])),
            ]
        );
    }

    #[test]
    fn mul_iter_09() {
        let jw_an = Map::try_from(An(Orbital::with_index(0))).unwrap();
        let jw_cr = Map::try_from(Cr(Orbital::with_index(1))).unwrap();

        let result: Vec<_> = jw_cr
            .mul_iter(jw_an.mul_iter([(Re(4.0), Pauli::identity())]))
            .collect();

        assert_eq!(
            result,
            &[
                (Im(1.0), Pauli::with_ops([Y, X])),
                (Re(1.0), Pauli::with_ops([Y, Y])),
                (Re(1.0), Pauli::with_ops([X, X])),
                (Im(-1.0), Pauli::with_ops([X, Y])),
            ]
        );
    }

    #[test]
    fn mul_iter_10() {
        let jw_an = Map::try_from(An(Orbital::with_index(1))).unwrap();
        let jw_cr = Map::try_from(Cr(Orbital::with_index(0))).unwrap();

        let result: Vec<_> = jw_cr
            .mul_iter(jw_an.mul_iter([(Re(4.0), Pauli::identity())]))
            .collect();

        assert_eq!(
            result,
            &[
                (Im(-1.0), Pauli::with_ops([Y, X])),
                (Re(1.0), Pauli::with_ops([X, X])),
                (Re(1.0), Pauli::with_ops([Y, Y])),
                (Im(1.0), Pauli::with_ops([X, Y])),
            ]
        );
    }

    #[test]
    fn mul_iter_11() {
        let jw_an = Map::try_from(An(Orbital::with_index(0))).unwrap();
        let jw_cr = Map::try_from(Cr(Orbital::with_index(2))).unwrap();

        let result: Vec<_> = jw_cr
            .mul_iter(jw_an.mul_iter([(Re(4.0), Pauli::identity())]))
            .collect();

        assert_eq!(
            result,
            &[
                (Im(1.0), Pauli::with_ops([Y, Z, X])),
                (Re(1.0), Pauli::with_ops([Y, Z, Y])),
                (Re(1.0), Pauli::with_ops([X, Z, X])),
                (Im(-1.0), Pauli::with_ops([X, Z, Y])),
            ]
        );
    }
}
