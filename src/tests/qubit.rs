use crate::{
    code::qubits::{
        Pauli,
        PauliGroup,
        PauliOp,
    },
    math::{
        Group,
        Root4,
    },
    Error,
};

#[test]
fn init() {
    let code = Pauli::new((0b01, 0b00));
    assert_eq!(code.index(), 0b01);
}

#[test]
fn default() {
    let code = Pauli::default();
    assert_eq!(code, Pauli::new((0, 0)));
}

#[test]
fn pauli_02() {
    let code = Pauli::new((0b0101, 0b00));

    assert_eq!(code.pauli(0), Some(PauliOp::X));
    assert_eq!(code.pauli(1), Some(PauliOp::X));
    assert_eq!(code.pauli(2), Some(PauliOp::I));
    assert_eq!(code.pauli(63), Some(PauliOp::I));

    assert_eq!(code.pauli(64), None);
    assert_eq!(code.pauli(123), None);
}

#[test]
fn pauli_mut_01() {
    let mut code = Pauli::default();
    assert_eq!(code.pauli(7).unwrap(), PauliOp::I);

    code.pauli_mut(7, |x| {
        if let Some(pauli) = x {
            *pauli = PauliOp::Z;
        }
    });
    assert_eq!(code.pauli(7).unwrap(), PauliOp::Z);
}

#[test]
fn set_pauli_01() {
    let mut code = Pauli::new((29_332_281_938, 0b00));
    assert_eq!(code.pauli(7).unwrap(), PauliOp::I);

    code.set(7, PauliOp::Y);
    assert_eq!(code.pauli(7).unwrap(), PauliOp::Y);
}

#[test]
#[should_panic(expected = "index should be within 0..64")]
fn set_pauli_02() {
    let mut code = Pauli::default();
    assert_eq!(code.pauli(7).unwrap(), PauliOp::I);

    code.set(65, PauliOp::Y);
    assert_eq!(code.pauli(7).unwrap(), PauliOp::Y);
}

#[test]
fn set_pauli_03() {
    let mut code = Pauli::default();

    for i in 0..13 {
        code.set(i, PauliOp::X);
    }
    for i in 13..29 {
        code.set(i, PauliOp::Y);
    }
    for i in 29..61 {
        code.set(i, PauliOp::Z);
    }

    for i in 0..13 {
        assert_eq!(code.pauli(i).unwrap(), PauliOp::X, "{i}");
    }
    for i in 13..29 {
        assert_eq!(code.pauli(i).unwrap(), PauliOp::Y, "{i}");
    }
    for i in 29..61 {
        assert_eq!(code.pauli(i).unwrap(), PauliOp::Z, "{i}");
    }
    for i in 61..64 {
        assert_eq!(code.pauli(i).unwrap(), PauliOp::I, "{i}");
    }
}

#[test]
fn codes_iter_01() {
    use PauliOp::*;
    let result = Pauli::new((0b01, 0b00))
        .into_iter()
        .take(3)
        .collect::<Vec<_>>();

    assert_eq!(result, &[X, I, I]);
}

#[test]
fn codes_iter_02() {
    use PauliOp::*;
    let result = Pauli::new((0b11_1001, 0b00))
        .into_iter()
        .take(5)
        .collect::<Vec<_>>();

    assert_eq!(result, &[X, Y, Z, I, I]);
}

#[test]
fn codes_iter_03() {
    use PauliOp::*;
    let result = Pauli::new((0b0101_0000, 0b1111_1010))
        .into_iter()
        .take(36)
        .collect::<Vec<_>>();

    assert_eq!(
        result,
        &[
            I, I, X, X, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I,
            I, I, I, I, I, I, I, I, I, I, Y, Y, Z, Z
        ]
    );
}

#[test]
fn from_paulis_01() {
    use PauliOp::*;

    assert_eq!(
        Pauli::with_ops([I, X, Y, Z]),
        Pauli::new((0b1110_0100, 0b00))
    );
}

#[test]
fn from_paulis_02() {
    use PauliOp::*;

    assert_eq!(
        Pauli::with_ops([
            I, I, X, X, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I,
            I, I, I, I, I, I, I, I, I, I, Y, Y, Z, Z
        ]),
        Pauli::new((0b0101_0000, 0b1111_1010))
    );
}

#[test]
fn from_u128() {
    assert_eq!(Pauli::from(0u128).index(), 0u128);
    assert_eq!(Pauli::from(1u128).index(), 1u128);
    assert_eq!(
        Pauli::from(11_111_111_111_111_111_u128).index(),
        11_111_111_111_111_111_u128
    );
    assert_eq!(
        Pauli::from(1_234_567_898_765_432_112_345_678_987_654_321_u128).index(),
        1_234_567_898_765_432_112_345_678_987_654_321_u128
    );
    assert_eq!(Pauli::from(u128::MAX).index(), u128::MAX);
}

