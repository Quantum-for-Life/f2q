use num::Float;

use crate::{
    code::{
        fermions::Fermions,
        qubits::{
            Pauli,
            Sigma,
        },
    },
    map::JordanWigner,
    terms::{
        SumRepr,
        Terms,
    },
};

fn jw_get_result<T: Float>(repr: &SumRepr<T, Fermions>) -> Vec<(T, Pauli)> {
    let jw_map = JordanWigner::new(repr);
    let mut pauli_repr = SumRepr::new();
    jw_map.add_to(&mut pauli_repr).unwrap();
    let mut result = vec![];
    pauli_repr.add_to(&mut result).unwrap();
    result.sort_by(|(_, pauli_a), (_, pauli_b)| pauli_a.cmp(pauli_b));
    result
}

fn jw_check_mapping<T: Float + std::fmt::Debug>(
    list: &[(T, Fermions)],
    expected: &[(T, Pauli)],
) {
    let repr: SumRepr<T, Fermions> =
        list.iter().map(|(x, f)| (*x, *f)).collect();
    assert_eq!(jw_get_result(&repr), expected);
}

#[test]
fn jw_offset() {
    jw_check_mapping(&[(1.0, Fermions::Offset)], &[(1.0, Pauli::identity())]);

    jw_check_mapping(
        &[(1.0, Fermions::Offset), (2.0, Fermions::Offset)],
        &[(3.0, Pauli::identity())],
    );
}

#[test]
fn jw_one_elec_01() {
    use Sigma::*;

    jw_check_mapping(
        &[(1.0, Fermions::try_from((0, 0)).unwrap())],
        &[(1.0, Pauli::identity()), (-1.0, Pauli::with_ops([Z]))],
    );

    jw_check_mapping(
        &[(1.0, Fermions::try_from((1, 1)).unwrap())],
        &[(1.0, Pauli::identity()), (-1.0, Pauli::with_ops([I, Z]))],
    );

    jw_check_mapping(
        &[(1.0, Fermions::try_from((2, 2)).unwrap())],
        &[(1.0, Pauli::identity()), (-1.0, Pauli::with_ops([I, I, Z]))],
    );

    jw_check_mapping(
        &[(1.0, Fermions::try_from((32, 32)).unwrap())],
        &[(1.0, Pauli::identity()), (-1.0, Pauli::new((0, 0b11)))],
    );

    jw_check_mapping(
        &[(1.0, Fermions::try_from((33, 33)).unwrap())],
        &[(1.0, Pauli::identity()), (-1.0, Pauli::new((0, 0b1100)))],
    );

    jw_check_mapping(
        &[(1.0, Fermions::try_from((63, 63)).unwrap())],
        &[
            (1.0, Pauli::identity()),
            (-1.0, Pauli::new((0, 0b11 << 62))),
        ],
    );
}

#[test]
fn jw_one_elec_02() {
    use Sigma::*;

    jw_check_mapping(
        &[(2.0, Fermions::try_from((0, 1)).unwrap())],
        &[
            (1.0, Pauli::with_ops([X, X])),
            (1.0, Pauli::with_ops([Y, Y])),
        ],
    );

    jw_check_mapping(
        &[(2.0, Fermions::try_from((0, 2)).unwrap())],
        &[
            (1.0, Pauli::with_ops([X, Z, X])),
            (1.0, Pauli::with_ops([Y, Z, Y])),
        ],
    );

    jw_check_mapping(
        &[(2.0, Fermions::try_from((1, 4)).unwrap())],
        &[
            (1.0, Pauli::with_ops([I, X, Z, Z, X])),
            (1.0, Pauli::with_ops([I, Y, Z, Z, Y])),
        ],
    );

    jw_check_mapping(
        &[(2.0, Fermions::try_from((2, 5)).unwrap())],
        &[
            (1.0, Pauli::with_ops([I, I, X, Z, Z, X])),
            (1.0, Pauli::with_ops([I, I, Y, Z, Z, Y])),
        ],
    );
}

