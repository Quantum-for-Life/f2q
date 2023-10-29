use f2q::code::qubits::Sigma;

#[test]
fn display() {
    assert_eq!(Sigma::I.to_string(), "I");
    assert_eq!(Sigma::X.to_string(), "X");
    assert_eq!(Sigma::Y.to_string(), "Y");
    assert_eq!(Sigma::Z.to_string(), "Z");
}

#[test]
fn pauli_serialize_01() {
    assert_eq!(
        serde_json::to_value(Sigma::I).unwrap().as_str().unwrap(),
        "I"
    );

    assert_eq!(
        serde_json::to_value(Sigma::X).unwrap().as_str().unwrap(),
        "X"
    );

    assert_eq!(
        serde_json::to_value(Sigma::Y).unwrap().as_str().unwrap(),
        "Y"
    );

    assert_eq!(
        serde_json::to_value(Sigma::Z).unwrap().as_str().unwrap(),
        "Z"
    );
}

#[test]
fn pauli_deserialize_01() {
    assert_eq!(
        serde_json::from_str::<Sigma>("\"I\"").unwrap(),
        Sigma::I
    );
    assert_eq!(
        serde_json::from_str::<Sigma>("\"X\"").unwrap(),
        Sigma::X
    );
    assert_eq!(
        serde_json::from_str::<Sigma>("\"Y\"").unwrap(),
        Sigma::Y
    );
    assert_eq!(
        serde_json::from_str::<Sigma>("\"Z\"").unwrap(),
        Sigma::Z
    );
}