#[test]
fn pauli_num_nontrivial() {
    use PauliOp::*;

    assert_eq!(Pauli::with_ops([]).num_nontrivial(), 0);
    assert_eq!(Pauli::with_ops([X]).num_nontrivial(), 1);
    assert_eq!(Pauli::with_ops([Y, X]).num_nontrivial(), 2);
    assert_eq!(Pauli::with_ops([Z, Y, X]).num_nontrivial(), 3);

    assert_eq!(Pauli::with_ops([Y, I, X]).num_nontrivial(), 2);
    assert_eq!(Pauli::with_ops([Z, I, Y, I, X]).num_nontrivial(), 3);
    assert_eq!(Pauli::with_ops([Z, I, I, X]).num_nontrivial(), 2);
    assert_eq!(Pauli::with_ops([Z, I, I, I, I, Z]).num_nontrivial(), 2);

    assert_eq!(Pauli::new((u64::MAX, 0)).num_nontrivial(), 32);
    assert_eq!(Pauli::new((0, u64::MAX)).num_nontrivial(), 32);
    assert_eq!(Pauli::new((u64::MAX, u64::MAX)).num_nontrivial(), 64);

    assert_eq!(
        Pauli::with_ops([Y, I, X, Y, I, X, Y, I, X, Y, I, X]).num_nontrivial(),
        8
    );
    assert_eq!(
        Pauli::with_ops([
            Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y,
            I, X
        ])
        .num_nontrivial(),
        16
    );
    assert_eq!(
        Pauli::with_ops([
            Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y,
            I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I,
            X, Y, I, X
        ])
        .num_nontrivial(),
        32
    );
    assert_eq!(
        Pauli::with_ops([
            Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y,
            I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I,
            X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y,
        ])
        .num_nontrivial(),
        43
    );
}

#[test]
fn pauli_min_register_size_01() {
    use PauliOp::{
        I,
        X,
        Y,
        Z,
    };

    assert_eq!(Pauli::with_ops([]).min_register_size(), 0);

    assert_eq!(Pauli::with_ops([X]).min_register_size(), 1);
    assert_eq!(Pauli::with_ops([Y]).min_register_size(), 1);
    assert_eq!(Pauli::with_ops([Z]).min_register_size(), 1);

    assert_eq!(Pauli::with_ops([X, Y]).min_register_size(), 2);
    assert_eq!(Pauli::with_ops([Y, Z]).min_register_size(), 2);

    assert_eq!(Pauli::with_ops([X, Y, Z]).min_register_size(), 3);
    assert_eq!(Pauli::with_ops([I, X, Y, Z]).min_register_size(), 4);
    assert_eq!(Pauli::with_ops([I, X, I, Y, Z]).min_register_size(), 5);
    assert_eq!(Pauli::with_ops([I, X, I, Y, I, Z]).min_register_size(), 6);
    assert_eq!(
        Pauli::with_ops([I, X, I, Y, I, Z, I]).min_register_size(),
        6
    );
}

#[test]
fn pauli_min_register_size_02() {
    use PauliOp::{
        I,
        X,
        Y,
        Z,
    };

    assert_eq!(Pauli::new((u64::MAX, 0)).min_register_size(), 32);
    assert_eq!(Pauli::new((0, u64::MAX)).min_register_size(), 64);
    assert_eq!(Pauli::new((u64::MAX, u64::MAX)).min_register_size(), 64);

    assert_eq!(
        Pauli::with_ops([Y, I, X, Y, I, X, Y, I, X, Y, I, X])
            .min_register_size(),
        12
    );
    assert_eq!(
        Pauli::with_ops([
            Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y,
            I, X
        ])
        .min_register_size(),
        24
    );
    assert_eq!(
        Pauli::with_ops([
            Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y,
            I, X, Y, I, X, Y, I, X, Z, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I,
            X, Y, I, X
        ])
        .min_register_size(),
        48
    );
    assert_eq!(
        Pauli::with_ops([
            Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y,
            I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I,
            X, Y, I, X, Y, I, X, Y, I, X, Y, I, X, Y, I, X,
        ])
        .min_register_size(),
        60
    );
}

#[test]
fn pauli_ord() {
    assert!(Pauli::new((0, 0)) == Pauli::new((0, 0)));
    assert!(Pauli::new((0, 0)) >= Pauli::new((0, 0)));
    assert!(Pauli::new((0, 0)) <= Pauli::new((0, 0)));

    assert!(Pauli::new((1, 0)) > Pauli::new((0, 0)));
    assert!(Pauli::new((2, 0)) > Pauli::new((1, 0)));
    assert!(Pauli::new((2, 0)) > Pauli::new((0, 0)));

    assert!(Pauli::new((0, 1)) > Pauli::new((0, 0)));
    assert!(Pauli::new((0, 2)) > Pauli::new((0, 1)));
    assert!(Pauli::new((0, 2)) > Pauli::new((0, 0)));

    assert!(Pauli::new((0, 0)) < Pauli::new((1, 0)));
    assert!(Pauli::new((1, 0)) < Pauli::new((2, 0)));
    assert!(Pauli::new((0, 0)) < Pauli::new((2, 0)));

    assert!(Pauli::new((0, 0)) < Pauli::new((0, 1)));
    assert!(Pauli::new((0, 1)) < Pauli::new((0, 2)));
    assert!(Pauli::new((0, 0)) < Pauli::new((0, 2)));

    assert!(Pauli::new((0, 1)) > Pauli::new((u64::MAX, 0)));
    assert!(Pauli::new((0, 2)) > Pauli::new((u64::MAX, 0)));
    assert!(Pauli::new((u64::MAX, u64::MAX)) > Pauli::new((u64::MAX, 0)));
}