#[test]
fn jw_one_elec_03() {
    jw_check_mapping(
        &[(2.0, Fermions::try_from((0, 32)).unwrap())],
        &[
            (1.0, Pauli::new((0xffff_ffff_ffff_fffd, 0x1))),
            (1.0, Pauli::new((0xffff_ffff_ffff_fffe, 0x2))),
        ],
    );

    jw_check_mapping(
        &[(2.0, Fermions::try_from((0, 63)).unwrap())],
        &[
            (
                1.0,
                Pauli::new((0xffff_ffff_ffff_fffd, 0x7fff_ffff_ffff_ffff)),
            ),
            (
                1.0,
                Pauli::new((0xffff_ffff_ffff_fffe, 0xbfff_ffff_ffff_ffff)),
            ),
        ],
    );
}

#[test]
fn jw_two_elec_01() {
    use Sigma::*;

    jw_check_mapping(
        &[(2.0, Fermions::try_from((0, 1, 1, 0)).unwrap())],
        &[
            (1.0, Pauli::identity()),
            (-1.0, Pauli::with_ops([Z])),
            (-1.0, Pauli::with_ops([I, Z])),
            (1.0, Pauli::with_ops([Z, Z])),
        ],
    );

    jw_check_mapping(
        &[(2.0, Fermions::try_from((0, 2, 2, 0)).unwrap())],
        &[
            (1.0, Pauli::identity()),
            (-1.0, Pauli::with_ops([Z])),
            (-1.0, Pauli::with_ops([I, I, Z])),
            (1.0, Pauli::with_ops([Z, I, Z])),
        ],
    );

    jw_check_mapping(
        &[(2.0, Fermions::try_from((1, 3, 3, 1)).unwrap())],
        &[
            (1.0, Pauli::identity()),
            (-1.0, Pauli::with_ops([I, Z])),
            (-1.0, Pauli::with_ops([I, I, I, Z])),
            (1.0, Pauli::with_ops([I, Z, I, Z])),
        ],
    );
}

#[test]
fn jw_two_elec_02() {
    jw_check_mapping(
        &[(2.0, Fermions::try_from((0, 32, 32, 0)).unwrap())],
        &[
            (1.0, Pauli::identity()),
            (-1.0, Pauli::new((3, 0))),
            (-1.0, Pauli::new((0, 3))),
            (1.0, Pauli::new((3, 3))),
        ],
    );

    jw_check_mapping(
        &[(2.0, Fermions::try_from((0, 63, 63, 0)).unwrap())],
        &[
            (1.0, Pauli::identity()),
            (-1.0, Pauli::new((3, 0))),
            (-1.0, Pauli::new((0, 0b11 << 62))),
            (1.0, Pauli::new((3, 0b11 << 62))),
        ],
    );

    jw_check_mapping(
        &[(2.0, Fermions::try_from((32, 63, 63, 32)).unwrap())],
        &[
            (1.0, Pauli::identity()),
            (-1.0, Pauli::new((0, 3))),
            (-1.0, Pauli::new((0, 0b11 << 62))),
            (1.0, Pauli::new((0, 3 + (0b11 << 62)))),
        ],
    );
}

#[test]
fn jw_two_elec_03() {
    use Sigma::*;

    jw_check_mapping(
        &[(4.0, Fermions::try_from((0, 2, 2, 1)).unwrap())],
        &[
            (1.0, Pauli::with_ops([X, X, I])),
            (1.0, Pauli::with_ops([Y, Y, I])),
            (-1.0, Pauli::with_ops([X, X, Z])),
            (-1.0, Pauli::with_ops([Y, Y, Z])),
        ],
    );

    jw_check_mapping(
        &[(4.0, Fermions::try_from((1, 3, 3, 2)).unwrap())],
        &[
            (1.0, Pauli::with_ops([I, X, X, I])),
            (1.0, Pauli::with_ops([I, Y, Y, I])),
            (-1.0, Pauli::with_ops([I, X, X, Z])),
            (-1.0, Pauli::with_ops([I, Y, Y, Z])),
        ],
    );

    jw_check_mapping(
        &[(4.0, Fermions::try_from((1, 4, 4, 2)).unwrap())],
        &[
            (1.0, Pauli::with_ops([I, X, X, I, I])),
            (1.0, Pauli::with_ops([I, Y, Y, I, I])),
            (-1.0, Pauli::with_ops([I, X, X, I, Z])),
            (-1.0, Pauli::with_ops([I, Y, Y, I, Z])),
        ],
    );
}

