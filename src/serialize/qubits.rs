use std::marker::PhantomData;

use num::Float;
use serde::{
    de::Visitor,
    ser::SerializeSeq,
    Deserialize,
    Serialize,
};

use crate::{
    codes::qubits::{
        Pauli,
        PauliCode,
        PauliSum,
    },
    serialize::Encoding,
};

impl Serialize for Pauli {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct PauliVisitor;

impl<'de> Visitor<'de> for PauliVisitor {
    type Value = Pauli;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        write!(formatter, "one-character string: I, X, Y, or Z")
    }

    fn visit_char<E>(
        self,
        v: char,
    ) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            'I' => Ok(Pauli::I),
            'X' => Ok(Pauli::X),
            'Y' => Ok(Pauli::Y),
            'Z' => Ok(Pauli::Z),
            _ => Err(E::custom("unknown symbol")),
        }
    }

    fn visit_str<E>(
        self,
        v: &str,
    ) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "I" => Ok(Pauli::I),
            "X" => Ok(Pauli::X),
            "Y" => Ok(Pauli::Y),
            "Z" => Ok(Pauli::Z),
            _ => Err(E::custom("unknown symbol")),
        }
    }
}

impl<'de> Deserialize<'de> for Pauli {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(PauliVisitor)
    }
}

impl Serialize for PauliCode {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct PauliCodeVisitor;

impl<'de> Visitor<'de> for PauliCodeVisitor {
    type Value = PauliCode;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str(
            "string of 64 Pauli operators (trailing identities truncated)",
        )
    }

    fn visit_str<E>(
        self,
        v: &str,
    ) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.len() > 64 || v.is_empty() {
            return Err(E::custom("str len out of range: 1..=64".to_string()));
        }

        let mut code = PauliCode::default();

        for (i, ch) in v.chars().enumerate() {
            let pauli = match ch {
                'I' => Ok(Pauli::I),
                'X' => Ok(Pauli::X),
                'Y' => Ok(Pauli::Y),
                'Z' => Ok(Pauli::Z),
                _ => Err(E::custom(
                    "character must be one of: I, X, Y, Z".to_string(),
                )),
            }?;
            let idx = u16::try_from(i)
                .expect("index out of range for u16. This is a bug.");
            code.set(idx, pauli);
        }

        Ok(code)
    }
}

impl<'de> Deserialize<'de> for PauliCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(PauliCodeVisitor)
    }
}

#[derive(Serialize, Deserialize)]
struct PauliSumTerm<T> {
    code:  PauliCode,
    value: T,
}

struct PauliSumSerSequence<'a, T>(&'a PauliSum<T>);

impl<'a, T> Serialize for PauliSumSerSequence<'a, T>
where
    T: Float + Serialize,
{
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for (&coeff, &code) in self.0 {
            seq.serialize_element(&PauliSumTerm {
                code,
                value: coeff,
            })?;
        }

        seq.end()
    }
}

#[derive(Serialize)]
struct PauliSumSer<'a, T>
where
    T: Float,
{
    r#type:   &'a str,
    encoding: Encoding,
    terms:    PauliSumSerSequence<'a, T>,
}

impl<T> Serialize for PauliSum<T>
where
    T: Float + Serialize,
{
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (PauliSumSer {
            r#type:   "sumrepr",
            encoding: Encoding::Qubits,
            terms:    PauliSumSerSequence(self),
        })
        .serialize(serializer)
    }
}

struct PauliSumDeSequence<T>(PauliSum<T>);

struct PauliSumVisitor<T> {
    _marker: PhantomData<T>,
}

impl<T> PauliSumVisitor<T> {
    fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for PauliSumVisitor<T>
where
    T: Float + Deserialize<'de>,
{
    type Value = PauliSumDeSequence<T>;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        write!(formatter, "sequence of objects with keys: 'code', 'value'")
    }

    fn visit_seq<A>(
        self,
        seq: A,
    ) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut seq = seq;
        let mut repr = PauliSum::new();

        while let Some(PauliSumTerm {
            code,
            value,
        }) = seq.next_element()?
        {
            repr.add_term(code, value);
        }

        Ok(PauliSumDeSequence(repr))
    }
}

impl<'de, T> Deserialize<'de> for PauliSumDeSequence<T>
where
    T: Float + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(PauliSumVisitor::new())
    }
}

#[derive(Deserialize)]
struct PauliSumDe<'a, T>
where
    T: Float,
{
    r#type:   &'a str,
    encoding: Encoding,
    terms:    PauliSumDeSequence<T>,
}

impl<'de, T> Deserialize<'de> for PauliSum<T>
where
    T: Float + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let sumde = PauliSumDe::deserialize(deserializer)?;

        if sumde.r#type != "sumrepr" {
            return Err(D::Error::custom("type should be: 'sumrepr'"));
        }

        if sumde.encoding != Encoding::Qubits {
            return Err(D::Error::custom("encoding should be: 'qubits'"));
        }

        Ok(sumde.terms.0)
    }
}
