use num::Float;

use crate::{
    q2::{
        Integral,
        Orbital,
    },
    Hamil,
    Pauli,
    PauliCode,
    SumRepr,
    Terms,
};

impl<T> From<SumRepr<T, Integral>> for SumRepr<T, PauliCode>
where
    T: Float,
{
    fn from(value: SumRepr<T, Integral>) -> Self {
        let mut pauli_repr = SumRepr::new();
        for (&code, &coeff) in value.as_map() {
            match code {
                Integral::Constant => {
                    pauli_repr.add(PauliCode::default(), coeff);
                }
                Integral::OneElectron {
                    cr,
                    an,
                } => pauli_add_one_electron_integral(
                    cr,
                    an,
                    coeff,
                    &mut pauli_repr,
                ),
                Integral::TwoElectron {
                    cr,
                    an,
                } => pauli_add_two_electron_integral(
                    cr,
                    an,
                    coeff,
                    &mut pauli_repr,
                ),
            }
        }
        pauli_repr
    }
}

fn pauli_add_one_electron_integral<T: Float>(
    cr: Orbital,
    an: Orbital,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    if cr == an {
        pauli_add_one_electron_integral_equal(cr, an, coeff, pauli_repr);
    } else {
        pauli_add_one_electron_integral_nonequal(cr, an, coeff, pauli_repr);
    }
}

fn pauli_add_one_electron_integral_equal<T: Float>(
    cr: Orbital,
    _an: Orbital,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    let one_half =
        T::from(0.5).expect("cannot obtain floating point fraction: 0.5");

    let code = PauliCode::default();
    pauli_repr.add(code, coeff * one_half);

    let mut code = PauliCode::default();
    code.set(cr.enumerate(), Pauli::Z);
    pauli_repr.add(code, -coeff * one_half);
}

fn pauli_add_one_electron_integral_nonequal<T: Float>(
    cr: Orbital,
    an: Orbital,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    let one_half =
        T::from(0.5).expect("cannot obtain floating point fraction: 0.5");

    let mut code = PauliCode::default();
    // we know that orbitals are ordered: cr <= an
    for i in cr.enumerate() + 1..an.enumerate() {
        code.set(i, Pauli::Z);
    }
    code.set(cr.enumerate(), Pauli::X);
    code.set(an.enumerate(), Pauli::X);
    pauli_repr.add(code, coeff * one_half);

    code.set(cr.enumerate(), Pauli::Y);
    code.set(an.enumerate(), Pauli::Y);
    pauli_repr.add(code, -coeff * one_half);
}

fn pauli_add_two_electron_integral<T: Float>(
    cr: (Orbital, Orbital),
    an: (Orbital, Orbital),
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    let (p, q, r, s) = (
        cr.0.enumerate(),
        cr.1.enumerate(),
        an.0.enumerate(),
        an.1.enumerate(),
    );

    if p == s && q == r {
        pauli_add_two_electron_integral_pq(p, q, coeff, pauli_repr);
    } else if q == r {
        pauli_add_two_electron_integral_pqs(p, q, s, coeff, pauli_repr);
    } else {
        todo!()
    }
}

fn pauli_add_two_electron_integral_pq<T: Float>(
    p: usize,
    q: usize,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    let frac =
        T::from(0.25).expect("cannot obtain floating point fraction: 0.25");

    let mut code = PauliCode::default();
    // I
    pauli_repr.add(code, coeff * frac);
    code.set(p, Pauli::Z);
    // Z_p
    pauli_repr.add(code, -coeff * frac);
    code.set(p, Pauli::I);
    code.set(q, Pauli::Z);
    // Z_q
    pauli_repr.add(code, -coeff * frac);
    code.set(p, Pauli::Z);
    // Z_p Z_q
    pauli_repr.add(code, coeff * frac);
}

fn pauli_add_two_electron_integral_pqs<T: Float>(
    _p: usize,
    _q: usize,
    _s: usize,
    _coeff: T,
    _pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    todo!()
}

impl<T> Terms<T, PauliCode> for Hamil<T, Integral>
where
    T: Float,
{
    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, PauliCode>,
    ) {
        let mut fermi_repr = SumRepr::new();
        <Self as Terms<T, Integral>>::add_to(self, &mut fermi_repr);
        let mut pauli_repr = SumRepr::<T, PauliCode>::from(fermi_repr);
        pauli_repr.add_to(repr);
    }
}