#[test]
fn jw_two_elec_04() {
    use Sigma::*;

    jw_check_mapping(
        &[(4.0, Fermions::try_from((0, 3, 3, 2)).unwrap())],
        &[
            (1.0, Pauli::with_ops([X, Z, X, I])),
            (1.0, Pauli::with_ops([Y, Z, Y, I])),
            (-1.0, Pauli::with_ops([X, Z, X, Z])),
            (-1.0, Pauli::with_ops([Y, Z, Y, Z])),
        ],
    );

    jw_check_mapping(
        &[(4.0, Fermions::try_from((1, 4, 4, 3)).unwrap())],
        &[
            (1.0, Pauli::with_ops([I, X, Z, X, I])),
            (1.0, Pauli::with_ops([I, Y, Z, Y, I])),
            (-1.0, Pauli::with_ops([I, X, Z, X, Z])),
            (-1.0, Pauli::with_ops([I, Y, Z, Y, Z])),
        ],
    );
}

#[test]
fn jw_two_elec_05() {
    jw_check_mapping(
        &[(4.0, Fermions::try_from((0, 32, 32, 1)).unwrap())],
        &[
            (1.0, Pauli::new((0b0101, 0))),
            (1.0, Pauli::new((0b1010, 0))),
            (-1.0, Pauli::new((0b0101, 0b11))),
            (-1.0, Pauli::new((0b1010, 0b11))),
        ],
    );

    jw_check_mapping(
        &[(4.0, Fermions::try_from((0, 63, 63, 2)).unwrap())],
        &[
            (1.0, Pauli::new((0b01_1101, 0))),
            (1.0, Pauli::new((0b10_1110, 0))),
            (-1.0, Pauli::new((0b01_1101, 0b11 << 62))),
            (-1.0, Pauli::new((0b10_1110, 0b11 << 62))),
        ],
    );

    jw_check_mapping(
        &[(4.0, Fermions::try_from((0, 63, 63, 32)).unwrap())],
        &[
            (1.0, Pauli::new((0b01 + (u64::MAX >> 2 << 2), 0b01))),
            (1.0, Pauli::new((0b10 + (u64::MAX >> 2 << 2), 0b10))),
            (
                -1.0,
                Pauli::new((
                    0b01 + (u64::MAX >> 2 << 2),
                    0b01 + (u64::MAX >> 2 << 62),
                )),
            ),
            (
                -1.0,
                Pauli::new((
                    0b10 + (u64::MAX >> 2 << 2),
                    0b10 + (u64::MAX >> 2 << 62),
                )),
            ),
        ],
    );

    jw_check_mapping(
        &[(4.0, Fermions::try_from((32, 63, 63, 34)).unwrap())],
        &[
            (1.0, Pauli::new((0, 0b01_1101))),
            (1.0, Pauli::new((0, 0b10_1110))),
            (-1.0, Pauli::new((0, 0b01_1101 + (0b11 << 62)))),
            (-1.0, Pauli::new((0, 0b10_1110 + (0b11 << 62)))),
        ],
    );
}

#[test]
fn jw_two_elec_06() {
    use Sigma::*;

    jw_check_mapping(
        &[(4.0, Fermions::try_from((0, 1, 2, 1)).unwrap())],
        &[
            (1.0, Pauli::with_ops([X, I, X])),
            (-1.0, Pauli::with_ops([X, Z, X])),
            (1.0, Pauli::with_ops([Y, I, Y])),
            (-1.0, Pauli::with_ops([Y, Z, Y])),
        ],
    );
}
