use std::marker::PhantomData;

use num::Num;
use serde::{
    de::Visitor,
    ser::SerializeSeq,
    Deserialize,
    Serialize,
};

use crate::{
    code::fermions::{
        An,
        Cr,
        Fermions,
        Orbital,
    },
    serialize::Encoding,
    terms::SumRepr,
};

impl Serialize for Fermions {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Fermions::Offset => {
                let seq = serializer.serialize_seq(Some(0))?;
                seq.end()
            }
            Fermions::One {
                cr,
                an,
            } => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(&cr.0.index())?;
                seq.serialize_element(&an.0.index())?;
                seq.end()
            }
            Fermions::Two {
                cr,
                an,
            } => {
                let mut seq = serializer.serialize_seq(Some(4))?;
                seq.serialize_element(&cr.0 .0.index())?;
                seq.serialize_element(&cr.1 .0.index())?;
                seq.serialize_element(&an.0 .0.index())?;
                seq.serialize_element(&an.1 .0.index())?;
                seq.end()
            }
        }
    }
}

struct FermionsVisitor;

impl<'de> Visitor<'de> for FermionsVisitor {
    type Value = Fermions;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str("sequence of 0, 2 or 4 orbital indices")
    }

    fn visit_seq<A>(
        self,
        seq: A,
    ) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        use serde::de::Error;

        let mut seq = seq;
        let idx_tup: (Option<u32>, Option<u32>, Option<u32>, Option<u32>) = (
            seq.next_element()?,
            seq.next_element()?,
            seq.next_element()?,
            seq.next_element()?,
        );

        match idx_tup {
            (None, None, None, None) => Ok(Fermions::Offset),
            (Some(p), Some(q), None, None) => Fermions::one_electron(
                Cr(Orbital::with_index(p)),
                An(Orbital::with_index(q)),
            )
            .ok_or(A::Error::custom("cannot parse one-electron term")),
            (Some(p), Some(q), Some(r), Some(s)) => Fermions::two_electron(
                (Cr(Orbital::with_index(p)), Cr(Orbital::with_index(q))),
                (An(Orbital::with_index(r)), An(Orbital::with_index(s))),
            )
            .ok_or(A::Error::custom("cannot parse two-electron term")),
            _ => Err(A::Error::custom("cannot parse sequence")),
        }
    }
}

impl<'de> Deserialize<'de> for Fermions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(FermionsVisitor)
    }
}

#[derive(Serialize, Deserialize)]
struct FermiSumTerm<T> {
    code:  Fermions,
    value: T,
}

struct FermiSumSerSequence<'a, T>(&'a SumRepr<T, Fermions>);

impl<'a, T> Serialize for FermiSumSerSequence<'a, T>
where
    T: Num + Serialize,
{
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for (coeff, &code) in self.0.iter() {
            seq.serialize_element(&FermiSumTerm {
                code,
                value: coeff,
            })?;
        }

        seq.end()
    }
}

#[derive(Serialize)]
struct FermiSumSer<'a, T>
where
    T: Num,
{
    r#type:   &'a str,
    encoding: Encoding,
    terms:    FermiSumSerSequence<'a, T>,
}

impl<T> Serialize for SumRepr<T, Fermions>
where
    T: Num + Serialize,
{
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (FermiSumSer {
            r#type:   "sumrepr",
            encoding: Encoding::Fermions,
            terms:    FermiSumSerSequence(self),
        })
        .serialize(serializer)
    }
}

struct FermiSumDeSequence<T>(SumRepr<T, Fermions>);

struct FermiSumVisitor<T> {
    _marker: PhantomData<T>,
}

impl<T> FermiSumVisitor<T> {
    fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for FermiSumVisitor<T>
where
    T: Num + Deserialize<'de>,
{
    type Value = FermiSumDeSequence<T>;

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
        let mut repr = SumRepr::new();

        while let Some(FermiSumTerm {
            code,
            value,
        }) = seq.next_element()?
        {
            repr.add_term(code, value);
        }

        Ok(FermiSumDeSequence(repr))
    }
}

impl<'de, T> Deserialize<'de> for FermiSumDeSequence<T>
where
    T: Num + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(FermiSumVisitor::new())
    }
}

#[derive(Deserialize)]
struct FermiSumDe<T>
where
    T: Num,
{
    r#type:   String,
    encoding: Encoding,
    terms:    FermiSumDeSequence<T>,
}

impl<'de, T> Deserialize<'de> for SumRepr<T, Fermions>
where
    T: Num + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let sumde = FermiSumDe::deserialize(deserializer)?;

        if sumde.r#type != "sumrepr" {
            return Err(D::Error::custom("type should be: 'sumrepr'"));
        }

        if sumde.encoding != Encoding::Fermions {
            return Err(D::Error::custom("encoding should be: 'fermions'"));
        }

        Ok(sumde.terms.0)
    }
}
