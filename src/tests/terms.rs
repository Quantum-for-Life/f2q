use crate::{
    code::qubits::Pauli,
    terms::{
        HeapRepr,
        StackRepr,
        SumRepr,
        Terms,
    },
};

#[test]
fn sumrepr_init_01() {
    let code = Pauli::new((1234, 0));
    let mut hamil = SumRepr::new();

    hamil.update(code, 4321.);
    let coeff = hamil.coeff(code).unwrap();
    assert!(f64::abs(coeff - 4321.) < f64::EPSILON);
}

#[test]
fn sumrepr_from_array() {
    let arr = [(1.0, 1), (2.0, 2)];
    let repr = SumRepr::from(arr);

    assert_eq!(repr.len(), 2);
}

#[test]
fn sumrepr_extend() {
    let iter = (0..3).map(|i| (1.0, i));
    let mut repr = SumRepr::new();

    repr.extend(iter);
    assert_eq!(repr.len(), 3);
}

#[test]
fn sumrepr_terms() {
    let arr = [(1.0, 1), (2.0, 2)];
    let repr = SumRepr::from(arr);
    assert_eq!(repr.len(), 2);

    let mut elems = vec![];
    repr.add_to(&mut elems).unwrap();
    elems.sort_by(|a, b| a.1.cmp(&b.1));

    assert_eq!(elems, &[(1.0, 1), (2.0, 2)]);
}

#[test]
fn stackrepr_terms() {
    let mut iter = [(1.0, 1), (2.0, 2)].into_iter();
    let repr = StackRepr::new(|| iter.next());

    let mut elems = vec![];
    repr.add_to(&mut elems).unwrap();
    elems.sort_by(|a, b| a.1.cmp(&b.1));
}

#[test]
fn heaprepr_terms() {
    let mut iter = [(1.0, 1), (2.0, 2)].into_iter();
    let repr = HeapRepr::new(|| iter.next());

    let mut elems = vec![];
    repr.add_to(&mut elems).unwrap();
    elems.sort_by(|a, b| a.1.cmp(&b.1));
}
