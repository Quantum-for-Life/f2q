use std::{
    fs::File,
    io::BufReader,
};

use f2q::{
    qubit::{
        Pauli,
        PauliCode,
    },
    terms::SumRepr,
};

const PAULICODES: &str = "./tests/serialize/paulicodes.json";

fn paulicodes_compare() -> [PauliCode; 8] {
    use Pauli::*;
    [
        PauliCode::from_paulis([]),
        PauliCode::from_paulis([X, X]),
        PauliCode::from_paulis([I, Y]),
        PauliCode::from_paulis([I, X, Y, Z]),
        PauliCode::from_paulis([
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
        ]),
        PauliCode::from_paulis([
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
            X, X, X, X, X, X, X, X, X, X,
        ]),
        PauliCode::from_paulis([
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
            X, X, X, X,
        ]),
        PauliCode::from_paulis([
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
        ]),
    ]
}

#[test]
fn deserialize_paulicodes() {
    // Open the file in read-only mode with buffer.
    let file = File::open(PAULICODES).unwrap();
    let reader = BufReader::new(file);

    let codes: Vec<PauliCode> = serde_json::from_reader(reader).unwrap();
    assert_eq!(codes, paulicodes_compare());
}

#[test]
fn serialize_sumrepr() {
    let mut repr = SumRepr::new();
    for code in paulicodes_compare() {
        repr.add_term(code, 0.1);
    }

    let json = serde_json::to_string(&repr).unwrap();
    let de_repr: SumRepr<f64, PauliCode> = serde_json::from_str(&json).unwrap();

    assert_eq!(de_repr.len(), 8);
}
