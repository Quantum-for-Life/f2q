use std::{
    fs::File,
    io::BufReader,
};

use f2q::code::qubits::{
    Paulis,
    Sigma,
};

#[test]
fn pauli_code_to_string() {
    assert_eq!(Paulis::default().to_string(), "I");
    assert_eq!(Paulis::new((1, 0)).to_string(), "X");
    assert_eq!(Paulis::new((2, 0)).to_string(), "Y");
    assert_eq!(Paulis::new((3, 0)).to_string(), "Z");

    assert_eq!(
        Paulis::new((0, 1)).to_string(),
        "IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIX"
    );
    assert_eq!(
        Paulis::new((0, 2)).to_string(),
        "IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIY"
    );
    assert_eq!(
        Paulis::new((0, 3)).to_string(),
        "IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIZ"
    );

    assert_eq!(
        Paulis::new((u64::MAX, u64::MAX)).to_string(),
        "ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ"
    );
}

#[test]
fn serialize_01() {
    let code = Paulis::default();
    let json = serde_json::to_string(&code).unwrap();

    assert_eq!(json, "\"I\"");

    let code = Paulis::with_ops([Sigma::I, Sigma::X, Sigma::Y, Sigma::Z]);
    let json = serde_json::to_string(&code).unwrap();

    assert_eq!(json, "\"IXYZ\"");
}

#[test]
fn deserialize_01() {
    let data = r#"
              "I" 
     "#;
    let code: Paulis = serde_json::from_str(data).unwrap();
    assert_eq!(code, Paulis::default());

    let data = r#"
              "IXYZ" 
     "#;
    let code: Paulis = serde_json::from_str(data).unwrap();
    assert_eq!(
        code,
        Paulis::with_ops([Sigma::I, Sigma::X, Sigma::Y, Sigma::Z])
    );
}

#[test]
fn deserialize_02() {
    let data = r#"
              "" 
     "#;
    let _ = serde_json::from_str::<Paulis>(data).unwrap_err();

    let data = r#"
              "IP" 
     "#;
    let _ = serde_json::from_str::<Paulis>(data).unwrap_err();

    // this is 65 chars
    let data = r#"
              "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX" 
     "#;
    let _ = serde_json::from_str::<Paulis>(data).unwrap_err();
}

fn check_serde(code: Paulis) {
    let json = serde_json::to_string(&code).unwrap();
    let result: Paulis = serde_json::from_str(&json).unwrap();
    assert_eq!(result, code);
}

#[test]
fn serde_01() {
    use Sigma::{
        I,
        X,
        Y,
        Z,
    };
    check_serde(Paulis::default());
    check_serde(Paulis::with_ops([I, X, Y, Z]));
    check_serde(Paulis::with_ops([X, X, X]));
    check_serde(Paulis::with_ops([
        I, X, X, X, I, Y, Y, Y, I, X, X, X, I, Y, Y, Y, I, X, X, X, I, Y, Y, Y,
        I, X, X, X, I, Y, Y, Y, I, X, X, X, I, Y, Y, Y, I, X, X, X, I, Y, Y, Y,
        I, X, X, X, I, Y, Y, Y, I, X, X, X, I, Y, Y, Y,
    ]));
}

const PAULI_CODES: &str = "./tests/serialize/paulicodes.json";

fn paulis_compare() -> [Paulis; 8] {
    use Sigma::*;
    [
        Paulis::with_ops([]),
        Paulis::with_ops([X, X]),
        Paulis::with_ops([I, Y]),
        Paulis::with_ops([I, X, Y, Z]),
        Paulis::with_ops([X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X]),
        Paulis::with_ops([
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
            X, X, X, X, X, X, X, X, X, X,
        ]),
        Paulis::with_ops([
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
            X, X, X, X,
        ]),
        Paulis::with_ops([
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
        ]),
    ]
}

#[test]
fn deserialize_paulis() {
    // Open the file in read-only mode with buffer.
    let file = File::open(PAULI_CODES).unwrap();
    let reader = BufReader::new(file);

    let codes: Vec<Paulis> = serde_json::from_reader(reader).unwrap();
    assert_eq!(codes, paulis_compare());
}
