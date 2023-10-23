use f2q::{
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
    map::JordanWigner,
    terms::{
        FermiSum,
        PauliSum,
        SumRepr,
        Terms,
    },
};

const MOCK_COEFF: f64 = 0.12345;

#[test]
fn jordan_wigner_01() {
    let mut fermi_sum = FermiSum::new();
    fermi_sum.add_term(Fermions::Offset, MOCK_COEFF);

    let mut pauli_sum = PauliSum::new();
    JordanWigner::new(&fermi_sum)
        .add_to(&mut pauli_sum)
        .unwrap();

    let coeff = pauli_sum.coeff(Pauli::default());
    assert!(
        (coeff - MOCK_COEFF).abs() < f64::EPSILON,
        "{MOCK_COEFF} {coeff}"
    );
}

fn check_jordan_wigner_one_pp(index: u32) {
    let mut fermi_sum = SumRepr::new();

    let p = Orbital::from_index(index);
    let integral = Fermions::one_electron(Cr(p), An(p)).unwrap();
    fermi_sum.add_term(integral, MOCK_COEFF);

    let mut pauli_sum = PauliSum::new();
    JordanWigner::new(&fermi_sum)
        .add_to(&mut pauli_sum)
        .unwrap();

    let code = Pauli::default();
    let coeff = pauli_sum.coeff(code);
    let expected = MOCK_COEFF * 0.5;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let code = {
        let mut code = Pauli::default();
        code.set(u16::try_from(index).unwrap(), PauliOp::Z);
        code
    };
    let coeff = pauli_sum.coeff(code);
    let expected = -MOCK_COEFF * 0.5;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );
}

#[test]
fn jordan_wigner_one_pp() {
    check_jordan_wigner_one_pp(0);
    check_jordan_wigner_one_pp(1);
    check_jordan_wigner_one_pp(2);
    check_jordan_wigner_one_pp(63);
}

fn check_jordan_wigner_one_pq(
    index1: u16,
    index2: u16,
) {
    let mut fermi_sum = SumRepr::new();

    assert!(index1 < index2);
    let p = Orbital::from_index(u32::from(index1));
    let q = Orbital::from_index(u32::from(index2));
    let integral = Fermions::one_electron(Cr(p), An(q)).unwrap();
    fermi_sum.add_term(integral, MOCK_COEFF);

    let mut pauli_sum = PauliSum::new();
    JordanWigner::new(&fermi_sum)
        .add_to(&mut pauli_sum)
        .unwrap();

    let mut code = Pauli::default();
    for i in index1 + 1..index2 {
        code.set(i, PauliOp::Z);
    }
    code.set(index1, PauliOp::X);
    code.set(index2, PauliOp::X);
    let coeff = pauli_sum.coeff(code);
    let expected = MOCK_COEFF * 0.5;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    code.set(index1, PauliOp::Y);
    code.set(index2, PauliOp::Y);
    let coeff = pauli_sum.coeff(code);
    let expected = MOCK_COEFF * 0.5;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );
}

#[test]
fn jordan_wigner_one_pq() {
    check_jordan_wigner_one_pq(0, 1);
    check_jordan_wigner_one_pq(0, 3);
    check_jordan_wigner_one_pq(0, 17);

    check_jordan_wigner_one_pq(11, 17);
    check_jordan_wigner_one_pq(11, 47);
}

fn check_jordan_wigner_two_pq(
    index1: u16,
    index2: u16,
) {
    let mut fermi_sum = SumRepr::new();

    assert!(index1 < index2);
    let p = Orbital::from_index(u32::from(index1));
    let q = Orbital::from_index(u32::from(index2));
    let integral =
        Fermions::two_electron((Cr(p), Cr(q)), (An(q), An(p))).unwrap();
    fermi_sum.add_term(integral, MOCK_COEFF);

    let mut pauli_sum = PauliSum::new();
    JordanWigner::new(&fermi_sum)
        .add_to(&mut pauli_sum)
        .unwrap();

    let code = Pauli::default();
    let coeff = pauli_sum.coeff(code);
    let expected = MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = Pauli::default();
    code.set(index1, PauliOp::Z);
    let coeff = pauli_sum.coeff(code);
    let expected = -MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = Pauli::default();
    code.set(index2, PauliOp::Z);
    let coeff = pauli_sum.coeff(code);
    let expected = -MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = Pauli::default();
    code.set(index1, PauliOp::Z);
    code.set(index2, PauliOp::Z);
    let coeff = pauli_sum.coeff(code);
    let expected = MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );
}

#[test]
fn jordan_wigner_two_pq() {
    check_jordan_wigner_two_pq(0, 1);
    check_jordan_wigner_two_pq(0, 2);
    check_jordan_wigner_two_pq(0, 3);

    check_jordan_wigner_two_pq(11, 13);
    check_jordan_wigner_two_pq(11, 33);
}

