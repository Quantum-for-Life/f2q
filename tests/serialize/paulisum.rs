use f2q::{
    code::qubits::{
        Paulis,
        Sigma,
    },
    terms::{
        PauliSum,
        SumRepr,
    },
};
use serde_json::Value;

#[test]
#[allow(clippy::float_cmp)]
fn paulisum_serialize_01() {
    let mut repr = SumRepr::new();

    repr.add_term(Paulis::identity(), 0.1);

    let json = serde_json::to_value(&repr).unwrap();
    let expected: serde_json::Value = serde_json::from_str(
        r#"
        {
            "type": "sumrepr",
            "encoding": "qubits",
            "terms":  [
                {
                    "code": "I",
                    "value": 0.1
                }
            ]
        }
        "#,
    )
    .unwrap();

    assert_eq!(json, expected);
}

#[test]
#[allow(clippy::float_cmp)]
fn pauliisum_serialize_02() {
    let mut repr = SumRepr::new();

    repr.add_term(Paulis::with_ops([Sigma::X, Sigma::Y]), 0.2);
    let json = serde_json::to_value(&repr).unwrap();
    let expected: serde_json::Value = serde_json::from_str(
        r#"
        {
            "type": "sumrepr",
            "encoding": "qubits",
            "terms":  [
                {
                    "code": "XY",
                    "value": 0.2
                }
            ]
        }
        "#,
    )
    .unwrap();

    assert_eq!(json, expected);
}

#[test]
#[allow(clippy::float_cmp)]
fn paulisum_serialize_03() {
    let mut repr = SumRepr::new();

    repr.add_term(
        Paulis::with_ops([Sigma::I, Sigma::X, Sigma::Y, Sigma::Z]),
        0.3,
    );
    let json = serde_json::to_value(&repr).unwrap();
    let expected: serde_json::Value = serde_json::from_str(
        r#"
        {
            "type": "sumrepr",
            "encoding": "qubits",
            "terms":  [
                {
                    "code": "IXYZ",
                    "value": 0.3
                }
            ]
        }
        "#,
    )
    .unwrap();

    assert_eq!(json, expected);
}

#[test]
fn paulisum_serialize_04() {
    let mut repr = SumRepr::new();

    repr.add_term(Paulis::identity(), 0.1);
    repr.add_term(Paulis::with_ops([Sigma::X, Sigma::Y]), 0.2);
    repr.add_term(
        Paulis::with_ops([Sigma::I, Sigma::X, Sigma::Y, Sigma::Z]),
        0.3,
    );
    let json = serde_json::to_value(&repr).unwrap();

    let map = json.as_object().unwrap();

    assert_eq!(
        map.get("encoding").unwrap(),
        &Value::String("qubits".to_string())
    );

    let Value::Array(arr) = map.get("terms").unwrap() else {
        panic!()
    };

    assert_eq!(arr.len(), 3);
}

#[test]
#[allow(clippy::float_cmp)]
fn paulisum_deserialize_01() {
    let data = r#"
        {
            "type": "sumrepr",
            "encoding": "qubits",
            "terms": [
                {
                    "code": "I",
                    "value": 0.1
                }
            ]
        }
    "#;

    let repr: PauliSum = serde_json::from_str(data).unwrap();

    assert_eq!(repr.len(), 1);
    assert_eq!(repr.coeff(Paulis::identity()).unwrap(), &0.1);
}

#[test]
#[allow(clippy::float_cmp)]
fn paulisum_deserialize_02() {
    let data = r#"
        {
            "type": "sumrepr",
            "encoding": "qubits",
            "terms": [
                {
                    "code": "I",
                    "value": 0.1
                },
                {
                    "code": "XY",
                    "value": 0.2
                }
            ]
        }
    "#;

    let repr: PauliSum = serde_json::from_str(data).unwrap();

    assert_eq!(repr.len(), 2);
    assert_eq!(repr.coeff(Paulis::identity()).unwrap(), &0.1);
    assert_eq!(
        repr.coeff(Paulis::with_ops([Sigma::X, Sigma::Y])).unwrap(),
        &0.2
    );
}

#[test]
#[allow(clippy::float_cmp)]
fn pauliisum_deserialize_03() {
    let data = r#"
        {
            "type": "sumrepr",
            "encoding": "qubits",
            "terms": [
                {
                    "code": "I",
                    "value": 0.1
                },
                {
                    "value": 0.09,
                    "code": "I"
                },
                {
                    "code": "XY",
                    "value": 0.2
                }, 
                {
                    "code": "IXYZ",
                    "value": 0.3
                }
            ]
        }
    "#;

    let repr: PauliSum = serde_json::from_str(data).unwrap();

    assert_eq!(repr.len(), 3);
    assert_eq!(repr.coeff(Paulis::identity()).unwrap(), &0.19);
    assert_eq!(
        repr.coeff(Paulis::with_ops([Sigma::X, Sigma::Y])).unwrap(),
        &0.2
    );
    assert_eq!(
        repr.coeff(Paulis::with_ops([Sigma::I, Sigma::X, Sigma::Y, Sigma::Z]),)
            .unwrap(),
        &0.3
    );
}