#[test]
fn identity() {
    let e = PauliGroup::identity();

    let g = PauliGroup::from(Pauli::new((0, 0)));
    assert_eq!(e * g, g);
    assert_eq!(g * e, g);

    let g = PauliGroup::from(Pauli::new((1, 2)));
    assert_eq!(e * g, g);
    assert_eq!(g * e, g);

    let g = PauliGroup::from(Pauli::new((12345, 67890)));
    assert_eq!(e * g, g);
    assert_eq!(g * e, g);
}

#[test]
fn paui_group_01() {
    let e = PauliGroup::identity();

    assert_eq!(e * PauliGroup::from(Root4::R0), PauliGroup::from(Root4::R0));
    assert_eq!(e * PauliGroup::from(Root4::R1), PauliGroup::from(Root4::R1));
    assert_eq!(e * PauliGroup::from(Root4::R2), PauliGroup::from(Root4::R2));
    assert_eq!(e * PauliGroup::from(Root4::R3), PauliGroup::from(Root4::R3));

    assert_eq!(PauliGroup::from(Root4::R0) * e, PauliGroup::from(Root4::R0));
    assert_eq!(PauliGroup::from(Root4::R1) * e, PauliGroup::from(Root4::R1));
    assert_eq!(PauliGroup::from(Root4::R2) * e, PauliGroup::from(Root4::R2));
    assert_eq!(PauliGroup::from(Root4::R3) * e, PauliGroup::from(Root4::R3));
}

#[test]
fn paui_group_02() {
    use PauliOp::*;
    let g = PauliGroup::new(Root4::R0, Pauli::with_ops([X, Y, Z]));
    let e = PauliGroup::identity();

    assert_eq!(g * g, e);
}

#[test]
fn paui_group_03() {
    use PauliOp::*;
    let g = PauliGroup::new(Root4::R0, Pauli::with_ops([X, Y, Z]));

    let h = PauliGroup::new(Root4::R0, Pauli::with_ops([X]));
    assert_eq!(
        g * h,
        PauliGroup::new(Root4::R0, Pauli::with_ops([I, Y, Z]))
    );
    assert_eq!(
        h * g,
        PauliGroup::new(Root4::R0, Pauli::with_ops([I, Y, Z]))
    );
}

#[test]
fn paui_group_04() {
    use PauliOp::*;
    let g = PauliGroup::new(Root4::R0, Pauli::with_ops([X, Y, Z]));

    let h = PauliGroup::new(Root4::R0, Pauli::with_ops([Y]));
    assert_eq!(
        g * h,
        PauliGroup::new(Root4::R2, Pauli::with_ops([Z, Y, Z]))
    );
    assert_eq!(
        h * g,
        PauliGroup::new(Root4::R3, Pauli::with_ops([Z, Y, Z]))
    );
}

#[test]
fn paui_group_05() {
    use PauliOp::*;
    let g = PauliGroup::new(Root4::R0, Pauli::with_ops([X, Y, Z]));

    let h = PauliGroup::new(Root4::R3, Pauli::with_ops([I, Z]));
    assert_eq!(
        g * h,
        PauliGroup::new(Root4::R0, Pauli::with_ops([X, X, Z]))
    );
    assert_eq!(
        h * g,
        PauliGroup::new(Root4::R1, Pauli::with_ops([X, X, Z]))
    );
}

#[test]
fn paui_group_06() {
    use PauliOp::*;
    let g = PauliGroup::new(Root4::R0, Pauli::with_ops([X, Y, Z]));

    let h = PauliGroup::new(Root4::R1, Pauli::with_ops([I, Z, X]));
    assert_eq!(
        g * h,
        PauliGroup::new(Root4::R0, Pauli::with_ops([X, X, Y]))
    );
    assert_eq!(
        h * g,
        PauliGroup::new(Root4::R0, Pauli::with_ops([X, X, Y]))
    );
}

#[test]
fn pauliop_01() {
    assert_eq!(PauliOp::try_from(0u32).unwrap(), PauliOp::I);
    assert_eq!(PauliOp::try_from(1u32).unwrap(), PauliOp::X);
    assert_eq!(PauliOp::try_from(2u32).unwrap(), PauliOp::Y);
    assert_eq!(PauliOp::try_from(3u32).unwrap(), PauliOp::Z);
}

#[test]
fn pauliop_02() {
    let err = PauliOp::try_from(4u16).unwrap_err();
    matches!(err, Error::QubitIndex { .. });
}

#[test]
fn pauliop_03() {
    assert_eq!(u8::from(PauliOp::I), 0);
    assert_eq!(u8::from(PauliOp::X), 1);
    assert_eq!(u8::from(PauliOp::Y), 2);
    assert_eq!(u8::from(PauliOp::Z), 3);
}