fn check_jordan_wigner_two_pqs(
    index1: u16,
    index2: u16,
    index3: u16,
) {
    let mut fermi_sum = SumRepr::new();

    assert!(index1 < index2);
    assert!(index2 > index3);
    assert!(index1 <= index3);

    let p = Orbital::from_index(u32::from(index1));
    let q = Orbital::from_index(u32::from(index2));
    let s = Orbital::from_index(u32::from(index3));
    let integral =
        Fermions::two_electron((Cr(p), Cr(q)), (An(q), An(s))).unwrap();
    fermi_sum.add_term(integral, MOCK_COEFF);

    let mut pauli_sum = PauliSum::new();
    JordanWigner::new(&fermi_sum)
        .add_to(&mut pauli_sum)
        .unwrap();

    let mut code = Pauli::default();
    for i in index1 + 1..index3 {
        code.set(i, PauliOp::Z);
    }
    code.set(index1, PauliOp::X);
    code.set(index3, PauliOp::X);
    let coeff = pauli_sum.coeff(code);
    let expected = MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = Pauli::default();
    for i in index1 + 1..index3 {
        code.set(i, PauliOp::Z);
    }
    code.set(index1, PauliOp::Y);
    code.set(index3, PauliOp::Y);
    let coeff = pauli_sum.coeff(code);
    let expected = MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = Pauli::default();
    for i in index1 + 1..index3 {
        code.set(i, PauliOp::Z);
    }
    code.set(index1, PauliOp::X);
    code.set(index3, PauliOp::X);
    code.set(index2, PauliOp::Z);
    let coeff = pauli_sum.coeff(code);
    let expected = -MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = Pauli::default();
    for i in index1 + 1..index3 {
        code.set(i, PauliOp::Z);
    }
    code.set(index1, PauliOp::Y);
    code.set(index3, PauliOp::Y);
    code.set(index2, PauliOp::Z);
    let coeff = pauli_sum.coeff(code);
    let expected = -MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );
}

#[test]
fn jordan_wigner_two_pqs() {
    check_jordan_wigner_two_pqs(0, 2, 1);
    check_jordan_wigner_two_pqs(0, 7, 3);
    check_jordan_wigner_two_pqs(11, 13, 12);

    check_jordan_wigner_two_pqs(11, 37, 22);
}

#[allow(clippy::too_many_lines)]
fn check_jordan_wigner_two_pqrs(
    index1: u16,
    index2: u16,
    index3: u16,
    index4: u16,
) {
    let mut fermi_sum = SumRepr::new();

    assert!(index1 < index2);
    assert!(index3 > index4);
    assert!(index1 <= index4);

    let p = Orbital::from_index(u32::from(index1));
    let q = Orbital::from_index(u32::from(index2));
    let r = Orbital::from_index(u32::from(index3));
    let s = Orbital::from_index(u32::from(index4));
    let integral =
        Fermions::two_electron((Cr(p), Cr(q)), (An(r), An(s))).unwrap();
    fermi_sum.add_term(integral, MOCK_COEFF);

    let mut pauli_sum = PauliSum::new();
    JordanWigner::new(&fermi_sum)
        .add_to(&mut pauli_sum)
        .unwrap();

    let base_code = {
        let mut code = Pauli::default();
        for i in index1 + 1..index2 {
            code.set(i, PauliOp::Z);
        }
        for i in index4 + 1..index3 {
            code.set(i, PauliOp::Z);
        }
        code
    };

    let mut code = base_code;
    code.set(index1, PauliOp::X);
    code.set(index2, PauliOp::X);
    code.set(index3, PauliOp::X);
    code.set(index4, PauliOp::X);
    let coeff = pauli_sum.coeff(code);
    let expected = MOCK_COEFF * 0.125;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = base_code;
    code.set(index1, PauliOp::X);
    code.set(index2, PauliOp::X);
    code.set(index3, PauliOp::Y);
    code.set(index4, PauliOp::Y);
    let coeff = pauli_sum.coeff(code);
    let expected = -MOCK_COEFF * 0.125;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = base_code;
    code.set(index1, PauliOp::X);
    code.set(index2, PauliOp::Y);
    code.set(index3, PauliOp::X);
    code.set(index4, PauliOp::Y);
    let coeff = pauli_sum.coeff(code);
    let expected = MOCK_COEFF * 0.125;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = base_code;
    code.set(index1, PauliOp::Y);
    code.set(index2, PauliOp::X);
    code.set(index3, PauliOp::X);
    code.set(index4, PauliOp::Y);
    let coeff = pauli_sum.coeff(code);
    let expected = MOCK_COEFF * 0.125;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = base_code;
    code.set(index1, PauliOp::Y);
    code.set(index2, PauliOp::X);
    code.set(index3, PauliOp::Y);
    code.set(index4, PauliOp::X);
    let coeff = pauli_sum.coeff(code);
    let expected = MOCK_COEFF * 0.125;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = base_code;
    code.set(index1, PauliOp::Y);
    code.set(index2, PauliOp::Y);
    code.set(index3, PauliOp::X);
    code.set(index4, PauliOp::X);
    let coeff = pauli_sum.coeff(code);
    let expected = -MOCK_COEFF * 0.125;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = base_code;
    code.set(index1, PauliOp::X);
    code.set(index2, PauliOp::Y);
    code.set(index3, PauliOp::Y);
    code.set(index4, PauliOp::X);
    let coeff = pauli_sum.coeff(code);
    let expected = MOCK_COEFF * 0.125;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = base_code;
    code.set(index1, PauliOp::Y);
    code.set(index2, PauliOp::Y);
    code.set(index3, PauliOp::Y);
    code.set(index4, PauliOp::Y);
    let coeff = pauli_sum.coeff(code);
    let expected = MOCK_COEFF * 0.125;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );
}

#[test]
fn jordan_wigner_two_pqrs() {
    check_jordan_wigner_two_pqrs(0, 1, 2, 0);
    check_jordan_wigner_two_pqrs(0, 1, 2, 1);
    check_jordan_wigner_two_pqrs(0, 1, 3, 2);

    check_jordan_wigner_two_pqrs(11, 32, 31, 19);
    check_jordan_wigner_two_pqrs(11, 31, 61, 29);
}
